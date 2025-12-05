use crossterm::{
    event::{self, Event, KeyCode},
    style::{Color, Print, ResetColor, SetForegroundColor, SetBackgroundColor},
    cursor::MoveTo,
    ExecutableCommand,
    terminal::{self, ClearType},
};
use std::{
    env,
    io::{self, Write},
    time::{Duration, Instant},
};
use crate::config::Config;
use crate::proxy::get_current_proxy;

pub enum MenuItem {
    EnableProxy,
    DisableProxy,
    SetPort,
    Exit,
}

impl MenuItem {
    pub fn as_str(&self) -> &str {
        match self {
            MenuItem::EnableProxy => "启用代理",
            MenuItem::DisableProxy => "禁用代理",
            MenuItem::SetPort => "设置端口",
            MenuItem::Exit => "退出",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            MenuItem::EnableProxy,
            MenuItem::DisableProxy,
            MenuItem::SetPort,
            MenuItem::Exit,
        ]
    }
}

pub fn render_ui(config: &Config, selected_idx: usize) -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(ClearType::All))?;
    stdout.execute(MoveTo(0, 0))?;

    // 渲染精美的标题（类似 Vue CLI 风格）
    stdout
        .execute(SetForegroundColor(Color::Cyan))?
        .execute(Print("  ╔═══════════════════════════════════════════════════════════════╗\n"))?
        .execute(Print("  ║                                                               ║\n"))?
        .execute(Print("  ║"))?
        .execute(SetForegroundColor(Color::Green))?
        .execute(Print("          🚀 Session Proxy Manager"))?
        .execute(SetForegroundColor(Color::Cyan))?
        .execute(Print("                          ║\n"))?
        .execute(Print("  ║                                                               ║\n"))?
        .execute(Print("  ╚═══════════════════════════════════════════════════════════════╝\n\n"))?
        .execute(ResetColor)?;

    // 渲染状态信息卡片（从环境变量读取实际状态）
    let (proxy_enabled, _) = get_current_proxy();
    let enabled_str = if proxy_enabled { "● 已启用" } else { "○ 已禁用" };
    let enabled_color = if proxy_enabled { Color::Green } else { Color::Red };
    let port_str = config.port.map(|p| p.to_string()).unwrap_or_else(|| "未设置".to_string());
    
    // 同时显示 HTTP 和 HTTPS 代理（从环境变量读取）
    let (http_proxy, https_proxy) = (
        env::var("http_proxy").or_else(|_| env::var("HTTP_PROXY")).ok(),
        env::var("https_proxy").or_else(|_| env::var("HTTPS_PROXY")).ok(),
    );

    stdout
        .execute(SetForegroundColor(Color::Yellow))?
        .execute(Print("  📊 状态信息\n"))?
        .execute(ResetColor)?
        .execute(Print("  ┌─────────────────────────────────────────────────────────────┐\n"))?
        .execute(Print("  │ "))?
        .execute(SetForegroundColor(Color::White))?
        .execute(Print("状态:     "))?
        .execute(SetForegroundColor(enabled_color))?
        .execute(Print(format!("{enabled_str:20}", enabled_str = enabled_str)))?
        .execute(SetForegroundColor(Color::White))?
        .execute(Print("                          │\n"))?
        .execute(Print("  │ "))?
        .execute(Print(format!("端口:     {port_str:20}", port_str = port_str)))?
        .execute(Print("                          │\n"))?
        .execute(Print("  │ "))?
        .execute(Print(format!("HTTP:     {http_str:20}", http_str = http_proxy.as_ref().map(|s| s.as_str()).unwrap_or("未设置"))))?
        .execute(Print("                          │\n"))?
        .execute(Print("  │ "))?
        .execute(Print(format!("HTTPS:    {https_str:20}", https_str = https_proxy.as_ref().map(|s| s.as_str()).unwrap_or("未设置"))))?
        .execute(Print("                          │\n"))?
        .execute(Print("  └─────────────────────────────────────────────────────────────┘\n\n"))?
        .execute(ResetColor)?;

    // 渲染菜单标题
    stdout
        .execute(SetForegroundColor(Color::Cyan))?
        .execute(Print("  🎯 菜单选项\n\n"))?
        .execute(ResetColor)?;

    // 渲染菜单选项（更美观的样式）
    let menu_items = MenuItem::all();
    for (idx, item) in menu_items.iter().enumerate() {
        let is_enabled_option = match item {
            MenuItem::EnableProxy => config.port.is_some(),
            _ => true,
        };
        
        let item_str = item.as_str();
        let icon = match item {
            MenuItem::EnableProxy => "▶",
            MenuItem::DisableProxy => "⏸",
            MenuItem::SetPort => "⚙",
            MenuItem::Exit => "🚪",
        };
        
        if idx == selected_idx {
            // 选中项：高亮显示
            stdout
                .execute(SetForegroundColor(Color::Black))?
                .execute(SetBackgroundColor(if is_enabled_option { Color::Cyan } else { Color::DarkGrey }))?
                .execute(Print(format!("  {} {} {}\n", icon, item_str, icon)))?
                .execute(ResetColor)?;
        } else {
            // 未选中项
            stdout
                .execute(SetForegroundColor(if is_enabled_option { Color::White } else { Color::DarkGrey }))?
                .execute(Print(format!("    {} {}\n", icon, item_str)))?
                .execute(ResetColor)?;
        }
    }

    // 操作提示（更友好的样式）
    stdout
        .execute(Print("\n"))?
        .execute(SetForegroundColor(Color::DarkGrey))?
        .execute(Print("  💡 提示: "))?
        .execute(SetForegroundColor(Color::White))?
        .execute(Print("方向键"))?
        .execute(SetForegroundColor(Color::DarkGrey))?
        .execute(Print(" 导航 | "))?
        .execute(SetForegroundColor(Color::White))?
        .execute(Print("Enter"))?
        .execute(SetForegroundColor(Color::DarkGrey))?
        .execute(Print(" 选择 | "))?
        .execute(SetForegroundColor(Color::White))?
        .execute(Print("Q/Esc"))?
        .execute(SetForegroundColor(Color::DarkGrey))?
        .execute(Print(" 退出\n"))?
        .execute(ResetColor)?;

    stdout.flush()?;
    Ok(())
}

