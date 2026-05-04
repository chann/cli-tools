use anyhow::Result;
use nanoid::nanoid;

pub fn generate(length: usize) -> Result<()> {
    if length == 0 {
        println!("{}", nanoid!());
    } else {
        println!("{}", nanoid!(length));
    }
    Ok(())
}
