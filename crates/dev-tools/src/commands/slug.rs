use anyhow::Result;

pub fn generate(text: &str) -> Result<()> {
    let slug = text
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() || c == '-' || c == '_' {
                '-'
            } else {
                '\0'
            }
        })
        .filter(|&c| c != '\0')
        .collect::<String>();

    // Remove duplicate hyphens
    let mut result = String::new();
    let mut last_was_hyphen = false;
    for c in slug.trim_matches('-').chars() {
        if c == '-' {
            if !last_was_hyphen {
                result.push(c);
            }
            last_was_hyphen = true;
        } else {
            result.push(c);
            last_was_hyphen = false;
        }
    }

    println!("{}", result);
    Ok(())
}
