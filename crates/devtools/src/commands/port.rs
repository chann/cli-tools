use anyhow::Result;
use std::time::{Duration, Instant};
use std::net::TcpStream;
use std::process::Command;

pub fn wait(port: u16, timeout: u64) -> Result<()> {
    let start = Instant::now();
    let timeout_duration = Duration::from_secs(timeout);
    let delay = Duration::from_millis(500);

    println!("Waiting for port {} to become active (timeout: {}s)...", port, timeout);

    while start.elapsed() < timeout_duration {
        if TcpStream::connect(format!("127.0.0.1:{}", port)).is_ok() {
            println!("Port {} is now active!", port);
            return Ok(());
        }
        std::thread::sleep(delay);
    }

    anyhow::bail!("Timeout waiting for port {} after {}s", port, timeout)
}

pub fn list() -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        let output = Command::new("lsof")
            .arg("-i")
            .arg("-P")
            .arg("-n")
            .arg("-sTCP:LISTEN")
            .output()?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
    }

    #[cfg(target_os = "linux")]
    {
        let output = Command::new("ss")
            .arg("-lntp")
            .output()?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
    }

    Ok(())
}

pub fn check(port: u16) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        let output = Command::new("lsof")
            .arg("-i")
            .arg(format!(":{}", port))
            .output()?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.is_empty() {
            println!("No process found on port {}", port);
        } else {
            println!("{}", stdout);
        }
    }

    #[cfg(target_os = "linux")]
    {
        let output = Command::new("ss")
            .arg("-lntp")
            .arg(format!("sport = :{}", port))
            .output()?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.is_empty() {
            println!("No process found on port {}", port);
        } else {
            println!("{}", stdout);
        }
    }

    Ok(())
}

pub fn kill(port: u16) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        let output = Command::new("lsof")
            .arg("-t")
            .arg("-i")
            .arg(format!(":{}", port))
            .output()?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        for pid_str in stdout.lines() {
            if let Ok(pid) = pid_str.parse::<u32>() {
                println!("Killing process {} on port {}", pid, port);
                Command::new("kill").arg("-9").arg(pid.to_string()).status()?;
            }
        }
    }
    Ok(())
}
