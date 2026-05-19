use uuid::Uuid;
use anyhow::{Result, Context};
use cli_core::ui::Theme;
use cli_core::output::TableFormatter;
use chrono::{Utc, TimeZone};

pub fn generate(count: usize, v7: bool) -> Result<()> {
    if count == 0 {
        return Ok(());
    }

    if count == 1 {
        let uuid = if v7 { Uuid::now_v7() } else { Uuid::new_v4() };
        println!("{}", Theme::highlight(uuid.to_string()));
        return Ok(());
    }

    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("#"),
        TableFormatter::header_cell("UUID"),
        TableFormatter::header_cell("Version"),
    ]);

    for i in 0..count {
        let uuid = if v7 { Uuid::now_v7() } else { Uuid::new_v4() };
        table.add_row(vec![
            TableFormatter::value_cell(i + 1),
            TableFormatter::highlight_cell(uuid.to_string()),
            TableFormatter::value_cell(if v7 { "v7 (Timestamp)" } else { "v4 (Random)" }),
        ]);
    }

    println!("\n{}", Theme::info(format!("Generated {} UUIDs:", count)));
    println!("{}", table);
    Ok(())
}

pub fn inspect(id: &str) -> Result<()> {
    let uuid = Uuid::parse_str(id)
        .with_context(|| format!("Failed to parse UUID: '{}'", id))?;

    let version = uuid.get_version();
    let variant = uuid.get_variant();

    println!("\n{}", Theme::header("UUID Inspection Report"));
    
    let mut table = TableFormatter::create_table();
    table.add_row(vec![
        TableFormatter::header_cell("Field"),
        TableFormatter::header_cell("Value"),
    ]);

    table.add_row(vec![
        TableFormatter::value_cell("UUID"),
        TableFormatter::highlight_cell(uuid.to_string()),
    ]);

    table.add_row(vec![
        TableFormatter::value_cell("Version"),
        TableFormatter::value_cell(format!("{:?}", version)),
    ]);

    table.add_row(vec![
        TableFormatter::value_cell("Variant"),
        TableFormatter::value_cell(format!("{:?}", variant)),
    ]);

    if let Some(version) = version {
        match version {
            uuid::Version::Nil => {
                table.add_row(vec![
                    TableFormatter::value_cell("Description"),
                    TableFormatter::value_cell("The Nil UUID is a special form of UUID that is specified to have all 128 bits set to zero."),
                ]);
            }
            uuid::Version::Mac => {
                table.add_row(vec![
                    TableFormatter::value_cell("Description"),
                    TableFormatter::value_cell("v1 (MAC address & timestamp)"),
                ]);
            }
            uuid::Version::Dce => {
                table.add_row(vec![
                    TableFormatter::value_cell("Description"),
                    TableFormatter::value_cell("v2 (DCE Security version)"),
                ]);
            }
            uuid::Version::Md5 => {
                table.add_row(vec![
                    TableFormatter::value_cell("Description"),
                    TableFormatter::value_cell("v3 (MD5 hash name-based)"),
                ]);
            }
            uuid::Version::Random => {
                table.add_row(vec![
                    TableFormatter::value_cell("Description"),
                    TableFormatter::value_cell("v4 (Fully random)"),
                ]);
            }
            uuid::Version::Sha1 => {
                table.add_row(vec![
                    TableFormatter::value_cell("Description"),
                    TableFormatter::value_cell("v5 (SHA-1 hash name-based)"),
                ]);
            }
            uuid::Version::SortMac => {
                table.add_row(vec![
                    TableFormatter::value_cell("Description"),
                    TableFormatter::value_cell("v6 (Reordered v1 for database locality)"),
                ]);
            }
            uuid::Version::SortRand => {
                table.add_row(vec![
                    TableFormatter::value_cell("Description"),
                    TableFormatter::value_cell("v7 (Unix timestamp & random)"),
                ]);

                // Extract timestamp from v7
                let (secs, nanos) = uuid.get_timestamp()
                    .map(|ts| ts.to_unix())
                    .unwrap_or((0, 0));
                
                if secs > 0 {
                    let dt = Utc.timestamp_opt(secs as i64, nanos).unwrap();
                    table.add_row(vec![
                        TableFormatter::value_cell("Timestamp"),
                        TableFormatter::highlight_cell(dt.to_rfc3339()),
                    ]);
                }
            }
            uuid::Version::Custom => {
                table.add_row(vec![
                    TableFormatter::value_cell("Description"),
                    TableFormatter::value_cell("v8 (Custom implementation)"),
                ]);
            }
            _ => {}
        }
    }

    println!("{}", table);
    Ok(())
}
