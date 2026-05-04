use anyhow::Result;
use serde_json::Value;
use owo_colors::OwoColorize;

pub fn compare(left_str: &str, right_str: &str) -> Result<()> {
    let left: Value = serde_json::from_str(left_str)?;
    let right: Value = serde_json::from_str(right_str)?;

    println!("{}", "--- JSON Diff ---".bold().cyan());
    
    diff_values(&left, &right, "$");

    Ok(())
}

fn diff_values(left: &Value, right: &Value, path: &str) {
    match (left, right) {
        (Value::Object(l_map), Value::Object(r_map)) => {
            for (k, v) in l_map {
                let new_path = format!("{}.{}", path, k);
                if let Some(rv) = r_map.get(k) {
                    diff_values(v, rv, &new_path);
                } else {
                    println!("{} {}: {}", "-".red(), new_path, v);
                }
            }
            for (k, v) in r_map {
                if !l_map.contains_key(k) {
                    let new_path = format!("{}.{}", path, k);
                    println!("{} {}: {}", "+".green(), new_path, v);
                }
            }
        }
        (Value::Array(l_arr), Value::Array(r_arr)) => {
            let len = std::cmp::max(l_arr.len(), r_arr.len());
            for i in 0..len {
                let new_path = format!("{}[{}]", path, i);
                match (l_arr.get(i), r_arr.get(i)) {
                    (Some(lv), Some(rv)) => diff_values(lv, rv, &new_path),
                    (Some(lv), None) => println!("{} {}: {}", "-".red(), new_path, lv),
                    (None, Some(rv)) => println!("{} {}: {}", "+".green(), new_path, rv),
                    (None, None) => unreachable!(),
                }
            }
        }
        (lv, rv) if lv != rv => {
            println!("{} {}:", "≠".yellow(), path);
            println!("  {} {}", "-".red(), lv);
            println!("  {} {}", "+".green(), rv);
        }
        _ => {}
    }
}
