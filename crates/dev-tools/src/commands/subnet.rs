use anyhow::{anyhow, Result};
use cli_core::output::format_integer;
use cli_core::ui::Theme;
use owo_colors::OwoColorize;
use std::net::Ipv4Addr;

pub fn run(cidr: &str) -> Result<()> {
    let info = analyze(cidr)?;

    println!("\n{}", Theme::header(&format!("Subnet {}", cidr)));
    let row = |label: &str, value: String| println!("  {:<14} {}", label.dimmed(), Theme::value(&value));

    row("Netmask:", info.netmask.to_string());
    row("Wildcard:", info.wildcard.to_string());
    row("Network:", format!("{}/{}", info.network, info.prefix));
    row("Broadcast:", info.broadcast.to_string());
    match (info.first_host, info.last_host) {
        (Some(first), Some(last)) => row("Host range:", format!("{} - {}", first, last)),
        _ => row("Host range:", "-".to_string()),
    }
    row("Usable hosts:", format_integer(info.usable_hosts as i64));
    row("Kind:", kind(info.ip).to_string());
    Ok(())
}

struct SubnetInfo {
    ip: Ipv4Addr,
    prefix: u8,
    netmask: Ipv4Addr,
    wildcard: Ipv4Addr,
    network: Ipv4Addr,
    broadcast: Ipv4Addr,
    first_host: Option<Ipv4Addr>,
    last_host: Option<Ipv4Addr>,
    usable_hosts: u64,
}

fn analyze(cidr: &str) -> Result<SubnetInfo> {
    let (ip_str, prefix_str) = cidr
        .split_once('/')
        .ok_or_else(|| anyhow!("Expected CIDR notation (e.g., 192.168.1.0/24), got: {:?}", cidr))?;
    let ip: Ipv4Addr = ip_str
        .parse()
        .map_err(|_| anyhow!("Invalid IPv4 address: {:?}", ip_str))?;
    let prefix: u8 = prefix_str
        .parse()
        .ok()
        .filter(|p| *p <= 32)
        .ok_or_else(|| anyhow!("Prefix must be 0-32, got: {:?}", prefix_str))?;

    let mask: u32 = if prefix == 0 { 0 } else { u32::MAX << (32 - prefix) };
    let network = u32::from(ip) & mask;
    let broadcast = network | !mask;

    let (first_host, last_host, usable_hosts) = match prefix {
        32 => (Some(ip), Some(ip), 1),
        31 => (Some(Ipv4Addr::from(network)), Some(Ipv4Addr::from(broadcast)), 2), // RFC 3021
        _ => (
            Some(Ipv4Addr::from(network + 1)),
            Some(Ipv4Addr::from(broadcast - 1)),
            (broadcast - network - 1) as u64,
        ),
    };

    Ok(SubnetInfo {
        ip,
        prefix,
        netmask: Ipv4Addr::from(mask),
        wildcard: Ipv4Addr::from(!mask),
        network: Ipv4Addr::from(network),
        broadcast: Ipv4Addr::from(broadcast),
        first_host,
        last_host,
        usable_hosts,
    })
}

fn kind(ip: Ipv4Addr) -> &'static str {
    if ip.is_loopback() {
        "Loopback"
    } else if ip.is_private() {
        "Private"
    } else if ip.is_link_local() {
        "Link-local"
    } else if ip.is_multicast() {
        "Multicast"
    } else {
        "Public"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ip(s: &str) -> Ipv4Addr {
        s.parse().unwrap()
    }

    #[test]
    fn analyzes_a_typical_slash_24() {
        let info = analyze("192.168.1.10/24").unwrap();
        assert_eq!(info.netmask, ip("255.255.255.0"));
        assert_eq!(info.wildcard, ip("0.0.0.255"));
        assert_eq!(info.network, ip("192.168.1.0"));
        assert_eq!(info.broadcast, ip("192.168.1.255"));
        assert_eq!(info.first_host, Some(ip("192.168.1.1")));
        assert_eq!(info.last_host, Some(ip("192.168.1.254")));
        assert_eq!(info.usable_hosts, 254);
    }

    #[test]
    fn analyzes_large_and_edge_prefixes() {
        assert_eq!(analyze("10.0.0.0/8").unwrap().usable_hosts, 16_777_214);
        assert_eq!(analyze("0.0.0.0/0").unwrap().usable_hosts, u32::MAX as u64 - 1);

        let p2p = analyze("10.0.0.0/31").unwrap();
        assert_eq!(p2p.usable_hosts, 2);
        assert_eq!(p2p.first_host, Some(ip("10.0.0.0")));

        let host = analyze("10.0.0.5/32").unwrap();
        assert_eq!(host.usable_hosts, 1);
        assert_eq!(host.first_host, Some(ip("10.0.0.5")));
    }

    #[test]
    fn rejects_malformed_input() {
        assert!(analyze("192.168.1.0").is_err()); // no prefix
        assert!(analyze("192.168.1.0/33").is_err());
        assert!(analyze("999.1.1.1/24").is_err());
        assert!(analyze("10.0.0.0/abc").is_err());
    }

    #[test]
    fn classifies_address_kind() {
        assert_eq!(kind(ip("127.0.0.1")), "Loopback");
        assert_eq!(kind(ip("192.168.1.1")), "Private");
        assert_eq!(kind(ip("8.8.8.8")), "Public");
    }
}
