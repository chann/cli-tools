use anyhow::Result;
use csv::Reader;
use serde_json::{Value, Map};

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
        // Generate Markdown table
        print!("| ");
        for header in &headers {
            print!(" {} |", header);
        }
        println!();
        print!("|");
        for _ in &headers {
            print!(" --- |");
        }
        println!();
        for row in rows {
            print!("| ");
            for field in row {
                print!(" {} |", field);
            }
            println!();
        }
    } else {
        // Default to JSON
        println!("{}", serde_json::to_string_pretty(&records)?);
    }

    Ok(())
}
