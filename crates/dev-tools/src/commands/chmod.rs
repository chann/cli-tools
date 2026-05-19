use anyhow::Result;
use cli_core::output::TableFormatter;
use cli_core::ui::Theme;

pub fn calculate(input: &str) -> Result<()> {
    let (numeric, symbolic) = if input.len() == 3 && input.chars().all(|c| c.is_ascii_digit()) {
        // Numeric to Symbolic
        let octal = input;
        let mut sym = String::new();
        for c in octal.chars() {
            let n = c.to_digit(8).ok_or_else(|| anyhow::anyhow!("Invalid octal digit: {}", c))?;
            sym.push_str(if n & 4 != 0 { "r" } else { "-" });
            sym.push_str(if n & 2 != 0 { "w" } else { "-" });
            sym.push_str(if n & 1 != 0 { "x" } else { "-" });
        }
        (octal.to_string(), sym)
    } else if input.len() == 9 || input.len() == 10 {
        // Symbolic to Numeric
        let sym_raw = if input.len() == 10 { &input[1..] } else { input };
        let mut num = String::new();
        for chunk in sym_raw.as_bytes().chunks(3) {
            let mut n = 0;
            if chunk[0] == b'r' { n += 4; }
            if chunk[1] == b'w' { n += 2; }
            if chunk[2] == b'x' { n += 1; }
            num.push_str(&n.to_string());
        }
        (num, sym_raw.to_string())
    } else {
        anyhow::bail!("Invalid input. Use numeric (e.g., 755) or symbolic (e.g., rwxr-xr-x) format.");
    };

    println!("{}", Theme::header("Chmod Permission Breakdown"));
    println!("  {} {} | {} {}", Theme::info("Numeric:"), Theme::value(&numeric), Theme::info("Symbolic:"), Theme::value(&symbolic));
    println!();

    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("Class"),
        TableFormatter::header_cell("Read (4)"),
        TableFormatter::header_cell("Write (2)"),
        TableFormatter::header_cell("Execute (1)"),
        TableFormatter::header_cell("Total"),
    ]);

    let classes = ["Owner", "Group", "Others"];
    let sym_chars: Vec<char> = symbolic.chars().collect();
    let num_chars: Vec<char> = numeric.chars().collect();

    for (i, class) in classes.iter().enumerate() {
        let r = if sym_chars[i * 3] == 'r' { Theme::success("Yes") } else { Theme::dim("No") };
        let w = if sym_chars[i * 3 + 1] == 'w' { Theme::success("Yes") } else { Theme::dim("No") };
        let x = if sym_chars[i * 3 + 2] == 'x' { Theme::success("Yes") } else { Theme::dim("No") };
        
        table.add_row(vec![
            TableFormatter::value_cell(class),
            r.into(),
            w.into(),
            x.into(),
            TableFormatter::highlight_cell(num_chars[i].to_string()),
        ]);
    }

    println!("{}", table);
    Ok(())
}
