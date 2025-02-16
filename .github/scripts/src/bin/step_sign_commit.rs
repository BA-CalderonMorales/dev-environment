use anyhow::{Context, Result};
use github_workflow_scripts::{get_logger, init};
use std::{env, process::Command};
use std::io::Write;

// Struct to hold common dependencies and state
struct CommitSigner {
    logger: Box<dyn github_workflow_scripts::Logger>,
    sha: String,
    position: String,
    remaining: String,
    est_time: String,
    passphrase: String,
}

impl CommitSigner {
    // Initialize with required inputs
    fn new() -> Result<Self> {
        let logger = get_logger(false);
        
        // Get all required inputs upfront
        let sha = env::var("INPUT_SHA").context("Missing SHA input")?;
        let position = env::var("QUEUE_POSITION").context("Missing queue position")?;
        let remaining = env::var("QUEUE_REMAINING").context("Missing remaining items")?;
        let est_time = env::var("QUEUE_ESTIMATED_TIME").context("Missing estimated time")?;
        let passphrase = env::var("INPUT_BOT_GPG_PASSPHRASE").context("Missing GPG passphrase")?;

        Ok(Self {
            logger,
            sha,
            position,
            remaining,
            est_time,
            passphrase,
        })
    }

    // Stage changes to the queue
    fn stage_changes(&self) -> Result<()> {
        self.logger.info("📦 Staging queue changes...");
        let output = Command::new("git")
            .args(["add", ".github/release_queue/"])
            .output()
            .context("Failed to execute git add")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            self.logger.warn(&format!("Failed to stage changes: {}", error));
            anyhow::bail!("Git add failed: {}", error);
        }

        Ok(())
    }

    // Create and sign the commit
    fn create_signed_commit(&self) -> Result<()> {
        self.logger.info("🔏 Creating signed commit...");

        // Prepare commit message
        let commit_msg = format!(
            "📦 Queue release for {}\n\nQueue Status:\n- Position: {}\n- Items needed: {} more\n- Estimated time: {}",
            self.sha, self.position, self.remaining, self.est_time
        );

        // Create the commit with signature
        let mut child = Command::new("git")
            .args([
                "commit",
                "-S",
                "-m", &commit_msg,
                "--allow-empty"
            ])
            .env("GPG_TTY", "/dev/tty")
            .stdin(std::process::Stdio::piped())
            .spawn()
            .context("Failed to spawn git commit")?;

        // Provide passphrase if needed
        if let Some(mut stdin) = child.stdin.take() {
            writeln!(stdin, "{}", self.passphrase)
                .context("Failed to write GPG passphrase")?;
        }

        let status = child.wait()
            .context("Failed to complete git commit")?;

        if !status.success() {
            self.logger.warn("Failed to create signed commit");
            anyhow::bail!("Commit signing failed with status: {}", status);
        }

        self.logger.info("✅ Successfully created signed commit");
        Ok(())
    }

    // Run the complete signing process
    async fn run(&self) -> Result<()> {
        self.stage_changes()?;
        self.create_signed_commit()?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    init();
    let signer = CommitSigner::new()?;
    signer.run().await
}
