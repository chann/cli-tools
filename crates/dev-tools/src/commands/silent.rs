use anyhow::Result;

pub fn run(command: &str, args: Vec<String>) -> Result<()> {
    dev_tools::silent_runner::run(command, args)
}
