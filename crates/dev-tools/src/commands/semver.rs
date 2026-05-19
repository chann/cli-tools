use anyhow::{Result, anyhow};
use semver::Version;

pub fn parse(text: &str) -> Result<()> {
    let version = Version::parse(text)
        .map_err(|e| anyhow!("Failed to parse semver '{}': {}", text, e))?;
    
    println!("Major: {}", version.major);
    println!("Minor: {}", version.minor);
    println!("Patch: {}", version.patch);
    if !version.pre.is_empty() {
        println!("Pre-release: {}", version.pre);
    }
    if !version.build.is_empty() {
        println!("Build: {}", version.build);
    }
    Ok(())
}

pub fn increment(text: &str, part: &str) -> Result<()> {
    let mut version = Version::parse(text)
        .map_err(|e| anyhow!("Failed to parse semver '{}': {}", text, e))?;
    
    match part.to_lowercase().as_str() {
        "major" => {
            version.major += 1;
            version.minor = 0;
            version.patch = 0;
            version.pre = semver::Prerelease::EMPTY;
            version.build = semver::BuildMetadata::EMPTY;
        }
        "minor" => {
            version.minor += 1;
            version.patch = 0;
            version.pre = semver::Prerelease::EMPTY;
            version.build = semver::BuildMetadata::EMPTY;
        }
        "patch" => {
            version.patch += 1;
            version.pre = semver::Prerelease::EMPTY;
            version.build = semver::BuildMetadata::EMPTY;
        }
        _ => return Err(anyhow!("Invalid part to increment: {}. Use major, minor, or patch.", part)),
    }
    
    println!("{}", version);
    Ok(())
}

pub fn compare(v1: &str, v2: &str) -> Result<()> {
    let ver1 = Version::parse(v1)
        .map_err(|e| anyhow!("Failed to parse semver '{}': {}", v1, e))?;
    let ver2 = Version::parse(v2)
        .map_err(|e| anyhow!("Failed to parse semver '{}': {}", v2, e))?;
    
    if ver1 > ver2 {
        println!("{} > {}", v1, v2);
    } else if ver1 < ver2 {
        println!("{} < {}", v1, v2);
    } else {
        println!("{} == {}", v1, v2);
    }
    Ok(())
}
