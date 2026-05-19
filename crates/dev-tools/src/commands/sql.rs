use anyhow::Result;
use sqlformat::{format as sql_format, FormatOptions, QueryParams, Indent};
use cli_core::ui::Theme;

pub fn format(sql: &str, indent_size: usize, use_tabs: bool, lowercase: bool) -> Result<()> {
    let indent = if use_tabs {
        Indent::Tabs
    } else {
        Indent::Spaces(indent_size as u8)
    };

    let options = FormatOptions {
        indent,
        uppercase: !lowercase,
        lines_between_queries: 1,
    };

    let formatted = sql_format(sql, &QueryParams::None, options);

    println!("{}", Theme::header("--- Formatted SQL ---"));
    println!("{}", formatted);
    
    println!("\n{}", Theme::dim(format!(
        "Indent: {} | Keywords: {}", 
        if use_tabs { "Tabs".to_string() } else { format!("{} spaces", indent_size) },
        if lowercase { "Lowercase" } else { "Uppercase" }
    )));

    Ok(())
}
