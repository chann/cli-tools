use anyhow::Result;
use std::path::Path;

pub fn guess(input: &str) -> Result<()> {
    let mime = mime_guess::from_path(Path::new(input)).first_or_octet_stream();
    println!("{}", mime);
    Ok(())
}

pub fn from_extension(ext: &str) -> Result<()> {
    let mimes = mime_guess::from_ext(ext);
    let mut found = false;
    for m in mimes {
        println!("{}", m);
        found = true;
    }
    if !found {
        println!("No mime type found for extension: {}", ext);
    }
    Ok(())
}
