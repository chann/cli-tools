mod git;
mod analyzer;
mod patterns;
mod summary;

use anyhow::{Context, Result};
use clap::Parser;
use comfy_table::{presets::UTF8_FULL, Cell, Color, Table};
use git::{time_estimator::TimeEstimator, CommitAnalyzer};
use owo_colors::OwoColorize;
use std::path::PathBuf;
use summary::{RepositorySummary, TotalSummary};

#[derive(Parser)]
#[command(name = "work-summary")]
#[command(about = "Analyze git commit history and summarize work activity", long_about = None)]
struct Cli {
    #[arg(value_name = "PATH", help = "Repository paths to analyze")]
    paths: Vec<PathBuf>,

    #[arg(short, long, default_value = "table", help = "Output format")]
    format: String,

    #[arg(long, help = "Export results to file")]
    export: Option<PathBuf>,

    #[arg(long, default_value = "10030", help = "Hourly rate in KRW")]
    hourly_rate: f64,

    #[arg(long, help = "Show simple summary only")]
    simple: bool,

    #[arg(long, help = "Show detailed analysis")]
    detail: bool,

    #[arg(long, help = "Start date (YYYY-MM-DD)")]
    from: Option<String>,

    #[arg(long, help = "End date (YYYY-MM-DD)")]
    to: Option<String>,

    #[arg(long, help = "Analyze today's commits only")]
    today: bool,

    #[arg(long, help = "Analyze this week's commits")]
    week: bool,

    #[arg(long, help = "Analyze this month's commits")]
    month: bool,

    #[arg(long, help = "Limit to N most recent commits")]
    limit: Option<usize>,
}

fn format_currency(value: f64) -> String {
    let value = value.round() as i64;
    let value_str = value.to_string();
    let mut result = String::new();
    let mut count = 0;

    for ch in value_str.chars().rev() {
        if count > 0 && count % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
        count += 1;
    }

    format!("₩{}", result.chars().rev().collect::<String>())
}

fn main() -> Result<()> {
    let mut cli = Cli::parse();

    let paths = if cli.paths.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        std::mem::take(&mut cli.paths)
    };

    println!("{}", "Work Summary".bold().bright_cyan());
    println!("{}\n", format!("v{}", env!("CARGO_PKG_VERSION")).dimmed());

    let mut summaries = Vec::new();

    for path in &paths {
        match analyze_repository(path, &cli) {
            Ok(summary) => summaries.push(summary),
            Err(e) => {
                eprintln!("{}: {} - {}", "Error".red(), path.display(), e);
            }
        }
    }

    if summaries.is_empty() {
        println!("{}", "No repositories analyzed successfully.".yellow());
        return Ok(());
    }

    let total_summary = TotalSummary::from_repositories(summaries.clone());

    if cli.simple {
        print_simple_summary(&total_summary);
    } else {
        print_detailed_summary(&total_summary);
    }

    if let Some(export_path) = cli.export {
        export_summary(&total_summary, &export_path, &cli.format)?;
        println!("\n{} {}", "Exported to:".green(), export_path.display());
    }

    Ok(())
}

fn analyze_repository(path: &PathBuf, cli: &Cli) -> Result<RepositorySummary> {
    let analyzer = CommitAnalyzer::new(path)
        .context(format!("Failed to open repository at {}", path.display()))?;

    let commits = analyzer.analyze_commits(cli.limit)?;

    if commits.is_empty() {
        return Ok(RepositorySummary::new(
            path.clone(),
            commits,
            0.0,
            cli.hourly_rate,
        ));
    }

    let estimator = TimeEstimator::new();
    let estimated_hours = estimator.estimate_work_hours(&commits);

    Ok(RepositorySummary::new(
        path.clone(),
        commits,
        estimated_hours,
        cli.hourly_rate,
    ))
}

fn print_simple_summary(summary: &TotalSummary) {
    println!("{}", "═".repeat(60).dimmed());
    println!("{}", "Total Summary".bold().bright_yellow());
    println!("{}", "═".repeat(60).dimmed());

    for repo in &summary.repositories {
        println!("\n{}: {}", "Repository".bold(), repo.path.display());
        println!("  {}: {}", "Period".dimmed(), repo.period.description);
        println!("  {}: {}", "Commits".dimmed(), repo.commits.len());
        println!(
            "  {}: {:.1}h",
            "Estimated Hours".dimmed(),
            repo.analysis.estimated_hours
        );
        println!(
            "  {}: {}",
            "Value (Mid-level)".dimmed(),
            format_currency(repo.analysis.value_estimate.recommended_value)
                .bright_green()
        );
    }

    if summary.repositories.len() > 1 {
        println!("\n{}", "─".repeat(60).dimmed());
        println!("{}", "Overall Total".bold().bright_cyan());
        println!("{}", "─".repeat(60).dimmed());
        println!("  Total Commits: {}", summary.total_commits);
        println!("  Total Hours: {:.1}h", summary.total_hours);
        println!(
            "  Total Value: {}",
            format_currency(summary.total_value).bright_green()
        );
        println!("  Contributors: {}", summary.total_contributors);
    }
}

