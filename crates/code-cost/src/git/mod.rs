use anyhow::Result;
use git2::Repository;
use std::collections::HashSet;
use std::path::Path;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct GitStats {
    pub commit_count: usize,
    pub contributor_count: usize,
    pub age_in_days: i64,
    pub first_commit: Option<DateTime<Utc>>,
    pub last_commit: Option<DateTime<Utc>>,
}

pub struct GitAnalyzer;

impl GitAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, path: &Path) -> Result<GitStats> {
        let repo = Repository::open(path)?;

        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;

        let mut commit_count = 0;
        let mut contributors = HashSet::new();
        let mut first_commit_time: Option<i64> = None;
        let mut last_commit_time: Option<i64> = None;

        for oid in revwalk {
            let oid = oid?;
            let commit = repo.find_commit(oid)?;

            commit_count += 1;

            if let Some(author) = commit.author().email() {
                contributors.insert(author.to_string());
            }

            let commit_time = commit.time().seconds();

            if first_commit_time.is_none() || commit_time < first_commit_time.unwrap() {
                first_commit_time = Some(commit_time);
            }

            if last_commit_time.is_none() || commit_time > last_commit_time.unwrap() {
                last_commit_time = Some(commit_time);
            }
        }

        let age_in_days = if let (Some(first), Some(last)) = (first_commit_time, last_commit_time)
        {
            ((last - first) / 86400).max(0)
        } else {
            0
        };

        let first_commit = first_commit_time.map(|t| DateTime::from_timestamp(t, 0).unwrap());
        let last_commit = last_commit_time.map(|t| DateTime::from_timestamp(t, 0).unwrap());

        Ok(GitStats {
            commit_count,
            contributor_count: contributors.len(),
            age_in_days,
            first_commit,
            last_commit,
        })
    }
}

impl Default for GitAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
