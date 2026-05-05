use anyhow::Result;
use similar::{ChangeTag, TextDiff};
use std::fs;
use std::path::Path;
use cli_core::ui::Theme;

pub fn compare(old: &str, new: &str, is_file: bool) -> Result<()> {
    let (old_content, new_content) = if is_file {
        let old_path = Path::new(old);
        let new_path = Path::new(new);
        
        if !old_path.exists() {
            return Err(anyhow::anyhow!("Old file does not exist: {}", old));
        }
        if !new_path.exists() {
            return Err(anyhow::anyhow!("New file does not exist: {}", new));
        }
        
        (fs::read_to_string(old_path)?, fs::read_to_string(new_path)?)
    } else {
        (old.to_string(), new.to_string())
    };

    let diff = TextDiff::from_lines(&old_content, &new_content);

    println!("{}", Theme::header("--- Diff Comparison ---"));
    if is_file {
        println!("{} {} vs {}", Theme::dim("Files:"), old, new);
    }
    println!();

    let mut additions = 0;
    let mut deletions = 0;

    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => {
                deletions += 1;
                print!("{}", Theme::red(format!("-{}", change.value())));
            }
            ChangeTag::Insert => {
                additions += 1;
                print!("{}", Theme::green(format!("+{}", change.value())));
            }
            ChangeTag::Equal => {
                print!(" {}", Theme::dim(change.value()));
            }
        };
    }

    println!("\n{}", Theme::header("--- Summary ---"));
    println!("{}: {}", Theme::success("Additions"), additions);
    println!("{}: {}", Theme::error("Deletions"), deletions);
    println!("{}: {}", Theme::info("Total Changes"), additions + deletions);

    Ok(())
}
