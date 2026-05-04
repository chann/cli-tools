use anyhow::Result;
use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::config::*;
use trust_dns_resolver::proto::rr::RecordType;
use owo_colors::OwoColorize;

pub async fn lookup(domain: &str, record_type: &str) -> Result<()> {
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());
    
    println!("{}", format!("--- DNS Lookup: {} ({}) ---", domain, record_type.to_uppercase()).bold().cyan());

    match record_type.to_lowercase().as_str() {
        "a" => {
            let response = resolver.lookup_ip(domain).await?;
            for ip in response.iter() {
                println!("A: {}", ip.green());
            }
        }
        "aaaa" => {
            let response = resolver.ipv6_lookup(domain).await?;
            for ip in response.iter() {
                println!("AAAA: {}", ip.green());
            }
        }
        "mx" => {
            let response = resolver.mx_lookup(domain).await?;
            for mx in response.iter() {
                println!("MX: {} (priority: {})", mx.exchange().to_string().yellow(), mx.preference());
            }
        }
        "txt" => {
            let response = resolver.txt_lookup(domain).await?;
            for txt in response.iter() {
                for data in txt.iter() {
                    println!("TXT: {}", String::from_utf8_lossy(data).blue());
                }
            }
        }
        "cname" => {
            let response = resolver.lookup(domain, RecordType::CNAME).await?;
            for record in response.records() {
                if let Some(cname) = record.data().and_then(|d| d.as_cname()) {
                    println!("CNAME: {}", cname.to_string().yellow());
                }
            }
        }
        "ns" => {
            let response = resolver.lookup(domain, RecordType::NS).await?;
            for record in response.records() {
                if let Some(ns) = record.data().and_then(|d| d.as_ns()) {
                    println!("NS: {}", ns.to_string().yellow());
                }
            }
        }
        _ => anyhow::bail!("Unsupported record type: {}. Supported: a, aaaa, mx, txt, cname, ns", record_type),
    }

    Ok(())
}
