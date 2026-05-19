use anyhow::Result;
use serde_json::Value;

pub fn format(text: &str) -> Result<()> {
    let value: Value = toml::from_str(text)?;
    println!("{}", toml::to_string_pretty(&value)?);
    Ok(())
}

pub fn to_json(text: &str) -> Result<()> {
    let value: Value = toml::from_str(text)?;
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}

pub fn to_yaml(text: &str) -> Result<()> {
    let value: Value = toml::from_str(text)?;
    println!("{}", serde_yaml::to_string(&value)?);
    Ok(())
}
