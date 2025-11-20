pub mod value_calculator;
pub mod contribution;

use crate::git::CommitInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAnalysis {
    pub total_commits: usize,
    pub unique_contributors: usize,
    pub total_files_changed: usize,
    pub total_insertions: usize,
    pub total_deletions: usize,
    pub estimated_hours: f64,
    pub language_breakdown: HashMap<String, LanguageStats>,
    pub value_estimate: value_calculator::ValueEstimate,
    pub contribution_breakdown: Vec<contribution::ContributorStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageStats {
    pub insertions: usize,
    pub deletions: usize,
    pub net_change: i64,
    pub percentage: f64,
}

impl WorkAnalysis {
    pub fn from_commits(
        commits: &[CommitInfo],
        estimated_hours: f64,
        hourly_rate: f64,
    ) -> Self {
        let total_commits = commits.len();

        let mut contributors = std::collections::HashSet::new();
        let mut total_files_changed = 0;
        let mut total_insertions = 0;
        let mut total_deletions = 0;
        let mut language_map: HashMap<String, (usize, usize)> = HashMap::new();

        for commit in commits {
            contributors.insert(commit.email.clone());
            total_files_changed += commit.files_changed;
            total_insertions += commit.insertions;
            total_deletions += commit.deletions;

            for (lang, changes) in &commit.language_changes {
                let entry = language_map.entry(lang.clone()).or_insert((0, 0));
                entry.0 += changes.insertions;
                entry.1 += changes.deletions;
            }
        }

        let total_changes = (total_insertions + total_deletions) as f64;
        let language_breakdown: HashMap<String, LanguageStats> = language_map
            .into_iter()
            .map(|(lang, (ins, del))| {
                let lang_total = (ins + del) as f64;
                let percentage = if total_changes > 0.0 {
                    (lang_total / total_changes) * 100.0
                } else {
                    0.0
                };

                (
                    lang,
                    LanguageStats {
                        insertions: ins,
                        deletions: del,
                        net_change: ins as i64 - del as i64,
                        percentage,
                    },
                )
            })
            .collect();

        let value_estimate = value_calculator::ValueEstimate::calculate(
            estimated_hours,
            hourly_rate,
            total_insertions + total_deletions,
        );

        let contribution_breakdown =
            contribution::ContributorStats::from_commits(commits);

        Self {
            total_commits,
            unique_contributors: contributors.len(),
            total_files_changed,
            total_insertions,
            total_deletions,
            estimated_hours,
            language_breakdown,
            value_estimate,
            contribution_breakdown,
        }
    }
}
