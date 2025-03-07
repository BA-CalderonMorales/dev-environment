//! GitHub release creation script for create-release action
//! Used by: ./.github/actions/create-release/action.yml
//! Purpose: Creates GitHub release with proper tag handling and semantic versioning

use anyhow::{Context, Result, anyhow};
use github_workflow_scripts::{get_logger, init, github, Logger};
use std::env;
use std::process::Command;
use std::fs;

// Main function to create a release with proper error handling
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging and configuration
    init();
    let logger = get_logger(false);
    
    logger.info("🚀 Creating GitHub release...");
    
    // Get required parameters from environment variables
    let version = env::var("INPUT_VERSION").context("Missing INPUT_VERSION")?;
    
    // Validate version is not empty
    if version.trim().is_empty() {
        return Err(anyhow!("Empty version provided. Please specify a valid version."));
    }
    
    let release_sha = env::var("INPUT_RELEASE_SHA").context("Missing INPUT_RELEASE_SHA")?;
    let github_token = env::var("INPUT_GITHUB_TOKEN").context("Missing INPUT_GITHUB_TOKEN")?;
    
    // Optional parameters with defaults
    let prerelease = env::var("INPUT_PRERELEASE")
        .map(|v| v == "true")
        .unwrap_or(false);
    
    let generate_notes = env::var("INPUT_GENERATE_RELEASE_NOTES")
        .map(|v| v == "true")
        .unwrap_or(true);
    
    let draft = env::var("INPUT_DRAFT")
        .map(|v| v == "true")
        .unwrap_or(false);
    
    let allow_unsigned = env::var("INPUT_ALLOW_UNSIGNED")
        .map(|v| v == "true")
        .unwrap_or(true);
    
    // No need to normalize for our custom version format - use as-is
    logger.info(&format!("Using version: {}", version));
    
    // Set environment variables for GitHub CLI
    env::set_var("GITHUB_TOKEN", &github_token);
    let github_repository = env::var("GITHUB_REPOSITORY")
        .context("Missing GITHUB_REPOSITORY environment variable")?;
    
    // Process flow: Check tag → Create tag → Push tag → Create release
    if !tag_exists(&version, logger.as_ref())? {
        // Try to create and push tag
        create_and_push_tag(&version, &release_sha, allow_unsigned, logger.as_ref())?;
    }
    
    // Create GitHub release
    create_github_release(
        &version, 
        &release_sha, 
        prerelease || is_beta_version(&version), 
        draft, 
        generate_notes,
        &github_repository,
        logger.as_ref()
    )?;
    
    Ok(())
}

/// Check if a tag already exists
fn tag_exists(version: &str, logger: &dyn Logger) -> Result<bool> {
    logger.info(&format!("Verifying tag {} exists...", version));
    
    // Fetch latest tags to ensure we see the new one
    let fetch_result = Command::new("git")
        .args(&["fetch", "--tags"])
        .output();
    
    if let Err(e) = fetch_result {
        logger.warn(&format!("Failed to fetch tags: {}", e));
    }
    
    // Verify our tag exists
    let tag_check = Command::new("git")
        .args(&["tag", "-l", version])
        .output()
        .context("Failed to check if tag exists")?;
    
    let tag_exists = String::from_utf8_lossy(&tag_check.stdout)
        .trim()
        .contains(version);
    
    if tag_exists {
        logger.info(&format!("Tag {} already exists, using existing tag", version));
    } else {
        logger.warn(&format!("Tag {} not found, creating it now...", version));
    }
    
    Ok(tag_exists)
}

/// Determine if a version is a beta release based on its prefix
fn is_beta_version(version: &str) -> bool {
    version.starts_with("beta-")
}

/// Create and push a tag to the remote repository
fn create_and_push_tag(version: &str, commit_sha: &str, allow_unsigned: bool, logger: &dyn Logger) -> Result<()> {
    // Configure GPG for batch mode operation
    configure_gpg_for_batch_mode(logger)?;
    
    // First, try to create a signed tag
    logger.info("Creating signed tag...");
    let signing_result = create_signed_tag(version, commit_sha);
    
    // If signing fails and unsigned tags are allowed, create an unsigned tag
    if let Err(e) = signing_result {
        logger.warn(&format!("Failed to create signed tag: {}", e));
        
        if allow_unsigned {
            logger.info("Falling back to unsigned tag...");
            
            // Properly disable GPG signing for this operation
            disable_git_signing(logger)?;
            
            // Create truly unsigned tag
            let unsigned_result = create_unsigned_tag(version, commit_sha);
            if let Err(e) = unsigned_result {
                return Err(anyhow!("Failed to create unsigned tag: {}", e));
            }
        } else {
            return Err(anyhow!("Failed to create signed tag and unsigned tags not allowed"));
        }
    }
    
    // Push the tag to remote
    logger.info("Pushing tag to remote...");
    let push_result = Command::new("git")
        .args(&["push", "origin", &format!("refs/tags/{}", version)])
        .output()
        .context("Failed to push tag")?;
        
    if !push_result.status.success() {
        return Err(anyhow!(
            "Failed to push tag: {}", 
            String::from_utf8_lossy(&push_result.stderr)
        ));
    }
    
    logger.info(&format!("✅ Successfully pushed tag {}", version));
    Ok(())
}

