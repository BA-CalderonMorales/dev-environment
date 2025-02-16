//! GPG key setup for create-release action
//! Used by: ./.github/actions/create-release/action.yml
//! Purpose: Imports and configures GPG keys for signing

use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD, Engine};
use github_workflow_scripts::{get_logger, init};
use std::{env, path::PathBuf, process::Command};
use std::fs;
use std::io::Write;

// Struct to hold common dependencies
struct GpgSetup {
    logger: Box<dyn github_workflow_scripts::Logger>,
    gpg_dir: PathBuf,
    bot_email: String,
}

impl GpgSetup {
    // Initialize setup with required inputs
    fn new() -> Result<Self> {
        let logger = get_logger(false);
        let bot_email = env::var("INPUT_BOT_EMAIL")
            .context("Missing bot email")?;
        let gpg_dir = PathBuf::from(env::var("HOME")?).join(".gnupg");
        
        Ok(Self {
            logger,
            gpg_dir,
            bot_email,
        })
    }

    // Setup GPG directory and permissions
    fn setup_gpg_directory(&self) -> Result<()> {
        self.logger.info("📁 Creating GPG configuration directory...");
        fs::create_dir_all(&self.gpg_dir)?;
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&self.gpg_dir, fs::Permissions::from_mode(0o700))?;
        }

        Ok(())
    }

    // Configure GPG agent settings
    fn configure_gpg_agent(&self) -> Result<()> {
        self.logger.info("⚙️ Configuring GPG agent...");
        fs::write(
            self.gpg_dir.join("gpg-agent.conf"),
            "allow-loopback-pinentry\n"
        )?;
        fs::write(
            self.gpg_dir.join("gpg.conf"),
            "pinentry-mode loopback\n"
        )?;
        Ok(())
    }

    // Process and decode GPG key
    fn decode_gpg_key(&self) -> Result<Vec<u8>> {
        self.logger.info("🔑 Processing GPG key...");
        let gpg_key = env::var("INPUT_BOT_GPG_PRIVATE_KEY")
            .context("BOT_GPG_PRIVATE_KEY not set")?
            .trim()
            .replace("\\n", "\n")
            .replace("\r", "");

        STANDARD.decode(&gpg_key).map_err(|e| {
            self.logger.warn(&format!("Failed to decode GPG key: {}", e));
            anyhow::anyhow!("Invalid GPG key format: {}", e)
        })
    }

    // Import GPG key into system
    fn import_gpg_key(&self, decoded_key: &[u8]) -> Result<()> {
        self.logger.info("🔑 Importing GPG key...");
        let mut child = Command::new("gpg")
            .args(["--batch", "--import"])
            .stdin(std::process::Stdio::piped())
            .spawn()
            .context("Failed to spawn GPG process")?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(decoded_key)
                .context("Failed to write GPG key to gpg process")?;
        }

        let status = child.wait()
            .context("Failed to wait for GPG process")?;

        if !status.success() {
            self.logger.warn("GPG key import failed");
            anyhow::bail!("GPG import process failed with status: {}", status);
        }

        Ok(())
    }

    // Get GPG key ID from imported key
    fn get_gpg_key_id(&self) -> Result<String> {
        self.logger.info("🔏 Getting GPG key ID...");
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

        self.logger.info(&format!("Found GPG key: {}", key_id));
        Ok(key_id.to_string())
    }

    // Configure git with GPG settings
    fn configure_git(&self, key_id: &str) -> Result<()> {
        self.logger.info("🔏 Configuring git signing...");
        
        let configs = [
            ("user.signingkey", key_id),
            ("user.name", "Development Environment Bot"),
            ("user.email", &self.bot_email),
            ("commit.gpgsign", "true"),
            ("gpg.program", "gpg"),
        ];

        for (key, value) in configs {
            let output = Command::new("git")
                .args(["config", "--global", key, value])
                .output()?;
                
            if !output.status.success() {
                self.logger.warn(&format!("Failed to set git config {}: {}", key, 
                    String::from_utf8_lossy(&output.stderr)));
                anyhow::bail!("Git configuration failed for key: {}", key);
            }
        }

        Ok(())
    }

    // Run the complete setup process
    async fn run(&self) -> Result<()> {
        self.logger.info("🔒 Setting up GPG keys and git signing...");
        
        self.setup_gpg_directory()
            .context("Failed to setup GPG directory")?;
        
        self.configure_gpg_agent()
            .context("Failed to configure GPG agent")?;
        
        let decoded_key = self.decode_gpg_key()
            .context("Failed to decode GPG key")?;
        
        self.import_gpg_key(&decoded_key)
            .context("Failed to import GPG key")?;
        
        let key_id = self.get_gpg_key_id()
            .context("Failed to get GPG key ID")?;
        
        self.configure_git(&key_id)
            .context("Failed to configure git")?;

        self.logger.info("✅ GPG setup completed successfully");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    init();
    let setup = GpgSetup::new()?;
    setup.run().await
}
