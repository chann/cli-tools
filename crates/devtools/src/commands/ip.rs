use anyhow::{Result, Context};
use std::net::UdpSocket;
use serde::Deserialize;
use cli_core::ui::Theme;
use cli_core::output::TableFormatter;
use owo_colors::OwoColorize;

#[derive(Deserialize, Debug)]
struct IpInfo {
    ip: String,
    city: Option<String>,
    region: Option<String>,
    country_name: Option<String>,
    org: Option<String>,
    timezone: Option<String>,
    asn: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    postal: Option<String>,
    currency: Option<String>,
}

pub async fn show(target: Option<String>) -> Result<()> {
    if target.is_none() {
        // Local IP
        if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
            if socket.connect("8.8.8.8:80").is_ok() {
                if let Ok(local_addr) = socket.local_addr() {
                    println!("{} {}", Theme::info("Local IP:"), local_addr.ip().bright_white().bold());
                }
            }
        }
    }

    let url = match target {
        Some(t) => format!("https://ipapi.co/{}/json/", t),
        None => "https://ipapi.co/json/".to_string(),
    };

    println!("{}", Theme::info("Fetching IP information..."));

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let resp = client.get(&url).send().await.context("Failed to send request to IP info service")?;
    
    if !resp.status().is_success() {
        anyhow::bail!("IP info service returned error: {}", resp.status());
    }

    let info = resp.json::<IpInfo>().await.context("Failed to parse IP info JSON")?;

    println!("\n{}", Theme::header(" IP Information "));
    
    let mut table = TableFormatter::create_table();
    
    table.add_row(vec![
        TableFormatter::header_cell("Field"),
        TableFormatter::header_cell("Value"),
    ]);

    table.add_row(vec![
        TableFormatter::value_cell("IP Address"),
        TableFormatter::highlight_cell(&info.ip),
    ]);

    if let Some(city) = info.city {
        let location = format!("{}, {}, {}", 
            city, 
            info.region.unwrap_or_default(), 
            info.country_name.unwrap_or_default()
        );
        table.add_row(vec![
            TableFormatter::value_cell("Location"),
            TableFormatter::value_cell(location),
        ]);
    }

    if let (Some(lat), Some(lon)) = (info.latitude, info.longitude) {
        table.add_row(vec![
            TableFormatter::value_cell("Coordinates"),
            TableFormatter::value_cell(format!("{:.4}, {:.4}", lat, lon)),
        ]);
    }

    if let Some(postal) = info.postal {
        table.add_row(vec![
            TableFormatter::value_cell("Postal Code"),
            TableFormatter::value_cell(postal),
        ]);
    }

    if let Some(org) = info.org {
        table.add_row(vec![
            TableFormatter::value_cell("ISP / Organization"),
            TableFormatter::value_cell(org.yellow()),
        ]);
    }

    if let Some(asn) = info.asn {
        table.add_row(vec![
            TableFormatter::value_cell("ASN"),
            TableFormatter::value_cell(asn),
        ]);
    }

    if let Some(tz) = info.timezone {
        table.add_row(vec![
            TableFormatter::value_cell("Timezone"),
            TableFormatter::value_cell(tz.blue()),
        ]);
    }

    if let Some(curr) = info.currency {
        table.add_row(vec![
            TableFormatter::value_cell("Currency"),
            TableFormatter::value_cell(curr),
        ]);
    }

    println!("{}", table);

    Ok(())
}
