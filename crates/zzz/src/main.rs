use anyhow::{Context, Result};
use clap::builder::styling::{AnsiColor, Styles};
use clap::{ColorChoice, CommandFactory, FromArgMatches, Parser};
use std::io::Write;
use std::process::ExitCode;

const HELP_STYLES: Styles = Styles::styled()
    .header(AnsiColor::Cyan.on_default().bold())
    .usage(AnsiColor::Cyan.on_default().bold())
    .literal(AnsiColor::Green.on_default().bold())
    .placeholder(AnsiColor::Yellow.on_default())
    .error(AnsiColor::Red.on_default().bold())
    .valid(AnsiColor::Green.on_default())
    .invalid(AnsiColor::Yellow.on_default());

const AFTER_HELP: &str = "\
Examples:
  zzz cargo test                 Run in the background and notify on completion
  zzz --wait cargo test          Wait and return cargo's exit status
  zzz --print-log make build     Print the background log path
  zzz --no-notify long-task      Skip the completion notification
  zzz -- rg --files -g '*.rs'    Explicitly end zzz options before the command";

#[derive(Debug, Parser)]
#[command(
    name = "zzz",
    version,
    about = "Run a command quietly in the background",
    long_about = "Run a command quietly in the background through your interactive shell, \
                  save its output under ~/.commands, and notify you when it finishes. \
                  Shell aliases and functions are supported.",
    trailing_var_arg = true,
    styles = HELP_STYLES,
    after_help = AFTER_HELP
)]
struct Cli {
    /// Disable the completion notification
    #[arg(long)]
    no_notify: bool,

    /// Wait for completion and return the command's exit status
    #[arg(short, long)]
    wait: bool,

    /// Print the command log path after launch
    #[arg(short, long)]
    print_log: bool,

    /// Control help and error colors
    #[arg(long, value_enum, default_value_t = ColorChoice::Auto)]
    color: ColorChoice,

    /// Command and arguments to run
    #[arg(
        required = true,
        num_args = 1..,
        allow_hyphen_values = true,
        value_name = "COMMAND"
    )]
    command: Vec<String>,
}

fn main() -> ExitCode {
    match run() {
        Ok(exit_code) => exit_code,
        Err(error) => {
            eprintln!("Error: {error:#}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<ExitCode> {
    let args: Vec<String> = std::env::args().collect();

    if args.get(1).map(String::as_str) == Some(cli_core::command_log::TERMINAL_FOCUS_FLAG) {
        let kind = args.get(2).context("Missing terminal focus kind")?;
        let locator = args.get(3).context("Missing terminal focus locator")?;
        if args.len() != 4 {
            anyhow::bail!("Unexpected terminal focus arguments");
        }
        cli_core::command_log::focus_terminal(kind, locator)?;
        return Ok(ExitCode::SUCCESS);
    }

    if args.get(1).map(String::as_str) == Some(cli_core::command_log::TERMINAL_NOTIFICATION_FLAG) {
        let kind = args.get(2).context("Missing terminal notification kind")?;
        let locator = args
            .get(3)
            .context("Missing terminal notification locator")?;
        let outcome = args
            .get(4)
            .context("Missing terminal notification outcome")?;
        let command_name = args
            .get(5)
            .context("Missing terminal notification command")?;
        if args.len() != 6 {
            anyhow::bail!("Unexpected terminal notification arguments");
        }
        cli_core::command_log::launch_terminal_notification(kind, locator, outcome, command_name)?;
        return Ok(ExitCode::SUCCESS);
    }

    if args.get(1).map(String::as_str)
        == Some(cli_core::command_log::TERMINAL_NOTIFICATION_WORKER_FLAG)
    {
        let kind = args.get(2).context("Missing notification worker kind")?;
        let locator = args.get(3).context("Missing notification worker locator")?;
        let outcome = args.get(4).context("Missing notification worker outcome")?;
        let command_name = args.get(5).context("Missing notification worker command")?;
        if args.len() != 6 {
            anyhow::bail!("Unexpected notification worker arguments");
        }
        cli_core::command_log::run_terminal_notification_worker(
            kind,
            locator,
            outcome,
            command_name,
        )?;
        return Ok(ExitCode::SUCCESS);
    }

    let color = requested_color(&args);
    let matches = Cli::command().color(color).get_matches_from(args);
    let cli = Cli::from_arg_matches(&matches)?;
    let _ = cli.color;

    let mut command = cli.command.into_iter();
    let command_name = command.next().context("Missing command")?;
    let command_args: Vec<String> = command.collect();
    let mut spawned = if cli.no_notify {
        cli_core::command_log::spawn(&command_name, command_args)?
    } else {
        cli_core::command_log::spawn_with_system_notification(&command_name, command_args)?
    };

    if cli.print_log {
        println!("{}", spawned.log_path.display());
        std::io::stdout()
            .flush()
            .context("Failed to print command log path")?;
    }

    if !cli.wait {
        return Ok(ExitCode::SUCCESS);
    }

    let status = spawned
        .child
        .wait()
        .context("Failed to wait for background command")?;
    let code = status
        .code()
        .and_then(|code| u8::try_from(code).ok())
        .unwrap_or(1);
    Ok(ExitCode::from(code))
}

fn requested_color(args: &[String]) -> ColorChoice {
    let mut args = args.iter().skip(1);
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--color" => {
                return args
                    .next()
                    .and_then(|value| parse_color(value))
                    .unwrap_or(ColorChoice::Auto);
            }
            "--" => break,
            value if value.starts_with("--color=") => {
                return parse_color(&value["--color=".len()..]).unwrap_or(ColorChoice::Auto);
            }
            value if value.starts_with('-') => {}
            _ => break,
        }
    }
    ColorChoice::Auto
}

fn parse_color(value: &str) -> Option<ColorChoice> {
    match value {
        "always" => Some(ColorChoice::Always),
        "auto" => Some(ColorChoice::Auto),
        "never" => Some(ColorChoice::Never),
        _ => None,
    }
}
