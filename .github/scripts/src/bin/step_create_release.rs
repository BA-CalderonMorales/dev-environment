//! GitHub release creation script for create-release action
//! Used by: ./.github/actions/create-release/action.yml
//! Purpose: Creates GitHub release with assets

use anyhow::{Context, Result};
use chrono::Utc;
use github_workflow_scripts::{get_logger, init_logging};
use octocrab::Octocrab;
use serde_json::json;
use std::{env, fs};

#[derive(Debug)]
struct ReleaseInfo {
    tag_name: String,
    name: String,
    prerelease: bool,
    body: String,
    files: Vec<String>,
}

impl ReleaseInfo {
    async fn new() -> Result<Self> {
        let version = env::var("VALIDATED_VERSION")
            .context("VALIDATED_VERSION not set")?;
        let prerelease = env::var("INPUT_PRERELEASE")
            .context("INPUT_PRERELEASE not set")?
            .parse::<bool>()?;
        let release_type = if prerelease { "Beta" } else { "Stable" };
        
        // Read checksum file
        let checksum = fs::read_to_string("checksum.txt")
            .context("Failed to read checksum.txt")?;

        let body = format!(
            r#"## Release Notes
            
### Distributions
This release includes:
- Complete development environment configuration
- Docker setup files for containerized usage
- Direct deployment scripts

### Installation
