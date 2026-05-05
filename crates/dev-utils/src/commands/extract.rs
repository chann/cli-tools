use anyhow::Result;
use regex::Regex;
use std::fs;
use cli_core::ui::Theme;
use cli_core::output::TableFormatter;

pub fn extract(input: &str, kind: &str, is_file: bool) -> Result<()> {
    let text = if is_file {
        fs::read_to_string(input)?
    } else {
        input.to_string()
    };

    let (pattern, label) = match kind.to_lowercase().as_str() {
        "email" => (r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}", "Email"),
        "url" => (r"https?://(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_\+.~#?&//=]*)", "URL"),
        "ip" => (r"(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)", "IP v4"),
        "ipv6" => (r"(([0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:|([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|:((:[0-9a-fA-F]{1,4}){1,7}|:)|fe80:(:[0-9a-fA-F]{0,4}){0,4}%[0-9a-zA-Z]{1,}|::(ffff(:0{1,4}){0,1}:){0,1}((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])|([0-9a-fA-F]{1,4}:){1,4}:((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9]))", "IP v6"),
        "mac" => (r"(?:[0-9a-fA-F]{2}[:-]){5}(?:[0-9a-fA-F]{2})", "MAC Address"),
        "phone" => (r"(?:\+?\d{1,3}[-.\s]?)?\(?\d{2,4}?\)?[-.\s]?\d{3,4}[-.\s]?\d{3,4}", "Phone Number"),
        "date" => (r"\d{4}[-/]\d{1,2}[-/]\d{1,2}|\d{1,2}[-/]\d{1,2}[-/]\d{4}", "Date"),
        "card" => (r"\d{4}[-\s]?\d{4}[-\s]?\d{4}[-\s]?\d{4}", "Credit Card"),
        _ => anyhow::bail!("Unsupported extraction kind: {}. Use email, url, ip, ipv6, mac, phone, date, card.", kind),
    };

    let re = Regex::new(pattern)?;
    let mut matches: Vec<&str> = re.find_iter(&text).map(|m| m.as_str()).collect();
    matches.sort_unstable();
    matches.dedup();

    if matches.is_empty() {
        println!("{}", Theme::warning(format!("No {} found in the input.", label)));
        return Ok(());
    }

    println!("{}", Theme::header(format!("--- Extracted {}: {} ---", label, matches.len())));
    
    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("#"),
        TableFormatter::header_cell(label),
    ]);

    for (i, m) in matches.iter().enumerate() {
        table.add_row(vec![
            TableFormatter::value_cell(i + 1),
            TableFormatter::highlight_cell(m),
        ]);
    }

    println!("{}", table);

    Ok(())
}
