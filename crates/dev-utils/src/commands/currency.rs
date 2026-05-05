use anyhow::{Result, Context};
use serde::Deserialize;
use std::collections::HashMap;
use cli_core::ui::Theme;
use cli_core::output::TableFormatter;

#[derive(Deserialize, Debug)]
struct CurrencyResponse {
    date: String,
    rates: HashMap<String, f64>,
}

pub async fn convert(amount: f64, from: &str, to: &str) -> Result<()> {
    let from = from.to_uppercase();
    let to = to.to_uppercase();

    println!("{}", Theme::info(format!("Converting {} {} to {}...", amount, from, to)));

    let url = format!(
        "https://api.frankfurter.app/latest?amount={}&from={}&to={}",
        amount, from, to
    );

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let resp = client
        .get(&url)
        .send()
        .await
        .context("Failed to connect to currency exchange service")?;

    if !resp.status().is_success() {
        if resp.status() == 404 {
            anyhow::bail!("Unsupported currency code(s). Check if both codes are valid (e.g., USD, EUR, GBP).");
        }
        anyhow::bail!("Currency exchange service returned error: {}", resp.status());
    }

    let data = resp
        .json::<CurrencyResponse>()
        .await
        .context("Failed to parse currency exchange response")?;

    if let Some(&converted_amount) = data.rates.get(&to) {
        let mut table = TableFormatter::create_table();
        table.set_header(vec![
            TableFormatter::header_cell("Field"),
            TableFormatter::header_cell("Value"),
        ]);

        table.add_row(vec![
            TableFormatter::value_cell("From"),
            TableFormatter::value_cell(format!("{} {}", amount, from)),
        ]);

        table.add_row(vec![
            TableFormatter::value_cell("To"),
            TableFormatter::value_cell(&to),
        ]);

        table.add_row(vec![
            TableFormatter::value_cell("Result"),
            TableFormatter::highlight_cell(format!("{:.2} {}", converted_amount, to)),
        ]);

        table.add_row(vec![
            TableFormatter::value_cell("Rate"),
            TableFormatter::value_cell(format!("1 {} = {:.4} {}", from, converted_amount / amount, to)),
        ]);

        table.add_row(vec![
            TableFormatter::value_cell("Date"),
            TableFormatter::value_cell(&data.date),
        ]);

        println!("\n{}", Theme::header(" Currency Conversion "));
        println!("{}", table);
    } else {
        anyhow::bail!("Failed to find target currency in response");
    }

    Ok(())
}
