use anyhow::Result;
use std::env;
use owo_colors::OwoColorize;

pub fn show() -> Result<()> {
    println!("{}", "--- User & Environment Information ---".bold().cyan());

    // User Info
    let user = env::var("USER").or_else(|_| env::var("USERNAME")).unwrap_or_else(|_| "Unknown".to_string());
    println!("{:<15}: {}", "Current User", user.green());

    // Shell Info
    let shell = env::var("SHELL").unwrap_or_else(|_| "Unknown".to_string());
    println!("{:<15}: {}", "Current Shell", shell.yellow());

    // Terminal Info
    let term = env::var("TERM").unwrap_or_else(|_| "Unknown".to_string());
    println!("{:<15}: {}", "Terminal", term.blue());

    // Directories
    if let Ok(home) = env::var("HOME").or_else(|_| env::var("USERPROFILE")) {
        println!("{:<15}: {}", "Home Dir", home.dimmed());
    }

    if let Ok(pwd) = env::current_dir() {
        println!("{:<15}: {}", "Current Dir", pwd.display().dimmed());
    }

    // Editor
    let editor = env::var("EDITOR").or_else(|_| env::var("VISUAL")).unwrap_or_else(|_| "Not set".to_string());
    println!("{:<15}: {}", "Editor", editor.magenta());

    // Language
    let lang = env::var("LANG").unwrap_or_else(|_| "Not set".to_string());
    println!("{:<15}: {}", "Language", lang.cyan());

    Ok(())
}
