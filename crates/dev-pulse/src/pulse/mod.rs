use anyhow::Result;
use cli_core::ui::Theme;
use std::path::Path;
use crate::{health, scanner, stats, git};
use owo_colors::OwoColorize;

pub async fn show(path: &Path) -> Result<()> {
    // 1. Health Score
    let (passed, total) = health::get_score(path);
    println!("{} {}/{}", Theme::info("Health Score:"), Theme::highlight(&passed.to_string()), total);
    
    let bar_width = 40;
    let filled = (passed as f32 / total as f32 * bar_width as f32) as usize;
    println!("  [{}{}]", "■".repeat(filled).bright_green(), "□".repeat(bar_width - filled).dimmed());
    println!();

    // 2. Git Status
    if let Ok(status) = git::get_status_summary(path) {
        println!("{}", Theme::info("Git Status:"));
        println!("  Branch:   {}", Theme::highlight(&status.branch_name));
        print!("  Changes:  ");
        if status.modified > 0 { print!("{} modified  ", status.modified.yellow()); }
        if status.added > 0 { print!("{} added  ", status.added.green()); }
        if status.deleted > 0 { print!("{} deleted  ", status.deleted.red()); }
        if status.untracked > 0 { print!("{} untracked", status.untracked.dimmed()); }
        if status.modified == 0 && status.added == 0 && status.deleted == 0 && status.untracked == 0 {
            print!("{}", "Clean".green());
        }
        println!("\n");
    }

    // 3. Project Stats
    let project_stats = stats::get_stats(path).await?;
    println!("{}", Theme::info("Project Stats:"));
    println!("  Files:    {}", Theme::value(&project_stats.total_files.to_string()));
    println!("  Lines:    {}", Theme::value(&project_stats.total_lines.to_string()));
    
    let mut sorted_exts: Vec<_> = project_stats.extension_counts.into_iter().collect();
    sorted_exts.sort_by(|a, b| b.1.1.cmp(&a.1.1));
    
    if let Some((ext, (_, lines))) = sorted_exts.first() {
        println!("  Primary:  {} ({} lines)", Theme::highlight(ext), lines);
    }
    println!();

    // 4. Marker Summary
    let marker_summary = scanner::get_summary(path, None, false).await?;
    println!("{}", Theme::info("Code Markers:"));
    if marker_summary.is_empty() {
        println!("  {}", Theme::success("No markers found."));
    } else {
        let mut sorted_markers: Vec<_> = marker_summary.into_iter().collect();
        sorted_markers.sort_by(|a, b| b.1.cmp(&a.1));
        
        for (kind, count) in sorted_markers {
            println!("  {:<10} {}", Theme::highlight(&kind), Theme::value(&count.to_string()));
        }
    }
    println!();

    // 5. Recent Activity
    if let Ok(activity) = git::get_recent_activity(path) {
        println!("{}", Theme::info("Recent Activity (Last 7 Days):"));
        println!("  Commits:  {}", Theme::value(&activity.total_commits_last_7_days.to_string()));
        
        if !activity.top_contributors.is_empty() {
            println!("  Top Contributors:");
            for (name, count) in activity.top_contributors {
                println!("    • {:<15} {}", name.dimmed(), Theme::value(&count.to_string()));
            }
        }
    }

    println!("\n{}", Theme::success("Project pulse is steady. Keep up the good work!"));

    Ok(())
}
