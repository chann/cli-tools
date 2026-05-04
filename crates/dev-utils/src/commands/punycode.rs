use anyhow::Result;

pub fn encode_puny(input: &str) -> Result<()> {
    let encoded = punycode::encode(input)
        .map_err(|_| anyhow::anyhow!("Failed to encode Punycode"))?;
    println!("xn--{}", encoded);
    Ok(())
}

pub fn decode_puny(input: &str) -> Result<()> {
    let target = input.trim_start_matches("xn--");
    let decoded = punycode::decode(target)
        .map_err(|_| anyhow::anyhow!("Failed to decode Punycode"))?;
    println!("{}", decoded);
    Ok(())
}
