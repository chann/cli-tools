use anyhow::Result;
use serde_json::Value;
use owo_colors::OwoColorize;
use cli_core::ui::Theme;
use cli_core::output::TableFormatter;

pub fn compare(left_str: &str, right_str: &str) -> Result<()> {
    let left: Value = serde_json::from_str(left_str)?;
    let right: Value = serde_json::from_str(right_str)?;

    println!("{}", Theme::header("--- JSON Structural Diff ---"));
    
    let mut differences = Vec::new();
    diff_values(&left, &right, "$", &mut differences);

    if differences.is_empty() {
        println!("{}", Theme::success("No differences found. JSON objects are structurally identical."));
        return Ok(());
    }

    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("Path"),
        TableFormatter::header_cell("Diff"),
        TableFormatter::header_cell("Details"),
    ]);

    for (path, diff_type, details) in differences {
        let (type_label, details_label) = match diff_type {
            DiffType::Added => (Theme::green("+ Added"), Theme::dim(details)),
            DiffType::Removed => (Theme::red("- Removed"), Theme::dim(details)),
            DiffType::Changed => (Theme::yellow("≠ Changed"), details),
        };

        table.add_row(vec![
            TableFormatter::highlight_cell(path),
            TableFormatter::value_cell(type_label),
            TableFormatter::value_cell(details_label),
        ]);
    }

    println!("{}", table);

    Ok(())
}

enum DiffType {
    Added,
    Removed,
    Changed,
}

fn diff_values(left: &Value, right: &Value, path: &str, differences: &mut Vec<(String, DiffType, String)>) {
    match (left, right) {
        (Value::Object(l_map), Value::Object(r_map)) => {
            for (k, v) in l_map {
                let new_path = format!("{}.{}", path, k);
                if let Some(rv) = r_map.get(k) {
                    diff_values(v, rv, &new_path, differences);
                } else {
                    differences.push((new_path, DiffType::Removed, v.to_string()));
                }
            }
            for (k, v) in r_map {
                if !l_map.contains_key(k) {
                    let new_path = format!("{}.{}", path, k);
                    differences.push((new_path, DiffType::Added, v.to_string()));
                }
            }
        }
        (Value::Array(l_arr), Value::Array(r_arr)) => {
            let len = std::cmp::max(l_arr.len(), r_arr.len());
            for i in 0..len {
                let new_path = format!("{}[{}]", path, i);
                match (l_arr.get(i), r_arr.get(i)) {
                    (Some(lv), Some(rv)) => diff_values(lv, rv, &new_path, differences),
                    (Some(lv), None) => differences.push((new_path, DiffType::Removed, lv.to_string())),
                    (None, Some(rv)) => differences.push((new_path, DiffType::Added, rv.to_string())),
                    (None, None) => unreachable!(),
                }
            }
        }
        (lv, rv) if lv != rv => {
            differences.push((
                path.to_string(),
                DiffType::Changed,
                format!("{} {} {}", lv.to_string().red(), "->".dimmed(), rv.to_string().green())
            ));
        }
        _ => {}
    }
}
