use crate::analyzer::WorkAnalysis;
use crate::git::CommitInfo;
use crate::patterns::WorkPatterns;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositorySummary {
    pub path: PathBuf,
    pub period: Period,
    pub commits: Vec<CommitInfo>,
    pub analysis: WorkAnalysis,
    pub patterns: WorkPatterns,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Period {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub description: String,
}

impl RepositorySummary {
    pub fn new(
        path: PathBuf,
        commits: Vec<CommitInfo>,
        estimated_hours: f64,
        hourly_rate: f64,
    ) -> Self {
        let period = if commits.is_empty() {
            Period {
                start: Utc::now(),
                end: Utc::now(),
                description: "No commits".to_string(),
            }
        } else {
            let start = commits.last().unwrap().timestamp;
            let end = commits.first().unwrap().timestamp;
            let description = format!(
                "{} ~ {}",
                start.format("%Y-%m-%d"),
                end.format("%Y-%m-%d")
            );

            Period {
                start,
                end,
                description,
            }
        };

        let analysis = WorkAnalysis::from_commits(&commits, estimated_hours, hourly_rate);
        let patterns = WorkPatterns::analyze(&commits);

        Self {
            path,
            period,
            commits,
            analysis,
            patterns,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalSummary {
    pub repositories: Vec<RepositorySummary>,
    pub total_commits: usize,
    pub total_hours: f64,
    pub total_value: f64,
    pub total_contributors: usize,
}

impl TotalSummary {
    pub fn from_repositories(repositories: Vec<RepositorySummary>) -> Self {
        let total_commits = repositories.iter().map(|r| r.commits.len()).sum();

        let total_hours = repositories
            .iter()
            .map(|r| r.analysis.estimated_hours)
            .sum();

        let total_value = repositories
            .iter()
            .map(|r| r.analysis.value_estimate.recommended_value)
            .sum();

        let mut all_contributors = std::collections::HashSet::new();
        for repo in &repositories {
            for commit in &repo.commits {
                all_contributors.insert(commit.email.clone());
            }
        }

        Self {
            repositories,
            total_commits,
            total_hours,
            total_value,
            total_contributors: all_contributors.len(),
        }
    }
}
