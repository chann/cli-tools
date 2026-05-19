use ignore::WalkBuilder;
use anyhow::Result;
use std::path::{Path, PathBuf};
use owo_colors::OwoColorize;
use std::collections::BTreeMap;
use git2::{Repository, StatusOptions};

struct Node {
    is_dir: bool,
    size: Option<u64>,
    git_status: Option<String>,
    children: BTreeMap<String, Node>,
}

pub fn print_tree(path: &Path, depth: Option<usize>, show_size: bool, show_git: bool) -> Result<()> {
    let mut walker = WalkBuilder::new(path);
    walker.standard_filters(true);
    if let Some(d) = depth {
        walker.max_depth(Some(d));
    }

    let mut git_statuses = BTreeMap::new();
    if show_git {
        if let Ok(repo) = Repository::discover(path) {
            let mut opts = StatusOptions::new();
            opts.include_untracked(true).recurse_untracked_dirs(true);
            if let Ok(statuses) = repo.statuses(Some(&mut opts)) {
                for entry in statuses.iter() {
                    if let Some(p) = entry.path() {
                        let status = entry.status();
                        let status_str = if status.is_index_new() || status.is_wt_new() {
                            "A".green().to_string()
                        } else if status.is_index_modified() || status.is_wt_modified() {
                            "M".yellow().to_string()
                        } else if status.is_index_deleted() || status.is_wt_deleted() {
                            "D".red().to_string()
                        } else if status.is_ignored() {
                            "I".dimmed().to_string()
                        } else if status.is_wt_renamed() || status.is_index_renamed() {
                            "R".blue().to_string()
                        } else {
                            " ".to_string()
                        };
                        git_statuses.insert(PathBuf::from(p), status_str);
                    }
                }
            }
        }
    }

    let mut root_node = Node {
        is_dir: true,
        size: None,
        git_status: None,
        children: BTreeMap::new(),
    };

    for result in walker.build() {
        let entry = result?;
        let rel_path = match entry.path().strip_prefix(path) {
            Ok(p) => p,
            Err(_) => continue,
        };
        
        if rel_path.as_os_str().is_empty() {
            continue;
        }

        let mut current = &mut root_node;
        let components: Vec<_> = rel_path.components().collect();
        let total_components = components.len();

        for (i, comp) in components.iter().enumerate() {
            let name = comp.as_os_str().to_string_lossy().to_string();
            let is_last = i == total_components - 1;
            
            let is_dir = if is_last {
                entry.file_type().map(|t| t.is_dir()).unwrap_or(false)
            } else {
                true
            };

            let size = if is_last && !is_dir {
                entry.metadata().ok().map(|m| m.len())
            } else {
                None
            };

            let git_status = if is_last {
                git_statuses.get(rel_path).cloned()
            } else {
                None
            };

            current = current.children.entry(name).or_insert(Node {
                is_dir,
                size,
                git_status,
                children: BTreeMap::new(),
            });
        }
    }

    println!("{}", path.display().bold().blue());
    render_node(&root_node, "", show_size, show_git);
    
    Ok(())
}

fn render_node(node: &Node, prefix: &str, show_size: bool, show_git: bool) {
    let entries: Vec<_> = node.children.iter().collect();
    let count = entries.len();
    
    for (i, (name, child)) in entries.into_iter().enumerate() {
        let is_last = i == count - 1;
        let connector = if is_last { "└── " } else { "├── " };
        let icon = if child.is_dir { "📁 " } else { "📄 " };
        
        let display_name = if child.is_dir {
            name.bold().blue().to_string()
        } else {
            name.to_string()
        };

        let mut meta = Vec::new();

        if show_git {
            if let Some(status) = &child.git_status {
                meta.push(format!("[{}]", status));
            } else {
                meta.push("   ".to_string());
            }
        }

        if show_size {
            if let Some(s) = child.size {
                meta.push(format!("({:>9})", format_size(s)));
            } else {
                meta.push(" ".repeat(11));
            }
        }

        let meta_str = if meta.is_empty() {
            "".to_string()
        } else {
            format!("{} ", meta.join(" ").dimmed())
        };

        println!("{}{}{}{}{}", prefix, connector, icon, meta_str, display_name);
        
        if !child.children.is_empty() {
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            render_node(child, &new_prefix, show_size, show_git);
        }
    }
}

fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.1} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.1} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.1} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}
