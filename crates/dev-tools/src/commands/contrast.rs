use super::color;
use anyhow::Result;
use cli_core::output::TableFormatter;
use cli_core::ui::Theme;
use owo_colors::OwoColorize;

pub fn run(foreground: &str, background: &str) -> Result<()> {
    let fg = color::parse(foreground)?;
    let bg = color::parse(background)?;
    let ratio = contrast_ratio(fg, bg);

    println!("\n{}", Theme::header("WCAG Contrast"));
    println!(
        "  {} {}  on  {}   {}",
        "Preview:".dimmed(),
        format!(" {} ", foreground).truecolor(fg.0, fg.1, fg.2).on_truecolor(bg.0, bg.1, bg.2),
        format!(" {} ", background).on_truecolor(bg.0, bg.1, bg.2).truecolor(fg.0, fg.1, fg.2),
        Theme::highlight(&format!("{:.2}:1", ratio))
    );

    let mut table = TableFormatter::create_table();
    table.set_header(vec![
        TableFormatter::header_cell("Level"),
        TableFormatter::header_cell("Required"),
        TableFormatter::header_cell("Result"),
    ]);
    for (label, required) in [
        ("AA normal text", 4.5),
        ("AA large text / UI", 3.0),
        ("AAA normal text", 7.0),
        ("AAA large text", 4.5),
    ] {
        let pass = ratio >= required;
        table.add_row(vec![
            TableFormatter::value_cell(label),
            TableFormatter::value_cell(format!("{:.1}:1", required)),
            TableFormatter::value_cell(if pass { Theme::success("Pass") } else { Theme::error("Fail") }),
        ]);
    }
    println!("{}", table);
    Ok(())
}

/// WCAG 2.x contrast ratio, from 1.0 (equal) to 21.0 (black on white).
fn contrast_ratio(a: (u8, u8, u8), b: (u8, u8, u8)) -> f64 {
    let (la, lb) = (relative_luminance(a), relative_luminance(b));
    let (lighter, darker) = if la > lb { (la, lb) } else { (lb, la) };
    (lighter + 0.05) / (darker + 0.05)
}

fn relative_luminance((r, g, b): (u8, u8, u8)) -> f64 {
    fn channel(v: u8) -> f64 {
        let v = v as f64 / 255.0;
        if v <= 0.03928 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    }
    0.2126 * channel(r) + 0.7152 * channel(g) + 0.0722 * channel(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    const BLACK: (u8, u8, u8) = (0, 0, 0);
    const WHITE: (u8, u8, u8) = (255, 255, 255);

    #[test]
    fn black_on_white_is_21_to_1() {
        assert!((contrast_ratio(BLACK, WHITE) - 21.0).abs() < 0.01);
        assert!((contrast_ratio(WHITE, BLACK) - 21.0).abs() < 0.01);
    }

    #[test]
    fn identical_colors_are_1_to_1() {
        assert!((contrast_ratio(WHITE, WHITE) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn gray_767676_on_white_barely_passes_aa() {
        let ratio = contrast_ratio((0x76, 0x76, 0x76), WHITE);
        assert!((ratio - 4.54).abs() < 0.01, "got {}", ratio);
    }
}
