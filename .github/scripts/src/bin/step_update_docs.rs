//! Documentation updater for create-release action
//! Used by: ./.github/actions/create-release/action.yml
//! Purpose: Updates documentation links and commits changes

use anyhow::{Context, Result};
use github_workflow_scripts::{get_logger, init_logging};
use octocrab::Octocrab;
use regex::Regex;
use std::{env, fs, process::Command};

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();
    let logger = get_logger(false);

    logger.info("📚 Updating documentation...");

    // Initialize GitHub client
    let token = env::var("GH_TOKEN").context("GH_TOKEN not set")?;
    let octocrab = Octocrab::builder()
        .personal_token(token)
        .build()?;

    // Get latest release tag
    let releases = octocrab
        .repos("BA-CalderonMorales", "dev-environment")
        .releases()
        .list()
        .per_page(1)
        .send()
        .await?;

    let latest_tag = releases
        .items
        .first()
        .context("No releases found")?
        .tag_name
        .clone();

    // Update documentation links
    let docs_path = "docs/QUICK_START.md";
    let content = fs::read_to_string(docs_path)?;
    
    let re = Regex::new(r"releases/[^/]*/download/")?;
    let updated = re.replace_all(&content, "releases/latest/download/");
    
    fs::write(docs_path, updated.as_bytes())?;

    // Commit changes
    let status = Command::new("git")
        .args(["add", docs_path])
        .status()?;

    if !status.success() {
        anyhow::bail!("Failed to stage documentation changes");
    }

    let commit_msg = format!("docs: update distribution links to {}", latest_tag);
    let status = Command::new("git")
        .args(["commit", "-S", "-m", &commit_msg])
        .status()?;

    if !status.success() {
        logger.info("No changes to commit");
        return Ok(());
    }

    // Push changes
    let branch = env::var("GITHUB_REF_NAME")?;
    let status = Command::new("git")
        .args(["push", "origin", &format!("HEAD:{}", branch)])
        .status()?;

    if !status.success() {
        anyhow::bail!("Failed to push documentation updates");
    }

    logger.info("✅ Documentation updated successfully");
    Ok(())
}
