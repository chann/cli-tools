use anyhow::Result;
use ascii85::encode;

pub fn encode_base85(input: &str) -> Result<()> {
    println!("{}", encode(input.as_bytes()));
    Ok(())
}

pub fn decode_base85(input: &str) -> Result<()> {
    let decoded = ascii85::decode(input)
        .map_err(|e| anyhow::anyhow!("Invalid Base85 input: {:?}", e))?;
    println!("{}", String::from_utf8_lossy(&decoded));
    Ok(())
}
