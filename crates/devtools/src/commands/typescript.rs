use anyhow::{Result, anyhow};
use serde_json::Value;

pub fn from_json(json_str: &str, interface_name: &str) -> Result<()> {
    let v: Value = serde_json::from_str(json_str)
        .map_err(|e| anyhow!("Invalid JSON: {}", e))?;

    println!("interface {} {{", interface_name);
    generate_interface(&v, 1);
    println!("}}");

    Ok(())
}

fn generate_interface(value: &Value, indent_level: usize) {
    let indent = "  ".repeat(indent_level);
    match value {
        Value::Object(map) => {
            for (key, val) in map {
                match val {
                    Value::Null => println!("{}{}: any;", indent, key),
                    Value::Bool(_) => println!("{}{}: boolean;", indent, key),
                    Value::Number(_) => println!("{}{}: number;", indent, key),
                    Value::String(_) => println!("{}{}: string;", indent, key),
                    Value::Array(arr) => {
                        let type_str = if let Some(first) = arr.first() {
                            get_type_name(first)
                        } else {
                            "any".to_string()
                        };
                        println!("{}{}: {}[];", indent, key, type_str);
                    }
                    Value::Object(_) => {
                        println!("{}{}: {{", indent, key);
                        generate_interface(val, indent_level + 1);
                        println!("{}}};", indent);
                    }
                }
            }
        }
        _ => {}
    }
}

fn get_type_name(value: &Value) -> String {
    match value {
        Value::Null => "any".to_string(),
        Value::Bool(_) => "boolean".to_string(),
        Value::Number(_) => "number".to_string(),
        Value::String(_) => "string".to_string(),
        Value::Array(arr) => {
            if let Some(first) = arr.first() {
                format!("{}[]", get_type_name(first))
            } else {
                "any[]".to_string()
            }
        }
        Value::Object(_) => "object".to_string(), // Simplified
    }
}