/// Configure GPG for batch mode operation
fn configure_gpg_for_batch_mode(logger: &dyn Logger) -> Result<()> {
    logger.info("Configuring GPG for batch mode operation");
    
    // Create necessary GPG configuration directories
    let home_dir = env::var("HOME").unwrap_or_else(|_| "/home/runner".to_string());
    let gpg_dir = format!("{}/.gnupg", home_dir);
    fs::create_dir_all(&gpg_dir).context("Failed to create GPG directory")?;
    
    // Create gpg.conf with appropriate settings
    let gpg_conf_path = format!("{}/gpg.conf", gpg_dir);
    let gpg_conf_content = "allow-loopback-pinentry\npinentry-mode loopback\nno-tty\nbatch\n";
    fs::write(&gpg_conf_path, gpg_conf_content)
        .context("Failed to write GPG configuration")?;
    
    // Create gpg-agent.conf
    let agent_conf_path = format!("{}/gpg-agent.conf", gpg_dir);
    let agent_conf_content = "allow-loopback-pinentry\nallow-preset-passphrase\n";
    fs::write(&agent_conf_path, agent_conf_content)
        .context("Failed to write GPG agent configuration")?;
    
    // Set proper permissions
    Command::new("chmod")
        .args(&["700", &gpg_dir])
        .output()
        .ok();
    
    Command::new("chmod")
        .args(&["600", &gpg_conf_path, &agent_conf_path])
        .output()
        .ok();
    
    // Set environment variables for GPG
    env::set_var("GPG_TTY", "");
    
    // Reload GPG agent
    Command::new("gpg-connect-agent")
        .args(&["reloadagent", "/bye"])
        .output()
        .ok();
    
    logger.info("✅ GPG configured for batch mode");
    Ok(())
}

/// Disable Git GPG signing temporarily
fn disable_git_signing(logger: &dyn Logger) -> Result<()> {
    logger.info("Temporarily disabling Git signing");
    
    // Unset signing key
    Command::new("git")
        .args(&["config", "--local", "--unset", "user.signingkey"])
        .output()
        .ok();
    
    // Disable commit signing
    Command::new("git")
        .args(&["config", "--local", "commit.gpgsign", "false"])
        .output()
        .context("Failed to disable commit signing")?;
    
    // Disable tag signing
    Command::new("git")
        .args(&["config", "--local", "tag.gpgsign", "false"])
        .output()
        .context("Failed to disable tag signing")?;
    
    logger.info("✅ Git signing disabled");
    Ok(())
}

/// Create a signed Git tag
fn create_signed_tag(version: &str, commit_sha: &str) -> Result<()> {
    // Create the tag message based on the version type
    let message = if is_beta_version(version) {
        format!("Beta Release {}", version)
    } else {
        format!("Stable Release {}", version)
    };

    let output = Command::new("git")
        .args(&["tag", "-s", version, commit_sha, "-m", &message])
        .output()
        .context("Failed to execute git tag command")?;
    
    if !output.status.success() {
        return Err(anyhow!(
            "{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    
    Ok(())
}

/// Create an unsigned Git tag
fn create_unsigned_tag(version: &str, commit_sha: &str) -> Result<()> {
    // Create the tag message based on the version type
    let message = if is_beta_version(version) {
        format!("Beta Release {} (unsigned)", version)
    } else {
        format!("Stable Release {} (unsigned)", version)  // Fixed: Added missing argument
    };

    let output = Command::new("git")
        .args(&["tag", "-a", version, commit_sha, "-m", &message])
        .output()
        .context("Failed to execute git tag command")?;
    
    if !output.status.success() {
        return Err(anyhow!(
            "{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    
    Ok(())
}

/// Create GitHub release using the GitHub CLI
fn create_github_release(
    version: &str,
    release_sha: &str,
    prerelease: bool,
    draft: bool,
    generate_notes: bool,
    github_repository: &str,
    logger: &dyn Logger
) -> Result<()> {
    logger.info("Creating GitHub release...");
    
    // Build command arguments
    let mut args = vec!["release", "create", version, "--target", release_sha];
    
    // Create appropriate title based on version type
    let title = if is_beta_version(version) {
        format!("Beta Release {}", version)
    } else {
        format!("Stable Release {}", version)
    };
    
    args.push("--title");
    args.push(&title);
    
    // Add optional flags
    if prerelease {
        args.push("--prerelease");
        logger.info("Creating as pre-release based on version format or input flag");
    }
    
    if draft {
        args.push("--draft");
    }
    
    if generate_notes {
        args.push("--generate-notes");
    }
    
    // Execute release command
    let release_result = Command::new("gh")
        .args(&args)
        .output()
        .context("Failed to execute GitHub CLI")?;
        
    if !release_result.status.success() {
        return Err(anyhow!(
            "Failed to create GitHub release: {}", 
            String::from_utf8_lossy(&release_result.stderr)
        ));
    }
    
    // Output release URL for action output
    let release_url = format!(
        "https://github.com/{}/releases/tag/{}", 
        github_repository,
        version
    );
    
    github::set_output("release_url", &release_url);
    logger.info(&format!("✅ Release created successfully: {}", release_url));
    
    // Add explanation of version scheme
    logger.info("Version Scheme Explanation:");
    logger.info("- a: proud version: bump when we're proud of a release");
    logger.info("- b: default version: just a normal/okay release");
    logger.info("- c: bump when fixing things too embarrassing to admit");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_beta_version() {
        assert_eq!(is_beta_version("beta-v0.0.1"), true);
        assert_eq!(is_beta_version("stable-v0.0.1"), false);
        assert_eq!(is_beta_version("v0.0.1"), false);
    }
}
