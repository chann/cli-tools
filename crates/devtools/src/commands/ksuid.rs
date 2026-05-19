use anyhow::Result;
use ksuid::Ksuid;
use cli_core::ui::Theme;
use cli_core::output::TableFormatter;
use chrono::{Utc, TimeZone};

pub fn generate(count: usize) -> Result<()> {
    if count > 1 {
        println!("{}", Theme::info(format!("Generating {} KSUIDs:", count)));
    }
    for _ in 0..count {
        let id = Ksuid::generate();
        println!("{}", Theme::value(id.to_base62()));
    }
    Ok(())
}

pub fn inspect(id_str: &str) -> Result<()> {
    let id = Ksuid::from_base62(id_str)
        .map_err(|_| anyhow::anyhow!("Invalid KSUID: {}", id_str))?;
    
    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("Property"),
        TableFormatter::header_cell("Value"),
    ]);
    
    table.add_row(vec![
        TableFormatter::value_cell("ID (Base62)"),
        TableFormatter::highlight_cell(id.to_base62()),
    ]);
    
    let timestamp = id.timestamp();
    // KSUID epoch is 14e8 (2014-05-13T16:53:20Z)
    let dt = Utc.timestamp_opt(timestamp as i64, 0).unwrap();
    
    table.add_row(vec![
        TableFormatter::value_cell("Timestamp"),
        TableFormatter::value_cell(format!("{} ({})", timestamp, dt.format("%Y-%m-%d %H:%M:%S UTC"))),
    ]);
    
    table.add_row(vec![
        TableFormatter::value_cell("Payload (Hex)"),
        TableFormatter::value_cell(hex::encode(id.payload())),
    ]);

    println!("\n{}", Theme::header("KSUID Inspection:"));
    println!("{}", table);
    
    Ok(())
}
