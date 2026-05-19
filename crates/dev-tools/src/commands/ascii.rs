use anyhow::Result;
use figlet_rs::FIGlet;

pub fn generate(text: &str, font_path: Option<String>) -> Result<()> {
    let font = if let Some(path) = font_path {
        FIGlet::from_file(&path).map_err(|_| anyhow::anyhow!("Failed to load font from {}", path))?
    } else {
        FIGlet::standard().map_err(|_| anyhow::anyhow!("Failed to load standard font"))?
    };

    let figure = font.convert(text);
    if let Some(fig) = figure {
        println!("{}", fig);
    } else {
        anyhow::bail!("Failed to convert text to ASCII art");
    }

    Ok(())
}
