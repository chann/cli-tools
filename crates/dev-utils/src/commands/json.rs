use serde_json::Value;
use anyhow::{Result, anyhow};
use jsonpath_rust::JsonPath;
use cli_core::ui::Theme;

pub fn process(text: &str, pretty: bool, query: Option<String>) -> Result<()> {
    let v: Value = serde_json::from_str(text)
        .map_err(|e| anyhow!("Invalid JSON: {}", e))?;
    
    let result = if let Some(q) = query {
        let matches = v.query(&q)
            .map_err(|e| anyhow!("Invalid JSON Path: {}", e))?;
        Value::Array(matches.into_iter().cloned().collect())
    } else {
        v
    };

    if pretty {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("{}", serde_json::to_string(&result)?);
    }
    Ok(())
}

pub fn to_yaml(text: &str) -> Result<()> {
    let v: Value = serde_json::from_str(text)?;
    println!("\n{}", Theme::header("--- YAML Output ---"));
    println!("{}", serde_yaml::to_string(&v)?);
    Ok(())
}

pub fn to_toml(text: &str) -> Result<()> {
    let v: Value = serde_json::from_str(text)?;
    println!("\n{}", Theme::header("--- TOML Output ---"));
    println!("{}", toml::to_string_pretty(&v)?);
    Ok(())
}

pub fn to_csv(text: &str) -> Result<()> {
    let v: Value = serde_json::from_str(text)?;
    let mut writer = csv::Writer::from_writer(std::io::stdout());

    match v {
        Value::Array(arr) => {
            if arr.is_empty() {
                return Ok(());
            }
            if let Some(Value::Object(obj)) = arr.first() {
                let headers: Vec<String> = obj.keys().cloned().collect();
                writer.write_record(&headers)?;
                for item in arr {
                    if let Value::Object(item_obj) = item {
                        let row: Vec<String> = headers.iter()
                            .map(|h| item_obj.get(h).map(|v| {
                                match v {
                                    Value::String(s) => s.clone(),
                                    _ => v.to_string()
                                }
                            }).unwrap_or_default())
                            .collect();
                        writer.write_record(&row)?;
                    }
                }
            }
        }
        Value::Object(obj) => {
            let headers: Vec<String> = obj.keys().cloned().collect();
            writer.write_record(&headers)?;
            let row: Vec<String> = headers.iter()
                .map(|h| obj.get(h).map(|v| {
                    match v {
                        Value::String(s) => s.clone(),
                        _ => v.to_string()
                    }
                }).unwrap_or_default())
                .collect();
            writer.write_record(&row)?;
        }
        _ => anyhow::bail!("JSON must be an object or an array of objects for CSV conversion"),
    }
    writer.flush()?;
    Ok(())
}

pub fn to_schema(text: &str) -> Result<()> {
    let v: Value = serde_json::from_str(text)?;
    let mut schema = generate_schema(&v);
    if let Value::Object(ref mut map) = schema {
        map.insert("$schema".to_string(), Value::String("http://json-schema.org/draft-07/schema#".to_string()));
        map.insert("title".to_string(), Value::String("Generated Schema".to_string()));
    }
    println!("\n{}", Theme::header("--- JSON Schema Output ---"));
    println!("{}", serde_json::to_string_pretty(&schema)?);
    Ok(())
}

fn generate_schema(value: &Value) -> Value {
    match value {
        Value::Null => serde_json::json!({"type": "null"}),
        Value::Bool(_) => serde_json::json!({"type": "boolean"}),
        Value::Number(n) => {
            if n.is_i64() || n.is_u64() {
                serde_json::json!({"type": "integer"})
            } else {
                serde_json::json!({"type": "number"})
            }
        }
        Value::String(_) => serde_json::json!({"type": "string"}),
        Value::Array(arr) => {
            let items = if let Some(first) = arr.first() {
                generate_schema(first)
            } else {
                serde_json::json!({})
            };
            serde_json::json!({"type": "array", "items": items})
        }
        Value::Object(obj) => {
            let mut properties = serde_json::Map::new();
            let mut required = Vec::new();
            for (key, val) in obj {
                properties.insert(key.clone(), generate_schema(val));
                required.push(key.clone());
            }
            let mut schema_obj = serde_json::Map::new();
            schema_obj.insert("type".to_string(), Value::String("object".to_string()));
            schema_obj.insert("properties".to_string(), Value::Object(properties));
            if !required.is_empty() {
                schema_obj.insert("required".to_string(), Value::Array(required.into_iter().map(Value::String).collect()));
            }
            Value::Object(schema_obj)
        }
    }
}
