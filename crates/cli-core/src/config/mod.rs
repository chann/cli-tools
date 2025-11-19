use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub hourly_rate: f64,
    pub currency: String,
}

impl Config {
    pub fn new(hourly_rate: f64, currency: impl Into<String>) -> Self {
        Self {
            hourly_rate,
            currency: currency.into(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hourly_rate: 10_030.0, // 2025년 대한민국 최저시급
            currency: "KRW".to_string(),
        }
    }
}
