use anyhow::Result;
use std::net::UdpSocket;
use owo_colors::OwoColorize;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct IpInfo {
    ip: String,
    city: Option<String>,
    region: Option<String>,
    country_name: Option<String>,
    org: Option<String>,
    timezone: Option<String>,
}

pub async fn show() -> Result<()> {
    // Local IP
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
        if socket.connect("8.8.8.8:80").is_ok() {
            if let Ok(local_addr) = socket.local_addr() {
                println!("{}: {}", "Local IP".bold(), local_addr.ip().cyan());
            }
        }
    }

    // Public IP and Info
    println!("{}: {}", "Public IP".bold(), "Fetching information...".dimmed());
    match reqwest::get("https://ipapi.co/json/").await {
        Ok(resp) => {
            if let Ok(info) = resp.json::<IpInfo>().await {
                println!("{}: {}", "Public IP".bold(), info.ip.cyan());
                if let Some(city) = info.city {
                    println!("{}: {}", "Location".bold(), format!("{}, {}, {}", city, info.region.unwrap_or_default(), info.country_name.unwrap_or_default()).green());
                }
                if let Some(org) = info.org {
                    println!("{}: {}", "ISP/Org".bold(), org.yellow());
                }
                if let Some(tz) = info.timezone {
                    println!("{}: {}", "Timezone".bold(), tz.blue());
                }
            }
        }
        Err(_) => {
            // Fallback to simple IP fetch
            if let Ok(resp) = reqwest::get("https://api.ipify.org").await {
                if let Ok(ip) = resp.text().await {
                    println!("{}: {}", "Public IP".bold(), ip.cyan());
                }
            } else {
                println!("{}: {}", "Public IP".bold(), "Failed to fetch".red());
            }
        }
    }

    Ok(())
}
