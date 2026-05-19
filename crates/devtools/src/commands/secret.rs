use anyhow::Result;
use rand::Rng;
use cli_core::ui::Theme;
use cli_core::output::TableFormatter;

pub fn generate(length: usize, count: usize, kind: &str) -> Result<()> {
    if count == 0 {
        return Ok(());
    }

    let chars = match kind.to_lowercase().as_str() {
        "django" => "abcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*(-_=+)",
        "rails" | "hex" | "flask" | "express" => "0123456789abcdef",
        "alphanumeric" => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
        "base64" => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+/",
        "url-safe" => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_",
        _ => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+[]{}|;:,.<>?",
    };

    let char_vec: Vec<char> = chars.chars().collect();
    let mut rng = rand::thread_rng();

    let generate_one = |len: usize, cv: &[char], r: &mut rand::rngs::ThreadRng| -> String {
        (0..len)
            .map(|_| {
                let idx = r.gen_range(0..cv.len());
                cv[idx]
            })
            .collect()
    };

    if count == 1 {
        let secret = generate_one(length, &char_vec, &mut rng);
        println!("{}", Theme::highlight(secret));
        return Ok(());
    }

    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("#"),
        TableFormatter::header_cell("Secret Key"),
    ]);

    for i in 0..count {
        let secret = generate_one(length, &char_vec, &mut rng);
        table.add_row(vec![
            TableFormatter::value_cell(i + 1),
            TableFormatter::highlight_cell(secret),
        ]);
    }

    println!("\n{}", Theme::info(format!("Generated {} Secret Keys (Kind: {}, Length: {}):", count, kind, length)));
    println!("{}", table);
    Ok(())
}
