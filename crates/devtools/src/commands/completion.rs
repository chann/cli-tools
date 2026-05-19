use anyhow::Result;
use clap::{Command, CommandFactory};
use clap_complete::{generate, Generator, Shell};
use std::io;

pub fn generate_completion<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, "devtools", &mut io::stdout());
}

pub fn run(shell: String) -> Result<()> {
    let mut cmd = crate::Cli::command();
    match shell.to_lowercase().as_str() {
        "bash" => generate_completion(Shell::Bash, &mut cmd),
        "zsh" => generate_completion(Shell::Zsh, &mut cmd),
        "fish" => generate_completion(Shell::Fish, &mut cmd),
        "powershell" => generate_completion(Shell::PowerShell, &mut cmd),
        "elvish" => generate_completion(Shell::Elvish, &mut cmd),
        _ => anyhow::bail!("Unsupported shell: {}", shell),
    }
    Ok(())
}
