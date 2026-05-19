use anyhow::Result;
use chrono::{Local, Utc};
use cli_core::output::TableFormatter;
use cli_core::ui::Theme;
use cron::Schedule;
use cron_descriptor::cronparser::cron_expression_descriptor;
use owo_colors::OwoColorize;
use std::str::FromStr;

pub fn explain(expression: &str) -> Result<()> {
    let description = cron_expression_descriptor::get_description_cron(expression)
        .map_err(|e| anyhow::anyhow!("Failed to parse cron expression: {:?}", e))?;

    println!("\n{}", Theme::header("Cron Expression Summary"));
    println!("  {} {}", "Expression:".dimmed(), Theme::value(expression));
    println!("  {} {}", "Description:".dimmed(), Theme::highlight(&description));

    // Breakdown components
    let parts: Vec<&str> = expression.split_whitespace().collect();
    if parts.len() >= 5 {
        println!("\n{}", Theme::header("Expression Breakdown"));
        let mut table = TableFormatter::create_table();
        table.set_header(vec![
            TableFormatter::header_cell("Component"),
            TableFormatter::header_cell("Value"),
            TableFormatter::header_cell("Description"),
        ]);

        let labels = if parts.len() == 6 {
            vec!["Second", "Minute", "Hour", "Day of Month", "Month", "Day of Week"]
        } else {
            vec!["Minute", "Hour", "Day of Month", "Month", "Day of Week"]
        };

        for (i, &part) in parts.iter().enumerate() {
            if i < labels.len() {
                table.add_row(vec![
                    TableFormatter::value_cell(labels[i]),
                    TableFormatter::highlight_cell(part),
                    TableFormatter::value_cell(get_part_description(labels[i], part)),
                ]);
            }
        }
        println!("{}", table);
    }

    // Upcoming runs
    let mut cron_expr = expression.to_string();
    if parts.len() == 5 {
        cron_expr = format!("0 {}", cron_expr);
    }

    if let Ok(schedule) = Schedule::from_str(&cron_expr) {
        println!("\n{}", Theme::header("Next 10 Scheduled Runs"));
        let mut table = TableFormatter::create_table();
        table.set_header(vec![
            TableFormatter::header_cell("#"),
            TableFormatter::header_cell("Scheduled Time (Local)"),
            TableFormatter::header_cell("Time Remaining"),
        ]);

        let now = Utc::now();
        for (i, datetime) in schedule.upcoming(Utc).take(10).enumerate() {
            let local_time = datetime.with_timezone(&Local);
            let duration = datetime.signed_duration_since(now);
            
            let remaining = if duration.num_days() > 0 {
                format!("{}d {}h", duration.num_days(), duration.num_hours() % 24)
            } else if duration.num_hours() > 0 {
                format!("{}h {}m", duration.num_hours(), duration.num_minutes() % 60)
            } else if duration.num_minutes() > 0 {
                format!("{}m {}s", duration.num_minutes(), duration.num_seconds() % 60)
            } else {
                format!("{}s", duration.num_seconds())
            };

            table.add_row(vec![
                TableFormatter::value_cell(i + 1),
                TableFormatter::highlight_cell(local_time.format("%Y-%m-%d %H:%M:%S")),
                TableFormatter::value_cell(remaining),
            ]);
        }
        println!("{}", table);
    }

    Ok(())
}

fn get_part_description(label: &str, value: &str) -> String {
    if value == "*" {
        return format!("Every {}", label.to_lowercase());
    }
    if value.contains("/") {
        let parts: Vec<&str> = value.split('/').collect();
        if parts.len() == 2 {
            return format!("Every {} {} starting from {}", parts[1], label.to_lowercase(), parts[0]);
        }
    }
    if value.contains(",") {
        return format!("Specific {}s: {}", label.to_lowercase(), value);
    }
    if value.contains("-") {
        return format!("Range of {}s: {}", label.to_lowercase(), value);
    }
    format!("At {}", label.to_lowercase())
}
