use anyhow::{Context, Result};
use cli_core::ui::Theme;
use std::io::{self, Write};
use git2::Repository;
use std::path::Path;

pub fn wizard(path: &Path) -> Result<()> {
    let repo = Repository::open(path).context("Failed to open repository")?;
    
    // Check if there are staged changes
    let mut status_opts = git2::StatusOptions::new();
    status_opts.include_untracked(true);
    let statuses = repo.statuses(Some(&mut status_opts))?;
    
    let staged_count = statuses.iter()
        .filter(|s| s.status().intersects(git2::Status::INDEX_NEW | git2::Status::INDEX_MODIFIED | git2::Status::INDEX_DELETED | git2::Status::INDEX_RENAMED | git2::Status::INDEX_TYPECHANGE))
        .count();

    if staged_count == 0 {
        println!("{}", Theme::warning("No staged changes found. Please stage your changes before committing."));
        return Ok(());
    }

    println!("{}", Theme::header("📝 Conventional Commit Wizard"));
    println!("Found {} staged changes.", Theme::highlight(&staged_count.to_string()));
    println!();

    let types = vec![
        ("feat", "A new feature"),
        ("fix", "A bug fix"),
        ("docs", "Documentation only changes"),
        ("style", "Changes that do not affect the meaning of the code"),
        ("refactor", "A code change that neither fixes a bug nor adds a feature"),
        ("perf", "A code change that improves performance"),
        ("test", "Adding missing tests or correcting existing tests"),
        ("build", "Changes that affect the build system or external dependencies"),
        ("ci", "Changes to our CI configuration files and scripts"),
        ("chore", "Other changes that don't modify src or test files"),
        ("revert", "Reverts a previous commit"),
    ];

    for (i, (name, desc)) in types.iter().enumerate() {
        println!("  {}. {:<10} - {}", i + 1, Theme::highlight(name), desc);
    }
    println!();

    let type_idx = prompt_range("Select commit type", 1, types.len())? - 1;
    let commit_type = types[type_idx].0;

    let scope = prompt("Scope (optional, e.g. parser, ui)")?;
    let description = prompt_required("Short description")?;
    let body = prompt("Longer description (optional)")?;
    let is_breaking = prompt_bool("Is this a breaking change?")?;
    let footer = if is_breaking {
        prompt_required("BREAKING CHANGE description")?
    } else {
        prompt("Footer (optional, e.g. closes #123)")?
    };

    let mut message = commit_type.to_string();
    if !scope.is_empty() {
        message.push_str(&format!("({})", scope));
    }
    if is_breaking && footer.is_empty() {
        message.push('!');
    }
    message.push_str(&format!(": {}", description));

    if !body.is_empty() {
        message.push_str(&format!("\n\n{}", body));
    }

    if is_breaking {
        message.push_str(&format!("\n\nBREAKING CHANGE: {}", footer));
    } else if !footer.is_empty() {
        message.push_str(&format!("\n\n{}", footer));
    }

    println!();
    println!("{}", Theme::highlight("Proposed commit message:"));
    println!("{}", Theme::dim("---"));
    println!("{}", message);
    println!("{}", Theme::dim("---"));
    println!();

    if prompt_bool("Commit with this message?")? {
        let signature = repo.signature()?;
        let mut index = repo.index()?;
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;
        
        let head = repo.head()?;
        let parent = repo.find_commit(head.target().unwrap())?;
        
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &message,
            &tree,
            &[&parent],
        )?;
        
        println!("{}", Theme::success("Changes committed successfully!"));
    } else {
        println!("{}", Theme::info("Commit cancelled."));
    }

    Ok(())
}

fn prompt(label: &str) -> Result<String> {
    print!("{}: ", label);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn prompt_required(label: &str) -> Result<String> {
    loop {
        let input = prompt(label)?;
        if !input.is_empty() {
            return Ok(input);
        }
        println!("{}", Theme::error("This field is required."));
    }
}

fn prompt_range(label: &str, min: usize, max: usize) -> Result<usize> {
    loop {
        let input = prompt(&format!("{} ({}-{})", label, min, max))?;
        if let Ok(val) = input.parse::<usize>() {
            if val >= min && val <= max {
                return Ok(val);
            }
        }
        println!("{} Please enter a number between {} and {}.", Theme::error("Invalid input:"), min, max);
    }
}

fn prompt_bool(label: &str) -> Result<bool> {
    loop {
        let input = prompt(&format!("{} (y/n)", label))?.to_lowercase();
        if input == "y" || input == "yes" {
            return Ok(true);
        }
        if input == "n" || input == "no" || input.is_empty() {
            return Ok(false);
        }
        println!("{} Please enter 'y' or 'n'.", Theme::error("Invalid input:"));
    }
}
