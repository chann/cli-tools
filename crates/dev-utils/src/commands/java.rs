use anyhow::{Result, anyhow};
use serde_json::Value;
use heck::{ToPascalCase, ToLowerCamelCase};

pub fn from_json(json_str: &str, class_name: &str) -> Result<()> {
    let v: Value = serde_json::from_str(json_str)
        .map_err(|e| anyhow!("Invalid JSON: {}", e))?;

    println!("import lombok.Data;");
    println!("import com.fasterxml.jackson.annotation.JsonProperty;\n");
    
    generate_class(&v, class_name);

    Ok(())
}

fn generate_class(value: &Value, name: &str) {
    let pascal_name = name.to_pascal_case();
    println!("@Data");
    println!("public class {} {{", pascal_name);

    let mut nested_classes = Vec::new();

    if let Value::Object(map) = value {
        for (key, val) in map {
            let field_name = key.to_lower_camel_case();
            let (type_name, nested) = get_java_type(val, key);
            
            println!("    @JsonProperty(\"{}\")", key);
            println!("    private {} {};", type_name, field_name);

            if let Some(n) = nested {
                nested_classes.push(n);
            }
        }
    }

    println!("}}\n");

    for (n, v) in nested_classes {
        generate_class(&v, &n);
    }
}

fn get_java_type(value: &Value, field_name: &str) -> (String, Option<(String, Value)>) {
    match value {
        Value::Null => ("Object".to_string(), None),
        Value::Bool(_) => ("Boolean".to_string(), None),
        Value::Number(n) => {
            if n.is_i64() {
                ("Long".to_string(), None)
            } else {
                ("Double".to_string(), None)
            }
        }
        Value::String(_) => ("String".to_string(), None),
        Value::Array(arr) => {
            if let Some(first) = arr.first() {
                let (inner_type, nested) = get_java_type(first, field_name);
                (format!("List<{}>", inner_type), nested)
            } else {
                ("List<Object>".to_string(), None)
            }
        }
        Value::Object(_) => {
            let class_name = field_name.to_pascal_case();
            (class_name.clone(), Some((class_name, value.clone())))
        }
    }
}
