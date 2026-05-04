use anyhow::Result;
use owo_colors::OwoColorize;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct ExchangeRates {
    rates: HashMap<String, f64>,
}

pub async fn convert(amount: f64, from: &str, to: &str) -> Result<()> {
    let from = from.to_uppercase();
    let to = to.to_uppercase();

    println!(
        "{} {} {} to {}...",
        "Converting".bold(),
        amount.cyan(),
        from.bold(),
        to.bold()
    );

    let url = format!("https://api.exchangerate-api.com/v4/latest/{}", from);
    let resp = reqwest::get(url).await?.json::<ExchangeRates>().await?;

    if let Some(rate) = resp.rates.get(&to) {
        let result = amount * rate;
        println!(
            "{} {} {} = {} {}",
            amount.cyan(),
            from.bold(),
            "is".dimmed(),
            format!("{:.2}", result).green().bold(),
            to.bold()
        );
        println!("{}: {}", "Rate".dimmed(), format!("{:.4}", rate).dimmed());
    } else {
        anyhow::bail!("Currency {} not found in exchange rates for {}", to, from);
    }

    Ok(())
}
