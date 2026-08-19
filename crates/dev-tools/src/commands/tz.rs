use anyhow::{bail, Result};
use chrono::{Local, NaiveDate, Utc};
use chrono_tz::{Tz, TZ_VARIANTS};
use cli_core::output::TableFormatter;
use cli_core::ui::Theme;

const WORLD_CLOCK: &[&str] = &[
    "Pacific/Auckland",
    "Australia/Sydney",
    "Asia/Tokyo",
    "Asia/Seoul",
    "Asia/Shanghai",
    "Asia/Singapore",
    "Asia/Kolkata",
    "Asia/Dubai",
    "Europe/Moscow",
    "Europe/Berlin",
    "Europe/Paris",
    "Europe/London",
    "UTC",
    "America/Sao_Paulo",
    "America/New_York",
    "America/Chicago",
    "America/Denver",
    "America/Los_Angeles",
];

pub fn show(query: Option<&str>) -> Result<()> {
    let zones: Vec<Tz> = match query {
        None => WORLD_CLOCK
            .iter()
            .map(|name| name.parse().expect("world clock zones are valid"))
            .collect(),
        Some(q) => {
            let matches = matching_zones(q);
            if matches.is_empty() {
                bail!("No timezone matches {:?}. Try a city (e.g., \"seoul\") or region (e.g., \"america\")", q);
            }
            matches
        }
    };

    let title = match query {
        None => "World Clock".to_string(),
        Some(q) => format!("Timezones matching {:?} ({})", q, zones.len()),
    };
    println!("\n{}", Theme::header(&title));

    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("Zone"),
        TableFormatter::header_cell("Current Time"),
        TableFormatter::header_cell("UTC Offset"),
        TableFormatter::header_cell("Day"),
    ]);

    let now = Utc::now();
    let local_date = now.with_timezone(&Local).date_naive();
    const LIMIT: usize = 30;
    for zone in zones.iter().take(LIMIT) {
        let time = now.with_timezone(zone);
        table.add_row(vec![
            TableFormatter::highlight_cell(zone.name()),
            TableFormatter::value_cell(time.format("%Y-%m-%d %H:%M (%a)")),
            TableFormatter::value_cell(time.format("%:z")),
            TableFormatter::value_cell(day_label(time.date_naive(), local_date)),
        ]);
    }
    println!("{}", table);
    if zones.len() > LIMIT {
        println!("{}", Theme::dim(format!("... and {} more. Narrow the query to see them.", zones.len() - LIMIT)));
    }
    Ok(())
}

/// Case-insensitive substring match over IANA zone names ("new york" matches America/New_York).
fn matching_zones(query: &str) -> Vec<Tz> {
    let needle = query.trim().to_lowercase().replace(' ', "_");
    TZ_VARIANTS
        .iter()
        .filter(|tz| tz.name().to_lowercase().contains(&needle))
        .copied()
        .collect()
}

/// Relative day marker vs. the local date ("", "+1d", "-1d").
fn day_label(zone_date: NaiveDate, local_date: NaiveDate) -> String {
    let diff = (zone_date - local_date).num_days();
    match diff {
        0 => String::new(),
        d if d > 0 => format!("+{}d", d),
        d => format!("{}d", d),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_clock_zones_all_parse() {
        for name in WORLD_CLOCK {
            assert!(name.parse::<Tz>().is_ok(), "invalid zone: {}", name);
        }
    }

    #[test]
    fn search_matches_city_case_insensitively() {
        let zones = matching_zones("SEOUL");
        assert_eq!(zones, vec![chrono_tz::Asia::Seoul]);
    }

    #[test]
    fn search_maps_spaces_to_underscores() {
        assert!(matching_zones("new york").contains(&chrono_tz::America::New_York));
        assert!(matching_zones("garbage-zone-xyz").is_empty());
    }

    #[test]
    fn day_label_marks_date_boundaries() {
        let d = |s: &str| s.parse::<NaiveDate>().unwrap();
        assert_eq!(day_label(d("2026-08-19"), d("2026-08-19")), "");
        assert_eq!(day_label(d("2026-08-20"), d("2026-08-19")), "+1d");
        assert_eq!(day_label(d("2026-08-18"), d("2026-08-19")), "-1d");
    }
}
