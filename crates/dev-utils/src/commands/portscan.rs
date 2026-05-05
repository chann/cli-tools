use anyhow::Result;
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;
use std::collections::HashMap;
use tokio::net::TcpStream;
use tokio::time::timeout;
use futures_util::stream::{self, StreamExt};
use cli_core::ui::Theme;
use cli_core::output::TableFormatter;
use comfy_table::Cell;

pub async fn scan(host: &str, start: u16, end: u16, timeout_ms: u64) -> Result<()> {
    println!("{}", Theme::info(format!("Scanning {} (ports {}-{})...", host, start, end)));

    // Resolve host once
    let addrs: Vec<_> = format!("{}:{}", host, 0).to_socket_addrs()?.collect();
    if addrs.is_empty() {
        anyhow::bail!("Could not resolve host: {}", host);
    }
    let ip = addrs[0].ip();

    let common_ports = get_common_ports();
    
    // Scan in parallel with a concurrency limit
    let concurrency_limit = 100;
    let ports = start..=end;
    
    let mut open_ports = Vec::new();

    let mut stream = stream::iter(ports)
        .map(|port| {
            let addr = SocketAddr::new(ip, port);
            let duration = Duration::from_millis(timeout_ms);
            async move {
                match timeout(duration, TcpStream::connect(&addr)).await {
                    Ok(Ok(_)) => Some(port),
                    _ => None,
                }
            }
        })
        .buffer_unordered(concurrency_limit);

    while let Some(res) = stream.next().await {
        if let Some(port) = res {
            open_ports.push(port);
        }
    }

    open_ports.sort_unstable();

    if open_ports.is_empty() {
        println!("\n{}", Theme::warning("No open ports found."));
    } else {
        println!("\n{}", Theme::success(format!("Scan complete. Found {} open ports.", open_ports.len())));
        
        let mut table = TableFormatter::create_table();
        table.set_header(vec![
            TableFormatter::header_cell("Port"),
            TableFormatter::header_cell("Status"),
            TableFormatter::header_cell("Service"),
        ]);

        for port in open_ports {
            let service = common_ports.get(&port).cloned().unwrap_or_else(|| "Unknown".to_string());
            table.add_row(vec![
                TableFormatter::value_cell(port),
                Cell::new("OPEN").fg(comfy_table::Color::Green).add_attribute(comfy_table::Attribute::Bold),
                TableFormatter::value_cell(service),
            ]);
        }
        
        println!("\n{}", table);
    }

    Ok(())
}

fn get_common_ports() -> HashMap<u16, String> {
    let mut m = HashMap::new();
    m.insert(20, "FTP-DATA".to_string());
    m.insert(21, "FTP".to_string());
    m.insert(22, "SSH".to_string());
    m.insert(23, "Telnet".to_string());
    m.insert(25, "SMTP".to_string());
    m.insert(53, "DNS".to_string());
    m.insert(80, "HTTP".to_string());
    m.insert(110, "POP3".to_string());
    m.insert(123, "NTP".to_string());
    m.insert(143, "IMAP".to_string());
    m.insert(443, "HTTPS".to_string());
    m.insert(465, "SMTPS".to_string());
    m.insert(587, "SMTP-STARTTLS".to_string());
    m.insert(993, "IMAPS".to_string());
    m.insert(995, "POP3S".to_string());
    m.insert(1433, "MSSQL".to_string());
    m.insert(3306, "MySQL".to_string());
    m.insert(3389, "RDP".to_string());
    m.insert(5432, "PostgreSQL".to_string());
    m.insert(6379, "Redis".to_string());
    m.insert(8080, "HTTP-Proxy".to_string());
    m.insert(27017, "MongoDB".to_string());
    m
}
