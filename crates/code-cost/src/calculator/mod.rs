use serde::{Deserialize, Serialize};

use crate::analyzer::Analysis;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostEstimate {
    pub base_hours: f64,
    pub complexity_multiplier: f64,
    pub language_adjusted_hours: f64,
    pub maturity_bonus_hours: f64,
    pub estimated_hours: f64,
    pub hourly_rate: f64,
    pub total_cost: f64,
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

        CostEstimate {
            base_hours,
            complexity_multiplier,
            language_adjusted_hours,
            maturity_bonus_hours,
            estimated_hours,
            hourly_rate: self.hourly_rate,
            total_cost,
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
}
