//! Packages release assets for create-release action
//! Used by: ./.github/actions/create-release/action.yml
//! Purpose: Creates distribution archive and checksums

use anyhow::{Context, Result};
use github_workflow_scripts::{get_logger, init};
use sha2::{Sha256, Digest};
use std::{fs, path::Path, process::Command};

#[tokio::main]
async fn main() -> Result<()> {
    init();
    let logger = get_logger(false);

    logger.info("📦 Starting asset packaging...");

    // Create staging area
    let staging_dir = Path::new("release_assets");
    fs::create_dir_all(staging_dir)?;

    // Create required directories if they don't exist
    for dir in &["distributions", "startup", "docs"] {
        fs::create_dir_all(dir)
            .context(format!("Failed to create {} directory", dir))?;
    }

    // Copy distribution files
    logger.info("Copying distribution files...");
    for dir in &["distributions", "startup", "docs"] {
        let target = staging_dir.join(dir);
        fs::create_dir_all(&target)?;
        
        // Only copy if directory exists and has contents
        if Path::new(dir).exists() && fs::read_dir(dir)?.next().is_some() {
            let status = Command::new("cp")
                .args(["-r", &format!("{}/*", dir), &target.to_string_lossy()])
                .status()
                .context(format!("Failed to copy {}", dir))?;

            if !status.success() {
                logger.warn(&format!("No contents to copy from {}", dir));
            }
        } else {
            logger.info(&format!("Directory {} is empty or doesn't exist, skipping", dir));
        }
    }

    // Copy Docker files
    logger.info("Copying Docker files...");
    for pattern in &["Dockerfile*", "docker-compose*.yml"] {
        match Command::new("cp")
            .args(["-v", pattern, &staging_dir.to_string_lossy()])
            .status()
        {
            Ok(_) => logger.info(&format!("Copied {}", pattern)),
            Err(_) => logger.info(&format!("No {} found", pattern)),
        }
    }

    // Create tarball even if some directories were empty
    logger.info("Creating tarball...");
    let output = Command::new("tar")
        .current_dir(staging_dir)
        .args(["-czf", "../dev-environment.tar.gz", "."])
        .output()?;

    if !output.status.success() {
        anyhow::bail!("Failed to create tarball");
    }

    // Generate checksum
    logger.info("Generating checksum...");
    let tar_contents = fs::read("dev-environment.tar.gz")?;
    let mut hasher = Sha256::new();
    hasher.update(&tar_contents);
    let hash = format!("{:x}", hasher.finalize());
    
    fs::write("checksum.txt", format!("{} dev-environment.tar.gz\n", hash))?;

    // Clean up staging directory
    fs::remove_dir_all(staging_dir)?;

    // Log completion status
    let tar_size = fs::metadata("dev-environment.tar.gz")?.len();
    logger.info(&format!("📦 Package complete! Tarball size: {} bytes", tar_size));
    logger.info(&format!("SHA256: {}", hash));

    Ok(())
}
