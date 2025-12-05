use crossterm::{
    event::{self, Event, KeyEvent},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode},
    ExecutableCommand,
};
use std::io::{self, Write};
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
            MenuItem::EnableProxy => "Enable Proxy",
            MenuItem::DisableProxy => "Disable Proxy",
            MenuItem::SetPort => "Set Port",
            MenuItem::Exit => "Exit",
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
    stdout.execute(crossterm::terminal::Clear(crossterm::terminal::ClearType::All))?;

    // 渲染标题
    stdout
        .execute(SetForegroundColor(Color::Cyan))?
        .execute(Print("====================== Session Proxy Manager ======================\n\n"))?
        .execute(ResetColor)?;

    // 渲染状态信息
    let (http_proxy, https_proxy) = get_current_proxy();
    let enabled_str = if config.enabled { "Enabled" } else { "Disabled" };
    let enabled_color = if config.enabled { Color::Green } else { Color::Red };
    let port_str = config.port.map(|p| p.to_string()).unwrap_or_else(|| "(none)".to_string());
    let http_str = http_proxy.unwrap_or_else(|| "(not set)".to_string());
    let https_str = https_proxy.unwrap_or_else(|| "(not set)".to_string());

    stdout
        .execute(Print("Status: "))?
        .execute(SetForegroundColor(enabled_color))?
        .execute(Print(format!("{enabled_str}\n")))?
        .execute(ResetColor)?
        .execute(Print(format!("Saved Port:    {port_str}\n")))?
        .execute(Print(format!("HTTP Proxy:    {http_str}\n")))?
        .execute(Print(format!("HTTPS Proxy:   {https_str}\n\n")))?;

    // 渲染配置文件路径（修复 DarkGray → DarkGrey）
    let config_path_buf = crate::config::get_config_path();
    let config_path = config_path_buf.to_string_lossy();
    stdout
        .execute(SetForegroundColor(Color::DarkGrey))?
        .execute(Print(format!("Config File:   {config_path}\n")))?
        .execute(ResetColor)?;

    // 分隔线
    stdout.execute(Print("===================================================================\n\n"))?;

    // 渲染菜单标题
    stdout
        .execute(SetForegroundColor(Color::Cyan))?
        .execute(Print("Menu Options:\n\n"))?
        .execute(ResetColor)?;

    // 渲染菜单选项
    let menu_items = MenuItem::all();
    for (idx, item) in menu_items.iter().enumerate() {
        if idx == selected_idx {
            stdout
                .execute(SetForegroundColor(Color::Green))?
                .execute(Print(format!("  [>] {}\n", item.as_str())))?
                .execute(ResetColor)?;
        } else {
            stdout.execute(Print(format!("  [ ] {}\n", item.as_str())))?;
        }
    }

    // 操作提示（修复 DarkGray → DarkGrey）
    stdout
        .execute(SetForegroundColor(Color::DarkGrey))?
        .execute(Print("\nNavigation: Arrow Keys | Select: Enter | Quit: Q/Esc\n"))?
        .execute(ResetColor)?;

    stdout.flush()?;
    Ok(())
}

pub fn read_key() -> io::Result<KeyEvent> {
    enable_raw_mode()?;
    let event = loop {
        if let Event::Key(key) = event::read()? {
            break key;
        }
    };
    disable_raw_mode()?;
    Ok(event)
}

pub fn input_port(current_port: Option<u16>) -> io::Result<Option<String>> {
    let mut stdout = io::stdout();
    let current_str = current_port.map(|p| p.to_string()).unwrap_or_else(|| "none".to_string());
    stdout
        .execute(Print(format!("\nEnter port (empty = clear, current: {current_str}): ")))?
        .flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();
    Ok(if input.is_empty() { None } else { Some(input) })
}