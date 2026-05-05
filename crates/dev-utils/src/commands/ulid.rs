use ulid::Ulid;
use anyhow::{Result, Context};
use cli_core::ui::Theme;
use cli_core::output::TableFormatter;
use chrono::{DateTime, Utc};

pub fn generate(count: usize) -> Result<()> {
    if count == 0 {
        return Ok(());
    }

    if count == 1 {
        let id = Ulid::new();
        println!("{}", Theme::highlight(id.to_string()));
        return Ok(());
    }

    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("#"),
        TableFormatter::header_cell("ULID"),
        TableFormatter::header_cell("Timestamp"),
    ]);

    for i in 0..count {
        let id = Ulid::new();
        let dt: DateTime<Utc> = id.datetime().into();
        table.add_row(vec![
            TableFormatter::value_cell(i + 1),
            TableFormatter::highlight_cell(id.to_string()),
            TableFormatter::value_cell(dt.to_rfc3339()),
        ]);
    }

    println!("\n{}", Theme::info(format!("Generated {} ULIDs:", count)));
    println!("{}", table);
    Ok(())
}

pub fn inspect(id: &str) -> Result<()> {
    let id = Ulid::from_string(id)
        .with_context(|| format!("Failed to parse ULID: '{}'", id))?;

    let dt: DateTime<Utc> = id.datetime().into();

    println!("\n{}", Theme::header("ULID Inspection Report"));
    
    let mut table = TableFormatter::create_table();
    table.add_row(vec![
        TableFormatter::header_cell("Field"),
        TableFormatter::header_cell("Value"),
    ]);

    table.add_row(vec![
        TableFormatter::value_cell("ULID"),
        TableFormatter::highlight_cell(id.to_string()),
    ]);

    table.add_row(vec![
        TableFormatter::value_cell("Timestamp"),
        TableFormatter::highlight_cell(dt.to_rfc3339()),
    ]);

    table.add_row(vec![
        TableFormatter::value_cell("Milliseconds"),
        TableFormatter::value_cell(id.timestamp_ms()),
    ]);

    table.add_row(vec![
        TableFormatter::value_cell("Random Part"),
        TableFormatter::value_cell(format!("{:020x}", id.random())),
    ]);

    println!("{}", table);
    Ok(())
}
