use crate::git::CommitInfo;
use chrono::{Datelike, Timelike};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkPatterns {
    pub hourly_distribution: HashMap<u32, usize>,
    pub daily_distribution: HashMap<String, usize>,
    pub peak_hours: Vec<u32>,
    pub most_active_day: String,
    pub commit_frequency: CommitFrequency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitFrequency {
    pub average_commits_per_day: f64,
    pub max_commits_per_day: usize,
    pub active_days: usize,
    pub total_days: i64,
}

impl WorkPatterns {
    pub fn analyze(commits: &[CommitInfo]) -> Self {
        if commits.is_empty() {
            return Self::default();
        }

        let mut hourly_distribution: HashMap<u32, usize> = HashMap::new();
        let mut daily_distribution: HashMap<String, usize> = HashMap::new();
        let mut daily_commit_count: HashMap<String, usize> = HashMap::new();

        for commit in commits {
            let hour = commit.timestamp.hour();
            *hourly_distribution.entry(hour).or_insert(0) += 1;

            let day_name = Self::weekday_to_string(commit.timestamp.weekday());
            *daily_distribution.entry(day_name).or_insert(0) += 1;

            let date_key = commit.timestamp.format("%Y-%m-%d").to_string();
            *daily_commit_count.entry(date_key).or_insert(0) += 1;
        }

        let mut peak_hours: Vec<(u32, usize)> = hourly_distribution.iter()
            .map(|(h, c)| (*h, *c))
            .collect();
        peak_hours.sort_by(|a, b| b.1.cmp(&a.1));
        let peak_hours: Vec<u32> = peak_hours.iter().take(3).map(|(h, _)| *h).collect();

        let most_active_day = daily_distribution
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(day, _)| day.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        let first_commit = commits.last().unwrap();
        let last_commit = commits.first().unwrap();
        let total_days = (last_commit.timestamp - first_commit.timestamp).num_days() + 1;

        let active_days = daily_commit_count.len();
        let max_commits_per_day = daily_commit_count.values().max().copied().unwrap_or(0);
        let average_commits_per_day = if total_days > 0 {
            commits.len() as f64 / total_days as f64
        } else {
            0.0
        };

        let commit_frequency = CommitFrequency {
            average_commits_per_day,
            max_commits_per_day,
            active_days,
            total_days,
        };

        Self {
            hourly_distribution,
            daily_distribution,
            peak_hours,
            most_active_day,
            commit_frequency,
        }
    }

    fn weekday_to_string(weekday: chrono::Weekday) -> String {
        match weekday {
            chrono::Weekday::Mon => "Monday",
            chrono::Weekday::Tue => "Tuesday",
            chrono::Weekday::Wed => "Wednesday",
            chrono::Weekday::Thu => "Thursday",
            chrono::Weekday::Fri => "Friday",
            chrono::Weekday::Sat => "Saturday",
            chrono::Weekday::Sun => "Sunday",
        }
        .to_string()
    }
}

impl Default for WorkPatterns {
    fn default() -> Self {
        Self {
            hourly_distribution: HashMap::new(),
            daily_distribution: HashMap::new(),
            peak_hours: Vec::new(),
            most_active_day: "Unknown".to_string(),
            commit_frequency: CommitFrequency {
                average_commits_per_day: 0.0,
                max_commits_per_day: 0,
                active_days: 0,
                total_days: 0,
            },
        }
    }
}
