use anyhow::{bail, Context, Result};
use cli_core::ui::Theme;
use std::collections::HashMap;
use std::path::Path;

pub fn run(file: &Path, min_depth: usize, max_depth: usize) -> Result<()> {
    if min_depth == 0 || max_depth > 6 || min_depth > max_depth {
        bail!("Depth range must satisfy 1 <= min <= max <= 6");
    }
    let content = std::fs::read_to_string(file)
        .with_context(|| format!("Failed to read {}", file.display()))?;
    let toc = generate(&content, min_depth, max_depth);
    if toc.is_empty() {
        println!("{}", Theme::info("No headings found in the given depth range"));
        return Ok(());
    }
    println!("{}", toc);
    Ok(())
}

/// Build a GitHub-style Markdown TOC from ATX headings, skipping fenced code blocks.
fn generate(content: &str, min_depth: usize, max_depth: usize) -> String {
    let headings = collect_headings(content, min_depth, max_depth);
    let base = headings.iter().map(|(level, _)| *level).min().unwrap_or(1);

    let mut seen: HashMap<String, usize> = HashMap::new();
    headings
        .iter()
        .map(|(level, text)| {
            let anchor = dedupe(slug(text), &mut seen);
            format!("{}- [{}](#{})", "  ".repeat(level - base), text, anchor)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn collect_headings(content: &str, min_depth: usize, max_depth: usize) -> Vec<(usize, String)> {
    let mut in_fence = false;
    let mut headings = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
            continue;
        }
        // ATX headings allow at most 3 leading spaces
        if in_fence || line.len() - trimmed.len() > 3 {
            continue;
        }
        let level = trimmed.chars().take_while(|c| *c == '#').count();
        if level == 0 || level > 6 || !matches!(trimmed.as_bytes().get(level), Some(b' ') | None) {
            continue;
        }
        if level < min_depth || level > max_depth {
            continue;
        }
        let text = strip_inline_markup(trimmed[level..].trim().trim_end_matches('#').trim_end());
        if !text.is_empty() {
            headings.push((level, text));
        }
    }
    headings
}

/// Remove backticks, emphasis markers, and link syntax ("[label](url)" -> "label").
fn strip_inline_markup(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '`' | '*' => {}
            '[' => {}
            ']' => {
                // Drop a "(url)" that immediately follows a link label
                if chars.peek() == Some(&'(') {
                    for c in chars.by_ref() {
                        if c == ')' {
                            break;
                        }
                    }
                }
            }
            _ => out.push(c),
        }
    }
    out
}

/// GitHub-style anchor: lowercase, keep alphanumerics/underscore/hyphen, spaces to hyphens.
fn slug(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter_map(|c| match c {
            ' ' => Some('-'),
            '-' | '_' => Some(c),
            c if c.is_alphanumeric() => Some(c),
            _ => None,
        })
        .collect()
}

fn dedupe(anchor: String, seen: &mut HashMap<String, usize>) -> String {
    let count = seen.entry(anchor.clone()).or_insert(0);
    let result = if *count == 0 {
        anchor.clone()
    } else {
        format!("{}-{}", anchor, count)
    };
    *seen.get_mut(&anchor).expect("just inserted") += 1;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nested_toc_with_anchors() {
        let md = "# Title\n\n## Getting Started\n\nText.\n\n### Install & Run\n";
        assert_eq!(
            generate(md, 1, 6),
            "- [Title](#title)\n  - [Getting Started](#getting-started)\n    - [Install & Run](#install--run)"
        );
    }

    #[test]
    fn skips_headings_inside_code_fences() {
        let md = "# Real\n```bash\n# comment, not a heading\n```\n## Also Real\n";
        assert_eq!(generate(md, 1, 6), "- [Real](#real)\n  - [Also Real](#also-real)");
    }

    #[test]
    fn dedupes_repeated_headings_like_github() {
        let md = "## Setup\n## Setup\n## Setup\n";
        assert_eq!(
            generate(md, 1, 6),
            "- [Setup](#setup)\n- [Setup](#setup-1)\n- [Setup](#setup-2)"
        );
    }

    #[test]
    fn respects_depth_range_and_rebases_indent() {
        let md = "# H1\n## H2\n### H3\n#### H4\n";
        assert_eq!(generate(md, 2, 3), "- [H2](#h2)\n  - [H3](#h3)");
    }

    #[test]
    fn strips_links_and_code_from_heading_text() {
        let md = "## Using [clap](https://clap.rs) with `derive`\n";
        assert_eq!(
            generate(md, 1, 6),
            "- [Using clap with derive](#using-clap-with-derive)"
        );
    }

    #[test]
    fn keeps_unicode_headings() {
        let md = "## 시작하기\n";
        assert_eq!(generate(md, 1, 6), "- [시작하기](#시작하기)");
    }

    #[test]
    fn ignores_non_headings_and_hashbangs() {
        let md = "#no-space\n    # indented code\ntext # inline\n####### seven\n";
        assert_eq!(generate(md, 1, 6), "");
    }
}
