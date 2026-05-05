use anyhow::{Context, Result};
use cli_core::ui::Theme;
use git2::Repository;
use std::collections::BTreeMap;
use std::path::Path;

pub struct ChangelogOptions {
    pub from: Option<String>,
    pub to: Option<String>,
    pub limit: Option<usize>,
    pub format: String,
}

pub fn generate(path: &Path, options: ChangelogOptions) -> Result<()> {
    let repo = Repository::open(path).context("Failed to open repository")?;
    
    let mut revwalk = repo.revwalk().context("Failed to create revwalk")?;
    
    if let Some(to) = &options.to {
        revwalk.push_ref(to).context(format!("Failed to find ref: {}", to))?;
    } else {
        revwalk.push_head().context("Failed to push HEAD to revwalk")?;
    }

    if let Some(from) = &options.from {
        revwalk.hide_ref(from).context(format!("Failed to hide ref: {}", from))?;
    }

    let mut categories: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut count = 0;

    for oid_res in revwalk {
        let oid = oid_res?;
        let commit = repo.find_commit(oid)?;
        let summary = commit.summary().unwrap_or("No summary");

        if let Some((cat, msg)) = parse_conventional(summary) {
            categories.entry(cat).or_default().push(msg);
        } else {
            categories.entry("other".to_string()).or_default().push(summary.to_string());
        }

        count += 1;
        if let Some(limit) = options.limit {
            if count >= limit {
                break;
            }
        }
    }

    if categories.is_empty() {
        println!("{}", Theme::warning("No commits found in the specified range."));
        return Ok(());
    }

    if options.format == "markdown" {
        println!("# Changelog");
        println!("\nGenerating changelog for {} commits", count);
        println!();
    } else {
        println!(
            "{} Generating changelog for {} commits",
            Theme::info("Info:"),
            count
        );
        println!();
    }

    // Custom order for categories
    let order = vec!["feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "ci", "chore", "other"];
    
    for cat_name in order {
        let (display_name, emoji) = match cat_name {
            "feat" => ("Features", "🚀"),
            "fix" => ("Bug Fixes", "🐛"),
            "docs" => ("Documentation", "📚"),
            "style" => ("Style", "🎨"),
            "refactor" => ("Refactoring", "🔨"),
            "perf" => ("Performance Improvements", "⚡"),
            "test" => ("Tests", "🧪"),
            "build" => ("Build System", "🏗️"),
            "ci" => ("Continuous Integration", "👷"),
            "chore" => ("Chore", "🧹"),
            "other" => ("Other Changes", "📝"),
            _ => (cat_name, "🔹"),
        };

        if let Some(messages) = categories.get(cat_name) {
            if options.format == "markdown" {
                println!("## {} {}", emoji, display_name);
                for msg in messages {
                    println!("- {}", msg);
                }
                println!();
            } else {
                println!("{} {}", emoji, Theme::highlight(display_name));
                for msg in messages {
                    println!("  • {}", msg);
                }
                println!();
            }
        }
    }

    Ok(())
}

fn parse_conventional(summary: &str) -> Option<(String, String)> {
    // Basic conventional commit parser: type(scope)!: description
    let re = regex::Regex::new(r"^(\w+)(?:\(([^)]+)\))?(!)?:\s+(.*)$").unwrap();
    if let Some(caps) = re.captures(summary) {
        let cat = caps.get(1).map(|m| m.as_str().to_lowercase()).unwrap_or_default();
        let scope = caps.get(2).map(|m| format!("**{}**: ", m.as_str())).unwrap_or_default();
        let breaking = caps.get(3).is_some();
        let description = caps.get(4).map(|m| m.as_str()).unwrap_or_default();

        let mut msg = format!("{}{}", scope, description);
        if breaking {
            msg = format!("{} [BREAKING CHANGE]", msg);
        }
        Some((cat, msg))
    } else {
        None
    }
}
