use anyhow::Result;
use rand::Rng;

pub fn generate(length: usize, kind: &str) -> Result<()> {
    let chars = match kind.to_lowercase().as_str() {
        "django" => "abcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*(-_=+)",
        "rails" | "hex" => "0123456789abcdef",
        "alphanumeric" => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
        _ => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+[]{}|;:,.<>?",
    };

    let mut rng = rand::thread_rng();
    let secret: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..chars.len());
            chars.chars().nth(idx).unwrap()
        })
        .collect();

    println!("{}", secret);
    Ok(())
}
