use clap::Parser;
use thiserror::Error;
// 导入必要的依赖
use crossterm::{
    event::KeyCode,
    ExecutableCommand,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen}
};

mod config;
mod proxy;
mod ui;

#[derive(Debug, Error)]
enum AppError {
    #[error("Config error: {0}")]
    Config(#[from] config::ConfigError),
    #[error("Proxy error: {0}")]
    Proxy(#[from] proxy::ProxyError),
    #[error("UI error: {0}")]
    Ui(#[from] std::io::Error),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

type AppResult<T> = Result<T, AppError>;

// 修复 clap default 语法
#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
enum Command {
    /// Enable proxy with saved port
    Enable,
    /// Disable proxy
    Disable,
    /// Set proxy port (empty = clear)
    SetPort {
        /// Port number (1-65535)
        port: Option<String>,
    },
    /// Show current proxy status
    Status,
    /// Start interactive menu (default)
    #[default]
    Interactive,
}

fn main() -> AppResult<()> {
    let mut config = config::load_config()?;
    
    // 检查是否提供了命令，如果没有则使用默认的Interactive
    let args = match std::env::args().len() {
        1 => Command::Interactive,
        _ => Command::parse(),
    };

    match args {
        Command::Enable => enable_proxy(&mut config)?,
        Command::Disable => disable_proxy(&mut config)?,
        Command::SetPort { port } => set_port(&mut config, port)?,
        Command::Status => show_status(&config)?,
        Command::Interactive => run_interactive(&mut config)?,
    }

    Ok(())
}

fn enable_proxy(config: &mut config::Config) -> AppResult<()> {
    match config.port {
        Some(port) => {
            proxy::enable_proxy(port)?;
            config.enabled = true;
            config::save_config(config)?;
            println!("\n✅ Session proxy enabled (HTTP/HTTPS: http://127.0.0.1:{port})");
        }
        None => {
            return Err(AppError::InvalidInput(
                "Please set a valid port first!".to_string(),
            ));
        }
    }
    Ok(())
}

fn disable_proxy(config: &mut config::Config) -> AppResult<()> {
    proxy::disable_proxy()?;
    config.enabled = false;
    config::save_config(config)?;
    println!("\n✅ Session proxy disabled");
    Ok(())
}

fn set_port(config: &mut config::Config, port_input: Option<String>) -> AppResult<()> {
    let input = match port_input {
        Some(input) => input,
        None => {
            let current_port = config.port;
            match ui::input_port(current_port)? {
                Some(input) => input,
                None => {
                    // 清空端口
                    config.port = None;
                    if config.enabled {
                        disable_proxy(config)?;
                    } else {
                        config::save_config(config)?;
                    }
                    println!("\n✅ Proxy port cleared");
                    return Ok(());
                }
            }
        }
    };

    // 验证端口
    let port = input
        .parse::<u32>()
        .map_err(|_| AppError::InvalidInput(format!("Invalid port: {input} (must be a number)")))?;
    let port = config::validate_port(port)?;

    config.port = Some(port);
    config::save_config(config)?;
    println!("\n✅ Proxy port set to: {port}");

    // 若已启用代理，同步更新环境变量
    if config.enabled {
        proxy::enable_proxy(port)?;
        println!("✅ Proxy environment variables updated");
    }

    Ok(())
}

fn show_status(config: &config::Config) -> AppResult<()> {
    let (http_proxy, https_proxy) = proxy::get_current_proxy();
    println!("\n====================== Session Proxy Manager ======================");
    println!("\nStatus:    {}", if config.enabled { "Enabled (Green)" } else { "Disabled (Red)" });
    println!("Saved Port:    {}", config.port.map(|p| p.to_string()).unwrap_or_else(|| "(none)".to_string()));
    println!("HTTP Proxy:    {}", http_proxy.unwrap_or_else(|| "(not set)".to_string()));
    println!("HTTPS Proxy:   {}", https_proxy.unwrap_or_else(|| "(not set)".to_string()));
    println!("\nConfig File:   {}", config::get_config_path().to_string_lossy());
    println!("===================================================================\n");
    Ok(())
}

fn run_interactive(config: &mut config::Config) -> AppResult<()> {
    let mut stdout = std::io::stdout();
    // 修复 EnterAlternateScreen 调用
    stdout.execute(EnterAlternateScreen)?;
    let mut selected_idx = 0;
    let menu_items = ui::MenuItem::all();

    loop {
        // 渲染UI
        ui::render_ui(config, selected_idx)?;

        // 读取键盘输入
        let key = ui::read_key()?;
        match key.code {
            // 上箭头：选中上一项
            KeyCode::Up => {
                if selected_idx > 0 {
                    selected_idx -= 1;
                }
            }
            // 下箭头：选中下一项
            KeyCode::Down => {
                if selected_idx < menu_items.len() - 1 {
                    selected_idx += 1;
                }
            }
            // 回车键：执行选中项
            KeyCode::Enter => {
                match menu_items[selected_idx] {
                    ui::MenuItem::EnableProxy => enable_proxy(config)?,
                    ui::MenuItem::DisableProxy => disable_proxy(config)?,
                    ui::MenuItem::SetPort => set_port(config, None)?,
                    ui::MenuItem::Exit => break,
                }
                // 执行后暂停0.5秒，让用户看到反馈
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            // Q/Esc：退出
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => break,
            _ => {}
        }
    }

    // 恢复终端状态
    stdout.execute(LeaveAlternateScreen)?;
    println!("\nℹ️  Proxy Manager exited (proxy settings persist for this session)");
    Ok(())
}