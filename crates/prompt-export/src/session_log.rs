use chrono::{DateTime, Utc};
use cli_core::date_range::DateRange;
use serde_json::Value;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Iterate over well-formed JSON values in a JSONL file.
///
/// Session logs are append-only runtime artifacts, so unreadable files and
/// malformed or partially written lines are skipped instead of aborting the
/// entire export.
pub fn json_values(path: &Path) -> impl Iterator<Item = Value> {
    fs::File::open(path)
        .ok()
        .into_iter()
        .flat_map(|file| BufReader::new(file).lines())
        .filter_map(Result::ok)
        .filter_map(|line| serde_json::from_str(&line).ok())
}

/// Return whether a log file may contain entries on or after the range start.
///
/// Missing metadata is treated conservatively: the caller should inspect the
/// file rather than risk dropping valid entries.
pub fn may_contain_entries(path: &Path, date_range: DateRange) -> bool {
    let Some(start) = date_range.start() else {
        return true;
    };

    fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .map_or(true, |modified| DateTime::<Utc>::from(modified) >= start)
}

pub fn timestamp(value: &Value) -> Option<DateTime<Utc>> {
    value
        .get("timestamp")
        .and_then(Value::as_str)
        .and_then(|raw| DateTime::parse_from_rfc3339(raw).ok())
        .map(|parsed| parsed.with_timezone(&Utc))
}

pub fn fallback_session_id(path: &Path) -> String {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("unknown")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use std::io::Write;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static NEXT_DIR: AtomicUsize = AtomicUsize::new(0);

    fn fixture_path() -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "prompt-export-session-log-test-{}-{}",
            std::process::id(),
            NEXT_DIR.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&dir).unwrap();
        dir.join("rollout-test.jsonl")
    }

    #[test]
    fn json_values_skip_malformed_and_partially_written_lines() {
        let path = fixture_path();
        let mut file = fs::File::create(&path).unwrap();
        writeln!(file, r#"{{"type":"valid"}}"#).unwrap();
        writeln!(file, "not json").unwrap();
        write!(file, r#"{{"type":"partial""#).unwrap();
        drop(file);

        let values: Vec<_> = json_values(&path).collect();

        assert_eq!(values.len(), 1);
        assert_eq!(values[0]["type"], "valid");
        fs::remove_dir_all(path.parent().unwrap()).ok();
    }

    #[test]
    fn modification_time_is_only_a_conservative_start_filter() {
        let path = fixture_path();
        fs::File::create(&path).unwrap();

        assert!(may_contain_entries(&path, DateRange::default()));
        assert!(!may_contain_entries(
            &path,
            DateRange::new(Some(Utc::now() + Duration::days(1)), None)
        ));
        fs::remove_dir_all(path.parent().unwrap()).ok();
    }

    #[test]
    fn extracts_utc_timestamp_and_fallback_session_id() {
        let value: Value =
            serde_json::from_str(r#"{"timestamp":"2026-08-01T23:30:00+09:00"}"#).unwrap();

        assert_eq!(
            timestamp(&value).unwrap().to_rfc3339(),
            "2026-08-01T14:30:00+00:00"
        );
        assert_eq!(
            fallback_session_id(Path::new("/tmp/rollout-session.jsonl")),
            "rollout-session"
        );
    }
}
