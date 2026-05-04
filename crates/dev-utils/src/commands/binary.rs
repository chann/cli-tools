use anyhow::Result;

pub fn to_binary(input: &str) -> Result<()> {
    let result: String = input.as_bytes()
        .iter()
        .map(|b| format!("{:08b}", b))
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", result);
    Ok(())
}

pub fn from_binary(input: &str) -> Result<()> {
    let clean_input = input.replace(' ', "");
    if clean_input.len() % 8 != 0 {
        anyhow::bail!("Invalid binary string length. Must be a multiple of 8.");
    }
    
    let mut bytes = Vec::new();
    for i in (0..clean_input.len()).step_by(8) {
        let byte_str = &clean_input[i..i+8];
        let byte = u8::from_str_radix(byte_str, 2)
            .map_err(|_| anyhow::anyhow!("Invalid binary digit at {}", byte_str))?;
        bytes.push(byte);
    }
    
    println!("{}", String::from_utf8_lossy(&bytes));
    Ok(())
}
