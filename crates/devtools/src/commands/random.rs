use anyhow::Result;
use cli_core::output::TableFormatter;
use cli_core::ui::Theme;
use rand::{thread_rng, Rng};

pub fn generate(
    kind: &str,
    count: usize,
    length: usize,
    min: usize,
    max: usize,
    numeric: bool,
    symbols: bool,
    uppercase: bool,
) -> Result<()> {
    let mut rng = thread_rng();
    let mut results = Vec::new();

    for _ in 0..count {
        let value = match kind.to_lowercase().as_str() {
            "string" | "s" => {
                let mut charset = String::from("abcdefghijklmnopqrstuvwxyz");
                if uppercase {
                    charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                }
                if numeric {
                    charset.push_str("0123456789");
                }
                if symbols {
                    charset.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?");
                }

                (0..length)
                    .map(|_| {
                        let idx = rng.gen_range(0..charset.len());
                        charset.chars().nth(idx).unwrap()
                    })
                    .collect::<String>()
            }
            "number" | "n" | "int" => {
                let val = rng.gen_range(min..=max);
                val.to_string()
            }
            "boolean" | "bool" | "b" => {
                let val: bool = rng.gen();
                val.to_string()
            }
            _ => anyhow::bail!(
                "Unsupported random kind: {}. Supported: string, number, boolean",
                kind
            ),
        };
        results.push(value);
    }

    if count == 1 {
        println!("{} {}", Theme::success(format!("Random {}:", kind)), Theme::highlight(&results[0]));
    } else {
        let mut table = TableFormatter::create_table();
        table.set_header(vec![
            TableFormatter::header_cell("#"),
            TableFormatter::header_cell("Value"),
        ]);

        for (i, val) in results.iter().enumerate() {
            table.add_row(vec![
                TableFormatter::value_cell(i + 1),
                TableFormatter::highlight_cell(val),
            ]);
        }

        println!("{}", Theme::header(format!("Generated {} random {}s", count, kind)));
        println!("{}", table);
    }

    Ok(())
}
