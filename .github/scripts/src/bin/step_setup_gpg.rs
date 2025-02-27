//! GPG key setup for create-release action
//! Used by: ./.github/actions/queue-release/action.yml
//! Purpose: Imports and configures GPG keys for signing

use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD, Engine};
use github_workflow_scripts::{get_logger, init};
use std::env;
use std::fs;
use std::io::Write;
use std::process::Command;

// Struct to hold common dependencies
struct GpgSetup {
    logger: Box<dyn github_workflow_scripts::Logger>,
    gpg_private_key: String,
    gpg_passphrase: Option<String>,
    bot_email: String,
    home_dir: String,
}

impl GpgSetup {
    // Initialize setup with required inputs
    fn new() -> Result<Self> {
        let logger = get_logger(false);
        
        // Get required environment variables
        let gpg_private_key = env::var("INPUT_BOT_GPG_PRIVATE_KEY")
            .context("Missing GPG private key input")?;
            
        // Optional passphrase
        let gpg_passphrase = env::var("INPUT_BOT_GPG_PASSPHRASE").ok();
        
        // Bot email for git config
        let bot_email = env::var("INPUT_BOT_EMAIL")
            .unwrap_or_else(|_| "actions@github.com".to_string());
            
        // Get home directory
        let home_dir = env::var("HOME")
            .unwrap_or_else(|_| "/home/runner".to_string());
            
        Ok(Self {
            logger,
            gpg_private_key,
            gpg_passphrase,
            bot_email,
            home_dir,
        })
    }

