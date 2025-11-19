use serde::Serialize;
use anyhow::Result;
use std::fs::File;

pub struct CsvExporter;

impl CsvExporter {
    pub fn new() -> Self {
        Self
    }

    pub fn export<T: Serialize>(&self, data: &[T], path: &str) -> Result<()> {
        let file = File::create(path)?;
        let mut writer = csv::Writer::from_writer(file);

        for record in data {
            writer.serialize(record)?;
        }

        writer.flush()?;
        Ok(())
    }
}

impl Default for CsvExporter {
    fn default() -> Self {
        Self::new()
    }
}
