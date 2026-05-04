use anyhow::Result;
use owo_colors::OwoColorize;
use std::fs;
use std::path::Path;

pub fn analyze(input: &str, is_file: bool) -> Result<()> {
    let content = if is_file {
        fs::read_to_string(Path::new(input))?
    } else {
        input.to_string()
    };

    let lines = content.lines().count();
    let words = content.split_whitespace().count();
    let chars = content.chars().count();
    let bytes = content.len();
    let non_whitespace_chars = content.chars().filter(|c| !c.is_whitespace()).count();

    println!("{}", "Text Statistics:".bold().green());
    if is_file {
        println!("  {}: {}", "File".cyan(), input);
    }
    println!("  {}: {}", "Lines".cyan(), lines);
    println!("  {}: {}", "Words".cyan(), words);
    println!("  {}: {}", "Characters".cyan(), chars);
    println!("  {}: {}", "Chars (no space)".cyan(), non_whitespace_chars);
    println!("  {}: {} bytes", "Size".cyan(), bytes);

    Ok(())
}
