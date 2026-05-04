use anyhow::{Context, Result};
use cli_core::ui::Theme;
use git2::{BranchType, Repository};
use std::path::Path;

pub fn cleanup(path: &Path, force: bool, target: Option<&str>) -> Result<()> {
    let repo = Repository::open(path).context("Failed to open repository")?;

    let target_branch_name = if let Some(t) = target {
        t.to_string()
    } else {
        // Try to find default branch
        if repo.find_branch("main", BranchType::Local).is_ok() {
            "main".to_string()
        } else if repo.find_branch("master", BranchType::Local).is_ok() {
            "master".to_string()
        } else {
            return Err(anyhow::anyhow!(
                "Could not find 'main' or 'master' branch. Please specify target branch with --target"
            ));
        }
    };

    println!(
        "{} Checking branches merged into {}",
        Theme::info("Info:"),
        Theme::highlight(&target_branch_name)
    );

    let target_branch = repo
        .find_branch(&target_branch_name, BranchType::Local)
        .context("Failed to find target branch")?;
    let target_commit = target_branch
        .get()
        .peel_to_commit()
        .context("Failed to get target commit")?;

    let mut merged_branches = Vec::new();

    let branches = repo
        .branches(Some(BranchType::Local))
        .context("Failed to list branches")?;

    for branch_res in branches {
        let (branch, _) = branch_res?;
        let name = branch
            .name()?
            .ok_or_else(|| anyhow::anyhow!("Branch name is not valid UTF-8"))?
            .to_string();

        if name == target_branch_name {
            continue;
        }

        let branch_commit = branch.get().peel_to_commit()?;

        // Check if branch is merged into target
        // A branch is merged if its tip is the same as or an ancestor of the target branch's tip
        if branch_commit.id() == target_commit.id()
            || repo.graph_descendant_of(target_commit.id(), branch_commit.id())?
        {
            merged_branches.push(name);
        }
    }

    if merged_branches.is_empty() {
        println!("{}", Theme::success("No merged branches found. Everything is clean!"));
        return Ok(());
    }

    println!(
        "{} Found {} merged branches:",
        Theme::warning("Found:"),
        merged_branches.len()
    );

    for name in &merged_branches {
        println!("  • {}", name);
    }

    if force {
        println!();
        println!("{}", Theme::info("Deleting branches..."));
        for name in merged_branches {
            let mut branch = repo.find_branch(&name, BranchType::Local)?;
            branch.delete()?;
            println!("  {} {}", Theme::success("Deleted:"), name);
        }
        println!();
        println!("{}", Theme::success("Cleanup completed successfully"));
    } else {
        println!();
        println!(
            "{}",
            Theme::dim("Tip: Run with --force to delete these branches.")
        );
    }

    Ok(())
}
