use anyhow::Result;
use std::collections::HashSet;
use rand::seq::SliceRandom;
use owo_colors::OwoColorize;

pub fn process(
    text: &str,
    op: &str,
    reverse: bool,
    unique: bool,
    from: Option<String>,
    to: Option<String>,
    line_numbers: bool,
    prefix: Option<String>,
    suffix: Option<String>,
    truncate: Option<usize>,
) -> Result<()> {
    let mut lines: Vec<String> = match op {
        "sort" => {
            let mut l: Vec<String> = text.lines().map(|s| s.to_string()).collect();
            if unique {
                let mut seen = HashSet::new();
                l.retain(|line| seen.insert(line.clone()));
            }
            l.sort();
            if reverse {
                l.reverse();
            }
            l
        }
        "reverse" => {
            let mut l: Vec<String> = text.lines().map(|s| s.to_string()).collect();
            l.reverse();
            l
        }
        "unique" => {
            let mut l = Vec::new();
            let mut seen = HashSet::new();
            for line in text.lines() {
                if seen.insert(line) {
                    l.push(line.to_string());
                }
            }
            l
        }
        "trim" => text.lines().map(|s| s.trim().to_string()).collect(),
        "upper" => text.lines().map(|s| s.to_uppercase()).collect(),
        "lower" => text.lines().map(|s| s.to_lowercase()).collect(),
        "shuffle" => {
            let mut l: Vec<String> = text.lines().map(|s| s.to_string()).collect();
            let mut rng = rand::thread_rng();
            l.shuffle(&mut rng);
            l
        }
        "replace" => {
            let f = from.ok_or_else(|| anyhow::anyhow!("Replace requires --from"))?;
            let t = to.ok_or_else(|| anyhow::anyhow!("Replace requires --to"))?;
            text.lines().map(|s| s.replace(&f, &t)).collect()
        }
        "prefix" => {
            let p = prefix.clone().ok_or_else(|| anyhow::anyhow!("Prefix requires --prefix"))?;
            text.lines().map(|s| format!("{}{}", p, s)).collect()
        }
        "suffix" => {
            let s = suffix.clone().ok_or_else(|| anyhow::anyhow!("Suffix requires --suffix"))?;
            text.lines().map(|line| format!("{}{}", line, s)).collect()
        }
        "truncate" => {
            let len = truncate.ok_or_else(|| anyhow::anyhow!("Truncate requires --truncate"))?;
            text.lines().map(|s| {
                if s.len() > len {
                    if len > 3 {
                        format!("{}...", &s[..len - 3])
                    } else {
                        s[..len].to_string()
                    }
                } else {
                    s.to_string()
                }
            }).collect()
        }
        "count" => {
            let lines_count = text.lines().count();
            let words = text.split_whitespace().count();
            let chars = text.chars().count();
            println!("{}: {}", "Lines".bold(), lines_count.cyan());
            println!("{}: {}", "Words".bold(), words.cyan());
            println!("{}: {}", "Chars".bold(), chars.cyan());
            return Ok(());
        }
        _ => anyhow::bail!("Unsupported text operation: {}", op),
    };

    // Apply global prefix/suffix/truncate if provided (even if op is not that)
    if op != "prefix" {
        if let Some(p) = prefix {
            lines = lines.into_iter().map(|line| format!("{}{}", p, line)).collect();
        }
    }
    if op != "suffix" {
        if let Some(s) = suffix {
            lines = lines.into_iter().map(|line| format!("{}{}", line, s)).collect();
        }
    }
    if op != "truncate" {
        if let Some(len) = truncate {
            lines = lines.into_iter().map(|s| {
                if s.len() > len {
                    if len > 3 {
                        format!("{}...", &s[..len - 3])
                    } else {
                        s[..len].to_string()
                    }
                } else {
                    s
                }
            }).collect();
        }
    }

    for (i, line) in lines.iter().enumerate() {
        if line_numbers {
            print!("{:>4} | ", (i + 1).dimmed());
        }
        println!("{}", line);
    }

    Ok(())
}
