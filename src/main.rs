use clap::Parser;
use thiserror::Error;
use std::io::{self, Write};
// 导入必要的依赖
use crossterm::{
    event::KeyCode,
    ExecutableCommand,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen}
};



mod config;
mod proxy;
mod ui;
mod message;

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

#[derive(Parser, Debug)]
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
    Interactive,
}

fn main() -> AppResult<()> {
    let mut config = config::load_config()?;
    
    // 确保程序启动时不会自动启用代理，无论配置文件中的enabled字段是什么值
    config.enabled = false;
    config::save_config(&config)?;
    
    // 清空事件队列，避免程序启动时受到残留键盘事件的影响
    let _ = ui::clear_event_queue();
    
    // 解析命令行参数，如果没有提供子命令，则默认使用Interactive
    let args = match Command::try_parse() {
        Ok(args) => args,
        Err(e) => {
            // 如果是因为缺少子命令而解析失败，则使用默认的Interactive命令
            if e.kind() == clap::error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand {
                Command::Interactive
            } else {
                e.exit();
            }
        }
    };

    println!("Command: {:?}", args);
    
    match args {
        Command::Enable => enable_proxy(&mut config)?,
        Command::Disable => disable_proxy(&mut config)?,
        Command::SetPort { port } => set_port(&mut config, port)?,
        Command::Status => show_status(&config)?,
        Command::Interactive => {
            println!("Entering interactive mode...");
            run_interactive(&mut config)?
        },
    }

    Ok(())
}

fn enable_proxy(config: &mut config::Config) -> AppResult<()> {
    match config.port {
        Some(port) => {
            proxy::enable_proxy(port)?;
            config.enabled = true;
            config::save_config(config)?;
            println!("\n✅ 代理已启用 (HTTP/HTTPS: http://127.0.0.1:{port})");
        }
        None => {
            return Err(AppError::InvalidInput(
                "请先设置有效的端口！".to_string(),
            ));
        }
    }
    Ok(())
}

fn disable_proxy(config: &mut config::Config) -> AppResult<()> {
    proxy::disable_proxy()?;
    config.enabled = false;
    config::save_config(config)?;
    println!("\n✅ 代理已禁用");
    Ok(())
}

fn set_port(config: &mut config::Config, port_input: Option<String>) -> AppResult<()> {
    // 如果提供了端口输入（命令行模式），直接使用
    // 否则（交互模式），调用 input_port 获取用户输入
    let input = match port_input {
        Some(input) => Some(input),
        None => {
            // 交互模式：获取用户输入
            let current_port = config.port;
            ui::input_port(current_port)?
        }
    };

    // 处理输入结果
    match input {
        None => {
            // 用户输入为空或取消：清空端口
            config.port = None;
            if config.enabled {
                disable_proxy(config)?;
            } else {
                config::save_config(config)?;
            }
            Ok(())
        }
        Some(input_str) => {
            // 验证并设置端口
            let port = input_str
                .trim()
                .parse::<u32>()
                .map_err(|_| AppError::InvalidInput(format!("无效的端口: {} (必须是数字)", input_str)))?;
            
            let port = config::validate_port(port)
                .map_err(|e| AppError::InvalidInput(format!("{}", e)))?;

            config.port = Some(port);
            config::save_config(config)?;

            // 若已启用代理，同步更新环境变量
            if config.enabled {
                proxy::enable_proxy(port)?;
            }

            Ok(())
        }
    }
}

