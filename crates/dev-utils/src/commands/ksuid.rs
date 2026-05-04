use anyhow::Result;
use ksuid::Ksuid;

pub fn generate(count: usize) -> Result<()> {
    for _ in 0..count {
        let id = Ksuid::generate();
        println!("{}", id.to_base62());
    }
    Ok(())
}

pub fn inspect(id_str: &str) -> Result<()> {
    let id = Ksuid::from_base62(id_str)
        .map_err(|_| anyhow::anyhow!("Invalid KSUID"))?;
    
    println!("ID: {}", id.to_base62());
    println!("Timestamp: {}", id.timestamp());
    println!("Payload: {:?}", id.payload());
    Ok(())
}
