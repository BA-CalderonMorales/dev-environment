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

    // Add method to verify GPG setup
    fn verify_gpg_setup(&self) -> Result<()> {
        self.logger.info("🔍 Verifying GPG setup...");
        
        let output = Command::new("gpg")
            .args(["--list-secret-keys"])
            .output()
            .context("Failed to list GPG keys")?;

        if !output.status.success() {
            self.logger.warn("No GPG keys found");
            anyhow::bail!("GPG verification failed: No keys available");
        }

        self.logger.info("✅ GPG setup verified");
        Ok(())
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

    // Modified create_signed_commit with improved GPG handling
    fn create_signed_commit(&self) -> Result<()> {
        self.logger.info("🔏 Creating signed commit...");

        // Verify GPG setup first
        self.verify_gpg_setup()?;

        // Prepare commit message
        let commit_msg = format!(
            "📦 Queue release for {}\n\nQueue Status:\n- Position: {}\n- Items needed: {} more\n- Estimated time: {}",
            self.sha, self.position, self.remaining, self.est_time
        );

        // Create the commit with signature
        let mut child = Command::new("git")
            .args([
                "-c", "gpg.program=gpg",  // Ensure using gpg explicitly
                "-c", "commit.gpgsign=true",
                "commit",
                "-S",
                "-m", &commit_msg,
                "--allow-empty"
            ])
            .env("GPG_TTY", "/dev/tty")
            .env("GNUPGHOME", std::env::var("HOME").unwrap_or_default() + "/.gnupg")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .context("Failed to spawn git commit")?;

        // Provide passphrase if needed
        if let Some(mut stdin) = child.stdin.take() {
            writeln!(stdin, "{}", self.passphrase)
                .context("Failed to write GPG passphrase")?;
        }

        let output = child.wait_with_output()
            .context("Failed to complete git commit")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            self.logger.warn(&format!("Commit failed: {}", error));
            anyhow::bail!("Commit signing failed: {}", error);
        }

        self.logger.info("✅ Successfully created signed commit");
        Ok(())
    }

    // Modified run method to include verification
    async fn run(&self) -> Result<()> {
        self.verify_gpg_setup()?;
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
