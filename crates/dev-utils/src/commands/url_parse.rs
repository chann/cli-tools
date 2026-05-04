use anyhow::Result;
use url::Url;
use owo_colors::OwoColorize;

pub fn parse(input: &str) -> Result<()> {
    let u = Url::parse(input)?;
    
    println!("{}: {}", "URL".bold(), input.cyan());
    println!("{}: {}", "Scheme".bold(), u.scheme().green());
    
    if let Some(host) = u.host_str() {
        println!("{}: {}", "Host".bold(), host.yellow());
    }
    
    if let Some(port) = u.port() {
        println!("{}: {}", "Port".bold(), port.blue());
    }
    
    println!("{}: {}", "Path".bold(), u.path());
    
    if let Some(query) = u.query() {
        println!("{}: {}", "Query".bold(), query);
        
        println!("\n{}", "--- Query Parameters ---".dimmed());
        for (key, value) in u.query_pairs() {
            println!("  {} = {}", key.bold(), value.green());
        }
    }
    
    if let Some(fragment) = u.fragment() {
        println!("{}: {}", "Fragment".bold(), fragment);
    }
    
    if !u.username().is_empty() {
        println!("{}: {}", "Username".bold(), u.username());
    }
    
    if let Some(password) = u.password() {
        println!("{}: {}", "Password".bold(), password);
    }

    Ok(())
}
