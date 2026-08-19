use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};
use cli_core::output::format_integer;
use cli_core::ui::Theme;
use owo_colors::OwoColorize;

pub fn run(from: &str, to: Option<&str>) -> Result<()> {
    let from_dt = parse_input(from)?;
    let to_dt = match to {
        Some(t) => parse_input(t)?,
        None => Local::now(),
    };

    println!("\n{}", Theme::header("Date Difference"));
    println!("  {} {}", "From:".dimmed(), Theme::value(from_dt.format("%Y-%m-%d %H:%M:%S %:z (%a)").to_string()));
    println!("  {} {}", "To:  ".dimmed(), Theme::value(to_dt.format("%Y-%m-%d %H:%M:%S %:z (%a)").to_string()));
    println!();

    let b = breakdown(from_dt.date_naive(), to_dt.date_naive());
    let direction = if to.is_none() {
        // D-day style: `from` is the target date, `to` is now
        if b.negative { " from now" } else { " ago" }
    } else if b.negative {
        " (to is before from)"
    } else {
        ""
    };

    println!("  {}{}", Theme::highlight(format!("{} days", format_integer(b.total_days))), direction.dimmed());
    if b.years > 0 || b.months > 0 {
        println!("  = {}", calendar_line(&b));
    }
    if b.total_days >= 7 {
        println!("  = {} weeks {} days", b.total_days / 7, b.total_days % 7);
    }

    let seconds = (to_dt - from_dt).num_seconds().abs();
    println!(
        "  = {} hours / {} minutes / {} seconds",
        format_integer(seconds / 3600),
        format_integer(seconds / 60),
        format_integer(seconds)
    );
    Ok(())
}

struct Breakdown {
    negative: bool,
    total_days: i64,
    years: i32,
    months: i32,
    days: i64,
}

fn calendar_line(b: &Breakdown) -> String {
    let mut parts = Vec::new();
    if b.years > 0 {
        parts.push(format!("{} years", b.years));
    }
    if b.months > 0 {
        parts.push(format!("{} months", b.months));
    }
    if b.days > 0 {
        parts.push(format!("{} days", b.days));
    }
    if parts.is_empty() {
        parts.push("0 days".to_string());
    }
    parts.join(" ")
}

fn breakdown(from: NaiveDate, to: NaiveDate) -> Breakdown {
    let negative = to < from;
    let (from, to) = if negative { (to, from) } else { (from, to) };

    let total_days = (to - from).num_days();

    let mut months_total = (to.year() - from.year()) * 12 + (to.month() as i32 - from.month() as i32);
    if to.day() < from.day() {
        months_total -= 1;
    }
    let anchor = add_months(from, months_total);
    let days = (to - anchor).num_days();

    Breakdown {
        negative,
        total_days,
        years: months_total / 12,
        months: months_total % 12,
        days,
    }
}

/// Add calendar months, clamping the day to the target month's length.
fn add_months(date: NaiveDate, months: i32) -> NaiveDate {
    let zero_based = date.year() * 12 + date.month() as i32 - 1 + months;
    let (year, month) = (zero_based.div_euclid(12), zero_based.rem_euclid(12) as u32 + 1);
    (1..=date.day())
        .rev()
        .find_map(|day| NaiveDate::from_ymd_opt(year, month, day))
        .expect("day 1 always exists")
}

fn parse_input(input: &str) -> Result<DateTime<Local>> {
    if let Ok(ts) = input.parse::<i64>() {
        return Utc
            .timestamp_opt(ts, 0)
            .single()
            .map(|dt| dt.with_timezone(&Local))
            .ok_or_else(|| anyhow!("Invalid unix timestamp: {}", ts));
    }
    if let Ok(dt) = DateTime::parse_from_rfc3339(input) {
        return Ok(dt.with_timezone(&Local));
    }
    let naive = NaiveDateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S")
        .or_else(|_| {
            NaiveDate::parse_from_str(input, "%Y-%m-%d")
                .map(|d| d.and_hms_opt(0, 0, 0).expect("midnight is valid"))
        })
        .map_err(|_| {
            anyhow!(
                "Invalid date {:?}. Use YYYY-MM-DD, \"YYYY-MM-DD HH:MM:SS\", RFC3339, or a unix timestamp",
                input
            )
        })?;
    Local
        .from_local_datetime(&naive)
        .earliest()
        .context("Date does not exist in the local timezone")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(s: &str) -> NaiveDate {
        s.parse().unwrap()
    }

    #[test]
    fn breakdown_computes_calendar_parts() {
        let b = breakdown(d("2026-01-01"), d("2026-08-19"));
        assert!(!b.negative);
        assert_eq!(b.total_days, 230);
        assert_eq!((b.years, b.months, b.days), (0, 7, 18));
    }

    #[test]
    fn breakdown_is_symmetric_for_reversed_inputs() {
        let b = breakdown(d("2026-08-19"), d("2026-01-01"));
        assert!(b.negative);
        assert_eq!(b.total_days, 230);
        assert_eq!((b.years, b.months, b.days), (0, 7, 18));
    }

    #[test]
    fn breakdown_clamps_month_end() {
        let b = breakdown(d("2026-01-31"), d("2026-02-28"));
        assert_eq!((b.years, b.months, b.days), (0, 0, 28));
        let b = breakdown(d("2024-02-29"), d("2026-02-28"));
        assert_eq!((b.years, b.months, b.days), (1, 11, 30));
    }

    #[test]
    fn add_months_clamps_to_month_length() {
        assert_eq!(add_months(d("2026-01-31"), 1), d("2026-02-28"));
        assert_eq!(add_months(d("2026-11-30"), 3), d("2027-02-28"));
        assert_eq!(add_months(d("2026-03-15"), -1), d("2026-02-15"));
    }

    #[test]
    fn parse_accepts_common_formats() {
        assert!(parse_input("2026-08-19").is_ok());
        assert!(parse_input("2026-08-19 09:30:00").is_ok());
        assert!(parse_input("2026-08-19T09:30:00+09:00").is_ok());
        assert!(parse_input("1755000000").is_ok());
        assert!(parse_input("not-a-date").is_err());
    }
}
