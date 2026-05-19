use anyhow::Result;
use std::fs;
use std::path::Path;
use termimad::MadSkin;

pub fn render(path: &str) -> Result<()> {
    let content = if Path::new(path).exists() {
        fs::read_to_string(path)?
    } else {
        path.to_string()
    };

    let skin = MadSkin::default();
    
    skin.print_text(&content);
    
    Ok(())
}
