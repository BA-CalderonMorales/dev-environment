use std::fs;
use std::path::Path;
use regex::Regex;
use anyhow::{Context, Result};

fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    log::info!("📚 Updating documentation...");

    // Read version from VERSION file
    let version = fs::read_to_string("../../VERSION")
        .context("Failed to read VERSION file")?
        .trim()
        .to_string();

    // Update README.md
    update_doc_version("../../docs/README.md", &version)?;

    Ok(())
}

fn update_doc_version(path: &str, version: &str) -> Result<()> {
    let path = Path::new(path);
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    // Replace version between VERSION tags
    let re = Regex::new(r"(?s)<!-- VERSION -->.*?<!-- /VERSION -->")
        .context("Failed to create regex")?;
    
    let new_content = re.replace_all(&content, format!("<!-- VERSION -->{}<!-- /VERSION -->", version));

    fs::write(path, new_content.as_bytes())
        .with_context(|| format!("Failed to write file: {}", path.display()))?;

    log::info!("✅ Updated version to {} in {}", version, path.display());
    Ok(())
}
