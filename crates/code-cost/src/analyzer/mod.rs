use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::git::GitAnalyzer;
use crate::metrics::MetricsCollector;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Analysis {
    pub total_lines: usize,
    pub code_lines: usize,
    pub comment_lines: usize,
    pub blank_lines: usize,
    pub total_files: usize,
    pub test_file_count: usize,
    pub commit_count: usize,
    pub contributor_count: usize,
    pub age_in_days: i64,
    pub language_stats: Vec<LanguageStat>,
    pub complexity_score: f64,
    pub maturity_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageStat {
    pub name: String,
    pub lines: usize,
    pub files: usize,
    pub weight: f64,
}

pub struct RepositoryAnalyzer {
    #[allow(dead_code)]
    hourly_rate: f64,
}

impl RepositoryAnalyzer {
    pub fn new(hourly_rate: f64) -> Self {
        Self { hourly_rate }
    }

    pub async fn analyze(&self, path: &Path) -> Result<Analysis> {
        let git_analyzer = GitAnalyzer::new();
        let metrics_collector = MetricsCollector::new();

        // Collect metrics
        let metrics = metrics_collector.collect(path)?;

        // Analyze git repository
        let git_stats = git_analyzer.analyze(path)?;

        // Calculate complexity score (1.0 - 5.0)
        let complexity_score = self.calculate_complexity(&metrics);

        // Calculate maturity score (0.0 - 1.0)
        let maturity_score = self.calculate_maturity(&metrics, &git_stats);

        Ok(Analysis {
            total_lines: metrics.total_lines,
            code_lines: metrics.code_lines,
            comment_lines: metrics.comment_lines,
            blank_lines: metrics.blank_lines,
            total_files: metrics.total_files,
            test_file_count: metrics.test_file_count,
            commit_count: git_stats.commit_count,
            contributor_count: git_stats.contributor_count,
            age_in_days: git_stats.age_in_days,
            language_stats: metrics.language_stats,
            complexity_score,
            maturity_score,
        })
    }

    fn calculate_complexity(&self, metrics: &crate::metrics::Metrics) -> f64 {
        // Base complexity from lines of code
        let loc_factor = (metrics.code_lines as f64 / 1000.0).min(3.0);

        // Language complexity weight
        let lang_factor = metrics
            .language_stats
            .iter()
            .map(|l| l.weight * (l.lines as f64 / metrics.total_lines as f64))
            .sum::<f64>();

        // File count factor
        let file_factor = (metrics.total_files as f64 / 100.0).min(1.5);

        // Combine factors (1.0 - 5.0 scale)
        (loc_factor + lang_factor + file_factor).clamp(1.0, 5.0)
    }

    fn calculate_maturity(&self, metrics: &crate::metrics::Metrics, git_stats: &crate::git::GitStats) -> f64 {
        let mut score = 0.0;

        // Test coverage indicator (based on test file presence)
        let test_ratio = metrics.test_file_count as f64 / metrics.total_files.max(1) as f64;
        score += (test_ratio * 0.3).min(0.3);

        // Documentation (README, docs, comments)
        let doc_ratio = metrics.comment_lines as f64 / metrics.code_lines.max(1) as f64;
        score += (doc_ratio * 0.2).min(0.2);
        if metrics.has_readme {
            score += 0.1;
        }

        // Project age and activity
        if git_stats.age_in_days > 365 {
            score += 0.1;
        }
        if git_stats.commit_count > 100 {
            score += 0.1;
        }

        // Multiple contributors
        if git_stats.contributor_count > 1 {
            score += 0.1;
        }
        if git_stats.contributor_count > 5 {
            score += 0.1;
        }

        score.min(1.0)
    }
}
