use anyhow::Result;
use url::Url;
use owo_colors::OwoColorize;
use cli_core::output::TableFormatter;
use cli_core::ui::Theme;

pub fn parse(input: &str) -> Result<()> {
    let u = Url::parse(input)?;

    println!("{}", Theme::header(" URL Analysis "));
    println!("{} {}\n", Theme::info("URL:"), input.bright_white().bold());

    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("Component"),
        TableFormatter::header_cell("Value"),
    ]);

    let is_secure = u.scheme() == "https";
    let scheme_icon = if is_secure { "🔒" } else { "🔓" };
    table.add_row(vec![
        TableFormatter::value_cell("Scheme"),
        TableFormatter::value_cell(format!("{} {}", u.scheme().green(), scheme_icon)),
    ]);

    if let Some(host) = u.host_str() {
        table.add_row(vec![
            TableFormatter::value_cell("Host"),
            TableFormatter::highlight_cell(host),
        ]);
    }

    if let Some(port) = u.port() {
        table.add_row(vec![
            TableFormatter::value_cell("Port"),
            TableFormatter::value_cell(port.to_string().blue()),
        ]);
    }

    table.add_row(vec![
        TableFormatter::value_cell("Path"),
        TableFormatter::value_cell(u.path()),
    ]);

    if let Some(fragment) = u.fragment() {
        table.add_row(vec![
            TableFormatter::value_cell("Fragment"),
            TableFormatter::value_cell(fragment),
        ]);
    }

    if !u.username().is_empty() {
        table.add_row(vec![
            TableFormatter::value_cell("Username"),
            TableFormatter::value_cell(u.username()),
        ]);
    }

    if u.password().is_some() {
        table.add_row(vec![
            TableFormatter::value_cell("Password"),
            TableFormatter::value_cell("********"),
        ]);
    }

    println!("{}", table);

    if u.query().is_some() {
        println!("\n{}", Theme::header(" Query Parameters "));
        let mut q_table = TableFormatter::create_table();
        q_table.set_header(vec![
            TableFormatter::header_cell("Key"),
            TableFormatter::header_cell("Value"),
        ]);

        for (key, value) in u.query_pairs() {
            q_table.add_row(vec![
                TableFormatter::highlight_cell(key),
                TableFormatter::value_cell(value),
            ]);
        }
        println!("{}", q_table);
    }

    Ok(())
}

