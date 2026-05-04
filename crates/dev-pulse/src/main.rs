mod scanner;
mod git;
mod health;
mod env;
mod changelog;
mod commit;

use anyhow::Result;
use clap::{Parser, Subcommand};
use cli_core::ui::Theme;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "dev-pulse",
    version,
    about = "Developer pulse check and convenience tools",
    long_about = "A collection of tools to keep your development environment healthy. \
                  Includes branch cleanup, TODO scanning, and project health checks."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Target directory
    #[arg(short, long, value_name = "PATH", default_value = ".")]
    path: PathBuf,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List and clean up merged git branches
    Cleanup {
        /// Actually delete the branches (dry-run by default)
        #[arg(short, long)]
        force: bool,

        /// Target branch to check against (default: main or master)
        #[arg(short, long)]
        target: Option<String>,
    },
    /// Scan for TODOs, FIXMEs, and other markers
    Scan {
        /// Show only these markers (e.g., TODO, FIXME)
        #[arg(short, long)]
        markers: Option<Vec<String>>,

        /// Include hidden files
        #[arg(long)]
        hidden: bool,
    },
    /// Check project health (README, LICENSE, etc.)
    Health {
        /// Show detailed check results
        #[arg(short, long)]
        verbose: bool,
    },
    /// Validate .env file parity with .env.example
    Env,
    /// Generate a changelog from git history
    Changelog {
        /// Start from this ref (e.g., tag or branch)
        #[arg(short, long)]
        from: Option<String>,

        /// End at this ref (default: HEAD)
        #[arg(short, long)]
        to: Option<String>,

        /// Limit the number of commits
        #[arg(short, long)]
        limit: Option<usize>,
    },
    /// Interactive conventional commit wizard
    Commit,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Cleanup { force, target } => {
            println!("{}", Theme::header("🧹 Git Branch Cleanup"));
            git::cleanup(&cli.path, force, target.as_deref())?;
        }
        Commands::Scan { markers, hidden } => {
            println!("{}", Theme::header("🔍 Marker Scanner"));
            scanner::scan(&cli.path, markers, hidden).await?;
        }
        Commands::Health { verbose } => {
            println!("{}", Theme::header("🏥 Project Health Check"));
            health::check(&cli.path, verbose)?;
        }
        Commands::Env => {
            println!("{}", Theme::header("🔐 .env Validator"));
            env::check(&cli.path)?;
        }
        Commands::Changelog { from, to, limit } => {
            println!("{}", Theme::header("📜 Changelog Generator"));
            changelog::generate(
                &cli.path,
                changelog::ChangelogOptions { from, to, limit },
            )?;
        }
        Commands::Commit => {
            commit::wizard(&cli.path)?;
        }
    }

    Ok(())
}
