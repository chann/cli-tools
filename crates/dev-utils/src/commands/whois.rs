use anyhow::Result;
use owo_colors::OwoColorize;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn lookup(domain: &str) -> Result<()> {
    println!("{} {}...", "Looking up WHOIS for".bold(), domain.cyan());

    // 1. Ask IANA which WHOIS server to use
    let mut iana_conn = TcpStream::connect("whois.iana.org:43")?;
    iana_conn.write_all(format!("{}\r\n", domain).as_bytes())?;
    
    let mut iana_response = String::new();
    iana_conn.read_to_string(&mut iana_response)?;

    // 2. Find "refer" or "whois" in IANA response
    let mut target_server = "whois.verisign-grs.com".to_string(); // Default fallback
    for line in iana_response.lines() {
        if line.starts_with("refer:") || line.starts_with("whois:") {
            if let Some(server) = line.split_whitespace().last() {
                target_server = server.to_string();
                break;
            }
        }
    }

    println!("{} {}...", "Querying".dimmed(), target_server.dimmed());

    // 3. Query the target server
    let mut target_conn = TcpStream::connect(format!("{}:43", target_server))?;
    target_conn.write_all(format!("{}\r\n", domain).as_bytes())?;

    let mut final_response = String::new();
    target_conn.read_to_string(&mut final_response)?;

    // 4. Print results (truncated or filtered)
    println!("\n{}", "--- WHOIS Information ---".bold().green());
    
    // Filter out some noise if needed, but for a dev tool, full info is usually fine
    // Let's just print the first 40 lines to avoid spamming the terminal, or just print everything
    println!("{}", final_response);

    Ok(())
}
