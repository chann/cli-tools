use anyhow::Result;
use cli_core::ui::Theme;
use std::path::Path;
use std::fs;
use git2::{Repository, StatusOptions};
use walkdir::WalkDir;

#[derive(Clone)]
struct HealthCheck {
    name: String,
    description: String,
    check_fn: fn(&Path) -> bool,
    fix_advice: String,
}

fn get_all_checks() -> Vec<HealthCheck> {
    vec![
        // 1. Documentation
        HealthCheck {
            name: "README".to_string(),
            description: "Project documentation".to_string(),
            check_fn: |p| p.join("README.md").exists() || p.join("README").exists(),
            fix_advice: "Create a README.md file to document your project.".to_string(),
        },
        HealthCheck {
            name: "CONTRIBUTING".to_string(),
            description: "Contribution guidelines".to_string(),
            check_fn: |p| p.join("CONTRIBUTING.md").exists() || p.join("CONTRIBUTING").exists(),
            fix_advice: "Add CONTRIBUTING.md to help others contribute to your project.".to_string(),
        },
        HealthCheck {
            name: "CHANGELOG".to_string(),
            description: "Project history and changes".to_string(),
            check_fn: |p| p.join("CHANGELOG.md").exists() || p.join("CHANGELOG").exists() || p.join("HISTORY.md").exists(),
            fix_advice: "Keep a CHANGELOG.md to track changes between versions.".to_string(),
        },
        HealthCheck {
            name: "GEMINI.md".to_string(),
            description: "Gemini CLI instructions".to_string(),
            check_fn: |p| p.join("GEMINI.md").exists(),
            fix_advice: "Add GEMINI.md with project-specific instructions for the agent.".to_string(),
        },
        // 2. Legal & Security
        HealthCheck {
            name: "LICENSE".to_string(),
            description: "Legal license file".to_string(),
            check_fn: |p| p.join("LICENSE").exists() || p.join("LICENSE.md").exists() || p.join("LICENSE.txt").exists(),
            fix_advice: "Add a LICENSE file to define how others can use your code.".to_string(),
        },
        HealthCheck {
            name: "SECURITY".to_string(),
            description: "Security policy".to_string(),
            check_fn: |p| p.join("SECURITY.md").exists(),
            fix_advice: "Add SECURITY.md to explain how to report vulnerabilities.".to_string(),
        },
        HealthCheck {
            name: "CODEOWNERS".to_string(),
            description: "Code ownership definitions".to_string(),
            check_fn: |p| p.join("CODEOWNERS").exists() || p.join(".github").join("CODEOWNERS").exists(),
            fix_advice: "Define CODEOWNERS to manage pull request reviews effectively.".to_string(),
        },
        // 3. Git Hygiene
        HealthCheck {
            name: ".gitignore".to_string(),
            description: "Git exclusion rules".to_string(),
            check_fn: |p| p.join(".gitignore").exists(),
            fix_advice: "Create a .gitignore file to avoid committing unnecessary files.".to_string(),
        },
        HealthCheck {
            name: "Clean Repo".to_string(),
            description: "No uncommitted changes".to_string(),
            check_fn: |p| {
                if let Ok(repo) = Repository::discover(p) {
                    if let Ok(statuses) = repo.statuses(None) {
                        return statuses.is_empty();
                    }
                }
                true
            },
            fix_advice: "Commit or stash your current changes for a clean state.".to_string(),
        },
        // 4. Standards
        HealthCheck {
            name: ".editorconfig".to_string(),
            description: "Consistent coding styles".to_string(),
            check_fn: |p| p.join(".editorconfig").exists(),
            fix_advice: "Add .editorconfig to maintain consistent styles across different editors.".to_string(),
        },
        HealthCheck {
            name: "Rust Toolchain".to_string(),
            description: "Rust toolchain configuration".to_string(),
            check_fn: |p| {
                if p.join("Cargo.toml").exists() {
                    p.join("rust-toolchain").exists() || p.join("rust-toolchain.toml").exists()
                } else {
                    true
                }
            },
            fix_advice: "Add rust-toolchain.toml to pin the Rust version for all contributors.".to_string(),
        },
        // 5. Structure & Manifests
        HealthCheck {
            name: "Manifest File".to_string(),
            description: "Dependency management file".to_string(),
            check_fn: |p| {
                let manifests = ["Cargo.toml", "package.json", "go.mod", "requirements.txt", "pyproject.toml"];
                manifests.iter().any(|f| p.join(f).exists())
            },
            fix_advice: "Ensure your project has a standard manifest file (e.g., Cargo.toml).".to_string(),
        },
        HealthCheck {
            name: "Lock File".to_string(),
            description: "Dependency lock file for deterministic builds".to_string(),
            check_fn: |p| {
                let locks = ["Cargo.lock", "package-lock.json", "yarn.lock", "pnpm-lock.yaml", "go.sum"];
                locks.iter().any(|f| p.join(f).exists())
            },
            fix_advice: "Commit your lock file (e.g., Cargo.lock) to ensure deterministic builds.".to_string(),
        },
        // 6. CI/CD
        HealthCheck {
            name: "CI Configuration".to_string(),
            description: "Continuous Integration setup".to_string(),
            check_fn: |p| {
                p.join(".github").exists() 
                    || p.join(".gitlab-ci.yml").exists() 
                    || p.join("circleci").exists()
                    || p.join(".travis.yml").exists()
            },
            fix_advice: "Setup CI (e.g., GitHub Actions) to automate testing and deployment.".to_string(),
        },
        // 7. Code Quality
        HealthCheck {
            name: "Rustfmt".to_string(),
            description: "Rust formatting configuration".to_string(),
            check_fn: |p| p.join(".rustfmt.toml").exists() || p.join("rustfmt.toml").exists(),
            fix_advice: "Add rustfmt.toml to enforce consistent code style.".to_string(),
        },
        HealthCheck {
            name: "Clippy".to_string(),
            description: "Rust linting configuration".to_string(),
            check_fn: |p| p.join(".clippy.toml").exists() || p.join("clippy.toml").exists(),
            fix_advice: "Add clippy.toml to configure custom lints for your project.".to_string(),
        },
        // 8. Security
        HealthCheck {
            name: "Large Files".to_string(),
            description: "No files larger than 50MB".to_string(),
            check_fn: |p| {
                for entry in WalkDir::new(p)
                    .into_iter()
                    .filter_entry(|e| !e.file_name().to_str().map(|s| s == ".git" || s == "target").unwrap_or(false))
                    .filter_map(|e| e.ok()) {
                    if entry.file_type().is_file() {
                        if let Ok(metadata) = entry.metadata() {
                            if metadata.len() > 50 * 1024 * 1024 {
                                return false;
                            }
                        }
                    }
                }
                true
            },
            fix_advice: "Remove large files from Git or use Git LFS.".to_string(),
        },
        HealthCheck {
            name: "Tracked Secrets".to_string(),
            description: "No sensitive files tracked in Git".to_string(),
            check_fn: |p| {
                if let Ok(repo) = Repository::discover(p) {
                    let mut opts = StatusOptions::new();
                    opts.include_untracked(false);
                    if let Ok(statuses) = repo.statuses(Some(&mut opts)) {
                        for entry in statuses.iter() {
                            if let Some(path) = entry.path() {
                                if path.contains(".env") || path.contains("id_rsa") || path.contains(".pem") {
                                    return false;
                                }
                            }
                        }
                    }
                }
                true
            },
            fix_advice: "Stop tracking sensitive files (git rm --cached) and add them to .gitignore.".to_string(),
        },
        // 9. Automation
        HealthCheck {
            name: "Automation".to_string(),
            description: "Task runner (Makefile, Justfile)".to_string(),
            check_fn: |p| p.join("Makefile").exists() || p.join("Justfile").exists() || p.join("justfile").exists(),
            fix_advice: "Add a Makefile or Justfile to automate common tasks.".to_string(),
        },
        // 10. Containerization
        HealthCheck {
            name: "Docker".to_string(),
            description: "Containerization configuration".to_string(),
            check_fn: |p| p.join("Dockerfile").exists() || p.join("docker-compose.yml").exists(),
            fix_advice: "Consider adding a Dockerfile for consistent development and deployment environments.".to_string(),
        },
        // 11. Configuration
        HealthCheck {
            name: "Registry Config".to_string(),
            description: "Registry or dependency configuration".to_string(),
            check_fn: |p| p.join(".npmrc").exists() || p.join(".cargo").join("config.toml").exists() || p.join(".yarnrc.yml").exists(),
            fix_advice: "Consider adding registry configuration (e.g., .npmrc) for shared dependency settings.".to_string(),
        },
        // 12. Testing
        HealthCheck {
            name: "Tests".to_string(),
            description: "Test directory presence".to_string(),
            check_fn: |p| p.join("tests").exists() || p.join("test").exists() || p.join("spec").exists(),
            fix_advice: "Create a tests/ directory to organize your integration tests.".to_string(),
        },
        // 13. Advanced Documentation
        HealthCheck {
            name: "Examples".to_string(),
            description: "Usage examples".to_string(),
            check_fn: |p| p.join("examples").exists() || p.join("samples").exists(),
            fix_advice: "Add an examples/ directory to demonstrate how to use your code.".to_string(),
        },
        HealthCheck {
            name: "Extended Docs".to_string(),
            description: "Dedicated documentation directory".to_string(),
            check_fn: |p| p.join("docs").exists() || p.join("documentation").exists(),
            fix_advice: "Consider adding a docs/ directory for in-depth documentation.".to_string(),
        },
        // 14. Rust Specific
        HealthCheck {
            name: "Crate Metadata".to_string(),
            description: "Rich Cargo.toml metadata".to_string(),
            check_fn: |p| {
                if let Ok(content) = fs::read_to_string(p.join("Cargo.toml")) {
                    content.contains("keywords =") && content.contains("categories =") && content.contains("repository =")
                } else {
                    true
                }
            },
            fix_advice: "Add keywords, categories, and repository fields to your Cargo.toml.".to_string(),
        },
        // 15. Modern Workflow
        HealthCheck {
            name: "Pre-commit Hooks".to_string(),
            description: "Pre-commit hook configuration".to_string(),
            check_fn: |p| p.join(".pre-commit-config.yaml").exists() || p.join(".husky").exists(),
            fix_advice: "Use pre-commit or husky to run checks before every commit.".to_string(),
        },
        HealthCheck {
            name: "Vuln Scanning".to_string(),
            description: "Security vulnerability scanning in CI".to_string(),
            check_fn: |p| {
                let github_workflows = p.join(".github").join("workflows");
                if let Ok(entries) = fs::read_dir(github_workflows) {
                    for entry in entries.flatten() {
                        if let Ok(content) = fs::read_to_string(entry.path()) {
                            if content.contains("audit") || content.contains("security-events") || content.contains("snyk") {
                                return true;
                            }
                        }
                    }
                }
                false
            },
            fix_advice: "Add vulnerability scanning (e.g., cargo-audit) to your CI workflow.".to_string(),
        },
        HealthCheck {
            name: "Pending Tasks".to_string(),
            description: "Check for TODO or FIXME comments".to_string(),
            check_fn: |p| {
                let mut found = false;
                for entry in WalkDir::new(p)
                    .into_iter()
                    .filter_entry(|e| !e.file_name().to_str().map(|s| s == ".git" || s == "target" || s == ".gemini").unwrap_or(false))
                    .filter_map(|e| e.ok()) {
                    if entry.file_type().is_file() {
                        if let Some(ext) = entry.path().extension() {
                            if ext == "rs" || ext == "md" || ext == "toml" {
                                if let Ok(content) = fs::read_to_string(entry.path()) {
                                    if content.contains("TODO") || content.contains("FIXME") {
                                        found = true;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                !found
            },
            fix_advice: "Address outstanding TODO or FIXME comments in your code.".to_string(),
        },
    ]
}

pub fn check(path: &Path, verbose: bool) -> Result<()> {
    let checks = get_all_checks();
    let mut results = Vec::new();
    
    for check in checks {
        let passed = (check.check_fn)(path);
        results.push((check, passed));
    }

    let passed_count = results.iter().filter(|(_, p)| *p).count();
    let total_count = results.len();

    println!(
        "{} Score: {}/{}",
        Theme::info("Overall Health:"),
        Theme::highlight(&passed_count.to_string()),
        total_count
    );
    println!();

    for (check, passed) in &results {
        let status = if *passed {
            Theme::success("PASS")
        } else {
            Theme::error("FAIL")
        };

        if verbose || !*passed {
            println!(
                "  [{}] {:<18} {}",
                status,
                Theme::value(&check.name),
                Theme::dim(&check.description)
            );
            
            if !*passed {
                println!("       {}", Theme::warning(format!("Advice: {}", check.fix_advice)));
            }
        }
    }

    if passed_count == total_count {
        println!();
        println!("{}", Theme::success("Perfect! Your project follows all industry standards and best practices."));
    } else if passed_count > total_count * 3 / 4 {
        println!();
        println!("{}", Theme::success("Great! Most essential files are present."));
    } else if passed_count > total_count / 2 {
        println!();
        println!("{}", Theme::warning("Good, but there's room for significant improvement."));
    } else {
        println!();
        println!("{}", Theme::error("Your project is missing many essential files. Consider adding them."));
    }

    Ok(())
}

pub fn get_score(path: &Path) -> (usize, usize) {
    let checks = get_all_checks();
    let total = checks.len();
    let passed = checks.iter().filter(|c| (c.check_fn)(path)).count();
    (passed, total)
}
