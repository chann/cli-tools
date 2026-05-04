use anyhow::{Result, anyhow};
use snowid::SnowID;

pub fn generate(count: usize) -> Result<()> {
    let generator = SnowID::new(1)
        .map_err(|e| anyhow!("Failed to create SnowID generator: {}", e))?;
    
    for _ in 0..count {
        let id = generator.generate();
        println!("{}", id);
    }
    Ok(())
}

pub fn inspect(id: i64) -> Result<()> {
    println!("ID: {}", id);
    Ok(())
}
