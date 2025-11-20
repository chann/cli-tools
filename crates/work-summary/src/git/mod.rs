pub mod time_estimator;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use git2::{Commit, DiffOptions, Repository};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    pub hash: String,
    pub author: String,
    pub email: String,
    pub timestamp: DateTime<Utc>,
    pub message: String,
    pub files_changed: usize,
    pub insertions: usize,
    pub deletions: usize,
    pub language_changes: HashMap<String, LanguageChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageChange {
    pub insertions: usize,
    pub deletions: usize,
}

pub struct CommitAnalyzer {
    repo: Repository,
}

impl CommitAnalyzer {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let repo = Repository::open(path).context("Failed to open git repository")?;
        Ok(Self { repo })
    }

    pub fn analyze_commits(&self, limit: Option<usize>) -> Result<Vec<CommitInfo>> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;

        let mut commits = Vec::new();
        let mut count = 0;

        for oid in revwalk {
            if let Some(lim) = limit {
                if count >= lim {
                    break;
                }
            }

            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;

            if let Ok(info) = self.extract_commit_info(&commit) {
                commits.push(info);
                count += 1;
            }
        }

        Ok(commits)
    }

    fn extract_commit_info(&self, commit: &Commit) -> Result<CommitInfo> {
        let author = commit.author();
        let timestamp = DateTime::from_timestamp(author.when().seconds(), 0)
            .unwrap_or_else(|| Utc::now());

        let hash = commit.id().to_string();
        let message = commit.message().unwrap_or("").to_string();
        let author_name = author.name().unwrap_or("Unknown").to_string();
        let email = author.email().unwrap_or("").to_string();

        let (files_changed, insertions, deletions, language_changes) =
            self.analyze_diff(commit)?;

        Ok(CommitInfo {
            hash,
            author: author_name,
            email,
            timestamp,
            message,
            files_changed,
            insertions,
            deletions,
            language_changes,
        })
    }

    fn analyze_diff(&self, commit: &Commit) -> Result<(usize, usize, usize, HashMap<String, LanguageChange>)> {
        let mut files_changed = 0;
        let mut total_insertions = 0;
        let mut total_deletions = 0;
        let mut language_changes: HashMap<String, LanguageChange> = HashMap::new();

        let tree = commit.tree()?;
        let parent_tree = if commit.parent_count() > 0 {
            Some(commit.parent(0)?.tree()?)
        } else {
            None
        };

        let diff = self.repo.diff_tree_to_tree(
            parent_tree.as_ref(),
            Some(&tree),
            Some(&mut DiffOptions::new()),
        )?;

        diff.foreach(
            &mut |delta, _| {
                files_changed += 1;

                if let Some(path) = delta.new_file().path() {
                    if let Some(ext) = path.extension() {
                        let lang = Self::extension_to_language(ext.to_str().unwrap_or(""));
                        language_changes.entry(lang.to_string()).or_insert(LanguageChange {
                            insertions: 0,
                            deletions: 0,
                        });
                    }
                }
                true
            },
            None,
            None,
            Some(&mut |_, _, line| {
                match line.origin() {
                    '+' => {
                        total_insertions += 1;
                        // Track by language if possible
                    }
                    '-' => {
                        total_deletions += 1;
                    }
                    _ => {}
                }
                true
            }),
        )?;

        Ok((files_changed, total_insertions, total_deletions, language_changes))
    }

    fn extension_to_language(ext: &str) -> &'static str {
        match ext {
            "rs" => "Rust",
            "py" => "Python",
            "js" => "JavaScript",
            "ts" => "TypeScript",
            "tsx" => "TypeScript",
            "jsx" => "JavaScript",
            "go" => "Go",
            "java" => "Java",
            "c" => "C",
            "cpp" | "cc" | "cxx" => "C++",
            "h" | "hpp" => "C/C++ Header",
            "rb" => "Ruby",
            "php" => "PHP",
            "swift" => "Swift",
            "kt" => "Kotlin",
            "cs" => "C#",
            "md" => "Markdown",
            "json" => "JSON",
            "yaml" | "yml" => "YAML",
            "toml" => "TOML",
            "xml" => "XML",
            "html" => "HTML",
            "css" => "CSS",
            "scss" => "SCSS",
            "sh" => "Shell",
            _ => "Other",
        }
    }
}
