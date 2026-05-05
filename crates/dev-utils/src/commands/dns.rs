use anyhow::Result;
use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::config::*;
use trust_dns_resolver::proto::rr::RecordType;
use cli_core::output::TableFormatter;
use cli_core::ui::Theme;

pub async fn lookup(domain: &str, record_type: &str) -> Result<()> {
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());
    
    let rt_upper = record_type.to_uppercase();
    println!("{}", Theme::info(format!("DNS Lookup: {} ({})", domain, rt_upper)));

    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("Type"),
        TableFormatter::header_cell("Value"),
    ]);

    match record_type.to_lowercase().as_str() {
        "a" => {
            let response = resolver.lookup_ip(domain).await?;
            for ip in response.iter() {
                table.add_row(vec![
                    TableFormatter::value_cell("A"),
                    TableFormatter::highlight_cell(ip.to_string()),
                ]);
            }
        }
        "aaaa" => {
            let response = resolver.ipv6_lookup(domain).await?;
            for ip in response.iter() {
                table.add_row(vec![
                    TableFormatter::value_cell("AAAA"),
                    TableFormatter::highlight_cell(ip.to_string()),
                ]);
            }
        }
        "mx" => {
            let response = resolver.mx_lookup(domain).await?;
            for mx in response.iter() {
                table.add_row(vec![
                    TableFormatter::value_cell("MX"),
                    TableFormatter::value_cell(format!("{} (priority: {})", mx.exchange(), mx.preference())),
                ]);
            }
        }
        "txt" => {
            let response = resolver.txt_lookup(domain).await?;
            for txt in response.iter() {
                for data in txt.iter() {
                    table.add_row(vec![
                        TableFormatter::value_cell("TXT"),
                        TableFormatter::value_cell(String::from_utf8_lossy(data)),
                    ]);
                }
            }
        }
        "cname" => {
            let response = resolver.lookup(domain, RecordType::CNAME).await?;
            for record in response.records() {
                if let Some(cname) = record.data().and_then(|d| d.as_cname()) {
                    table.add_row(vec![
                        TableFormatter::value_cell("CNAME"),
                        TableFormatter::value_cell(cname.to_string()),
                    ]);
                }
            }
        }
        "ns" => {
            let response = resolver.lookup(domain, RecordType::NS).await?;
            for record in response.records() {
                if let Some(ns) = record.data().and_then(|d| d.as_ns()) {
                    table.add_row(vec![
                        TableFormatter::value_cell("NS"),
                        TableFormatter::value_cell(ns.to_string()),
                    ]);
                }
            }
        }
        _ => anyhow::bail!("Unsupported record type: {}. Supported: a, aaaa, mx, txt, cname, ns", record_type),
    }

    if table.is_empty() {
        println!("{}", Theme::warning("No records found."));
    } else {
        println!("\n{}", table);
    }

    Ok(())
}
