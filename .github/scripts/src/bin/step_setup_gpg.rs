//! GPG key setup for create-release action
//! Used by: ./.github/actions/queue-release/action.yml
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

    // Process and decode GPG key with improved error handling
    fn decode_gpg_key(&self) -> Result<Vec<u8>> {
        self.logger.info("🔑 Processing GPG key...");
        let raw_key = env::var("INPUT_BOT_GPG_PRIVATE_KEY")
            .context("BOT_GPG_PRIVATE_KEY not set")?;

        // Method 1: Try direct import if key appears to be in proper PGP format
        if raw_key.contains("-----BEGIN PGP PRIVATE KEY BLOCK-----") {
            self.logger.info("Attempting direct PGP key import...");
            return Ok(raw_key.into_bytes());
        }

        // Method 2: Try base64 decode after cleaning
        self.logger.info("Attempting base64 decode...");
        let cleaned_key = raw_key
            .trim()
            .replace("\\n", "\n")
            .replace("\r", "");

        // Clone the cleaned key for each closure
        let key1 = cleaned_key.clone();
        let key2 = cleaned_key.clone();
        let key3 = cleaned_key;

        // Define closure type
        type DecodeAttempt = Box<dyn Fn() -> Result<Vec<u8>, base64::DecodeError>>;
        
        // Create vector of boxed decode attempts
        let decode_attempts: Vec<DecodeAttempt> = vec![
            Box::new(move || STANDARD.decode(key1.as_bytes())),
            Box::new(move || {
                let no_headers = key2
                    .replace("-----BEGIN PGP PRIVATE KEY BLOCK-----", "")
                    .replace("-----END PGP PRIVATE KEY BLOCK-----", "")
                    .replace("\n", "");
                STANDARD.decode(no_headers)
            }),
            Box::new(move || {
                let padded = match key3.len() % 4 {
                    2 => format!("{}==", key3),
                    3 => format!("{}=", key3),
                    _ => key3.clone(),
                };
                STANDARD.decode(padded)
            }),
        ];

        // Try each decode method
        for (i, attempt) in decode_attempts.into_iter().enumerate() {
            match attempt() {
                Ok(decoded) => {
                    self.logger.info(&format!("Successfully decoded using method {}", i + 1));
                    return Ok(decoded);
                }
                Err(e) => {
                    self.logger.warn(&format!("Decode method {} failed: {}", i + 1, e));
                }
            }
        }

        // If all attempts fail, try to use the key as-is
        self.logger.warn("All decode attempts failed. Trying to use key as-is...");
        
        // Ensure the key has proper PGP headers if missing
        let formatted_key = if !raw_key.contains("-----BEGIN PGP PRIVATE KEY BLOCK-----") {
            format!(
                "-----BEGIN PGP PRIVATE KEY BLOCK-----\n\n{}\n-----END PGP PRIVATE KEY BLOCK-----",
                raw_key.trim()
            )
        } else {
            raw_key
        };

        Ok(formatted_key.into_bytes())
    }

    // Modified import_gpg_key to own its data
    fn import_gpg_key(&self, key_data: Vec<u8>) -> Result<()> {
        self.logger.info("🔑 Importing GPG key...");
        
        // Define closure type for owned data
        type ImportAttempt = Box<dyn Fn() -> std::io::Result<std::process::ExitStatus>>;
        
        // Create vector of boxed import attempts
        let key_data1 = key_data.clone();
        let key_data2 = key_data;

        let import_attempts: Vec<ImportAttempt> = vec![
            Box::new(move || {
                Command::new("gpg")
                    .args(["--batch", "--import"])
                    .stdin(std::process::Stdio::piped())
                    .spawn()
                    .and_then(|mut child| {
                        if let Some(mut stdin) = child.stdin.take() {
                            stdin.write_all(&key_data1)?;
                        }
                        child.wait()
                    })
            }),
            Box::new(move || {
                Command::new("gpg")
                    .args(["--batch", "--import", "--armor"])
                    .stdin(std::process::Stdio::piped())
                    .spawn()
                    .and_then(|mut child| {
                        if let Some(mut stdin) = child.stdin.take() {
                            stdin.write_all(&key_data2)?;
                        }
                        child.wait()
                    })
            }),
        ];

        for (i, attempt) in import_attempts.into_iter().enumerate() {
            match attempt() {
                Ok(status) if status.success() => {
                    self.logger.info(&format!("Successfully imported key using method {}", i + 1));
                    return Ok(());
                }
                Ok(_) => {
                    self.logger.warn(&format!("Import method {} failed", i + 1));
                }
                Err(e) => {
                    self.logger.warn(&format!("Import method {} error: {}", i + 1, e));
                }
            }
        }

        anyhow::bail!("All GPG key import attempts failed")
    }

    // Modify get_gpg_key_id to include "0x" prefix
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
            .map(|id| format!("0x{}", id))  // Add "0x" prefix
            .context("Could not find GPG key ID")?;

        self.logger.info(&format!("Found GPG key: {}", key_id));
        Ok(key_id.to_string())
    }

    // Modify trust_gpg_key to use full key ID and retry logic
    fn trust_gpg_key(&self, key_id: &str) -> Result<()> {
        self.logger.info("🔐 Setting trust level for GPG key...");
        
        // Create trust command input
        let trust_cmd = "5\ny\n";  // 5 = ultimate trust, y = confirm
        
        // Retry logic
        for attempt in 1..=3 {
            self.logger.info(&format!("Attempt {} to trust key...", attempt));

            let mut child = Command::new("gpg")
                .args(["--command-fd", "0", "--default-key", key_id, "--batch", "--local-user", key_id, "--trust-model", "always", "--edit-key", key_id, "trust"])
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .context("Failed to spawn gpg trust command")?;

            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(trust_cmd.as_bytes())
                    .context("Failed to write trust command")?;
            }

            let output = child.wait_with_output()
                .context("Failed to complete trust command")?;

            if output.status.success() {
                self.logger.info("✅ GPG key trust level set successfully");
                return Ok(());
            } else {
                let error = String::from_utf8_lossy(&output.stderr);
                self.logger.warn(&format!("Failed to trust key (attempt {}): {}", attempt, error));
            }
        }

        anyhow::bail!("Failed to trust GPG key after multiple attempts");
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

    // Modified run to continue on non-critical errors
    async fn run(&self) -> Result<()> {
        self.logger.info("🔒 Setting up GPG keys and git signing...");
        
        if let Err(e) = self.setup_gpg_directory() {
            self.logger.warn(&format!("GPG directory setup failed: {}", e));
            // Continue anyway
        }
        
        if let Err(e) = self.configure_gpg_agent() {
            self.logger.warn(&format!("GPG agent configuration failed: {}", e));
            // Continue anyway
        }
        
        let key_data = self.decode_gpg_key()
            .context("Failed to process GPG key")?;
        
        self.import_gpg_key(key_data)
            .context("Failed to import GPG key")?;
        
        let key_id = self.get_gpg_key_id()
            .context("Failed to get GPG key ID")?;
        
        // Add trust step before git configuration
        self.trust_gpg_key(&key_id)
            .context("Failed to trust GPG key")?;
        
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
