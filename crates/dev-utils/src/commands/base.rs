use anyhow::{Result, anyhow};
use cli_core::ui::Theme;
use cli_core::output::TableFormatter;

pub fn convert(value: &str, from_base: u32, to_base: Option<u32>, all: bool) -> Result<()> {
    if from_base < 2 || from_base > 36 {
        return Err(anyhow!("Source base must be between 2 and 36"));
    }

    let num = i64::from_str_radix(value, from_base)
        .map_err(|e| anyhow!("Failed to parse '{}' in base {}: {}", value, from_base, e))?;

    if let Some(target) = to_base {
        if target < 2 || target > 36 {
            return Err(anyhow!("Target base must be between 2 and 36"));
        }
        let result = to_radix(num, target);
        println!("{}", Theme::highlight(result));
        return Ok(());
    }

    // Dashboard view
    println!("\n{}", Theme::header(format!("Base Conversion: {} (base {})", value, from_base)));
    
    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("Base"),
        TableFormatter::header_cell("Name"),
        TableFormatter::header_cell("Value"),
    ]);

    let targets = if all {
        vec![2, 8, 10, 16, 32, 36]
    } else {
        vec![2, 8, 10, 16]
    };

    for t in targets {
        let name = match t {
            2 => "Binary",
            8 => "Octal",
            10 => "Decimal",
            16 => "Hexadecimal",
            32 => "Base32",
            36 => "Base36",
            _ => "Custom",
        };
        table.add_row(vec![
            TableFormatter::value_cell(t),
            TableFormatter::value_cell(name),
            TableFormatter::highlight_cell(to_radix(num, t)),
        ]);
    }

    println!("{}", table);
    Ok(())
}

fn to_radix(mut n: i64, base: u32) -> String {
    if n == 0 {
        return "0".to_string();
    }
    
    let mut result = String::new();
    let is_negative = n < 0;
    if is_negative {
        n = n.abs();
    }

    let base = base as i64;
    let digits = "0123456789abcdefghijklmnopqrstuvwxyz";

    while n > 0 {
        let rem = n % base;
        result.push(digits.chars().nth(rem as usize).unwrap());
        n /= base;
    }

    if is_negative {
        result.push('-');
    }

    result.chars().rev().collect()
}
