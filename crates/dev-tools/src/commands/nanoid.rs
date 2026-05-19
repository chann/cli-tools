use anyhow::Result;
use cli_core::ui::Theme;
use cli_core::output::TableFormatter;

pub fn generate(length: usize, count: usize, alphabet: Option<&str>) -> Result<()> {
    if count == 0 {
        return Ok(());
    }

    let alphabet_chars: Vec<char> = if let Some(a) = alphabet {
        a.chars().collect()
    } else {
        "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_-".chars().collect()
    };

    if count == 1 {
        let id = nanoid::nanoid!(length, &alphabet_chars);
        println!("{}", Theme::highlight(id));
        return Ok(());
    }

    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("#"),
        TableFormatter::header_cell("NanoID"),
    ]);

    for i in 0..count {
        let id = nanoid::nanoid!(length, &alphabet_chars);
        table.add_row(vec![
            TableFormatter::value_cell(i + 1),
            TableFormatter::highlight_cell(id),
        ]);
    }

    println!("\n{}", Theme::info(format!("Generated {} NanoIDs (Length: {}):", count, length)));
    if let Some(a) = alphabet {
        println!("{}", Theme::dim(format!("Using custom alphabet: {}", a)));
    }
    println!("{}", table);
    Ok(())
}
