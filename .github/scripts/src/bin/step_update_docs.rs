//! Documentation update script
//! 
//! Updates version references in documentation files

use anyhow::{Context, Result};
use std::path::Path; // Removed unused PathBuf import
use std::fs;
use github_workflow_scripts::{init, get_logger};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize our custom logger
    init();
    let logger = get_logger(false);
    
    logger.info("📚 Updating documentation...");
    
    // Get version from environment or read from VERSION file
    let version = std::env::var("INPUT_VERSION")
        .unwrap_or_else(|_| {
            fs::read_to_string("VERSION")
                .unwrap_or_else(|_| "v0.1.0".to_string())
                .trim()
                .to_string()
        });
    
    // Update documentation files
    update_file("README.md", &version)?;
    update_file("docs/installation.md", &version)?;
    update_file("docs/development.md", &version)?;
    
    logger.info("✅ Documentation updated successfully!");
    
    Ok(())
}

// Updates version references in a file
fn update_file(path: impl AsRef<Path>, version: &str) -> Result<()> {
    let path = path.as_ref();
    let logger = get_logger(false);
    
    // Skip if file doesn't exist
    if !path.exists() {
        return Ok(());
    }
    
    // Read file content
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    
    // Replace version references
    // This is a simplified example - actual regex patterns would be more complex
    let updated = content.replace("v0.1.0", version)
        .replace("VERSION=0.1.0", &format!("VERSION={}", version.trim_start_matches('v')));
    
    // Write updated content back
    fs::write(path, updated)
        .with_context(|| format!("Failed to write to {}", path.display()))?;
    
    logger.info(&format!("✅ Updated version to {} in {}", version, path.display()));
    
    Ok(())
}
