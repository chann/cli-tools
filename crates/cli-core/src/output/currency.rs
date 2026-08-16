/// Korean Won currency formatting with comma separators.
///
/// Formats a monetary value in KRW with `₩` prefix and comma grouping.
/// For display-only use; not suitable for accounting precision.
pub fn format_currency_krw(value: f64) -> String {
    let integer = value.round() as i64;
    format!("₩{}", format_integer(integer))
}

/// Format an unsigned integer with comma separators.
///
/// ```ignore
/// assert_eq!(format_integer(1234567 as i64), "1,234,567");
/// ```
pub fn format_integer(value: i64) -> String {
    let negative = value < 0;
    let abs_str = value.unsigned_abs().to_string();
    let formatted = abs_str
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .expect("digits are valid UTF-8")
        .join(",");
    if negative {
        format!("-{}", formatted)
    } else {
        formatted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn currency_adds_won_sign_and_comma() {
        assert_eq!(format_currency_krw(1_000_000.0), "₩1,000,000");
        assert_eq!(format_currency_krw(0.0), "₩0");
        assert_eq!(format_currency_krw(999.9), "₩1,000");
    }

    #[test]
    fn integer_handles_large_and_small() {
        assert_eq!(format_integer(0), "0");
        assert_eq!(format_integer(1), "1");
        assert_eq!(format_integer(12), "12");
        assert_eq!(format_integer(123), "123");
        assert_eq!(format_integer(1234), "1,234");
        assert_eq!(format_integer(1234567), "1,234,567");
    }
}