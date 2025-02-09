//! Version validation script for create-release action
//! Used by: ./.github/actions/create-release/action.yml
//! Purpose: Validates and normalizes version strings for releases

use anyhow::{Context, Result};
use github_workflow_scripts::{get_logger, init_logging};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct VersionInfo {
    raw: String,
    normalized: String,
    is_valid: bool,
    is_prerelease: bool,
}

impl VersionInfo {
    fn new(version: String) -> Self {
        Self {
            raw: version,
            normalized: String::new(),
            is_valid: false,
            is_prerelease: false,
        }
    }

    fn validate(&mut self) -> Result<()> {
        let logger = get_logger(false);
        
        // Remove 'v' prefix if present
        let mut normalized = self.raw.trim().to_string();
        normalized = normalized.trim_start_matches('v').to_string();
        
        // Add 'v' prefix back
        normalized = format!("v{}", normalized);
        
        // Validate against semver pattern
        let pattern = Regex::new(
            r"^v(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(-beta\.[0-9]+)?$"
        )?;
        
        self.is_valid = pattern.is_match(&normalized);
        self.normalized = normalized.clone();
        self.is_prerelease = normalized.contains("-beta.");

        if self.is_valid {
            logger.info(&format!("✅ Version '{}' is valid", self.normalized));
        } else {
            logger.warn(&format!("❌ Invalid version format: {}", self.raw));
            logger.warn("Version must match pattern: v1.2.3 or v1.2.3-beta.1");
        }

        Ok(())
    }

    fn to_env_outputs(&self) -> Vec<(String, String)> {
        vec![
            ("VALIDATED_VERSION".into(), self.normalized.clone()),
            ("VERSION_VALID".into(), self.is_valid.to_string()),
            ("IS_PRERELEASE".into(), self.is_prerelease.to_string()),
        ]
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();
    let logger = get_logger(false);

    logger.info("🔍 Starting version validation...");

    // Get version from environment
    let version = env::var("INPUT_VERSION")
        .context("INPUT_VERSION environment variable not set")?;

    logger.debug(&format!("Raw version input: {}", version));

    if version.is_empty() {
        logger.warn("❌ No version provided");
        anyhow::bail!("Version input is empty");
    }

    // Validate version
    let mut version_info = VersionInfo::new(version);
    version_info.validate()?;

    if !version_info.is_valid {
        anyhow::bail!("Invalid version format");
    }

    // Set GitHub environment outputs
    for (key, value) in version_info.to_env_outputs() {
        println!("::set-output name={}::{}", key, value);
    }

    logger.info("✅ Version validation completed successfully");
    Ok(())
}
