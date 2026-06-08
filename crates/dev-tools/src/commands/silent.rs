use anyhow::Result;

pub fn run(command: &str, args: Vec<String>) -> Result<()> {
    cli_core::command_log::run(command, args)
}
