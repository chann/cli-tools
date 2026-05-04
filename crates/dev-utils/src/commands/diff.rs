use anyhow::Result;
use owo_colors::OwoColorize;
use similar::{ChangeTag, TextDiff};
use std::fs;
use std::path::Path;

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

    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => {
                print!("-{}", change.value().red());
            }
            ChangeTag::Insert => {
                print!("+{}", change.value().green());
            }
            ChangeTag::Equal => {
                print!(" {}", change.value().dimmed());
            }
        };
    }

    Ok(())
}