fn print_detailed_summary(summary: &TotalSummary) {
    for repo in &summary.repositories {
        println!("\n{}", "═".repeat(80).dimmed());
        println!(
            "{}: {}",
            "Repository".bold().bright_cyan(),
            repo.path.display()
        );
        println!("{}", "═".repeat(80).dimmed());

        print_basic_info(repo);
        print_language_breakdown(repo);
        print_contributor_breakdown(repo);
        print_work_patterns(repo);
        print_value_estimates(repo);
    }

    if summary.repositories.len() > 1 {
        print_total_summary(summary);
    }
}

fn print_basic_info(repo: &RepositorySummary) {
    println!("\n{}", "Basic Information".bold().yellow());
    println!("  Period: {}", repo.period.description);
    println!("  Total Commits: {}", repo.commits.len());
    println!("  Contributors: {}", repo.analysis.unique_contributors);
    println!("  Files Changed: {}", repo.analysis.total_files_changed);
    println!(
        "  Lines: {} / {}",
        format!("+{}", repo.analysis.total_insertions).green(),
        format!("-{}", repo.analysis.total_deletions).red()
    );
    println!("  Estimated Hours: {:.1}h", repo.analysis.estimated_hours);
}

fn print_language_breakdown(repo: &RepositorySummary) {
    if repo.analysis.language_breakdown.is_empty() {
        return;
    }

    println!("\n{}", "Language Breakdown".bold().yellow());

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["Language", "Insertions", "Deletions", "Net Change", "%"]);

    let mut langs: Vec<_> = repo.analysis.language_breakdown.iter().collect();
    langs.sort_by(|a, b| b.1.percentage.partial_cmp(&a.1.percentage).unwrap());

    for (lang, stats) in langs.iter().take(10) {
        table.add_row(vec![
            Cell::new(lang),
            Cell::new(format!("+{}", stats.insertions)).fg(Color::Green),
            Cell::new(format!("-{}", stats.deletions)).fg(Color::Red),
            Cell::new(stats.net_change),
            Cell::new(format!("{:.1}%", stats.percentage)),
        ]);
    }

    println!("{table}");
}

fn print_contributor_breakdown(repo: &RepositorySummary) {
    if repo.analysis.contribution_breakdown.is_empty() {
        return;
    }

    println!("\n{}", "Top Contributors".bold().yellow());

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["Name", "Commits", "Insertions", "Deletions", "%"]);

    for contributor in repo.analysis.contribution_breakdown.iter().take(5) {
        table.add_row(vec![
            Cell::new(&contributor.name),
            Cell::new(contributor.commit_count),
            Cell::new(format!("+{}", contributor.insertions)).fg(Color::Green),
            Cell::new(format!("-{}", contributor.deletions)).fg(Color::Red),
            Cell::new(format!("{:.1}%", contributor.percentage)),
        ]);
    }

    println!("{table}");
}

fn print_work_patterns(repo: &RepositorySummary) {
    println!("\n{}", "Work Patterns".bold().yellow());

    println!(
        "  Peak Hours: {}",
        repo.patterns
            .peak_hours
            .iter()
            .map(|h| format!("{}:00", h))
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("  Most Active Day: {}", repo.patterns.most_active_day);
    println!(
        "  Avg Commits/Day: {:.1}",
        repo.patterns.commit_frequency.average_commits_per_day
    );
    println!(
        "  Active Days: {} / {}",
        repo.patterns.commit_frequency.active_days,
        repo.patterns.commit_frequency.total_days
    );
}

fn print_value_estimates(repo: &RepositorySummary) {
    println!("\n{}", "Value Estimates".bold().yellow());

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["Level", "Multiplier", "Hourly Rate", "Total Value"]);

    for level in &repo.analysis.value_estimate.developer_levels {
        let is_recommended = level.level == "Mid-level";
        let total_value = format_currency(level.total_value);

        table.add_row(vec![
            if is_recommended {
                Cell::new(format!("{} ⭐", level.level)).fg(Color::Yellow)
            } else {
                Cell::new(&level.level)
            },
            Cell::new(format!("{}x", level.multiplier)),
            Cell::new(format_currency(level.hourly_rate)),
            if is_recommended {
                Cell::new(total_value).fg(Color::Green)
            } else {
                Cell::new(total_value)
            },
        ]);
    }

    println!("{table}");
}

fn print_total_summary(summary: &TotalSummary) {
    println!("\n{}", "═".repeat(80).dimmed());
    println!("{}", "Overall Summary".bold().bright_cyan());
    println!("{}", "═".repeat(80).dimmed());

    println!("\n  Total Repositories: {}", summary.repositories.len());
    println!("  Total Commits: {}", summary.total_commits);
    println!("  Total Hours: {:.1}h", summary.total_hours);
    println!(
        "  Total Value (Mid-level): {}",
        format_currency(summary.total_value).bright_green()
    );
    println!("  Unique Contributors: {}", summary.total_contributors);
}

fn export_summary(
    summary: &TotalSummary,
    path: &PathBuf,
    format: &str,
) -> Result<()> {
    let content = match format {
        "json" => serde_json::to_string_pretty(summary)?,
        _ => serde_json::to_string_pretty(summary)?,
    };

    std::fs::write(path, content)?;
    Ok(())
}
