use serde::{Deserialize, Serialize};

use crate::analyzer::Analysis;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperLevel {
    pub level: String,
    pub hourly_rate: f64,
    pub estimated_cost: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIAnalysis {
    pub estimated_ai_usage: f64,
    pub code_quality_score: f64,
    pub potential_ai_indicators: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostEstimate {
    pub base_hours: f64,
    pub complexity_multiplier: f64,
    pub language_adjusted_hours: f64,
    pub maturity_bonus_hours: f64,
    pub estimated_hours: f64,
    pub hourly_rate: f64,
    pub total_cost: f64,
    pub developer_levels: Vec<DeveloperLevel>,
    pub ai_analysis: AIAnalysis,
}

pub struct CostCalculator {
    hourly_rate: f64,
}

impl CostCalculator {
    pub fn new(hourly_rate: f64) -> Self {
        Self { hourly_rate }
    }

    pub fn calculate(&self, analysis: &Analysis) -> CostEstimate {
        // Base calculation: assume 20 lines per hour for average code
        let base_hours = analysis.code_lines as f64 / 20.0;

        // Apply language weights
        let language_adjusted_hours = self.apply_language_weights(analysis, base_hours);

        // Apply complexity multiplier (1.0 - 2.0)
        let complexity_multiplier = self.calculate_complexity_multiplier(analysis);
        let complexity_adjusted_hours = language_adjusted_hours * complexity_multiplier;

        // Apply maturity bonus (up to 30% more)
        let maturity_bonus_hours = complexity_adjusted_hours * analysis.maturity_score * 0.3;

        // Total estimated hours including learning time
        let learning_time = self.estimate_learning_time(analysis);
        let estimated_hours = complexity_adjusted_hours + maturity_bonus_hours + learning_time;

        // Calculate total cost
        let total_cost = estimated_hours * self.hourly_rate;

        // Calculate developer level costs
        let developer_levels = self.calculate_developer_levels(estimated_hours);

        // Analyze AI usage
        let ai_analysis = self.analyze_ai_usage(analysis);

        CostEstimate {
            base_hours,
            complexity_multiplier,
            language_adjusted_hours,
            maturity_bonus_hours,
            estimated_hours,
            hourly_rate: self.hourly_rate,
            total_cost,
            developer_levels,
            ai_analysis,
        }
    }

    fn apply_language_weights(&self, analysis: &Analysis, base_hours: f64) -> f64 {
        if analysis.language_stats.is_empty() {
            return base_hours;
        }

        let total_lines = analysis.total_lines.max(1) as f64;

        analysis
            .language_stats
            .iter()
            .map(|lang| {
                let ratio = lang.lines as f64 / total_lines;
                let lang_hours = (lang.lines as f64 / 20.0) * lang.weight;
                lang_hours * ratio
            })
            .sum::<f64>()
            .max(base_hours)
    }

    fn calculate_complexity_multiplier(&self, analysis: &Analysis) -> f64 {
        // Complexity score is 1.0 - 5.0
        // Map it to multiplier 1.0 - 2.0
        let base_multiplier = 1.0 + (analysis.complexity_score - 1.0) * 0.25;

        // Adjust for file count (more files = better organization = slight reduction)
        let file_factor = if analysis.total_files > 50 {
            0.95
        } else {
            1.0
        };

        // Adjust for test coverage
        let test_factor = if analysis.maturity_score > 0.5 {
            0.98
        } else {
            1.0
        };

        (base_multiplier * file_factor * test_factor).clamp(1.0, 2.0)
    }

    fn estimate_learning_time(&self, analysis: &Analysis) -> f64 {
        // Estimate learning time based on language diversity and complexity
        let language_count = analysis.language_stats.len() as f64;
        let base_learning = language_count * 20.0; // 20 hours per language

        // Adjust for complexity
        let complexity_learning = analysis.complexity_score * 10.0;

        // Adjust for project size
        let size_learning = (analysis.code_lines as f64 / 1000.0).min(50.0);

        base_learning + complexity_learning + size_learning
    }

    fn calculate_developer_levels(&self, estimated_hours: f64) -> Vec<DeveloperLevel> {
        // Developer level hourly rates in KRW (South Korea market rates as of 2025)
        let levels = vec![
            ("Junior", 15_000.0),      // 1-3년차
            ("Mid-level", 25_000.0),   // 3-5년차
            ("Senior", 40_000.0),      // 5-10년차
            ("Lead", 60_000.0),        // 10+년차, 팀 리드
            ("Principal", 100_000.0),  // 아키텍트, 시니어 엔지니어
        ];

        levels
            .into_iter()
            .map(|(level, rate)| DeveloperLevel {
                level: level.to_string(),
                hourly_rate: rate,
                estimated_cost: estimated_hours * rate,
            })
            .collect()
    }

    fn analyze_ai_usage(&self, analysis: &Analysis) -> AIAnalysis {
        let mut ai_indicators = Vec::new();
        let mut ai_score: f64 = 0.0;

        // Indicator 1: High code quality with low comment ratio might indicate AI assistance
        let comment_ratio = analysis.comment_lines as f64 / analysis.code_lines.max(1) as f64;
        if comment_ratio < 0.05 && analysis.maturity_score > 0.5 {
            ai_indicators.push("Low comment ratio with high code quality".to_string());
            ai_score += 0.2;
        }

        // Indicator 2: Consistent file structure and organization
        let avg_lines_per_file = analysis.code_lines / analysis.total_files.max(1);
        if (50..=200).contains(&avg_lines_per_file) {
            ai_indicators.push("Consistent file size distribution".to_string());
            ai_score += 0.15;
        }

        // Indicator 3: High complexity with good maturity (tests, docs)
        if analysis.complexity_score > 3.0 && analysis.maturity_score > 0.6 {
            ai_indicators.push("High complexity with comprehensive testing".to_string());
            ai_score += 0.25;
        }

        // Indicator 4: Modern language usage
        let has_modern_lang = analysis.language_stats.iter().any(|l| {
            matches!(
                l.name.as_str(),
                "Rust" | "TypeScript" | "Go" | "Kotlin" | "Swift"
            )
        });
        if has_modern_lang {
            ai_indicators.push("Use of modern programming languages".to_string());
            ai_score += 0.1;
        }

        // Indicator 5: Good code-to-test ratio
        let test_ratio = analysis.test_file_count as f64 / analysis.total_files.max(1) as f64;
        if test_ratio > 0.2 {
            ai_indicators.push("Strong test coverage".to_string());
            ai_score += 0.2;
        }

        // Code quality score based on maturity and complexity balance
        let code_quality_score = (analysis.maturity_score * 0.6
            + (1.0 - (analysis.complexity_score - 1.0) / 4.0).max(0.0) * 0.4)
            .clamp(0.0, 1.0);

        AIAnalysis {
            estimated_ai_usage: ai_score.clamp(0.0, 1.0),
            code_quality_score,
            potential_ai_indicators: ai_indicators,
        }
    }
}