// 读取键盘输入 - 实现防抖机制
pub fn read_key() -> io::Result<event::KeyEvent> {
    // 等待第一个键事件
    let mut key = loop {
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                break key;
            }
        }
    };
    
    // 对于上下键，检查是否有连续的相同键事件
    if matches!(key.code, KeyCode::Up | KeyCode::Down) {
        // 等待一小段时间，看看是否有连续的相同键事件
        let debounce_delay = Duration::from_millis(100);
        let start_time = Instant::now();
        
        // 在防抖时间内，如果有相同的键事件，忽略它们
        while start_time.elapsed() < debounce_delay {
            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(new_key) = event::read()? {
                    // 如果是不同的键，返回新键
                    if new_key.code != key.code {
                        key = new_key;
                        break;
                    }
                }
            }
        }
    }
    
    Ok(key)
}

pub fn input_port(current_port: Option<u16>) -> io::Result<Option<String>> {
    // 参照 PowerShell 版本的简单输入方式
    let current_str = current_port.map(|p| p.to_string()).unwrap_or_else(|| "未设置".to_string());
    
    // 显示输入提示
    print!("设置代理端口 (当前: {}) › ", current_str);
    io::stdout().flush()?;
    
    // 读取用户输入
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            // 清除输入行（回到行首，清除整行）
            print!("\r\x1B[K"); // \r 回到行首，\x1B[K 清除到行尾
            io::stdout().flush()?;
            
            // ⚠️ 关键修复：清空 crossterm 事件队列，避免回车键事件被主循环捕获
            // 在退出 alternate screen 后，crossterm 事件系统可能还在运行
            // 用户按回车确认输入时，这个回车键事件可能残留在事件队列中
            // 需要在输入完成后清空事件队列
            let _ = clear_event_queue();
            
            let trimmed = input.trim().to_string();
            // 空输入表示清除端口
            Ok(if trimmed.is_empty() { None } else { Some(trimmed) })
        }
        Err(e) => {
            // 出错时也清除输入行和事件队列
            print!("\r\x1B[K");
            io::stdout().flush()?;
            let _ = clear_event_queue();
            Err(e)
        }
    }
}

/// 清空 crossterm 事件队列，避免残留的键盘事件被主循环捕获
pub fn clear_event_queue() -> io::Result<()> {
    // 先短暂延迟，确保所有事件都已进入队列
    std::thread::sleep(std::time::Duration::from_millis(50));
    
    // 非阻塞地读取并丢弃所有待处理的事件
    // 最多清空 100 个事件，避免无限循环
    let mut count = 0;
    while count < 100 {
        if event::poll(Duration::from_millis(0))? {
            // 读取并丢弃事件
            let _ = event::read();
            count += 1;
        } else {
            // 没有更多事件了
            break;
        }
    }
    
    // 再次延迟，确保系统处理完所有事件
    std::thread::sleep(std::time::Duration::from_millis(50));
    
    Ok(())
}