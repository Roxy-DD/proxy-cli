use crate::config::AppConfig;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Input, Select};

pub enum Action {
    Toggle(bool), // true = enable, false = disable
    ChangePort,
    Exit,
}

pub fn show_status(config: &AppConfig) {
    let term = Term::stderr();
    term.clear_screen().unwrap();

    eprintln!("Proxy CLI Tool");
    eprintln!("----------------");
    if config.enabled {
        eprintln!("Status: \x1b[32mEnabled\x1b[0m"); // Green
        eprintln!("HTTP:   http://127.0.0.1:{}", config.port);
        eprintln!("HTTPS:  http://127.0.0.1:{}", config.port);
    } else {
        eprintln!("Status: \x1b[31mDisabled\x1b[0m"); // Red
        eprintln!("Port:   {}", config.port);
    }
    eprintln!();
}

pub fn main_menu(config: &AppConfig) -> Action {
    let items = if config.enabled {
        vec!["Disable Proxy", "Change Port", "Exit"]
    } else {
        vec!["Enable Proxy", "Change Port", "Exit"]
    };

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an action")
        .default(0)
        .items(&items)
        .interact_on(&Term::stderr())
        .unwrap();

    match selection {
        0 => Action::Toggle(!config.enabled),
        1 => Action::ChangePort,
        _ => Action::Exit,
    }
}

pub fn prompt_port(current_port: u16) -> u16 {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter new proxy port")
        .default(current_port)
        .interact_text()
        .unwrap()
}
