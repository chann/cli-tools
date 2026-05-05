use anyhow::Result;
use rand::seq::SliceRandom;
use cli_core::output::TableFormatter;
use cli_core::ui::Theme;
use woothee::parser::Parser;
use comfy_table::Cell;

const USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:122.0) Gecko/20100101 Firefox/122.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:122.0) Gecko/20100101 Firefox/122.0",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 17_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Mobile/15E148 Safari/604.1",
    "Mozilla/5.0 (iPad; CPU OS 17_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Mobile/15E148 Safari/604.1",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36 Edg/121.0.0.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15",
];

pub fn generate() -> Result<()> {
    let mut rng = rand::thread_rng();
    if let Some(ua) = USER_AGENTS.choose(&mut rng) {
        println!("{}", Theme::header("Generated User Agent"));
        println!("  {}", Theme::highlight(ua.to_string()));
        println!();
        inspect(ua)?;
    }
    Ok(())
}

pub fn inspect(ua: &str) -> Result<()> {
    let parser = Parser::new();
    let result = parser.parse(ua);

    println!("{}", Theme::header("User Agent Inspection"));
    
    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("Property"),
        TableFormatter::header_cell("Value"),
    ]);

    if let Some(r) = result {
        table.add_row(vec![
            TableFormatter::value_cell("Browser"),
            TableFormatter::highlight_cell(r.name),
        ]);
        table.add_row(vec![
            TableFormatter::value_cell("Browser Version"),
            TableFormatter::value_cell(r.version),
        ]);
        table.add_row(vec![
            TableFormatter::value_cell("Operating System"),
            TableFormatter::highlight_cell(r.os),
        ]);
        table.add_row(vec![
            TableFormatter::value_cell("OS Version"),
            TableFormatter::value_cell(r.os_version),
        ]);
        table.add_row(vec![
            TableFormatter::value_cell("Category"),
            TableFormatter::value_cell(r.category),
        ]);
        table.add_row(vec![
            TableFormatter::value_cell("Vendor"),
            TableFormatter::value_cell(r.vendor),
        ]);
    } else {
        table.add_row(vec![
            TableFormatter::value_cell("Status"),
            Cell::new(Theme::warning("Unable to parse User Agent")),
        ]);
    }

    println!("{}", table);
    Ok(())
}
