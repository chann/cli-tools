use anyhow::Result;
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;
use owo_colors::OwoColorize;
use tokio::net::TcpStream as TokioTcpStream;
use tokio::time::timeout as tokio_timeout;

pub async fn scan(host: &str, start: u16, end: u16, timeout_ms: u64) -> Result<()> {
    println!("Scanning {} (ports {}-{})...", host.cyan(), start, end);

    let mut open_ports = Vec::new();

    // Resolve host once
    let addrs: Vec<_> = format!("{}:{}", host, 0).to_socket_addrs()?.collect();
    if addrs.is_empty() {
        anyhow::bail!("Could not resolve host: {}", host);
    }
    let ip = addrs[0].ip();

    for port in start..=end {
        let addr = SocketAddr::new(ip, port);
        let duration = Duration::from_millis(timeout_ms);

        match tokio_timeout(duration, TokioTcpStream::connect(&addr)).await {
            Ok(Ok(_)) => {
                println!("Port {} is {}", port, "OPEN".green().bold());
                open_ports.push(port);
            }
            _ => {
                // Port is closed or timed out
            }
        }
    }

    if open_ports.is_empty() {
        println!("{}", "No open ports found.".yellow());
    } else {
        println!(
            "\nScan complete. {} open ports: {:?}",
            open_ports.len(),
            open_ports
        );
    }

    Ok(())
}
