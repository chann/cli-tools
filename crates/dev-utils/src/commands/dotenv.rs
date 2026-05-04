use anyhow::Result;
use std::fs;
use std::path::Path;
use owo_colors::OwoColorize;

pub fn init(path: &str) -> Result<()> {
    let p = Path::new(path);
    if p.exists() {
        anyhow::bail!("File {} already exists", path);
    }

    let template = "# Environment Variables Template\n\
                    # APP_ENV=development\n\
                    # DATABASE_URL=postgres://user:pass@localhost:5432/db\n\
                    # API_KEY=your_api_key_here\n";

    fs::write(p, template)?;
    println!("Created {} with template.", path.green());
    Ok(())
}

pub fn example(path: &str) -> Result<()> {
    let p = Path::new(path);
    if !p.exists() {
        anyhow::bail!("File {} does not exist", path);
    }

    let content = fs::read_to_string(p)?;
    let mut example_content = String::new();

    for line in content.lines() {
        if line.trim().is_empty() || line.trim().starts_with('#') {
            example_content.push_str(line);
            example_content.push('\n');
            continue;
        }

        if let Some(pos) = line.find('=') {
            let key = &line[..pos];
            example_content.push_str(key);
            example_content.push_str("=\n");
        } else {
            example_content.push_str(line);
            example_content.push('\n');
        }
    }

    let example_path = format!("{}.example", path);
    fs::write(&example_path, example_content)?;
    println!("Generated {} from {}.", example_path.green(), path.cyan());
    Ok(())
}

pub fn load(path: &str) -> Result<()> {
    let p = Path::new(path);
    if !p.exists() {
        anyhow::bail!("File {} does not exist", path);
    }

    let content = fs::read_to_string(p)?;
    println!("Loading variables from {}:", path.cyan());
    
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        
        if let Some(pos) = line.find('=') {
            let key = &line[..pos].trim();
            let value = &line[pos+1..].trim();
            println!("  {} = {}", key.bold(), value.yellow());
        }
    }

    Ok(())
}

pub fn compare(path: &str, example_path: &str) -> Result<()> {
    let p = Path::new(path);
    let ep = Path::new(example_path);

    if !p.exists() { anyhow::bail!("File {} does not exist", path); }
    if !ep.exists() { anyhow::bail!("File {} does not exist", example_path); }

    let env_content = fs::read_to_string(p)?;
    let example_content = fs::read_to_string(ep)?;

    let get_keys = |content: String| {
        content.lines()
            .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
            .filter_map(|l| l.find('=').map(|pos| l[..pos].trim().to_string()))
            .collect::<std::collections::HashSet<String>>()
    };

    let env_keys = get_keys(env_content);
    let example_keys = get_keys(example_content);

    let mut missing = Vec::new();
    for key in &example_keys {
        if !env_keys.contains(key) {
            missing.push(key);
        }
    }

    if missing.is_empty() {
        println!("{}", "All keys from example file are present in the env file.".green());
    } else {
        println!("{}", format!("Missing {} keys in {}:", missing.len(), path).red());
        for key in missing {
            println!("  - {}", key.bold());
        }
    }

    Ok(())
}
