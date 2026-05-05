use anyhow::Result;
use csv::Reader;
use serde_json::{Value, Map};
use cli_core::ui::Theme;
use cli_core::output::TableFormatter;

pub fn convert(input_path: &str, to_json: bool, to_yaml: bool, to_markdown: bool) -> Result<()> {
    let mut reader = Reader::from_path(input_path)?;
    let headers = reader.headers()?.clone();
    
    let mut records = Vec::new();
    let mut rows = Vec::new();
    for result in reader.records() {
        let record = result?;
        let mut map = Map::new();
        let mut row = Vec::new();
        for (header, field) in headers.iter().zip(record.iter()) {
            map.insert(header.to_string(), Value::String(field.to_string()));
            row.push(field.to_string());
        }
        records.push(Value::Object(map));
        rows.push(row);
    }

    if to_json {
        println!("{}", serde_json::to_string_pretty(&records)?);
    } else if to_yaml {
        println!("{}", serde_yaml::to_string(&records)?);
    } else if to_markdown {
        println!("{}", Theme::header("Markdown Table Output"));
        println!();
        // Generate Markdown table
        let mut output = String::new();
        output.push_str("|");
        for header in &headers {
            output.push_str(&format!(" {} |", header));
        }
        output.push_str("\n|");
        for _ in &headers {
            output.push_str(" --- |");
        }
        output.push_str("\n");
        for row in rows {
            output.push_str("|");
            for field in row {
                output.push_str(&format!(" {} |", field));
            }
            output.push_str("\n");
        }
        println!("{}", output);
    } else {
        // Default: Show terminal preview and then JSON
        println!("{}", Theme::header("CSV Data Preview"));
        let mut table = TableFormatter::create_table();
        
        let header_row: Vec<_> = headers.iter().map(TableFormatter::header_cell).collect();
        table.set_header(header_row);

        // Limit preview to first 10 rows
        for row in rows.iter().take(10) {
            let cells: Vec<_> = row.iter().map(TableFormatter::value_cell).collect();
            table.add_row(cells);
        }

        println!("{}", table);
        if rows.len() > 10 {
            println!("{}\n", Theme::dim(format!("... and {} more rows", rows.len() - 10)));
        }

        println!("{}", Theme::info("Full JSON Output:"));
        println!("{}", serde_json::to_string_pretty(&records)?);
    }

    Ok(())
}
