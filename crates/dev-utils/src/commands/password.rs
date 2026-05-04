use anyhow::Result;
use rand::{thread_rng, Rng};
use owo_colors::OwoColorize;

pub fn generate(length: usize, use_numbers: bool, use_symbols: bool, use_uppercase: bool, use_lowercase: bool) -> Result<()> {
    let mut charset = String::new();
    
    if use_lowercase {
        charset.push_str("abcdefghijklmnopqrstuvwxyz");
    }
    if use_uppercase {
        charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if use_numbers {
        charset.push_str("0123456789");
    }
    if use_symbols {
        charset.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?");
    }

    if charset.is_empty() {
        // Default to alphanumeric if nothing selected
        charset.push_str("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
    }

    let mut rng = thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect();

    println!("{}", password);
    Ok(())
}

pub fn check(password: &str) -> Result<()> {
    let mut score = 0;
    let length = password.len();

    if length >= 8 { score += 1; }
    if length >= 12 { score += 1; }
    if password.chars().any(|c| c.is_lowercase()) { score += 1; }
    if password.chars().any(|c| c.is_uppercase()) { score += 1; }
    if password.chars().any(|c| c.is_numeric()) { score += 1; }
    if password.chars().any(|c| !c.is_alphanumeric()) { score += 1; }

    print!("{}: ", "Strength".bold());
    match score {
        0..=2 => println!("{}", "Very Weak".red()),
        3 => println!("{}", "Weak".yellow()),
        4 => println!("{}", "Medium".blue()),
        5 => println!("{}", "Strong".green()),
        6 => println!("{}", "Very Strong".bright_green().bold()),
        _ => println!("{}", "Incredible".magenta().bold()),
    }

    println!("{} {} characters", "Length:".dimmed(), length);
    
    Ok(())
}
