use anyhow::Result;
use serde::Serialize;

pub trait Formatter<T: Serialize> {
    fn format(&self, data: &T) -> Result<String>;
}

pub trait Exporter<T: Serialize> {
    fn export(&self, data: &T, path: &str) -> Result<()>;
}
