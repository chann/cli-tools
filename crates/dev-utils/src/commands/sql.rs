use anyhow::Result;

pub fn format(sql: &str) -> Result<()> {
    let keywords = [
        "SELECT", "FROM", "WHERE", "INSERT", "UPDATE", "DELETE", "JOIN", "LEFT", "RIGHT",
        "INNER", "OUTER", "ON", "GROUP BY", "ORDER BY", "HAVING", "LIMIT", "OFFSET",
        "VALUES", "SET", "AND", "OR", "IN", "BETWEEN", "LIKE", "IS NULL", "IS NOT NULL",
        "CREATE TABLE", "ALTER TABLE", "DROP TABLE", "UNION", "ALL", "AS", "DISTINCT",
    ];

    let words: Vec<String> = sql.split_whitespace().map(|s| s.to_string()).collect();
    let mut output = String::new();
    let indent = 0;

    for word in words {
        let upper = word.to_uppercase();
        if keywords.contains(&upper.as_str()) {
            output.push('\n');
            for _ in 0..indent {
                output.push_str("  ");
            }
            output.push_str(&upper);
            output.push(' ');
        } else {
            output.push_str(&word);
            output.push(' ');
        }
    }

    println!("{}", output.trim());
    Ok(())
}
