//! GitHub release creation script for create-release action
//! Used by: ./.github/actions/create-release/action.yml
//! Purpose: Creates GitHub release with proper tag handling

use anyhow::{Context, Result};
use github_workflow_scripts::{get_logger, init, github}; // Now properly imported
use std::env;
use std::process::Command;

// Main function to create a release with proper error handling
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    init();
    let logger = get_logger(false);
    
    logger.info("🚀 Creating GitHub release...");
    
    // Get required parameters from environment variables
    let version = env::var("INPUT_VERSION").context("Missing INPUT_VERSION")?;
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
    
    // Determine GitHub repository
    let github_repository = env::var("GITHUB_REPOSITORY")
        .context("Missing GITHUB_REPOSITORY environment variable")?;
    
    // Set environment variables for GitHub CLI
    env::set_var("GITHUB_TOKEN", &github_token);
    
    // Check if tag has been created and pushed successfully
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
        .args(&["tag", "-l", &version])
        .output()
        .context("Failed to check if tag exists")?;
    
    let tag_exists = String::from_utf8_lossy(&tag_check.stdout)
        .trim()
        .contains(&version);
    
    if !tag_exists {
        logger.warn(&format!("Tag {} not found, creating it now...", version));
        
        // Check if we should sign the tag
        let gpg_check = Command::new("git")
            .args(&["config", "--get", "user.signingkey"])
            .output()
            .context("Failed to check git signing key")?;
        
        let signing_configured = gpg_check.status.success() && 
                                String::from_utf8_lossy(&gpg_check.stdout).trim().len() > 0;
        
        // Create signed or unsigned tag as appropriate
        if signing_configured {
            logger.info("Creating signed tag...");
            let signed_tag = Command::new("git")
                .args(&["tag", "-s", &version, &release_sha, "-m", &format!("Release {}", version)])
                .output()
                .context("Failed to create signed tag")?;
            
            if !signed_tag.status.success() {
                logger.warn(&format!("Failed to create signed tag: {}", 
                    String::from_utf8_lossy(&signed_tag.stderr)));
                    
                // Try without signing as fallback
                logger.info("Falling back to unsigned tag...");
                let unsigned_tag = Command::new("git")
                    .args(&["tag", "-a", &version, &release_sha, "-m", &format!("Release {}", version)])
                    .output()
                    .context("Failed to create unsigned tag")?;
                    
                if !unsigned_tag.status.success() {
                    return Err(anyhow::anyhow!(
                        "Failed to create unsigned tag: {}", 
                        String::from_utf8_lossy(&unsigned_tag.stderr)
                    ));
                }
            }
        } else {
            logger.info("Creating unsigned tag...");
            let unsigned_tag = Command::new("git")
                .args(&["tag", "-a", &version, &release_sha, "-m", &format!("Release {}", version)])
                .output()
                .context("Failed to create unsigned tag")?;
                
            if !unsigned_tag.status.success() {
                return Err(anyhow::anyhow!(
                    "Failed to create unsigned tag: {}", 
                    String::from_utf8_lossy(&unsigned_tag.stderr)
                ));
            }
        }
        
        // Push the tag
        logger.info("Pushing tag to remote...");
        let push_result = Command::new("git")
            .args(&["push", "origin", &format!("refs/tags/{}", version)])
            .output()
            .context("Failed to push tag")?;
            
        if !push_result.status.success() {
            return Err(anyhow::anyhow!(
                "Failed to push tag: {}", 
                String::from_utf8_lossy(&push_result.stderr)
            ));
        }
    } else {
        logger.info(&format!("Tag {} already exists, using existing tag", version));
    }
    
    // Create GitHub release using gh CLI
    logger.info("Creating GitHub release...");
    
    // Build command arguments
    let mut args = vec!["release", "create", &version, "--target", &release_sha];
    
    // Add title
    args.push("--title");
    args.push(&version);
    
    // Add optional flags
    if prerelease {
        args.push("--prerelease");
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
        return Err(anyhow::anyhow!(
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
    
    Ok(())
}
