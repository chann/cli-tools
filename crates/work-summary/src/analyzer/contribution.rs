use crate::git::CommitInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributorStats {
    pub name: String,
    pub email: String,
    pub commit_count: usize,
    pub insertions: usize,
    pub deletions: usize,
    pub files_changed: usize,
    pub percentage: f64,
}

impl ContributorStats {
    pub fn from_commits(commits: &[CommitInfo]) -> Vec<Self> {
        let mut contributor_map: HashMap<String, ContributorData> = HashMap::new();

        let total_commits = commits.len();

        for commit in commits {
            let entry = contributor_map
                .entry(commit.email.clone())
                .or_insert_with(|| ContributorData {
                    name: commit.author.clone(),
                    email: commit.email.clone(),
                    commit_count: 0,
                    insertions: 0,
                    deletions: 0,
                    files_changed: 0,
                });

            entry.commit_count += 1;
            entry.insertions += commit.insertions;
            entry.deletions += commit.deletions;
            entry.files_changed += commit.files_changed;
        }

        let mut stats: Vec<ContributorStats> = contributor_map
            .into_iter()
            .map(|(_, data)| {
                let percentage = if total_commits > 0 {
                    (data.commit_count as f64 / total_commits as f64) * 100.0
                } else {
                    0.0
                };

                ContributorStats {
                    name: data.name,
                    email: data.email,
                    commit_count: data.commit_count,
                    insertions: data.insertions,
                    deletions: data.deletions,
                    files_changed: data.files_changed,
                    percentage,
                }
            })
            .collect();

        stats.sort_by(|a, b| b.commit_count.cmp(&a.commit_count));

        stats
    }
}

struct ContributorData {
    name: String,
    email: String,
    commit_count: usize,
    insertions: usize,
    deletions: usize,
    files_changed: usize,
}
