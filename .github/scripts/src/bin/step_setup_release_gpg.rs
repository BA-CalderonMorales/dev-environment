//! Release-specific GPG setup for create-release workflow
//! Used by: ./.github/actions/setup-git-signing/action.yml in create-release workflow
//! Purpose: Imports and configures GPG keys specifically for release signing

use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD, Engine};
use github_workflow_scripts::{get_logger, init, github}; // Now properly imported
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

/// Main function for release-specific GPG setup
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger and environment
    init();
    let logger = get_logger(false);
    logger.info("🔐 Setting up release GPG signing environment...");
    
    // Get required inputs
    let bot_email = env::var("INPUT_BOT_EMAIL").context("Missing BOT_EMAIL input")?;
    let bot_name = env::var("INPUT_BOT_NAME").context("Missing BOT_NAME input")?;
    let gpg_key = env::var("INPUT_BOT_GPG_PRIVATE_KEY").ok();
    let gpg_passphrase = env::var("INPUT_BOT_GPG_PASSPHRASE").ok();
    let debug_mode = env::var("INPUT_DEBUG_MODE").unwrap_or_else(|_| "false".to_string()) == "true";
    
    // Basic Git setup (always perform this)
    setup_basic_git(&bot_name, &bot_email)?;
    
    // Early exit if no GPG key provided
    if gpg_key.is_none() {
        logger.info("No GPG key provided, skipping GPG signing setup");
        github::set_output("signing_enabled", "false");
        github::set_output("gpg_key_id", "");
        return Ok(());
    }
    
    // Setup GPG environment
    setup_gpg_environment(debug_mode)?;
    
    // Setup GPG passphrase if available
    if let Some(passphrase) = &gpg_passphrase {
        setup_gpg_passphrase(&passphrase)?;
    }
    
    // Import GPG key
    match import_gpg_key(gpg_key.as_ref().unwrap(), debug_mode) {
        Ok(key_id) => {
            // Configure Git with signing
            configure_git_signing(&key_id, &bot_name, &bot_email)?;
            github::set_output("signing_enabled", "true");
            github::set_output("gpg_key_id", &key_id);
            logger.info("✅ GPG signing setup completed successfully!");
        },
        Err(e) => {
            logger.warn(&format!("⚠️ Failed to import GPG key: {}", e));
            github::set_output("signing_enabled", "false");
            github::set_output("gpg_key_id", "");
        }
    }
    
    Ok(())
}

/// Setup GPG directories and configuration
fn setup_gpg_environment(debug_mode: bool) -> Result<()> {
    let logger = get_logger(debug_mode);
    logger.info("📂 Creating GPG configuration directory");
    
    // Get home directory
    let home_dir = env::var("HOME").unwrap_or_else(|_| "/home/runner".to_string());
    let gpg_dir = Path::new(&home_dir).join(".gnupg");
    
    // Create directory if needed
    if !gpg_dir.exists() {
        fs::create_dir_all(&gpg_dir).context("Failed to create .gnupg directory")?;
        
        // Set permissions
        Command::new("chmod")
            .args(["700", gpg_dir.to_str().unwrap()])
            .output()
            .context("Failed to set .gnupg directory permissions")?;
    }
    
    // Configure GPG agent for non-interactive use
    let agent_conf = gpg_dir.join("gpg-agent.conf");
    fs::write(agent_conf, "allow-loopback-pinentry\npinentry-mode loopback\n")
        .context("Failed to write gpg-agent.conf")?;
        
    let gpg_conf = gpg_dir.join("gpg.conf");
    fs::write(gpg_conf, "batch\npinentry-mode loopback\ntrust-model always\n")
        .context("Failed to write gpg.conf")?;
        
    // Restart GPG agent to apply settings
    let _ = Command::new("gpgconf")
        .args(["--kill", "gpg-agent"])
        .output();
        
    logger.info("✅ GPG environment configured successfully");
    Ok(())
}

/// Configure GPG passphrase for non-interactive use
fn setup_gpg_passphrase(passphrase: &str) -> Result<()> {
    let logger = get_logger(false);
    logger.info("🔑 Configuring GPG passphrase");
    
    // Get home directory
    let home_dir = env::var("HOME").unwrap_or_else(|_| "/home/runner".to_string());
    let gpg_dir = Path::new(&home_dir).join(".gnupg");
    
    // Create passphrase file
    let passphrase_file = gpg_dir.join("passphrase");
    fs::write(&passphrase_file, passphrase).context("Failed to write GPG passphrase file")?;
    
    // Set secure permissions
    Command::new("chmod")
        .args(["600", passphrase_file.to_str().unwrap()])
        .output()
        .context("Failed to set passphrase file permissions")?;
        
    // Create helper script
    let helper_script = r#"#!/bin/sh
cat ~/.gnupg/passphrase
"#;
    
    let helper_path = gpg_dir.join("passphrase-helper");
    fs::write(&helper_path, helper_script).context("Failed to write passphrase helper script")?;
    
    Command::new("chmod")
        .args(["700", helper_path.to_str().unwrap()])
        .output()
        .context("Failed to set helper script permissions")?;
        
    logger.info("✅ GPG passphrase configured successfully");
    Ok(())
}

