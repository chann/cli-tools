use anyhow::Result;
use std::env;
use cli_core::ui::Theme;
use cli_core::output::TableFormatter;

pub fn list(filter: Option<String>) -> Result<()> {
    let mut vars: Vec<(String, String)> = env::vars().collect();
    vars.sort_by(|a, b| a.0.cmp(&b.0));
    
    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("Variable"),
        TableFormatter::header_cell("Value"),
    ]);
    
    let mut count = 0;
    for (key, value) in vars {
        if let Some(ref f) = filter {
            if key.to_lowercase().contains(&f.to_lowercase()) {
                table.add_row(vec![
                    TableFormatter::highlight_cell(key),
                    TableFormatter::value_cell(value),
                ]);
                count += 1;
            }
        } else {
            table.add_row(vec![
                TableFormatter::value_cell(key),
                TableFormatter::value_cell(value),
            ]);
            count += 1;
        }
    }
    
    if let Some(f) = filter {
        println!("{}", Theme::info(format!("Filtered environment variables (matching \"{}\"): {} found", f, count)));
    } else {
        println!("{}", Theme::header(format!("Environment Variables ({} total):", count)));
    }
    
    if count > 0 {
        println!("{}", table);
    } else {
        println!("{}", Theme::warning("No matching environment variables found."));
    }
    
    Ok(())
}
