use anyhow::Result;
use totp_rs::{Algorithm, Secret, TOTP};

pub fn generate(secret: &str, digits: usize, skew: u8) -> Result<()> {
    let secret = Secret::Encoded(secret.to_string()).to_bytes()?;
    
    let totp = TOTP::new(
        Algorithm::SHA1,
        digits,
        skew,
        30,
        secret,
    )?;

    let code = totp.generate_current()?;
    println!("{}", code);
    
    Ok(())
}
