use anyhow::Result;
use cron_descriptor::cronparser::cron_expression_descriptor;
use owo_colors::OwoColorize;
use cron::Schedule;
use std::str::FromStr;
use chrono::Local;

pub fn explain(expression: &str) -> Result<()> {
    match cron_expression_descriptor::get_description_cron(expression) {
        Ok(description) => {
            println!("{}: {}", "Expression".bold().blue(), expression);
            println!("{}: {}", "Description".bold().green(), description);

            // Calculate next 5 runs
            let mut cron_expr = expression.to_string();
            if cron_expr.split_whitespace().count() == 5 {
                cron_expr = format!("0 {}", cron_expr);
            }

            if let Ok(schedule) = Schedule::from_str(&cron_expr) {
                println!("\n{}", "--- Next 5 Scheduled Runs ---".bold().yellow());
                for (i, datetime) in schedule.upcoming(chrono::Utc).take(5).enumerate() {
                    // Convert Utc to Local for display
                    let local_time = datetime.with_timezone(&Local);
                    println!("  {}. {}", i + 1, local_time.format("%Y-%m-%d %H:%M:%S").green());
                }
            }
        }
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to parse cron expression: {:?}", e));
        }
    }
    Ok(())
}
