use anyhow::Result;
use ulid::Ulid;

pub fn generate(count: usize) -> Result<()> {
    for _ in 0..count {
        println!("{}", Ulid::new());
    }
    Ok(())
}
