use anyhow::Result;
use std::collections::HashSet;

pub fn sort(text: &str, reverse: bool, unique: bool) -> Result<()> {
    let mut lines: Vec<&str> = text.lines().collect();
    
    if unique {
        let mut seen = HashSet::new();
        lines.retain(|&line| seen.insert(line));
    }

    lines.sort();
    
    if reverse {
        lines.reverse();
    }

    for line in lines {
        println!("{}", line);
    }
    Ok(())
}

pub fn filter_unique(text: &str) -> Result<()> {
    let mut seen = HashSet::new();
    for line in text.lines() {
        if seen.insert(line) {
            println!("{}", line);
        }
    }
    Ok(())
}

pub fn reverse(text: &str) -> Result<()> {
    let mut lines: Vec<&str> = text.lines().collect();
    lines.reverse();
    for line in lines {
        println!("{}", line);
    }
    Ok(())
}

pub fn trim(text: &str) -> Result<()> {
    for line in text.lines() {
        println!("{}", line.trim());
    }
    Ok(())
}

pub fn to_upper(text: &str) -> Result<()> {
    println!("{}", text.to_uppercase());
    Ok(())
}

pub fn to_lower(text: &str) -> Result<()> {
    println!("{}", text.to_lowercase());
    Ok(())
}

pub fn replace(text: &str, from: &str, to: &str) -> Result<()> {
    println!("{}", text.replace(from, to));
    Ok(())
}