fn show_status(config: &config::Config) -> AppResult<()> {
    let (proxy_enabled, _) = proxy::get_current_proxy();
    let http_proxy = std::env::var("http_proxy")
        .or_else(|_| std::env::var("HTTP_PROXY"))
        .ok();
    let https_proxy = std::env::var("https_proxy")
        .or_else(|_| std::env::var("HTTPS_PROXY"))
        .ok();
    
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║                    🚀 Session Proxy Manager                  ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
    println!("状态:        {}", if proxy_enabled { "● 已启用" } else { "○ 已禁用" });
    println!("保存的端口:  {}", config.port.map(|p| p.to_string()).unwrap_or_else(|| "未设置".to_string()));
    println!("HTTP 代理:   {}", http_proxy.as_ref().map(|s| s.as_str()).unwrap_or("未设置"));
    println!("HTTPS 代理:  {}", https_proxy.as_ref().map(|s| s.as_str()).unwrap_or("未设置"));
    println!("\n配置文件:    {}", config::get_config_path().to_string_lossy());
    println!("═══════════════════════════════════════════════════════════════\n");
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
                    ui::MenuItem::EnableProxy => {
                        if config.port.is_some() {
                            match enable_proxy(config) {
                                Ok(()) => {
                                    let (_, height) = crossterm::terminal::size()?;
                                    message::show_message(
                                        message::MessageType::Success,
                                        &format!("代理已启用 (端口: {})", config.port.unwrap()),
                                        height,
                                    )?;
                                    std::thread::sleep(std::time::Duration::from_millis(800));
                                    ui::render_ui(config, selected_idx)?;
                                }
                                Err(e) => {
                                    let (_, height) = crossterm::terminal::size()?;
                                    message::show_message(
                                        message::MessageType::Error,
                                        &format!("错误: {}", e),
                                        height,
                                    )?;
                                    std::thread::sleep(std::time::Duration::from_millis(1500));
                                    ui::render_ui(config, selected_idx)?;
                                }
                            }
                        } else {
                            let (_, height) = crossterm::terminal::size()?;
                            message::show_message(
                                message::MessageType::Warning,
                                "请先设置有效的端口！",
                                height,
                            )?;
                            std::thread::sleep(std::time::Duration::from_millis(1000));
                            ui::render_ui(config, selected_idx)?;
                        }
                    },
                    ui::MenuItem::DisableProxy => {
                        match disable_proxy(config) {
                            Ok(()) => {
                                let (_, height) = crossterm::terminal::size()?;
                                message::show_message(
                                    message::MessageType::Success,
                                    "代理已禁用",
                                    height,
                                )?;
                                std::thread::sleep(std::time::Duration::from_millis(800));
                                ui::render_ui(config, selected_idx)?;
                            }
                            Err(e) => {
                                let (_, height) = crossterm::terminal::size()?;
                                message::show_message(
                                    message::MessageType::Error,
                                    &format!("错误: {}", e),
                                    height,
                                )?;
                                std::thread::sleep(std::time::Duration::from_millis(1500));
                                ui::render_ui(config, selected_idx)?;
                            }
                        }
                    },
                    ui::MenuItem::SetPort => {
                        // 退出 alternate screen
                        stdout.execute(LeaveAlternateScreen)?;
                        stdout.flush()?;
                        
                        // 增加延迟，确保终端状态稳定
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        
                        // ⚠️ 关键修复：在退出 alternate screen 后，清空事件队列
                        // 避免之前残留的键盘事件干扰输入
                        let _ = ui::clear_event_queue();
                        
                        // 清除屏幕，准备输入（清除所有内容并移动光标到左上角）
                        print!("\x1B[2J\x1B[H");
                        io::stdout().flush()?;
                        
                        // 保存旧端口用于判断是否改变
                        let old_port = config.port;
                        
                        // 执行端口设置（input_port 内部会处理输入和清除）
                        let result = set_port(config, None);
                        
                        // 确保清除所有输出（清除屏幕）
                        print!("\x1B[2J\x1B[H");
                        io::stdout().flush()?;
                        
                        // 增加延迟，确保输入操作完全完成
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        
                        // ⚠️ 关键修复：输入完成后，再次清空事件队列
                        // 确保用户按回车确认输入时，这个回车键事件不会触发主循环
                        let _ = ui::clear_event_queue();
                        
                        // 重新进入 alternate screen
                        stdout.execute(EnterAlternateScreen)?;
                        stdout.flush()?;
                        
                        // 增加延迟，确保终端状态稳定
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        
                        // 最后一次清空事件队列，确保所有残留事件都被清除
                        let _ = ui::clear_event_queue();
                        
                        // 显示结果消息
                        let (_, height) = crossterm::terminal::size()?;
                        match result {
                            Ok(()) => {
                                // 根据端口变化显示消息
                                match (old_port, config.port) {
                                    (Some(old), Some(new)) if old == new => {
                                        // 端口未改变，不显示消息
                                    }
                                    (_, None) => {
                                        message::show_message(
                                            message::MessageType::Warning,
                                            "端口已清除",
                                            height,
                                        )?;
                                        std::thread::sleep(std::time::Duration::from_millis(800));
                                    }
                                    (_, Some(port)) => {
                                        message::show_message(
                                            message::MessageType::Success,
                                            &format!("端口已设置为: {}", port),
                                            height,
                                        )?;
                                        std::thread::sleep(std::time::Duration::from_millis(800));
                                    }
                                }
                                ui::render_ui(config, selected_idx)?;
                            }
                            Err(e) => {
                                message::show_message(
                                    message::MessageType::Error,
                                    &format!("错误: {}", e),
                                    height,
                                )?;
                                std::thread::sleep(std::time::Duration::from_millis(1500));
                                ui::render_ui(config, selected_idx)?;
                            }
                        }
                    },
                    ui::MenuItem::Exit => break,
                }
            }
            // Q/Esc：退出
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => break,
            _ => {}
        }
    }

    // 恢复终端状态
    stdout.execute(LeaveAlternateScreen)?;
    println!("\nℹ️  代理管理器已退出（代理设置在当前会话中保持有效）");
    Ok(())
}