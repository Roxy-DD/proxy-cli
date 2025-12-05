use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Confirm};
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn check_and_install() {
    // 1. Get the current executable path
    let current_exe = env::current_exe().expect("Failed to get current executable path");
    let current_exe_str = current_exe
        .to_str()
        .expect("Invalid path")
        .replace("\\", "\\\\");

    // 2. Locate PowerShell profile
    // We try to find the standard Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1
    // or Documents\PowerShell\Microsoft.PowerShell_profile.ps1
    let profile_path = find_powershell_profile();

    if let Some(path) = profile_path {
        if is_installed(&path) {
            return;
        }

        // Not installed, prompt user
        let term = Term::stderr();
        term.clear_screen().unwrap();
        eprintln!(
            "{}",
            style("Proxy CLI is not configured in your PowerShell profile.").yellow()
        );
        eprintln!(
            "Current executable: {}",
            style(current_exe.display()).cyan()
        );
        eprintln!("Profile path: {}", style(path.display()).cyan());
        eprintln!();

        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to install the 'proxy' command to your profile?")
            .default(true)
            .interact_on(&term)
            .unwrap_or(false)
        {
            install_to_profile(&path, &current_exe_str);
            eprintln!();
            eprintln!("{}", style("Installation Complete!").green());
            eprintln!("Please restart your PowerShell terminal or run:");
            eprintln!("{}", style(format!(". \"{}\"", path.display())).cyan());
            eprintln!();
            eprintln!("Press any key to continue...");
            term.read_key().unwrap();
        }
    } else {
        eprintln!("{}", style("Could not locate PowerShell profile.").red());
    }
}

fn find_powershell_profile() -> Option<PathBuf> {
    // Try to get from env var first (most reliable if running in PS)
    if let Ok(profile) = env::var("PROFILE") {
        let path = PathBuf::from(profile);
        // If file doesn't exist, we might still want to use this path if parent exists
        return Some(path);
    }

    // Fallback to standard locations
    if let Some(docs) =
        directories::UserDirs::new().and_then(|ud| ud.document_dir().map(|p| p.to_path_buf()))
    {
        let ps5 = docs
            .join("WindowsPowerShell")
            .join("Microsoft.PowerShell_profile.ps1");
        if ps5.parent()?.exists() {
            return Some(ps5);
        }
        let ps7 = docs
            .join("PowerShell")
            .join("Microsoft.PowerShell_profile.ps1");
        if ps7.parent()?.exists() {
            return Some(ps7);
        }
        // Default to PS5 path if nothing exists, we will create dir later if needed
        return Some(ps5);
    }

    None
}

fn is_installed(profile_path: &Path) -> bool {
    if !profile_path.exists() {
        return false;
    }
    let content = fs::read_to_string(profile_path).unwrap_or_default();
    content.contains("function proxy") && content.contains("# Proxy CLI Wrapper")
}

fn install_to_profile(profile_path: &Path, exe_path: &str) {
    // Ensure directory exists
    if let Some(parent) = profile_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create profile directory");
    }

    let function_code = r##"
# Proxy CLI Wrapper
function proxy {
    $binaryPath = "__PROXY_EXE_PATH__"
    
    if (-not (Test-Path $binaryPath)) {
        Write-Error "Binary not found at $binaryPath"
        return
    }

    $output = & $binaryPath

    $output | ForEach-Object {
        if ($_ -match "^#SET_PROXY:(.+)$") {
            $proxyUrl = $matches[1]
            $env:HTTP_PROXY = $proxyUrl
            $env:HTTPS_PROXY = $proxyUrl
            Write-Host "Proxy set to $proxyUrl" -ForegroundColor Green
        }
        elseif ($_ -eq "#CLEAR_PROXY") {
            $env:HTTP_PROXY = ""
            $env:HTTPS_PROXY = ""
            Write-Host "Proxy cleared" -ForegroundColor Yellow
        }
    }
}
"##
    .replace("__PROXY_EXE_PATH__", exe_path);

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(profile_path)
        .expect("Failed to open profile for writing");

    writeln!(file, "{}", function_code).expect("Failed to write to profile");
}
