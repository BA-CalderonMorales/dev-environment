//! GPG key setup for create-release action
//! Used by: ./.github/actions/create-release/action.yml
//! Purpose: Imports and configures GPG keys for signing

use anyhow::{Context, Result};
use github_workflow_scripts::{get_logger, init_logging};
use std::{env, fs, path::PathBuf, process::Command};

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();
    let logger = get_logger(false);

    logger.info("🔑 Setting up GPG keys...");

    let gpg_key = env::var("INPUT_BOT_GPG_PRIVATE_KEY")
        .context("BOT_GPG_PRIVATE_KEY not set")?;
    let passphrase = env::var("INPUT_BOT_GPG_PASSPHRASE")
        .context("BOT_GPG_PASSPHRASE not set")?;

    // Setup GPG config directory
    let gpg_dir = PathBuf::from(env::var("HOME")?).join(".gnupg");
    fs::create_dir_all(&gpg_dir)?;
    fs::write(
        gpg_dir.join("gpg-agent.conf"),
        "allow-preset-passphrase\n",
    )?;

    // Import GPG key
    let mut child = Command::new("gpg")
        .args(["--batch", "--import"])
        .stdin(std::process::Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        stdin.write_all(gpg_key.as_bytes())?;
    }

    let status = child.wait()?;
    if (!status.success()) {
        anyhow::bail!("Failed to import GPG key");
    }

    // Reload agent
    let status = Command::new("gpg-connect-agent")
        .args(["reloadagent", "/bye"])
        .status()?;

    if (!status.success()) {
        anyhow::bail!("Failed to reload GPG agent");
    }

    // Export for git signing
    env::set_var("GPG_TTY", "/dev/tty");
    env::set_var("GPG_PASSPHRASE", passphrase);

    logger.info("✅ GPG setup completed successfully");
    Ok(())
}
