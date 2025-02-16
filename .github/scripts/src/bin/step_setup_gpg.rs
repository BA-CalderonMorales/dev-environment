//! GPG key setup for create-release action
//! Used by: ./.github/actions/create-release/action.yml
//! Purpose: Imports and configures GPG keys for signing

use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD, Engine};
use github_workflow_scripts::{get_logger, init};
use std::{env, path::PathBuf, process::Command};
use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    init();
    let logger = get_logger(false);

    logger.info("🔒 Setting up GPG keys and git signing...");

    // Get required inputs
    let gpg_key = env::var("INPUT_BOT_GPG_PRIVATE_KEY")
        .context("BOT_GPG_PRIVATE_KEY not set")?;
    let bot_email = env::var("INPUT_BOT_EMAIL")
        .context("Missing bot email")?;

    // Setup GPG directory
    logger.info("📁 Creating GPG configuration directory...");
    let gpg_dir = PathBuf::from(env::var("HOME")?).join(".gnupg");
    fs::create_dir_all(&gpg_dir)?;
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&gpg_dir, fs::Permissions::from_mode(0o700))?;
    }

    // Configure GPG
    logger.info("⚙️ Configuring GPG agent...");
    fs::write(gpg_dir.join("gpg-agent.conf"), "allow-loopback-pinentry\n")?;
    fs::write(gpg_dir.join("gpg.conf"), "pinentry-mode loopback\n")?;

    // Import key
    logger.info("🔑 Importing GPG key...");
    let decoded_key = STANDARD.decode(gpg_key)?;
    let mut child = Command::new("gpg")
        .args(["--batch", "--import"])
        .stdin(std::process::Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;  // Fixed syntax
        stdin.write_all(&decoded_key)?;
    }

    let status = child.wait()?;
    if !status.success() {
        logger.warn("Failed to import GPG key");
        anyhow::bail!("GPG key import failed");
    }

    // Configure git signing
    logger.info("🔏 Configuring git signing...");
    let key_id = String::from_utf8(
        Command::new("gpg")
            .args(["--list-secret-keys", "--keyid-format=long"])
            .output()?
            .stdout
    )?;

    let key_id = key_id.lines()
        .find(|line| line.starts_with("sec"))
        .and_then(|line| line.split('/').nth(1))
        .context("Could not find GPG key ID")?;

    logger.info(&format!("Found GPG key: {}", key_id));

    // Configure git
    for (key, value) in [
        ("user.signingkey", key_id),
        ("user.name", "Development Environment Bot"),
        ("user.email", &bot_email),
        ("commit.gpgsign", "true"),
        ("gpg.program", "gpg"),
    ] {
        let output = Command::new("git")
            .args(["config", "--global", key, value])
            .output()?;
            
        if !output.status.success() {  // Removed unnecessary parentheses
            logger.warn(&format!("Failed to set git config {}: {}", key, 
                String::from_utf8_lossy(&output.stderr)));
            anyhow::bail!("Git configuration failed");
        }
    }

    logger.info("✅ GPG setup completed successfully");
    Ok(())
}
