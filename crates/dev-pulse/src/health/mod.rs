use anyhow::Result;
use cli_core::ui::Theme;
use std::path::Path;

struct Check {
    name: String,
    passed: bool,
    description: String,
}

pub fn check(path: &Path, verbose: bool) -> Result<()> {
    let mut checks = Vec::new();

    // 1. README check
    let readme_exists = path.join("README.md").exists() || path.join("README").exists();
    checks.push(Check {
        name: "README".to_string(),
        passed: readme_exists,
        description: "Project documentation".to_string(),
    });

    // 2. LICENSE check
    let license_exists = path.join("LICENSE").exists() || path.join("LICENSE.md").exists() || path.join("LICENSE.txt").exists();
    checks.push(Check {
        name: "LICENSE".to_string(),
        passed: license_exists,
        description: "Legal license file".to_string(),
    });

    // 3. Git ignore check
    let gitignore_exists = path.join(".gitignore").exists();
    checks.push(Check {
        name: ".gitignore".to_string(),
        passed: gitignore_exists,
        description: "Git exclusion rules".to_string(),
    });

    // 4. Git repository check
    let git_repo_exists = path.join(".git").exists();
    checks.push(Check {
        name: "Git Repository".to_string(),
        passed: git_repo_exists,
        description: "Initialized git repository".to_string(),
    });

    // 5. Test directory check
    let tests_exists = path.join("tests").exists() || path.join("test").exists() || path.join("src").join("tests").exists();
    checks.push(Check {
        name: "Tests".to_string(),
        passed: tests_exists,
        description: "Test suite directory".to_string(),
    });

    // 6. CI config check
    let ci_exists = path.join(".github").exists() || path.join(".gitlab-ci.yml").exists() || path.join("circleci").exists();
    checks.push(Check {
        name: "CI Configuration".to_string(),
        passed: ci_exists,
        description: "Continuous Integration setup".to_string(),
    });

    let passed_count = checks.iter().filter(|c| c.passed).count();
    let total_count = checks.len();

    println!(
        "{} Score: {}/{}",
        Theme::info("Overall:"),
        Theme::highlight(&passed_count.to_string()),
        total_count
    );
    println!();

    for check in &checks {
        let status = if check.passed {
            Theme::success("PASS")
        } else {
            Theme::error("FAIL")
        };

        if verbose || !check.passed {
            println!(
                "  [{}] {:<18} {}",
                status,
                Theme::value(&check.name),
                Theme::dim(&check.description)
            );
        }
    }

    if passed_count == total_count {
        println!();
        println!("{}", Theme::success("Excellent! Your project follows all best practices."));
    } else if passed_count > total_count / 2 {
        println!();
        println!("{}", Theme::warning("Good, but there's room for improvement."));
    } else {
        println!();
        println!("{}", Theme::error("Your project is missing several essential files."));
    }

    Ok(())
}
