use anyhow::{Context, Result};
use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, Utc};
use clap::Args;

/// Reusable calendar-date filters for commands that operate on timestamped items.
#[derive(Args, Debug, Clone, Default)]
pub struct DateRangeArgs {
    #[arg(long, help = "Start date (YYYY-MM-DD)")]
    from: Option<String>,

    #[arg(long, help = "End date (YYYY-MM-DD)")]
    to: Option<String>,

    #[arg(long, help = "Include only items from today")]
    today: bool,

    #[arg(long, help = "Include items from this week (since Monday)")]
    week: bool,

    #[arg(long, help = "Include items from this month")]
    month: bool,
}

impl DateRangeArgs {
    /// Resolve the selected calendar dates in the current local timezone.
    ///
    /// Preset precedence intentionally matches the original CLIs: today, week,
    /// month, then explicit bounds.
    pub fn resolve(&self) -> Result<DateRange> {
        self.resolve_for(Local::now().date_naive())
    }

    fn resolve_for(&self, today: NaiveDate) -> Result<DateRange> {
        if self.today {
            return Ok(DateRange::new(
                Some(day_start(today)?),
                Some(day_end(today)?),
            ));
        }

        if self.week {
            let monday = today - Duration::days(today.weekday().num_days_from_monday() as i64);
            return Ok(DateRange::new(Some(day_start(monday)?), None));
        }

        if self.month {
            let first = today.with_day(1).expect("every month has a first day");
            return Ok(DateRange::new(Some(day_start(first)?), None));
        }

        let start = self
            .from
            .as_deref()
            .map(|raw| parse_date(raw, "--from").and_then(day_start))
            .transpose()?;
        let end = self
            .to
            .as_deref()
            .map(|raw| parse_date(raw, "--to").and_then(day_end))
            .transpose()?;

        Ok(DateRange::new(start, end))
    }
}

/// An inclusive start and exclusive end in UTC.
///
/// Exclusive end bounds represent the first instant after the selected range,
/// so sub-second timestamps on the final day are not accidentally dropped.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DateRange {
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
}

impl DateRange {
    pub const fn new(start: Option<DateTime<Utc>>, end: Option<DateTime<Utc>>) -> Self {
        Self { start, end }
    }

    pub const fn start(self) -> Option<DateTime<Utc>> {
        self.start
    }

    pub const fn end(self) -> Option<DateTime<Utc>> {
        self.end
    }

    pub fn contains(self, timestamp: DateTime<Utc>) -> bool {
        self.start.is_none_or(|start| timestamp >= start)
            && self.end.is_none_or(|end| timestamp < end)
    }
}

fn parse_date(raw: &str, flag: &str) -> Result<NaiveDate> {
    NaiveDate::parse_from_str(raw, "%Y-%m-%d")
        .with_context(|| format!("Invalid {flag} date format. Use YYYY-MM-DD"))
}

fn day_start(date: NaiveDate) -> Result<DateTime<Utc>> {
    date.and_hms_opt(0, 0, 0)
        .expect("midnight is a valid time")
        .and_local_timezone(Local)
        .earliest()
        .map(|resolved| resolved.with_timezone(&Utc))
        .with_context(|| format!("Could not interpret {date} midnight in the local timezone"))
}

fn day_end(date: NaiveDate) -> Result<DateTime<Utc>> {
    let next_day = date
        .checked_add_signed(Duration::days(1))
        .context("Date range end exceeds the supported calendar")?;
    day_start(next_day)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args() -> DateRangeArgs {
        DateRangeArgs::default()
    }

    #[test]
    fn explicit_end_date_is_exclusive_next_midnight() {
        let mut args = args();
        args.from = Some("2026-08-01".to_string());
        args.to = Some("2026-08-07".to_string());

        let range = args
            .resolve_for(NaiveDate::from_ymd_opt(2026, 8, 8).unwrap())
            .unwrap();
        let start = day_start(NaiveDate::from_ymd_opt(2026, 8, 1).unwrap()).unwrap();
        let final_second = day_start(NaiveDate::from_ymd_opt(2026, 8, 7).unwrap()).unwrap()
            + Duration::hours(23)
            + Duration::minutes(59)
            + Duration::seconds(59);
        let next_day = day_start(NaiveDate::from_ymd_opt(2026, 8, 8).unwrap()).unwrap();

        assert!(range.contains(start));
        assert!(range.contains(final_second));
        assert!(!range.contains(next_day));
    }

    #[test]
    fn week_starts_on_monday_and_month_starts_on_first_day() {
        let today = NaiveDate::from_ymd_opt(2026, 8, 8).unwrap();

        let mut week = args();
        week.week = true;
        assert_eq!(
            week.resolve_for(today).unwrap().start(),
            Some(day_start(NaiveDate::from_ymd_opt(2026, 8, 3).unwrap()).unwrap())
        );

        let mut month = args();
        month.month = true;
        assert_eq!(
            month.resolve_for(today).unwrap().start(),
            Some(day_start(NaiveDate::from_ymd_opt(2026, 8, 1).unwrap()).unwrap())
        );
    }

    #[test]
    fn preset_precedence_matches_existing_cli_behavior() {
        let today = NaiveDate::from_ymd_opt(2026, 8, 8).unwrap();
        let mut args = args();
        args.today = true;
        args.week = true;
        args.month = true;
        args.from = Some("2020-01-01".to_string());

        let range = args.resolve_for(today).unwrap();

        assert_eq!(range.start(), Some(day_start(today).unwrap()));
        assert_eq!(range.end(), Some(day_end(today).unwrap()));
    }

    #[test]
    fn invalid_dates_name_the_originating_flag() {
        let mut args = args();
        args.to = Some("08/08/2026".to_string());

        let error = args
            .resolve_for(NaiveDate::from_ymd_opt(2026, 8, 8).unwrap())
            .unwrap_err();

        assert_eq!(
            error.to_string(),
            "Invalid --to date format. Use YYYY-MM-DD"
        );
    }
}
