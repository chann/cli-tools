use chrono::{Utc, TimeZone, DateTime, Local};
use anyhow::{Result, anyhow};
use owo_colors::OwoColorize;

pub fn current() -> Result<()> {
    let now_utc = Utc::now();
    let now_local = Local::now();
    
    println!("{}: {}", "Unix".bold(), now_utc.timestamp().cyan());
    println!("{}: {}", "ISO8601 (UTC)".bold(), now_utc.to_rfc3339().green());
    println!("{}: {}", "ISO8601 (Local)".bold(), now_local.to_rfc3339().green());
    println!("{}: {}", "RFC2822".bold(), now_utc.to_rfc2822().yellow());
    println!("{}: {}", "Custom".bold(), now_utc.format("%Y-%m-%d %H:%M:%S").magenta());
    
    Ok(())
}

pub fn convert(input: &str) -> Result<()> {
    let dt = if let Ok(ts) = input.parse::<i64>() {
        // Assume unix timestamp
        if let Some(dt) = Utc.timestamp_opt(ts, 0).single() {
            dt
        } else {
            return Err(anyhow!("Invalid unix timestamp: {}", ts));
        }
    } else if let Ok(dt) = DateTime::parse_from_rfc3339(input) {
        dt.with_timezone(&Utc)
    } else if let Ok(dt) = DateTime::parse_from_rfc2822(input) {
        dt.with_timezone(&Utc)
    } else {
        return Err(anyhow!("Invalid input. Provide a unix timestamp, ISO8601 string, or RFC2822 string."));
    };

    println!("{}: {}", "Unix".bold(), dt.timestamp().cyan());
    println!("{}: {}", "ISO8601".bold(), dt.to_rfc3339().green());
    println!("{}: {}", "RFC2822".bold(), dt.to_rfc2822().yellow());
    println!("{}: {}", "Local".bold(), dt.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").magenta());
    
    Ok(())
}
