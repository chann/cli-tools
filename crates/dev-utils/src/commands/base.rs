use anyhow::{Result, anyhow};

pub fn convert(value: &str, from_base: u32, to_base: u32) -> Result<()> {
    let num = i64::from_str_radix(value, from_base)
        .map_err(|e| anyhow!("Failed to parse '{}' in base {}: {}", value, from_base, e))?;
    
    let result = match to_base {
        2 => format!("{:b}", num),
        8 => format!("{:o}", num),
        10 => format!("{}", num),
        16 => format!("{:x}", num),
        _ => return Err(anyhow!("Unsupported target base: {}. Supported: 2, 8, 10, 16", to_base)),
    };
    
    println!("{}", result);
    Ok(())
}
