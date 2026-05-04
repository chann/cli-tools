use anyhow::Result;
use std::process::Command;
use std::time::Instant;
use owo_colors::OwoColorize;

pub fn run(command: &str, args: Vec<String>, count: usize) -> Result<()> {
    println!("{}", format!("Benchmarking '{}' ({} runs)...", command, count).dimmed());

    let mut total_duration = std::time::Duration::default();
    let mut min_duration = std::time::Duration::from_secs(u64::MAX);
    let mut max_duration = std::time::Duration::default();

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
            println!("{} Run {} failed with status: {}", "Warning:".yellow(), i + 1, status);
        }
    }

    let avg_duration = total_duration / count as u32;

    println!("\n{}", "--- Benchmark Results ---".bold().cyan());
    println!("{:<15}: {} runs", "Total", count);
    println!("{:<15}: {:?}", "Average", avg_duration.green());
    println!("{:<15}: {:?}", "Minimum", min_duration.blue());
    println!("{:<15}: {:?}", "Maximum", max_duration.yellow());

    Ok(())
}
