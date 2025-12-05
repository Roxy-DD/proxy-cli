mod config;
mod installer;
mod ui;

use config::ConfigManager;
use ui::{main_menu, prompt_port, show_status, Action};

fn main() {
    // Check for installation first
    installer::check_and_install();

    let config_manager = ConfigManager::new();
    let mut config = config_manager.load();

    loop {
        show_status(&config);
        match main_menu(&config) {
            Action::Toggle(enabled) => {
                config.enabled = enabled;
                config_manager.save(&config).expect("Failed to save config");
            }
            Action::ChangePort => {
                let new_port = prompt_port(config.port);
                config.port = new_port;
                config_manager.save(&config).expect("Failed to save config");
            }
            Action::Exit => break,
        }
    }

    // Output for the shell wrapper
    if config.enabled {
        println!("#SET_PROXY:http://127.0.0.1:{}", config.port);
    } else {
        println!("#CLEAR_PROXY");
    }
}
