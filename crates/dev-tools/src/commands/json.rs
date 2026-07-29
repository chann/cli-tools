use anyhow::{anyhow, Result};
use clap::ValueEnum;
use cli_core::ui::Theme;
use jsonpath_rust::JsonPath;
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Serialize, Serializer};
use serde_json::Value;

fn parse_json(text: &str) -> Result<Value> {
    serde_json::from_str(text).map_err(|error| anyhow!("Invalid JSON: {error}"))
}

pub fn validate(text: &str) -> Result<()> {
    parse_json(text).map(|_| ())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum SortOrder {
    Asc,
    Desc,
}

struct SortedValue<'a> {
    value: &'a Value,
    order: SortOrder,
}

impl Serialize for SortedValue<'_> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.value {
            Value::Null => serializer.serialize_unit(),
            Value::Bool(value) => serializer.serialize_bool(*value),
            Value::Number(value) => value.serialize(serializer),
            Value::String(value) => serializer.serialize_str(value),
            Value::Array(values) => {
                let mut sequence = serializer.serialize_seq(Some(values.len()))?;
                for value in values {
                    sequence.serialize_element(&SortedValue {
                        value,
                        order: self.order,
                    })?;
                }
                sequence.end()
            }
            Value::Object(values) => {
                let mut object = serializer.serialize_map(Some(values.len()))?;
                let mut entries: Vec<_> = values.iter().collect();
                entries.sort_unstable_by_key(|(key, _)| *key);
                if self.order == SortOrder::Desc {
                    entries.reverse();
                }
                for (key, value) in entries {
                    object.serialize_entry(
                        key,
                        &SortedValue {
                            value,
                            order: self.order,
                        },
                    )?;
                }
                object.end()
            }
        }
    }
}

fn serialize_json(value: &Value, pretty: bool, sort: Option<SortOrder>) -> Result<String> {
    let output = match sort {
        Some(order) => {
            let sorted = SortedValue { value, order };
            if pretty {
                serde_json::to_string_pretty(&sorted)?
            } else {
                serde_json::to_string(&sorted)?
            }
        }
        None if pretty => serde_json::to_string_pretty(value)?,
        None => serde_json::to_string(value)?,
    };
    Ok(output)
}

pub fn transform(
    text: &str,
    pretty: bool,
    query: Option<&str>,
    sort: Option<SortOrder>,
) -> Result<String> {
    let value = parse_json(text)?;

    let result = if let Some(query) = query {
        let matches = value
            .query(query)
            .map_err(|error| anyhow!("Invalid JSON Path: {error}"))?;
        Value::Array(matches.into_iter().cloned().collect())
    } else {
        value
    };

    serialize_json(&result, pretty, sort)
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

#[cfg(test)]
mod tests {
    use super::{serialize_json, transform, validate, SortOrder};
    use serde_json::Value;

    #[test]
    fn validate_accepts_valid_json() {
        assert!(validate(r#"{"a":[1,true,null]}"#).is_ok());
    }

    #[test]
    fn validate_reports_invalid_json_location() {
        let error = validate("{\n  \"a\":\n}")
            .expect_err("input should be invalid")
            .to_string();

        assert!(error.starts_with("Invalid JSON:"));
        assert!(error.contains("line 3 column 1"));
    }

    #[test]
    fn serializer_pretty_prints_without_sorting() {
        let value: Value = serde_json::from_str(r#"{"b":2,"a":1}"#).unwrap();

        let output = serialize_json(&value, true, None).unwrap();

        assert_eq!(output, "{\n  \"a\": 1,\n  \"b\": 2\n}");
    }

    #[test]
    fn serializer_minifies_without_sorting() {
        let value: Value = serde_json::from_str(r#"{ "b": 2, "a": 1 }"#).unwrap();

        let output = serialize_json(&value, false, None).unwrap();

        assert_eq!(output, r#"{"a":1,"b":2}"#);
    }

    #[test]
    fn sorted_serializer_orders_every_object_ascending() {
        let value: Value =
            serde_json::from_str(r#"{"z":{"b":2,"a":1},"items":[{"d":4,"c":3},0]}"#).unwrap();

        let output = serialize_json(&value, false, Some(SortOrder::Asc)).unwrap();

        assert_eq!(output, r#"{"items":[{"c":3,"d":4},0],"z":{"a":1,"b":2}}"#);
    }

    #[test]
    fn sorted_serializer_orders_every_object_descending() {
        let value: Value =
            serde_json::from_str(r#"{"a":{"x":1,"z":3},"m":[{"a":1,"b":2},0],"z":0}"#).unwrap();

        let output = serialize_json(&value, false, Some(SortOrder::Desc)).unwrap();

        assert_eq!(output, r#"{"z":0,"m":[{"b":2,"a":1},0],"a":{"z":3,"x":1}}"#);
    }

    #[test]
    fn transform_sorts_jsonpath_matches() {
        let output = transform(
            r#"{"payload":{"a":1,"b":2}}"#,
            false,
            Some("$.payload"),
            Some(SortOrder::Desc),
        )
        .unwrap();

        assert_eq!(output, r#"[{"b":2,"a":1}]"#);
    }
}