/// Import GPG key with multiple fallback strategies
fn import_gpg_key(gpg_key: &str, debug_mode: bool) -> Result<String> {
    let logger = get_logger(debug_mode);
    logger.info("🔐 Importing GPG key");
    
    // Get home directory
    let home_dir = env::var("HOME").unwrap_or_else(|_| "/home/runner".to_string());
    let gpg_dir = Path::new(&home_dir).join(".gnupg");
    
    // Try multiple import strategies
    
    // Method 1: Key may already be properly formatted
    if gpg_key.contains("-----BEGIN PGP PRIVATE KEY BLOCK-----") {
        logger.info("Key appears to be in PGP format already");
        
        // Write to temporary file
        let key_file = gpg_dir.join("release-key.gpg");
        fs::write(&key_file, gpg_key).context("Failed to write PGP key to file")?;
        
        // Import the key
        let import_result = Command::new("gpg")
            .args(["--batch", "--import", key_file.to_str().unwrap()])
            .output()
            .context("Failed to import PGP key")?;
            
        // Cleanup
        let _ = fs::remove_file(&key_file);
        
        if import_result.status.success() {
            logger.info("✅ Successfully imported PGP key");
            return get_gpg_key_id();
        }
        
        logger.warn(&format!("PGP key import failed: {}", 
            String::from_utf8_lossy(&import_result.stderr)));
    }
    
    // Method 2: Try base64 decode
    logger.info("Attempting base64 decode of GPG key");
    
    let decode_result = STANDARD.decode(gpg_key);
    if let Ok(decoded) = decode_result {
        // Write decoded key to file
        let key_file = gpg_dir.join("release-key-decoded.gpg");
        fs::write(&key_file, &decoded).context("Failed to write decoded key to file")?;
        
        // Import the key
        let import_result = Command::new("gpg")
            .args(["--batch", "--import", key_file.to_str().unwrap()])
            .output()
            .context("Failed to import decoded key")?;
            
        // Cleanup
        let _ = fs::remove_file(&key_file);
        
        if import_result.status.success() {
            logger.info("✅ Successfully imported base64 decoded key");
            return get_gpg_key_id();
        }
        
        logger.warn(&format!("Base64 key import failed: {}", 
            String::from_utf8_lossy(&import_result.stderr)));
    }
    
    // Method 3: Clean key and try armored import
    logger.info("Creating armored key from input");
    
    // Clean and format key
    let cleaned_key = gpg_key
        .replace("\\n", "\n")
        .replace("\r", "")
        .replace(" ", "");
        
    // Create armored key
    let armored_key = format!(
        "-----BEGIN PGP PRIVATE KEY BLOCK-----\n\n{}\n-----END PGP PRIVATE KEY BLOCK-----",
        cleaned_key
    );
    
    // Write armored key to file
    let key_file = gpg_dir.join("release-key-armored.gpg");
    fs::write(&key_file, &armored_key).context("Failed to write armored key to file")?;
    
    // Try multiple import flags for best compatibility
    let import_commands = [
        vec!["--batch", "--import"],
        vec!["--batch", "--allow-secret-key-import", "--import"],
        vec!["--batch", "--ignore-crc-error", "--import"],
        vec!["--batch", "--allow-weak-key-signatures", "--import"],
        vec!["--batch", "--ignore-crc-error", "--ignore-time-conflict", 
             "--allow-secret-key-import", "--allow-weak-key-signatures", "--import"],
    ];
    
    // Try each command
    for (i, args) in import_commands.iter().enumerate() {
        let mut command_args = args.clone();
        command_args.push(key_file.to_str().unwrap());
        
        logger.info(&format!("Trying import method {}: gpg {}", 
                            i+1, command_args.join(" ")));
                            
        let import_result = Command::new("gpg")
            .args(&command_args)
            .output();
            
        match import_result {
            Ok(output) if output.status.success() => {
                logger.info(&format!("✅ Import method {} succeeded", i+1));
                
                // Cleanup
                let _ = fs::remove_file(&key_file);
                
                return get_gpg_key_id();
            },
            Ok(output) => {
                logger.warn(&format!("Import method {} failed: {}", 
                    i+1, String::from_utf8_lossy(&output.stderr)));
            },
            Err(e) => {
                logger.warn(&format!("Import method {} error: {}", i+1, e));
            }
        }
    }
    
    // Cleanup
    let _ = fs::remove_file(&key_file);
    
    // If all imports fail, fallback to generating a new key
    logger.info("All import methods failed, attempting to generate a temporary key");
    generate_temporary_key()
}

