use ignore::WalkBuilder;
use anyhow::Result;
use std::path::Path;

pub fn print_tree(path: &Path, depth: Option<usize>) -> Result<()> {
    let mut walker = WalkBuilder::new(path);
    walker.standard_filters(true);
    if let Some(d) = depth {
        walker.max_depth(Some(d));
    }

    let mut entries: Vec<_> = walker.build()
        .filter_map(|e| e.ok())
        .collect();
    
    // Sort entries to ensure deterministic output
    entries.sort_by(|a, b| a.path().cmp(b.path()));

    for entry in entries {
        let depth = entry.depth();
        if depth == 0 {
            println!("{}", entry.path().display());
            continue;
        }

        let indent = "│   ".repeat(depth - 1);
        let name = entry.file_name().to_string_lossy();
        let prefix = if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            "📁 "
        } else {
            "📄 "
        };
        
        println!("{}└── {}{}", indent, prefix, name);
    }
    Ok(())
}
