use anyhow::Result;
use cli_core::output::TableFormatter;
use cli_core::ui::Theme;
use comfy_table::{Cell, Color};

pub fn show() -> Result<()> {
    println!("{}", Theme::header(" ASCII Table (0-127) "));
    println!();
    
    let mut table = TableFormatter::create_table();
    
    table.set_header(vec![
        TableFormatter::header_cell("Dec"),
        TableFormatter::header_cell("Hex"),
        TableFormatter::header_cell("Char"),
        TableFormatter::header_cell(" "),
        TableFormatter::header_cell("Dec"),
        TableFormatter::header_cell("Hex"),
        TableFormatter::header_cell("Char"),
        TableFormatter::header_cell(" "),
        TableFormatter::header_cell("Dec"),
        TableFormatter::header_cell("Hex"),
        TableFormatter::header_cell("Char"),
        TableFormatter::header_cell(" "),
        TableFormatter::header_cell("Dec"),
        TableFormatter::header_cell("Hex"),
        TableFormatter::header_cell("Char"),
    ]);

    for i in 0..32 {
        let mut row = Vec::new();
        
        // Column 1
        add_cols(&mut row, i);
        row.push(Cell::new("│").fg(Color::Grey));
        
        // Column 2
        add_cols(&mut row, i + 32);
        row.push(Cell::new("│").fg(Color::Grey));
        
        // Column 3
        add_cols(&mut row, i + 64);
        row.push(Cell::new("│").fg(Color::Grey));
        
        // Column 4
        add_cols(&mut row, i + 96);
        
        table.add_row(row);
    }

    println!("{}", table);

    Ok(())
}

fn add_cols(row: &mut Vec<Cell>, i: u8) {
    row.push(Cell::new(format!("{:>3}", i)).fg(Color::Cyan));
    row.push(Cell::new(format!("{:>3X}", i)).fg(Color::Yellow));
    row.push(get_char_cell(i));
}

fn get_char_cell(i: u8) -> Cell {
    let char_repr = match i {
        0 => "NUL".to_string(),
        1 => "SOH".to_string(),
        2 => "STX".to_string(),
        3 => "ETX".to_string(),
        4 => "EOT".to_string(),
        5 => "ENQ".to_string(),
        6 => "ACK".to_string(),
        7 => "BEL".to_string(),
        8 => "BS ".to_string(),
        9 => "TAB".to_string(),
        10 => "LF ".to_string(),
        11 => "VT ".to_string(),
        12 => "FF ".to_string(),
        13 => "CR ".to_string(),
        14 => "SO ".to_string(),
        15 => "SI ".to_string(),
        16 => "DLE".to_string(),
        17 => "DC1".to_string(),
        18 => "DC2".to_string(),
        19 => "DC3".to_string(),
        20 => "DC4".to_string(),
        21 => "NAK".to_string(),
        22 => "SYN".to_string(),
        23 => "ETB".to_string(),
        24 => "CAN".to_string(),
        25 => "EM ".to_string(),
        26 => "SUB".to_string(),
        27 => "ESC".to_string(),
        28 => "FS ".to_string(),
        29 => "GS ".to_string(),
        30 => "RS ".to_string(),
        31 => "US ".to_string(),
        32 => "SPC".to_string(),
        127 => "DEL".to_string(),
        _ => format!(" {} ", i as char),
    };
    
    if i <= 32 || i == 127 {
        Cell::new(char_repr).fg(Color::Grey)
    } else {
        TableFormatter::highlight_cell(char_repr)
    }
}
