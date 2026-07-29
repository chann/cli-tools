use anyhow::{Context, Result};

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let Some(command) = args.next() else {
        anyhow::bail!("Usage: zzz <command> [args...]");
    };

    if command == "--version" || command == "-V" {
        println!("zzz {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if command == cli_core::command_log::TERMINAL_FOCUS_FLAG {
        let kind = args.next().context("Missing terminal focus kind")?;
        let locator = args.next().context("Missing terminal focus locator")?;
        if args.next().is_some() {
            anyhow::bail!("Unexpected terminal focus arguments");
        }
        return cli_core::command_log::focus_terminal(&kind, &locator);
    }

    cli_core::command_log::run_with_system_notification(&command, args.collect())
}
