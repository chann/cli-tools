use anyhow::Result;
use owo_colors::OwoColorize;

pub fn show() -> Result<()> {
    println!("{}", "ASCII Table (0-127)".bold().underline());
    println!();
    
    println!("  Dec  Hex  Char  │   Dec  Hex  Char  │   Dec  Hex  Char  │   Dec  Hex  Char");
    println!("  ──── ──── ────  │   ──── ──── ────  │   ──── ──── ────  │   ──── ──── ────");

    for i in 0..32 {
        print_row(i);
        print!("  │ ");
        print_row(i + 32);
        print!("  │ ");
        print_row(i + 64);
        print!("  │ ");
        print_row(i + 96);
        println!();
    }

    Ok(())
}

fn print_row(i: u8) {
    let char_repr = match i {
        0 => "NUL".dimmed().to_string(),
        1 => "SOH".dimmed().to_string(),
        2 => "STX".dimmed().to_string(),
        3 => "ETX".dimmed().to_string(),
        4 => "EOT".dimmed().to_string(),
        5 => "ENQ".dimmed().to_string(),
        6 => "ACK".dimmed().to_string(),
        7 => "BEL".dimmed().to_string(),
        8 => "BS ".dimmed().to_string(),
        9 => "TAB".dimmed().to_string(),
        10 => "LF ".dimmed().to_string(),
        11 => "VT ".dimmed().to_string(),
        12 => "FF ".dimmed().to_string(),
        13 => "CR ".dimmed().to_string(),
        14 => "SO ".dimmed().to_string(),
        15 => "SI ".dimmed().to_string(),
        16 => "DLE".dimmed().to_string(),
        17 => "DC1".dimmed().to_string(),
        18 => "DC2".dimmed().to_string(),
        19 => "DC3".dimmed().to_string(),
        20 => "DC4".dimmed().to_string(),
        21 => "NAK".dimmed().to_string(),
        22 => "SYN".dimmed().to_string(),
        23 => "ETB".dimmed().to_string(),
        24 => "CAN".dimmed().to_string(),
        25 => "EM ".dimmed().to_string(),
        26 => "SUB".dimmed().to_string(),
        27 => "ESC".dimmed().to_string(),
        28 => "FS ".dimmed().to_string(),
        29 => "GS ".dimmed().to_string(),
        30 => "RS ".dimmed().to_string(),
        31 => "US ".dimmed().to_string(),
        32 => "SPC".dimmed().to_string(),
        127 => "DEL".dimmed().to_string(),
        _ => format!(" {} ", i as char).green().to_string(),
    };
    
    print!("  {:>3}  {:>3X}  {:>3}", i.cyan(), i.yellow(), char_repr);
}
