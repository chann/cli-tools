use anyhow::{Context, Result};
use cli_core::ui::Theme;
use git2::{BranchType, Repository, StatusOptions};
use std::path::Path;
use std::collections::HashMap;

pub fn cleanup(path: &Path, force: bool, target: Option<&str>) -> Result<()> {
    // ... (existing cleanup implementation)
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

pub struct GitStatusSummary {
    pub modified: usize,
    pub added: usize,
    pub deleted: usize,
    pub untracked: usize,
    pub branch_name: String,
}

pub fn get_status_summary(path: &Path) -> Result<GitStatusSummary> {
    let repo = Repository::open(path).context("Failed to open repository")?;
    
    let mut status_opts = StatusOptions::new();
    status_opts.include_untracked(true);
    let statuses = repo.statuses(Some(&mut status_opts))?;

    let mut modified = 0;
    let mut added = 0;
    let mut deleted = 0;
    let mut untracked = 0;

    for entry in statuses.iter() {
        let status = entry.status();
        if status.is_wt_new() || status.is_index_new() {
            untracked += 1;
        } else if status.is_wt_modified() || status.is_index_modified() {
            modified += 1;
        } else if status.is_wt_deleted() || status.is_index_deleted() {
            deleted += 1;
        } else if status.is_index_renamed() || status.is_wt_renamed() {
            modified += 1;
        }
        
        if status.is_index_new() {
            added += 1;
            untracked -= 1; // Correct untracked if it's already staged
        }
    }

    let head = repo.head()?;
    let branch_name = head.shorthand().unwrap_or("HEAD").to_string();

    Ok(GitStatusSummary {
        modified,
        added,
        deleted,
        untracked,
        branch_name,
    })
}

pub struct RecentActivity {
    pub top_contributors: Vec<(String, usize)>,
    pub total_commits_last_7_days: usize,
}

pub fn get_recent_activity(path: &Path) -> Result<RecentActivity> {
    let repo = Repository::open(path).context("Failed to open repository")?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let mut contributors = HashMap::new();
    let mut recent_commits = 0;
    
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    let seven_days_ago = now - (7 * 24 * 60 * 60);

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let author = commit.author();
        let name = author.name().unwrap_or("Unknown").to_string();
        
        *contributors.entry(name).or_insert(0) += 1;
        
        if commit.time().seconds() as u64 > seven_days_ago {
            recent_commits += 1;
        }
        
        // Only look at the last 100 commits for top contributors performance
        if contributors.values().sum::<usize>() >= 100 {
            // But we still want to count recent commits if we haven't reached 7 days?
            // Actually, revwalk is usually ordered by time, so we can stop if we're past 7 days.
            if (commit.time().seconds() as u64) < seven_days_ago {
                break;
            }

        }
    }

    let mut top_contributors: Vec<_> = contributors.into_iter().collect();
    top_contributors.sort_by(|a, b| b.1.cmp(&a.1));
    top_contributors.truncate(3);

    Ok(RecentActivity {
        top_contributors,
        total_commits_last_7_days: recent_commits,
    })
}
