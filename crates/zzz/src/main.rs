use anyhow::Result;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let Some(command) = args.next() else {
        anyhow::bail!("Usage: zzz <command> [args...]");
    };

    if command == "--version" || command == "-V" {
        println!("zzz {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    cli_core::command_log::run_with_system_notification(&command, args.collect())
}
