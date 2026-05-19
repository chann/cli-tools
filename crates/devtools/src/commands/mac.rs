use anyhow::Result;
use mac_address::get_mac_address;
use rand::{thread_rng, Rng};

pub fn generate(count: usize) -> Result<()> {
    let mut rng = thread_rng();
    for _ in 0..count {
        let mac: [u8; 6] = rng.gen();
        println!("{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}", 
            mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]);
    }
    Ok(())
}

pub fn show_local() -> Result<()> {
    match get_mac_address()? {
        Some(mac) => println!("{}", mac),
        None => println!("No MAC address found"),
    }
    Ok(())
}
