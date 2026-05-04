use anyhow::Result;
use std::env;

pub fn list(filter: Option<String>) -> Result<()> {
    let vars = env::vars();
    
    for (key, value) in vars {
        if let Some(ref f) = filter {
            if key.to_lowercase().contains(&f.to_lowercase()) {
                println!("{}={}", key, value);
            }
        } else {
            println!("{}={}", key, value);
        }
    }
    
    Ok(())
}
