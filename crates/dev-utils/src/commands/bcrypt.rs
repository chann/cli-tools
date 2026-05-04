use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str, cost: Option<u32>) -> Result<()> {
    let cost = cost.unwrap_or(DEFAULT_COST);
    let hashed = hash(password, cost)?;
    println!("{}", hashed);
    Ok(())
}

pub fn verify_password(password: &str, hashed: &str) -> Result<()> {
    let valid = verify(password, hashed)?;
    if valid {
        println!("Password matches!");
    } else {
        println!("Password does NOT match.");
    }
    Ok(())
}
