use anyhow::{Context, Result};
use github_workflow_scripts::{get_logger, init};
use std::{env, process::Command};

#[tokio::main]
async fn main() -> Result<()> {
    init();
    let logger = get_logger(false);

    logger.info("📝 Preparing to sign commit...");

    // Get required inputs
    let sha = env::var("INPUT_SHA").context("Missing SHA input")?;
    let position = env::var("QUEUE_POSITION").context("Missing queue position")?;
    let remaining = env::var("QUEUE_REMAINING").context("Missing remaining items")?;
    let est_time = env::var("QUEUE_ESTIMATED_TIME").context("Missing estimated time")?;
    let passphrase = env::var("INPUT_BOT_GPG_PASSPHRASE").context("Missing GPG passphrase")?;

    // Stage changes
    logger.info("📦 Staging queue changes...");
    let output = Command::new("git")
        .args(["add", ".github/release_queue/"])
        .output()?;

    if !output.status.success() {
        logger.warn("Failed to stage changes");
        anyhow::bail!("Git add failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Prepare commit message
    let commit_msg = format!(
        "📦 Queue release for {}\n\nQueue Status:\n- Position: {}\n- Items needed: {} more\n- Estimated time: {}",
        sha, position, remaining, est_time
    );

    // Sign commit
    logger.info("🔏 Creating signed commit...");
    let mut child = Command::new("git")
        .args(["commit", "-S", "-m", &commit_msg, "--allow-empty"])
        .env("GPG_TTY", "/dev/tty")
        .stdin(std::process::Stdio::piped())
        .spawn()?;

    // Provide passphrase
    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        writeln!(stdin, "{}", passphrase)?;
    }

    let status = child.wait()?;
    if !status.success() {
        logger.warn("Failed to create signed commit");
        anyhow::bail!("Commit signing failed");
    }

    logger.info("✅ Successfully created signed commit");
    Ok(())
}
