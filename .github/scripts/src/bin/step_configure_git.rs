//! Git configuration for create-release action
//! Used by: ./.github/actions/create-release/action.yml
//! Purpose: Sets up git config for bot commits

use anyhow::{Context, Result};
use github_workflow_scripts::{get_logger, init};
use std::{env, process::Command};

#[tokio::main]
async fn main() -> Result<()> {
    init();
    let logger = get_logger(false);

    logger.info("🔧 Configuring Git...");

    // Get required inputs
    let bot_name = env::var("INPUT_BOT_NAME").context("BOT_NAME not set")?;
    let bot_email = env::var("INPUT_BOT_EMAIL").context("BOT_EMAIL not set")?;
    let gpg_key = env::var("INPUT_BOT_GPG_PRIVATE_KEY").context("BOT_GPG_PRIVATE_KEY not set")?;

    // Configure git
    let configs = [
        ("user.name", bot_name),
        ("user.email", bot_email),
        ("user.signingkey", gpg_key),
        ("commit.gpgsign", "true".to_string()),
    ];

    for (key, value) in configs {
        let status = Command::new("git")
            .args(["config", "--global", key, &value])
            .status()
            .context(format!("Failed to set git config {}", key))?;

        if !status.success() {
            anyhow::bail!("Failed to configure git {}", key);
        }
    }

    logger.info("✅ Git configuration completed");
    Ok(())
}
