mod analyzer;
mod calculator;
mod git;
mod metrics;

use anyhow::Result;
use clap::Parser;
use cli_core::ui::Theme;
use std::path::PathBuf;

use crate::analyzer::RepositoryAnalyzer;
use crate::calculator::CostCalculator;

#[derive(serde::Serialize)]
struct ExportRow {
    path: String,
    lines: usize,
    files: usize,
    commits: usize,
    estimated_hours: f64,
    total_cost_krw: f64,
}

fn get_repository_name(path: &PathBuf) -> String {
    // Try to get git repository name first
    if let Ok(repo) = git2::Repository::open(path) {
        if let Some(name) = repo
            .workdir()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
        {
            return name.to_string();
        }
    }

    // Fallback to directory name
    path.canonicalize()
        .ok()
        .and_then(|p| p.file_name().and_then(|n| n.to_str().map(String::from)))
        .unwrap_or_else(|| path.to_string_lossy().to_string())
}

#[derive(Parser, Debug)]
#[command(
    name = "code-cost",
    version,
    about = "Analyze code repositories and calculate their monetary value",
    long_about = "A tool to analyze code repositories, calculate development effort, \
                  and estimate monetary value based on various metrics including LOC, \
                  complexity, commit history, and project maturity."
)]
struct Cli {
    /// Paths to repositories to analyze
    #[arg(value_name = "PATH", default_value = ".")]
    paths: Vec<PathBuf>,

    /// Output format
    #[arg(short, long, value_name = "FORMAT", default_value = "table")]
    format: String,

    /// Export results to a file (supports .csv, .html, .md)
    #[arg(short, long, value_name = "FILE")]
    export: Option<PathBuf>,

    /// Hourly rate in KRW (default: 10030 - 2025 minimum wage)
    #[arg(long, value_name = "RATE", default_value = "10030")]
    hourly_rate: f64,

    /// Simple output mode (hide detailed analysis)
    #[arg(short, long)]
    simple: bool,

    /// Show developer level breakdown
    #[arg(long)]
    dev_levels: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("{}", Theme::header("🔍 Code Cost Analyzer"));
    println!();

    let analyzer = RepositoryAnalyzer::new(cli.hourly_rate);
    let calculator = CostCalculator::new(cli.hourly_rate);

    let mut results = Vec::new();

    for path in &cli.paths {
        println!("{} {}", Theme::info("Analyzing:"), path.display());

        match analyzer.analyze(path).await {
            Ok(analysis) => {
                let cost = calculator.calculate(&analysis);
                results.push((path, analysis, cost));
                println!("{}", Theme::success("Analysis completed"));
            }
            Err(e) => {
                println!("{} {}", Theme::error("Analysis failed:"), e);
            }
        }
        println!();
    }

    if results.is_empty() {
        println!("{}", Theme::warning("No repositories were successfully analyzed"));
        return Ok(());
    }

    // Display results
    display_results(&results, &cli)?;

    // Export if requested
    if let Some(export_path) = cli.export {
        export_results(&results, &export_path)?;
        println!(
            "{} {}",
            Theme::success("Exported to:"),
            export_path.display()
        );
    }

    Ok(())
}

