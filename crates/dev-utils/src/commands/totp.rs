use anyhow::Result;
use totp_rs::{Algorithm, Secret, TOTP};
use cli_core::ui::Theme;
use cli_core::output::TableFormatter;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

pub fn generate(secret_str: &str, digits: usize, skew: u8) -> Result<()> {
    let secret = Secret::Encoded(secret_str.to_string()).to_bytes()?;
    
    let totp = TOTP::new(
        Algorithm::SHA1,
        digits,
        skew,
        30,
        secret,
    )?;

    let code = totp.generate_current()?;
    
    // Calculate remaining time
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let remaining = 30 - (now % 30);

    println!("\n{}", Theme::header("TOTP Code Generation"));
    
    let mut table = TableFormatter::create_table();
    table.add_row(vec![
        TableFormatter::header_cell("Field"),
        TableFormatter::header_cell("Value"),
    ]);

    table.add_row(vec![
        TableFormatter::value_cell("Current Code"),
        TableFormatter::highlight_cell(code),
    ]);

    table.add_row(vec![
        TableFormatter::value_cell("Remaining Time"),
        TableFormatter::value_cell(format!("{} seconds", remaining)),
    ]);

    table.add_row(vec![
        TableFormatter::value_cell("Step"),
        TableFormatter::value_cell("30 seconds"),
    ]);

    table.add_row(vec![
        TableFormatter::value_cell("Algorithm"),
        TableFormatter::value_cell("SHA1"),
    ]);

    println!("{}", table);
    Ok(())
}

pub fn generate_new_secret() -> Result<()> {
    let mut rng = rand::thread_rng();
    let mut data = [0u8; 20];
    rng.fill(&mut data);
    
    let encoded = base32::encode(base32::Alphabet::Rfc4648 { padding: false }, &data);

    println!("\n{}", Theme::success("New TOTP Secret Generated!"));
    
    let mut table = TableFormatter::create_table();
    table.add_row(vec![
        TableFormatter::header_cell("Field"),
        TableFormatter::header_cell("Value"),
    ]);

    table.add_row(vec![
        TableFormatter::value_cell("Secret (Base32)"),
        TableFormatter::highlight_cell(encoded),
    ]);

    table.add_row(vec![
        TableFormatter::value_cell("Length"),
        TableFormatter::value_cell("20 bytes"),
    ]);

    table.add_row(vec![
        TableFormatter::value_cell("Info"),
        TableFormatter::value_cell("Save this secret securely. You can use it to generate TOTP codes."),
    ]);

    println!("{}", table);
    Ok(())
}
