use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
    cursor::MoveTo,
    terminal::ClearType,
};
use std::io::{self, Write};

/// 消息类型
#[allow(dead_code)]
pub enum MessageType {
    Success,
    Error,
    Warning,
    Info,
}

/// 在交互式界面中显示消息
pub fn show_message(msg_type: MessageType, message: &str, height: u16) -> io::Result<()> {
    let mut stdout = io::stdout();
    let msg_y = height - 2;
    
    stdout.execute(MoveTo(0, msg_y))?;
    stdout.execute(crossterm::terminal::Clear(ClearType::FromCursorDown))?;
    
    let (icon, color) = match msg_type {
        MessageType::Success => ("✅", Color::Green),
        MessageType::Error => ("❌", Color::Red),
        MessageType::Warning => ("⚠️", Color::Yellow),
        MessageType::Info => ("ℹ️", Color::Cyan),
    };
    
    stdout
        .execute(SetForegroundColor(color))?
        .execute(Print(format!("  {} {}\n", icon, message)))?
        .execute(ResetColor)?
        .flush()?;
    
    Ok(())
}

/// 清除消息区域
#[allow(dead_code)]
pub fn clear_message(height: u16) -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(MoveTo(0, height - 2))?;
    stdout.execute(crossterm::terminal::Clear(ClearType::FromCursorDown))?;
    stdout.flush()?;
    Ok(())
}

