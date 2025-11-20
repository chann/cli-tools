use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueEstimate {
    pub estimated_hours: f64,
    pub base_hourly_rate: f64,
    pub developer_levels: Vec<DeveloperLevel>,
    pub recommended_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperLevel {
    pub level: String,
    pub multiplier: f64,
    pub hourly_rate: f64,
    pub total_value: f64,
}

impl ValueEstimate {
    pub fn calculate(estimated_hours: f64, base_hourly_rate: f64, total_changes: usize) -> Self {
        let complexity_factor = Self::calculate_complexity_factor(total_changes);

        let levels = vec![
            ("Junior", 1.0),
            ("Mid-level", 1.5),
            ("Senior", 2.0),
            ("Lead", 2.5),
            ("Principal", 3.0),
        ];

        let developer_levels: Vec<DeveloperLevel> = levels
            .iter()
            .map(|(level, multiplier)| {
                let hourly_rate = base_hourly_rate * multiplier * complexity_factor;
                let total_value = estimated_hours * hourly_rate;

                DeveloperLevel {
                    level: level.to_string(),
                    multiplier: *multiplier,
                    hourly_rate,
                    total_value,
                }
            })
            .collect();

        let mid_level_value = developer_levels
            .iter()
            .find(|l| l.level == "Mid-level")
            .map(|l| l.total_value)
            .unwrap_or(0.0);

        Self {
            estimated_hours,
            base_hourly_rate,
            developer_levels,
            recommended_value: mid_level_value,
        }
    }

    fn calculate_complexity_factor(total_changes: usize) -> f64 {
        if total_changes < 100 {
            0.8
        } else if total_changes < 500 {
            1.0
        } else if total_changes < 2000 {
            1.2
        } else {
            1.4
        }
    }
}