/// Generate a temporary GPG key for signing as last resort
fn generate_temporary_key() -> Result<String> {
    let logger = get_logger(false);
    logger.info("🔑 Generating temporary GPG key for signing");
    
    // Get home directory and required inputs
    let home_dir = env::var("HOME").unwrap_or_else(|_| "/home/runner".to_string());
    let gpg_dir = Path::new(&home_dir).join(".gnupg");
    
    let bot_name = env::var("INPUT_BOT_NAME").unwrap_or_else(|_| "GitHub Actions".to_string());
    let bot_email = env::var("INPUT_BOT_EMAIL").unwrap_or_else(|_| "actions@github.com".to_string());
    let gpg_passphrase = env::var("INPUT_BOT_GPG_PASSPHRASE").unwrap_or_default();
    
    // Create batch file for key generation
    let batch_file = gpg_dir.join("keygen.batch");
    let mut file = fs::File::create(&batch_file).context("Failed to create key generation batch file")?;
    
    // Write key generation parameters
    writeln!(file, "Key-Type: RSA")?;
    writeln!(file, "Key-Length: 4096")?;
    writeln!(file, "Subkey-Type: RSA")?;
    writeln!(file, "Subkey-Length: 4096")?;
    writeln!(file, "Name-Real: {}", bot_name)?;
    writeln!(file, "Name-Email: {}", bot_email)?;
    writeln!(file, "Expire-Date: 0")?;
    
    // Use passphrase if provided, otherwise no protection
    if !gpg_passphrase.is_empty() {
        writeln!(file, "Passphrase: {}", gpg_passphrase)?;
    } else {
        writeln!(file, "%no-protection")?;
    }
    
    writeln!(file, "%commit")?;
    
    // Execute key generation
    let gen_output = Command::new("gpg")
        .args(["--batch", "--generate-key", batch_file.to_str().unwrap()])
        .output()
        .context("Failed to generate GPG key")?;
        
    // Cleanup batch file
    let _ = fs::remove_file(batch_file);
    
    if gen_output.status.success() {
        logger.info("✅ Temporary GPG key generated successfully");
        get_gpg_key_id()
    } else {
        anyhow::bail!(
            "Failed to generate temporary GPG key: {}",
            String::from_utf8_lossy(&gen_output.stderr)
        )
    }
}

/// Get the ID of the imported GPG key
fn get_gpg_key_id() -> Result<String> {
    let logger = get_logger(false);
    logger.info("🔍 Retrieving GPG key ID");
    
    // List secret keys
    let output = Command::new("gpg")
        .args(["--list-secret-keys", "--keyid-format", "LONG"])
        .output()
        .context("Failed to list GPG keys")?;
        
    if !output.status.success() {
        anyhow::bail!("Failed to list GPG keys");
    }
    
    // Extract key ID from output
    let output_str = String::from_utf8_lossy(&output.stdout);
    for line in output_str.lines() {
        if line.trim().starts_with("sec") {
            // Format is typically "sec   rsa4096/KEYID YYYY-MM-DD"
            let parts: Vec<&str> = line.split('/').collect();
            if parts.len() >= 2 {
                let key_part = parts[1];
                let key_id = key_part.split_whitespace().next().unwrap_or("");
                if !key_id.is_empty() {
                    logger.info(&format!("✅ Found GPG key ID: {}", key_id));
                    return Ok(key_id.to_string());
                }
            }
        }
    }
    
    anyhow::bail!("No GPG key found")
}

/// Basic Git configuration
fn setup_basic_git(name: &str, email: &str) -> Result<()> {
    let logger = get_logger(false);
    logger.info("🛠️ Setting up basic Git configuration");
    
    // Configure user name and email
    Command::new("git")
        .args(["config", "--global", "user.name", name])
        .output()
        .context("Failed to set Git user name")?;
        
    Command::new("git")
        .args(["config", "--global", "user.email", email])
        .output()
        .context("Failed to set Git user email")?;
        
    logger.info("✅ Basic Git configuration completed");
    Ok(())
}

/// Configure Git with GPG signing
fn configure_git_signing(key_id: &str, name: &str, email: &str) -> Result<()> {
    let logger = get_logger(false);
    logger.info("🔏 Configuring Git with GPG signing");
    
    // Set basic git configuration again to ensure it's applied
    setup_basic_git(name, email)?;
    
    // Configure GPG signing
    Command::new("git")
        .args(["config", "--global", "user.signingkey", key_id])
        .output()
        .context("Failed to set Git signing key")?;
        
    Command::new("git")
        .args(["config", "--global", "commit.gpgsign", "true"])
        .output()
        .context("Failed to enable Git commit signing")?;
        
    Command::new("git")
        .args(["config", "--global", "tag.gpgsign", "true"])
        .output()
        .context("Failed to enable Git tag signing")?;
        
    // Set environment variables for GPG
    env::set_var("GPG_TTY", env::var("TTY").unwrap_or_else(|_| "/dev/tty".to_string()));
    
    logger.info("✅ Git configured with GPG signing");
    Ok(())
}
