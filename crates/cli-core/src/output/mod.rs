mod formatter;
mod table;
mod json;
mod csv_export;
mod html;
mod markdown;
mod currency;

pub use formatter::Formatter;
pub use table::TableFormatter;
pub use json::JsonFormatter;
pub use csv_export::CsvExporter;
pub use html::HtmlExporter;
pub use markdown::MarkdownExporter;
pub use currency::{format_currency_krw, format_integer};

use anyhow::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Table,
    Json,
    JsonPretty,
}

impl OutputFormat {
    /// Parses an output format without requiring callers to import [`std::str::FromStr`].
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self> {
        s.parse()
    }
}

impl std::str::FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
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

#[cfg(test)]
mod tests {
    use super::OutputFormat;

    #[test]
    fn output_format_parses_supported_names_case_insensitively() {
        for (input, expected) in [
            ("table", OutputFormat::Table),
            ("JSON", OutputFormat::Json),
            ("json-pretty", OutputFormat::JsonPretty),
            ("pretty", OutputFormat::JsonPretty),
        ] {
            assert_eq!(input.parse::<OutputFormat>().unwrap(), expected);
        }
    }

    #[test]
    fn output_format_rejects_unknown_names() {
        let error = "yaml".parse::<OutputFormat>().unwrap_err();

        assert_eq!(error.to_string(), "Unknown output format: yaml");
    }

    #[test]
    fn output_format_preserves_inherent_parser() {
        assert_eq!(
            OutputFormat::from_str("pretty").unwrap(),
            OutputFormat::JsonPretty
        );
    }
}
