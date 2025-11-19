use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;

use crate::analyzer::LanguageStat;

#[derive(Debug, Clone)]
pub struct Metrics {
    pub total_lines: usize,
    pub code_lines: usize,
    pub comment_lines: usize,
    pub blank_lines: usize,
    pub total_files: usize,
    pub test_file_count: usize,
    pub has_readme: bool,
    pub language_stats: Vec<LanguageStat>,
}

pub struct MetricsCollector;

impl MetricsCollector {
    pub fn new() -> Self {
        Self
    }

    pub fn collect(&self, path: &Path) -> Result<Metrics> {
        let mut total_lines = 0;
        let mut code_lines = 0;
        let mut comment_lines = 0;
        let mut blank_lines = 0;
        let mut total_files = 0;
        let mut test_file_count = 0;
        let mut has_readme = false;
        let mut language_map: HashMap<String, (usize, usize)> = HashMap::new();

        for entry in WalkDir::new(path)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| !is_ignored(e.path()))
        {
            let entry = entry?;
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            // Check for README
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                if file_name.to_lowercase().starts_with("readme") {
                    has_readme = true;
                }
            }

            // Detect language
            let lang = detect_language(path);
            if lang.is_none() {
                continue;
            }

            let lang_name = lang.unwrap();
            total_files += 1;

            // Check if test file
            if is_test_file(path) {
                test_file_count += 1;
            }

            // Count lines
            if let Ok(content) = std::fs::read_to_string(path) {
                let lines: Vec<&str> = content.lines().collect();
                let line_count = lines.len();

                total_lines += line_count;

                let (code, comments, blanks) = count_line_types(&lines, &lang_name);
                code_lines += code;
                comment_lines += comments;
                blank_lines += blanks;

                let entry = language_map.entry(lang_name.clone()).or_insert((0, 0));
                entry.0 += line_count;
                entry.1 += 1;
            }
        }

        let mut language_stats: Vec<LanguageStat> = language_map
            .into_iter()
            .map(|(name, (lines, files))| LanguageStat {
                weight: get_language_weight(&name),
                name,
                lines,
                files,
            })
            .collect();

        language_stats.sort_by(|a, b| b.lines.cmp(&a.lines));

        Ok(Metrics {
            total_lines,
            code_lines,
            comment_lines,
            blank_lines,
            total_files,
            test_file_count,
            has_readme,
            language_stats,
        })
    }
}

fn is_ignored(path: &Path) -> bool {
    let ignored_dirs = [
        // Build outputs
        "target",
        "dist",
        "build",
        "out",
        // Dependencies
        "node_modules",
        "vendor",
        // Python virtual environments
        ".venv",
        "venv",
        "env",
        "virtualenv",
        "__pycache__",
        ".pytest_cache",
        ".mypy_cache",
        ".tox",
        // Version control
        ".git",
        ".svn",
        ".hg",
        // Framework-specific
        ".next",
        ".nuxt",
        ".cache",
        // IDE/Editor
        ".idea",
        ".vscode",
        ".vs",
    ];

    path.components().any(|c| {
        if let Some(s) = c.as_os_str().to_str() {
            ignored_dirs.contains(&s)
        } else {
            false
        }
    })
}

fn detect_language(path: &Path) -> Option<String> {
    path.extension()?.to_str().and_then(|ext| {
        match ext {
            "rs" => Some("Rust"),
            "py" => Some("Python"),
            "js" | "jsx" => Some("JavaScript"),
            "ts" | "tsx" => Some("TypeScript"),
            "go" => Some("Go"),
            "java" => Some("Java"),
            "c" => Some("C"),
            "cpp" | "cc" | "cxx" => Some("C++"),
            "h" | "hpp" => Some("C/C++ Header"),
            "cs" => Some("C#"),
            "rb" => Some("Ruby"),
            "php" => Some("PHP"),
            "swift" => Some("Swift"),
            "kt" | "kts" => Some("Kotlin"),
            "scala" => Some("Scala"),
            "sh" | "bash" => Some("Shell"),
            "sql" => Some("SQL"),
            "html" => Some("HTML"),
            "css" => Some("CSS"),
            "scss" | "sass" => Some("SCSS"),
            "md" | "markdown" => Some("Markdown"),
            "yaml" | "yml" => Some("YAML"),
            "json" => Some("JSON"),
            "toml" => Some("TOML"),
            "xml" => Some("XML"),
            _ => None,
        }
        .map(String::from)
    })
}

fn get_language_weight(lang: &str) -> f64 {
    match lang {
        "Rust" => 1.5,
        "C++" | "C" => 1.4,
        "Go" => 1.3,
        "Java" | "C#" | "Scala" => 1.2,
        "TypeScript" | "Swift" | "Kotlin" => 1.2,
        "Python" | "Ruby" | "PHP" => 1.1,
        "JavaScript" => 1.0,
        "Shell" | "SQL" => 0.9,
        "HTML" | "CSS" | "SCSS" => 0.7,
        "Markdown" | "YAML" | "JSON" | "TOML" | "XML" => 0.5,
        _ => 1.0,
    }
}

fn is_test_file(path: &Path) -> bool {
    let path_str = path.to_string_lossy();
    let lower = path_str.to_lowercase();

    lower.contains("test") || lower.contains("spec") || lower.contains("__tests__")
}

fn count_line_types(lines: &[&str], lang: &str) -> (usize, usize, usize) {
    let mut code = 0;
    let mut comments = 0;
    let mut blanks = 0;

    let (single_comment, multi_start, multi_end) = match lang {
        "Rust" | "C++" | "C" | "Go" | "Java" | "C#" | "JavaScript" | "TypeScript"
        | "Swift" | "Kotlin" | "Scala" | "PHP" => ("//", "/*", "*/"),
        "Python" | "Ruby" | "Shell" => ("#", "", ""),
        "HTML" | "XML" => ("", "<!--", "-->"),
        "CSS" | "SCSS" => ("", "/*", "*/"),
        _ => ("", "", ""),
    };

    let mut in_multiline_comment = false;

    for line in lines {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            blanks += 1;
            continue;
        }

        if !multi_start.is_empty() && trimmed.contains(multi_start) {
            in_multiline_comment = true;
            comments += 1;
            if trimmed.contains(multi_end) {
                in_multiline_comment = false;
            }
            continue;
        }

        if in_multiline_comment {
            comments += 1;
            if !multi_end.is_empty() && trimmed.contains(multi_end) {
                in_multiline_comment = false;
            }
            continue;
        }

        if !single_comment.is_empty() && trimmed.starts_with(single_comment) {
            comments += 1;
            continue;
        }

        code += 1;
    }

    (code, comments, blanks)
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}
