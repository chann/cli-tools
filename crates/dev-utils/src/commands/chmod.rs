use anyhow::Result;

pub fn calculate(input: &str) -> Result<()> {
    if input.len() == 3 && input.chars().all(|c| c.is_ascii_digit()) {
        // Numeric to Symbolic
        let octal = input;
        let mut symbolic = String::new();
        for c in octal.chars() {
            let n = c.to_digit(8).ok_or_else(|| anyhow::anyhow!("Invalid octal digit: {}", c))?;
            symbolic.push_str(if n & 4 != 0 { "r" } else { "-" });
            symbolic.push_str(if n & 2 != 0 { "w" } else { "-" });
            symbolic.push_str(if n & 1 != 0 { "x" } else { "-" });
        }
        println!("Numeric: {}", octal);
        println!("Symbolic: {}", symbolic);
    } else if input.len() == 9 || input.len() == 10 {
        // Symbolic to Numeric
        let sym = if input.len() == 10 { &input[1..] } else { input };
        let mut numeric = String::new();
        for chunk in sym.as_bytes().chunks(3) {
            let mut n = 0;
            if chunk[0] == b'r' { n += 4; }
            if chunk[1] == b'w' { n += 2; }
            if chunk[2] == b'x' { n += 1; }
            numeric.push_str(&n.to_string());
        }
        println!("Symbolic: {}", input);
        println!("Numeric: {}", numeric);
    } else {
        anyhow::bail!("Invalid input. Use numeric (e.g., 755) or symbolic (e.g., rwxr-xr-x) format.");
    }
    Ok(())
}
