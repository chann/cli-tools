use anyhow::Result;
use std::process::Command;
use std::time::Instant;
use cli_core::output::TableFormatter;
use cli_core::ui::Theme;

pub fn run(command: &str, args: Vec<String>, count: usize) -> Result<()> {
    println!("{}", Theme::info(format!("Benchmarking '{}' ({} runs)...", command, count)));

    let mut total_duration = std::time::Duration::default();
    let mut min_duration = std::time::Duration::from_secs(u64::MAX);
    let mut max_duration = std::time::Duration::default();
    let mut failures = 0;

    for i in 0..count {
        let start = Instant::now();
        let status = Command::new(command)
            .args(&args)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()?;
        
        let duration = start.elapsed();
        total_duration += duration;
        
        if duration < min_duration {
            min_duration = duration;
        }
        if duration > max_duration {
            max_duration = duration;
        }

        if !status.success() {
            failures += 1;
            println!("{}", Theme::warning(format!("Run {} failed with status: {}", i + 1, status)));
        }
    }

    let avg_duration = total_duration / count as u32;

    println!("\n{}", Theme::header("Benchmark Results"));
    
    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("Metric"),
        TableFormatter::header_cell("Value"),
    ]);

    table.add_row(vec![
        TableFormatter::value_cell("Total Runs"),
        TableFormatter::value_cell(count.to_string()),
    ]);
    table.add_row(vec![
        TableFormatter::value_cell("Failures"),
        TableFormatter::value_cell(if failures > 0 { Theme::error(failures.to_string()) } else { "0".to_string() }),
    ]);
    table.add_row(vec![
        TableFormatter::value_cell("Average"),
        TableFormatter::highlight_cell(format!("{:?}", avg_duration)),
    ]);
    table.add_row(vec![
        TableFormatter::value_cell("Minimum"),
        TableFormatter::value_cell(format!("{:?}", min_duration)),
    ]);
    table.add_row(vec![
        TableFormatter::value_cell("Maximum"),
        TableFormatter::value_cell(format!("{:?}", max_duration)),
    ]);

    println!("{}", table);

    Ok(())
}