    // Setup GPG directory and permissions
    fn setup_gpg_dirs(&self) -> Result<()> {
        self.logger.info("📁 Creating GPG configuration directory...");
        
        // Create .gnupg directory
        let gpg_dir = format!("{}/.gnupg", self.home_dir);
        fs::create_dir_all(&gpg_dir).context("Failed to create GPG directory")?;
        
        // Set strict permissions
        Command::new("chmod")
            .args(&["700", &gpg_dir])
            .status()
            .context("Failed to set permissions on GPG directory")?;
            
        // Configure gpg-agent
        self.logger.info("⚙️ Configuring GPG agent...");
        let gpg_agent_conf = format!("{}/gpg-agent.conf", gpg_dir);
        let mut file = fs::File::create(&gpg_agent_conf)
            .context("Failed to create gpg-agent.conf")?;
            
        // Write configuration
        writeln!(file, "default-cache-ttl 7200")?;
        writeln!(file, "max-cache-ttl 31536000")?;
        writeln!(file, "allow-preset-passphrase")?;
        
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

    // Import the GPG private key
    fn import_gpg_key(&self) -> Result<String> {
        self.logger.info("🔑 Processing GPG key...");
        
        // First try direct import
        self.logger.info("Attempting direct PGP key import...");
        let import_result = self.import_key_direct();
        
        match import_result {
            Ok(key_id) => {
                self.logger.info("Successfully imported key using method 1");
                Ok(key_id)
            },
            Err(e) => {
                self.logger.warn(&format!("Direct import failed: {}", e));
                self.logger.info("Attempting armored PGP key import...");
                
                // Try importing as armored key
                let import_result_2 = self.import_key_armored();
                match import_result_2 {
                    Ok(key_id) => {
                        self.logger.info("Successfully imported key using method 2");
                        Ok(key_id)
                    },
                    Err(e2) => {
                        anyhow::bail!("All key import methods failed: {} / {}", e, e2);
                    }
                }
            }
        }
    }
    
    // Import key directly from base64
    fn import_key_direct(&self) -> Result<String> {
        self.logger.info("🔑 Importing GPG key...");
        
        // Write key to temporary file
        let key_path = format!("{}/.gnupg/private-key.gpg", self.home_dir);
        
        // Use STANDARD engine for decoding instead of deprecated base64::decode
        let decoded = STANDARD.decode(&self.gpg_private_key)
            .context("Failed to decode GPG key from base64")?;
            
        std::fs::write(&key_path, decoded)
            .context("Failed to write GPG key to file")?;
            
        // Import the key
        let output = Command::new("gpg")
            .args(&["--batch", "--import", &key_path])
            .output()
            .context("Failed to run GPG import command")?;
            
        // Remove temporary file
        std::fs::remove_file(&key_path)
            .context("Failed to remove temporary GPG key file")?;
            
        if !output.status.success() {
            anyhow::bail!(
                "GPG key import failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        
        // Get key ID
        self.get_gpg_key_id()
    }
    
    // Import key in armored format
    fn import_key_armored(&self) -> Result<String> {
        self.logger.info("🔑 Importing armored GPG key...");
        
        // Write armored key to temporary file
        let key_path = format!("{}/.gnupg/private-key-armored.gpg", self.home_dir);
        let mut file = fs::File::create(&key_path)
            .context("Failed to create armored key file")?;
            
        writeln!(file, "-----BEGIN PGP PRIVATE KEY BLOCK-----")?;
        
        // Write key in chunks of 64 characters
        let key = self.gpg_private_key.replace("\n", "");
        for chunk in key.as_bytes().chunks(64) {
            writeln!(file, "{}", String::from_utf8_lossy(chunk))?;
        }
        
        writeln!(file, "-----END PGP PRIVATE KEY BLOCK-----")?;
        
        // Import the key
        let output = Command::new("gpg")
            .args(&["--batch", "--import", &key_path])
            .output()
            .context("Failed to run GPG import command for armored key")?;
            
        // Remove temporary file
        std::fs::remove_file(&key_path)
            .context("Failed to remove temporary armored GPG key file")?;
            
        if !output.status.success() {
            anyhow::bail!(
                "Armored GPG key import failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        
        // Get key ID
        self.get_gpg_key_id()
    }
    
    // Extract key ID from imported key
    fn get_gpg_key_id(&self) -> Result<String> {
        self.logger.info("🔏 Getting GPG key ID...");
        
        // List secret keys
        let output = Command::new("gpg")
            .args(&["--list-secret-keys", "--keyid-format", "0xlong"])
            .output()
            .context("Failed to list GPG keys")?;
            
        if !output.status.success() {
            anyhow::bail!(
                "Failed to list GPG keys: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        
        // Extract key ID from output
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines() {
            if line.trim().starts_with("sec") {
                // Key line format: sec   rsa4096/0xKEYID 2023-01-01 [SC]
                if let Some(key_part) = line.split_whitespace().nth(1) {
                    let key_id = if key_part.contains('/') {
                        // Extract key ID from format like rsa4096/0xKEYID
                        key_part.split('/').nth(1).unwrap_or(key_part)
                    } else {
                        key_part
                    };
                    
                    self.logger.info(&format!("Found GPG key: {}", line.trim()));
                    return Ok(key_id.to_string());
                }
            }
        }
        
        anyhow::bail!("No GPG key found in output: {}", output_str)
    }
    
    // Set trust level for the key
    fn trust_gpg_key(&self, key_id: &str) -> Result<()> {
        self.logger.info("🔐 Setting trust level for GPG key...");
        
        // Extract just the key ID without any extra information
        let clean_key_id = if key_id.contains(' ') {
            // If there's a space, take only the first part (the actual key ID)
            key_id.split_whitespace().next().unwrap_or(key_id)
        } else {
            key_id
        };
        
        self.logger.info(&format!("Using key ID for trust: {}", clean_key_id));
        
        // Try multiple trust commands
        let trust_commands = [
            // Echo commands to automate the trust dialog
            format!("echo -e '5\ny\n' | gpg --command-fd 0 --edit-key {} trust quit", clean_key_id),
            // Direct ultimate trust command
            format!("gpg --batch --yes --trust-model always --command-fd 0 --edit-key {} trust quit", clean_key_id),
            // Alternative format just using the key ID portion
            format!("echo -e '5\ny\n' | gpg --command-fd 0 --edit-key {} trust quit", 
                    if clean_key_id.contains("0x") { 
                        clean_key_id.to_string() 
                    } else { 
                        format!("0x{}", clean_key_id)
                    })
        ];
        
        // Try each command until one succeeds
        for (i, cmd) in trust_commands.iter().enumerate() {
            self.logger.info(&format!("Attempt {} to trust key...", i+1));
            
            let output = Command::new("sh")
                .args(&["-c", cmd])
                .output();
                
            match output {
                Ok(result) => {
                    if result.status.success() {
                        self.logger.info(&format!("Successfully set trust level with command {}", i+1));
                        return Ok(());
                    } else {
                        self.logger.warn(&format!("Failed to trust key (attempt {}): {}", 
                            i+1, String::from_utf8_lossy(&result.stderr)));
                    }
                },
                Err(e) => {
                    self.logger.warn(&format!("Error executing trust command {}: {}", i+1, e));
                }
            }
        }
        
        // If we've tried all commands and none worked, but the key is imported,
        // let's proceed anyway since some git operations don't strictly require trust level
        self.logger.warn("Could not set explicit trust level, but key is imported");
        Ok(())
    }
    
    // Configure Git for signing
    fn configure_git(&self, key_id: &str) -> Result<()> {
        self.logger.info("⚙️ Configuring Git for commit signing...");
        
        // Extract clean key ID
        let clean_key_id = if key_id.starts_with("0x") {
            key_id.to_string()
        } else {
            format!("0x{}", key_id)
        };
        
        // Add email and username config using bot_email
        let git_configs = [
            ("user.signingkey", clean_key_id.clone()),
            ("commit.gpgsign", "true".to_string()),
            ("tag.gpgsign", "true".to_string()),
            ("gpg.program", "gpg".to_string()),
            // Add user email from the struct field
            ("user.email", self.bot_email.clone()),
            // Extract username from email for git config
            ("user.name", self.bot_email.split('@').next().unwrap_or("Bot").to_string()),
        ];
        
        for (key, value) in &git_configs {
            let output = Command::new("git")
                .args(&["config", "--global", key, value])
                .output()
                .context(format!("Failed to set git config {}", key))?;
                
            if !output.status.success() {
                anyhow::bail!(
                    "Failed to set git config {}: {}",
                    key,
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        
        self.logger.info("✅ Git signing configuration complete");
        Ok(())
    }

    // Helper method to set up GPG passphrase if provided
    fn setup_gpg_passphrase(&self) -> Result<()> {
        // If passphrase is provided, configure it for non-interactive usage
        if let Some(passphrase) = &self.gpg_passphrase {
            self.logger.info("🔐 Setting up GPG passphrase...");
            
            // Create a gpg-preset-passphrase script
            let passphrase_script = format!(
                "#!/bin/sh\necho '{}' | /usr/lib/gnupg/gpg-preset-passphrase --preset \"$1\"",
                passphrase
            );
            
            // Write the script to a file
            let script_path = format!("{}/.gnupg/preset-passphrase.sh", self.home_dir);
            std::fs::write(&script_path, passphrase_script)
                .context("Failed to write GPG passphrase script")?;
                
            // Make it executable
            Command::new("chmod")
                .args(&["+x", &script_path])
                .status()
                .context("Failed to make passphrase script executable")?;
                
            self.logger.info("✅ GPG passphrase helper configured");
        }
        
        Ok(())
    }

    // Run the setup process 
    async fn run(&self) -> Result<()> {
        self.logger.info("🔒 Setting up GPG keys and git signing...");
        
        if let Err(e) = self.setup_gpg_dirs() {
            self.logger.warn(&format!("GPG directory setup failed: {}", e));
            // Continue anyway
        }
        
        // Setup passphrase if provided
        if self.gpg_passphrase.is_some() {
            if let Err(e) = self.setup_gpg_passphrase() {
                self.logger.warn(&format!("GPG passphrase setup failed: {}", e));
                // Continue anyway
            }
        }
        
        // We don't actually use the decoded key data directly, so we can ignore it
        let _ = self.decode_gpg_key()
            .context("Failed to process GPG key")?;
        
        let key_id = self.import_gpg_key()
            .context("Failed to import GPG key")?;
        
        // Add trust step before git configuration
        if let Err(e) = self.trust_gpg_key(&key_id) {
            // Just warn instead of failing - key might still be usable
            self.logger.warn(&format!("Warning: Failed to trust key: {}", e));
        }
        
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
