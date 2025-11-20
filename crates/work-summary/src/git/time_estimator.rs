use super::CommitInfo;
use chrono::Duration;

const MAX_SESSION_GAP_HOURS: i64 = 4;
const LINES_PER_HOUR: f64 = 20.0;
const TIME_WEIGHT: f64 = 0.6;
const CHANGE_WEIGHT: f64 = 0.4;

pub struct TimeEstimator {
    language_weights: fn(&str) -> f64,
}

impl TimeEstimator {
    pub fn new() -> Self {
        Self {
            language_weights: get_language_weight,
        }
    }

    pub fn estimate_work_hours(&self, commits: &[CommitInfo]) -> f64 {
        if commits.is_empty() {
            return 0.0;
        }

        let mut total_hours = 0.0;

        for i in 0..commits.len() {
            let commit = &commits[i];

            let time_based = if i < commits.len() - 1 {
                let next_commit = &commits[i + 1];
                let time_diff = commit.timestamp.signed_duration_since(next_commit.timestamp);
                self.estimate_from_time_gap(time_diff)
            } else {
                0.0
            };

            let change_based = self.estimate_from_changes(commit);

            let hybrid_estimate = if time_based > 0.0 {
                (time_based * TIME_WEIGHT) + (change_based * CHANGE_WEIGHT)
            } else {
                change_based
            };

            total_hours += hybrid_estimate;
        }

        total_hours
    }

    fn estimate_from_time_gap(&self, duration: Duration) -> f64 {
        let hours = duration.num_hours() as f64 + (duration.num_minutes() % 60) as f64 / 60.0;

        if hours < 0.0 {
            return 0.0;
        }

        hours.min(MAX_SESSION_GAP_HOURS as f64)
    }

    fn estimate_from_changes(&self, commit: &CommitInfo) -> f64 {
        let total_lines = (commit.insertions + commit.deletions) as f64;

        let mut weighted_lines = 0.0;
        let mut total_changes = 0;

        for (lang, changes) in &commit.language_changes {
            let lang_lines = (changes.insertions + changes.deletions) as f64;
            let weight = (self.language_weights)(lang);
            weighted_lines += lang_lines * weight;
            total_changes += changes.insertions + changes.deletions;
        }

        if total_changes == 0 && total_lines > 0.0 {
            weighted_lines = total_lines;
        }

        let base_hours = weighted_lines / LINES_PER_HOUR;

        let complexity_factor = if commit.files_changed > 5 {
            1.2
        } else if commit.files_changed > 10 {
            1.4
        } else {
            1.0
        };

        base_hours * complexity_factor
    }

    pub fn estimate_session_hours(&self, commits: &[CommitInfo]) -> Vec<WorkSession> {
        let mut sessions = Vec::new();

        if commits.is_empty() {
            return sessions;
        }

        let mut current_session = WorkSession {
            start: commits[0].timestamp,
            end: commits[0].timestamp,
            commits: vec![commits[0].clone()],
            estimated_hours: 0.0,
        };

        for i in 1..commits.len() {
            let prev_commit = &commits[i - 1];
            let curr_commit = &commits[i];

            let time_gap = prev_commit.timestamp.signed_duration_since(curr_commit.timestamp);

            if time_gap.num_hours() <= MAX_SESSION_GAP_HOURS {
                current_session.end = curr_commit.timestamp;
                current_session.commits.push(curr_commit.clone());
            } else {
                current_session.estimated_hours = self.estimate_work_hours(&current_session.commits);
                sessions.push(current_session);

                current_session = WorkSession {
                    start: curr_commit.timestamp,
                    end: curr_commit.timestamp,
                    commits: vec![curr_commit.clone()],
                    estimated_hours: 0.0,
                };
            }
        }

        current_session.estimated_hours = self.estimate_work_hours(&current_session.commits);
        sessions.push(current_session);

        sessions
    }
}

impl Default for TimeEstimator {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct WorkSession {
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: chrono::DateTime<chrono::Utc>,
    pub commits: Vec<CommitInfo>,
    pub estimated_hours: f64,
}

fn get_language_weight(language: &str) -> f64 {
    match language {
        "Rust" => 1.5,
        "C++" | "C" => 1.4,
        "Go" => 1.3,
        "Java" | "C#" | "TypeScript" => 1.2,
        "Python" | "Ruby" | "PHP" => 1.1,
        "JavaScript" => 1.0,
        "Swift" | "Kotlin" => 1.2,
        "Shell" => 0.9,
        "HTML" | "CSS" | "SCSS" => 0.7,
        "Markdown" | "JSON" | "YAML" | "TOML" | "XML" => 0.5,
        _ => 1.0,
    }
}
