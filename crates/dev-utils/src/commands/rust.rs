use anyhow::{Result, anyhow};
use serde_json::Value;
use heck::{ToPascalCase, ToSnakeCase};

pub fn from_json(json_str: &str, struct_name: &str) -> Result<()> {
    let v: Value = serde_json::from_str(json_str)
        .map_err(|e| anyhow!("Invalid JSON: {}", e))?;

    println!("use serde::{{Serialize, Deserialize}};\n");
    generate_struct(&v, struct_name);

    Ok(())
}

fn generate_struct(value: &Value, name: &str) {
    let pascal_name = name.to_pascal_case();
    println!("#[derive(Debug, Serialize, Deserialize)]");
    println!("pub struct {} {{", pascal_name);

    let mut nested_structs = Vec::new();

    if let Value::Object(map) = value {
        for (key, val) in map {
            let field_name = key.to_snake_case();
            let (type_name, nested) = get_rust_type(val, key);
            
            if field_name != *key {
                println!("    #[serde(rename = \"{}\")]", key);
            }
            println!("    pub {}: {},", field_name, type_name);

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

fn get_rust_type(value: &Value, field_name: &str) -> (String, Option<(String, Value)>) {
    match value {
        Value::Null => ("Option<Value>".to_string(), None),
        Value::Bool(_) => ("bool".to_string(), None),
        Value::Number(n) => {
            if n.is_i64() {
                ("i64".to_string(), None)
            } else {
                ("f64".to_string(), None)
            }
        }
        Value::String(_) => ("String".to_string(), None),
        Value::Array(arr) => {
            if let Some(first) = arr.first() {
                let (inner_type, nested) = get_rust_type(first, field_name);
                (format!("Vec<{}>", inner_type), nested)
            } else {
                ("Vec<Value>".to_string(), None)
            }
        }
        Value::Object(_) => {
            let struct_name = field_name.to_pascal_case();
            (struct_name.clone(), Some((struct_name, value.clone())))
        }
    }
}
