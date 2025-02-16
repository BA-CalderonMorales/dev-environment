use anyhow::{Context, Result};
use chrono::Utc;
use github_workflow_scripts::{get_logger, init};
use std::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
    init();
    let logger = get_logger(false);
    
    logger.info("🔄 Setting up queue branch...");
    
    let timestamp = Utc::now().timestamp();
    let branch_name = format!("queue-update-{}", timestamp);

    // Create new branch
    let output = Command::new("git")
        .args(["checkout", "-b", &branch_name])
        .output()
        .context("Failed to create queue branch")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        logger.warn(&format!("Failed to create branch: {}", error));
        anyhow::bail!("Branch creation failed");
    }

    logger.info(&format!("✅ Created branch: {}", branch_name));
    println!("::set-output name=branch::{}", branch_name);
    Ok(())
}
