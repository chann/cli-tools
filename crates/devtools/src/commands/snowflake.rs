use anyhow::{Result, anyhow};
use snowid::SnowID;
use cli_core::ui::Theme;
use cli_core::output::TableFormatter;
use chrono::{Utc, TimeZone};

pub fn generate(count: usize) -> Result<()> {
    let generator = SnowID::new(1)
        .map_err(|e| anyhow!("Failed to create SnowID generator: {}", e))?;
    
    if count > 1 {
        println!("{}", Theme::info(format!("Generating {} Snowflake IDs:", count)));
    }
    
    for _ in 0..count {
        let id = generator.generate();
        println!("{} {}", Theme::dim("u64:"), Theme::value(id.to_string()));
    }
    Ok(())
}

pub fn inspect(id: i64) -> Result<()> {
    let generator = SnowID::new(1).unwrap();
    let (timestamp, node, sequence) = generator.extract.decompose(id as u64);
    
    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("Property"),
        TableFormatter::header_cell("Value"),
    ]);
    
    table.add_row(vec![
        TableFormatter::value_cell("Raw ID"),
        TableFormatter::highlight_cell(id.to_string()),
    ]);
    
    // Default epoch is 2024-01-01T00:00:00Z (1704067200000 ms)
    let epoch_ms = 1704067200000i64;
    let absolute_ts = epoch_ms + timestamp as i64;
    let dt = Utc.timestamp_millis_opt(absolute_ts).unwrap();
    
    table.add_row(vec![
        TableFormatter::value_cell("Timestamp"),
        TableFormatter::value_cell(format!("{}ms ({})", timestamp, dt.format("%Y-%m-%d %H:%M:%S.%3f UTC"))),
    ]);
    
    table.add_row(vec![
        TableFormatter::value_cell("Node ID"),
        TableFormatter::value_cell(node.to_string()),
    ]);
    
    table.add_row(vec![
        TableFormatter::value_cell("Sequence"),
        TableFormatter::value_cell(sequence.to_string()),
    ]);

    println!("\n{}", Theme::header("Snowflake Inspection:"));
    println!("{}", table);
    
    Ok(())
}
