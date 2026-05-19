use anyhow::{Context, Result};
use std::process::{Command, Stdio};

pub fn run(command: &str, args: Vec<String>) -> Result<()> {
    let mut cmd = Command::new(command);
    cmd.args(&args)
       .stdin(Stdio::null())
       .stdout(Stdio::null())
       .stderr(Stdio::null());

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        unsafe {
            cmd.pre_exec(|| {
                extern "C" {
                    fn setsid() -> i32;
                }
                setsid();
                Ok(())
            });
        }
    }

    let child = cmd.spawn().context(format!("Failed to spawn command '{}'", command))?;
    
    println!("Successfully detached process. PID: {}", child.id());

    Ok(())
}
