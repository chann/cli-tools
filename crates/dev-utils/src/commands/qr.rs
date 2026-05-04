use anyhow::Result;

pub fn generate(text: &str) -> Result<()> {
    qr2term::print_qr(text).map_err(|e| anyhow::anyhow!("Failed to generate QR code: {}", e))?;
    Ok(())
}
