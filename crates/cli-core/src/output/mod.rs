mod formatter;
mod table;
mod json;
mod csv_export;
mod html;
mod markdown;

pub use formatter::Formatter;
pub use table::TableFormatter;
pub use json::JsonFormatter;
pub use csv_export::CsvExporter;
pub use html::HtmlExporter;
pub use markdown::MarkdownExporter;

use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Table,
    Json,
    JsonPretty,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "table" => Ok(Self::Table),
            "json" => Ok(Self::Json),
            "json-pretty" | "pretty" => Ok(Self::JsonPretty),
            _ => anyhow::bail!("Unknown output format: {}", s),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Csv,
    Html,
    Markdown,
}

impl ExportFormat {
    pub fn from_extension(ext: &str) -> Result<Self> {
        match ext.to_lowercase().as_str() {
            "csv" => Ok(Self::Csv),
            "html" | "htm" => Ok(Self::Html),
            "md" | "markdown" => Ok(Self::Markdown),
            _ => anyhow::bail!("Unsupported export format: {}", ext),
        }
    }
}
