use owo_colors::OwoColorize;

pub struct Theme;

impl Theme {
    pub fn success(text: impl AsRef<str>) -> String {
        format!("✓ {}", text.as_ref().green())
    }

    pub fn error(text: impl AsRef<str>) -> String {
        format!("✗ {}", text.as_ref().red())
    }

    pub fn info(text: impl AsRef<str>) -> String {
        format!("ℹ {}", text.as_ref().cyan())
    }

    pub fn warning(text: impl AsRef<str>) -> String {
        format!("⚠ {}", text.as_ref().yellow())
    }

    pub fn header(text: impl AsRef<str>) -> String {
        text.as_ref().bold().blue().to_string()
    }

    pub fn value(text: impl AsRef<str>) -> String {
        text.as_ref().bright_white().to_string()
    }

    pub fn highlight(text: impl AsRef<str>) -> String {
        text.as_ref().bright_green().bold().to_string()
    }

    pub fn dim(text: impl AsRef<str>) -> String {
        text.as_ref().dimmed().to_string()
    }
}
