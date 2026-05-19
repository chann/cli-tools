use anyhow::{Result, anyhow};
use serde_json::Value;
use heck::ToPascalCase;

pub fn from_json(json_str: &str, struct_name: &str) -> Result<()> {
    let v: Value = serde_json::from_str(json_str)
        .map_err(|e| anyhow!("Invalid JSON: {}", e))?;

    generate_struct(&v, struct_name);

    Ok(())
}

fn generate_struct(value: &Value, name: &str) {
    let pascal_name = name.to_pascal_case();
    println!("type {} struct {{", pascal_name);

    let mut nested_structs = Vec::new();

    if let Value::Object(map) = value {
        for (key, val) in map {
            let field_name = key.to_pascal_case();
            let (type_name, nested) = get_go_type(val, key);
            
            println!("    {} {} `json:\"{}\"`", field_name, type_name, key);

            if let Some(n) = nested {
                nested_structs.push(n);
            }
        }
    }

    println!("}}\n");

    for (n, v) in nested_structs {
        generate_struct(&v, &n);
    }
}

fn get_go_type(value: &Value, field_name: &str) -> (String, Option<(String, Value)>) {
    match value {
        Value::Null => ("interface{}".to_string(), None),
        Value::Bool(_) => ("bool".to_string(), None),
        Value::Number(n) => {
            if n.is_i64() {
                ("int64".to_string(), None)
            } else {
                ("float64".to_string(), None)
            }
        }
        Value::String(_) => ("string".to_string(), None),
        Value::Array(arr) => {
            if let Some(first) = arr.first() {
                let (inner_type, nested) = get_go_type(first, field_name);
                (format!("[]{}", inner_type), nested)
            } else {
                ("[]interface{}".to_string(), None)
            }
        }
        Value::Object(_) => {
            let struct_name = field_name.to_pascal_case();
            (struct_name.clone(), Some((struct_name, value.clone())))
        }
    }
}
