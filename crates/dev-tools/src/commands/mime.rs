use anyhow::Result;
use std::path::Path;
use cli_core::ui::Theme;

pub fn guess(input: &str) -> Result<()> {
    println!("{}", Theme::header("Mime Type Guess"));
    
    let path = Path::new(input);
    let mime = mime_guess::from_path(path).first_or_octet_stream();
    
    println!("  {} {}", Theme::info("Input:"), Theme::value(input));
    println!("  {} {}", Theme::info("Mime Type:"), Theme::highlight(mime.to_string()));
    
    Ok(())
}

pub fn from_extension(ext: &str) -> Result<()> {
    println!("{}", Theme::header(format!("Mime Types for extension: {}", ext)));
    
    let mimes = mime_guess::from_ext(ext);
    let mut count = 0;
    
    for m in mimes {
        println!("  {} {}", Theme::success("Found:"), Theme::highlight(m.to_string()));
        count += 1;
    }
    
    if count == 0 {
        println!("  {}", Theme::warning(format!("No mime type found for extension: {}", ext)));
    } else {
        println!("\n  {}", Theme::dim(format!("Total: {} found", count)));
    }
    
    Ok(())
}