fn display_results(
    results: &[(&PathBuf, analyzer::Analysis, calculator::CostEstimate)],
    cli: &Cli,
) -> Result<()> {
    use cli_core::output::{OutputFormat, TableFormatter};
    use comfy_table::{Cell, Color};

    let format = OutputFormat::from_str(&cli.format)?;

    match format {
        OutputFormat::Table => {
            let mut table = TableFormatter::create_table();

            table.set_header(vec![
                TableFormatter::header_cell("Repository"),
                TableFormatter::header_cell("Lines"),
                TableFormatter::header_cell("Files"),
                TableFormatter::header_cell("Commits"),
                TableFormatter::header_cell("Est. Hours"),
                TableFormatter::header_cell("Total Cost (KRW)"),
            ]);

            for (path, analysis, cost) in results {
                let repo_name = get_repository_name(path);

                table.add_row(vec![
                    Cell::new(&repo_name),
                    Cell::new(format!("{:>10}", analysis.total_lines)),
                    Cell::new(format!("{:>6}", analysis.total_files)),
                    Cell::new(format!("{:>7}", analysis.commit_count)),
                    Cell::new(format!("{:>10.1}", cost.estimated_hours)),
                    Cell::new(format!("₩{:>12}", format_number(cost.total_cost as u64)))
                        .fg(Color::Green),
                ]);
            }

            println!("{table}");

            // Detailed analysis (unless --simple)
            if !cli.simple {
                println!();
                for (path, analysis, cost) in results {
                    let repo_name = get_repository_name(path);
                    println!("{}", Theme::header(&format!("📁 {}", repo_name)));
                    println!();

                    // Language breakdown
                    if !analysis.language_stats.is_empty() {
                        println!("{}", Theme::info("Languages:"));
                        for lang in &analysis.language_stats {
                            let percentage =
                                (lang.lines as f64 / analysis.total_lines as f64) * 100.0;
                            println!(
                                "  • {} {} {}% ({} lines, {} files)",
                                Theme::value(&lang.name),
                                Theme::dim("-"),
                                Theme::highlight(&format!("{:.1}", percentage)),
                                format_number(lang.lines as u64),
                                lang.files
                            );
                        }
                        println!();
                    }

                    // Project metrics
                    println!("{}", Theme::info("Project Metrics:"));
                    println!(
                        "  • Complexity Score: {}",
                        Theme::highlight(&format!("{:.2}/5.0", analysis.complexity_score))
                    );
                    println!(
                        "  • Maturity Score: {}",
                        Theme::highlight(&format!("{:.1}%", analysis.maturity_score * 100.0))
                    );
                    println!(
                        "  • Code Quality: {}",
                        Theme::highlight(&format!("{:.1}%", cost.ai_analysis.code_quality_score * 100.0))
                    );
                    println!(
                        "  • Test Files: {} ({:.1}%)",
                        Theme::highlight(&analysis.test_file_count.to_string()),
                        (analysis.test_file_count as f64 / analysis.total_files as f64) * 100.0
                    );
                    println!();

                    // AI Analysis
                    println!("{}", Theme::info("AI Usage Analysis:"));
                    println!(
                        "  • Estimated AI Usage: {}",
                        Theme::highlight(&format!("{:.1}%", cost.ai_analysis.estimated_ai_usage * 100.0))
                    );
                    if !cost.ai_analysis.potential_ai_indicators.is_empty() {
                        println!("  {} Indicators:", Theme::dim("•"));
                        for indicator in &cost.ai_analysis.potential_ai_indicators {
                            println!("    - {}", Theme::dim(indicator));
                        }
                    }
                    println!();

                    // Developer level breakdown (if requested)
                    if cli.dev_levels {
                        println!("{}", Theme::info("Developer Level Breakdown:"));
                        for level in &cost.developer_levels {
                            println!(
                                "  • {:<12} ₩{:>8}/hr → ₩{}",
                                Theme::value(&level.level),
                                format_number(level.hourly_rate as u64),
                                Theme::highlight(&format_number(level.estimated_cost as u64))
                            );
                        }
                        println!();
                    }
                }
            }

            // Summary
            let total_cost: f64 = results.iter().map(|(_, _, c)| c.total_cost).sum();
            let total_hours: f64 = results.iter().map(|(_, _, c)| c.estimated_hours).sum();

            println!();
            println!("{}", Theme::header("📊 Summary"));
            println!(
                "  {} {}",
                Theme::dim("Total repositories:"),
                Theme::highlight(&results.len().to_string())
            );
            println!(
                "  {} {}",
                Theme::dim("Total estimated hours:"),
                Theme::highlight(&format!("{:.1} hours", total_hours))
            );
            println!(
                "  {} {}",
                Theme::dim("Total estimated cost:"),
                Theme::highlight(&format!("₩{}", format_number(total_cost as u64)))
            );
        }
        OutputFormat::Json | OutputFormat::JsonPretty => {
            use cli_core::output::{Formatter, JsonFormatter};

            let json_results: Vec<_> = results
                .iter()
                .map(|(path, analysis, cost)| {
                    serde_json::json!({
                        "path": path.to_string_lossy(),
                        "metrics": analysis,
                        "cost": cost,
                    })
                })
                .collect();

            let formatter = JsonFormatter::new(format == OutputFormat::JsonPretty);
            let output = formatter.format(&json_results)?;
            println!("{}", output);
        }
    }

    Ok(())
}

fn export_results(
    results: &[(&PathBuf, analyzer::Analysis, calculator::CostEstimate)],
    export_path: &PathBuf,
) -> Result<()> {
    use cli_core::output::ExportFormat;

    let ext = export_path
        .extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| anyhow::anyhow!("No file extension provided"))?;

    let format = ExportFormat::from_extension(ext)?;

    let export_data: Vec<ExportRow> = results
        .iter()
        .map(|(path, analysis, cost)| ExportRow {
            path: path.to_string_lossy().to_string(),
            lines: analysis.total_lines,
            files: analysis.total_files,
            commits: analysis.commit_count,
            estimated_hours: cost.estimated_hours,
            total_cost_krw: cost.total_cost,
        })
        .collect();

    match format {
        ExportFormat::Csv => {
            use cli_core::output::CsvExporter;
            let exporter = CsvExporter::new();
            exporter.export(&export_data, export_path.to_str().unwrap())?;
        }
        ExportFormat::Html => {
            use cli_core::output::HtmlExporter;
            let exporter = HtmlExporter::new();
            exporter.export(&export_data, export_path.to_str().unwrap())?;
        }
        ExportFormat::Markdown => {
            use cli_core::output::MarkdownExporter;
            let exporter = MarkdownExporter::new();
            exporter.export(&export_data, export_path.to_str().unwrap())?;
        }
    }

    Ok(())
}

fn format_number(n: u64) -> String {
    n.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(",")
}
