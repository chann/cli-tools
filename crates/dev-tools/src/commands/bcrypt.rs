use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};
use cli_core::ui::Theme;
use cli_core::output::TableFormatter;

pub fn hash_password(password: &str, cost: Option<u32>) -> Result<()> {
    let cost = cost.unwrap_or(DEFAULT_COST);
    
    println!("{}", Theme::info(format!("Hashing password with cost factor: {}", Theme::highlight(cost.to_string()))));
    
    let hashed = hash(password, cost)?;
    
    println!("\n{}", Theme::header("Hashed Password:"));
    println!("{}", Theme::value(&hashed));
    
    Ok(())
}

pub fn verify_password(password: &str, hashed: &str) -> Result<()> {
    let valid = verify(password, hashed)?;
    
    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("Property"),
        TableFormatter::header_cell("Value"),
    ]);
    
    table.add_row(vec![
        TableFormatter::value_cell("Status"),
        if valid {
            TableFormatter::highlight_cell("MATCH")
        } else {
            TableFormatter::value_cell(Theme::red("MISMATCH"))
        },
    ]);
    
    table.add_row(vec![
        TableFormatter::value_cell("Algorithm"),
        TableFormatter::value_cell("Bcrypt"),
    ]);

    // Extract cost if possible (bcrypt hashes usually start with $2b$cost$...)
    if hashed.starts_with("$2") {
        let parts: Vec<&str> = hashed.split('$').collect();
        if parts.len() >= 3 {
            table.add_row(vec![
                TableFormatter::value_cell("Cost Factor"),
                TableFormatter::value_cell(parts[2]),
            ]);
        }
    }

    println!("\n{}", Theme::header("Verification Results:"));
    println!("{}", table);
    
    if valid {
        println!("{}", Theme::success("Password matches the hash."));
    } else {
        println!("{}", Theme::error("Password does NOT match the hash."));
    }
    
    Ok(())
}
