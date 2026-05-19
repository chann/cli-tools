use anyhow::Result;
use cli_core::output::TableFormatter;
use cli_core::ui::Theme;
use std::path::Path;

pub fn analyze(path: &str) -> Result<()> {
    let p = Path::new(path);
    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("Property"),
        TableFormatter::header_cell("Value"),
    ]);

    table.add_row(vec![
        TableFormatter::value_cell("Input"),
        TableFormatter::value_cell(path),
    ]);

    if let Ok(abs) = std::fs::canonicalize(p) {
        table.add_row(vec![
            TableFormatter::value_cell("Absolute"),
            TableFormatter::highlight_cell(abs.display()),
        ]);
    } else {
        table.add_row(vec![
            TableFormatter::value_cell("Absolute"),
            TableFormatter::value_cell(Theme::dim("Could not resolve")),
        ]);
    }

    if let Some(parent) = p.parent() {
        table.add_row(vec![
            TableFormatter::value_cell("Parent"),
            TableFormatter::value_cell(parent.display()),
        ]);
    }

    if let Some(name) = p.file_name() {
        table.add_row(vec![
            TableFormatter::value_cell("File Name"),
            TableFormatter::value_cell(name.to_string_lossy()),
        ]);
    }

    if let Some(ext) = p.extension() {
        table.add_row(vec![
            TableFormatter::value_cell("Extension"),
            TableFormatter::value_cell(ext.to_string_lossy()),
        ]);
    }

    let exists = p.exists();
    table.add_row(vec![
        TableFormatter::value_cell("Exists"),
        if exists {
            TableFormatter::highlight_cell("Yes")
        } else {
            TableFormatter::value_cell("No")
        },
    ]);

    if exists {
        table.add_row(vec![
            TableFormatter::value_cell("Type"),
            if p.is_dir() {
                TableFormatter::value_cell("Directory")
            } else if p.is_file() {
                TableFormatter::value_cell("File")
            } else if p.is_symlink() {
                TableFormatter::value_cell("Symlink")
            } else {
                TableFormatter::value_cell("Other")
            },
        ]);

        if let Ok(meta) = p.metadata() {
            table.add_row(vec![
                TableFormatter::value_cell("Size"),
                TableFormatter::value_cell(format!("{} bytes", meta.len())),
            ]);
        }
    }

    println!("{}", Theme::header("Path Analysis"));
    println!("{}", table);

    Ok(())
}

pub fn normalize(path: &str) -> Result<()> {
    analyze(path)
}

pub fn resolve(path: &str) -> Result<()> {
    analyze(path)
}
