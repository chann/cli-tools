use serde::Serialize;
use anyhow::Result;
use super::Formatter;

pub struct JsonFormatter {
    pretty: bool,
}

impl JsonFormatter {
    pub fn new(pretty: bool) -> Self {
        Self { pretty }
    }
}

impl<T: Serialize> Formatter<T> for JsonFormatter {
    fn format(&self, data: &T) -> Result<String> {
        if self.pretty {
            Ok(serde_json::to_string_pretty(data)?)
        } else {
            Ok(serde_json::to_string(data)?)
        }
    }
}
