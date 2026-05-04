use uuid::Uuid;
use anyhow::Result;

pub fn generate(count: usize, v7: bool) -> Result<()> {
    for _ in 0..count {
        if v7 {
            println!("{}", Uuid::now_v7());
        } else {
            println!("{}", Uuid::new_v4());
        }
    }
    Ok(())
}
