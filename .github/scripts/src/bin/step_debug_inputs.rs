//! Debug inputs script for create-release action
//! Used by: ./.github/actions/create-release/action.yml
//! Purpose: Validates and displays input parameters

use anyhow::Result;
use github_workflow_scripts::{get_logger, init};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    init();
    let logger = get_logger(false);

    logger.info("=== Input Validation ===");

    // Get required environment variables
    let version = env::var("INPUT_VERSION").unwrap_or_else(|_| "not set".to_string());
    let prerelease = env::var("INPUT_PRERELEASE").unwrap_or_else(|_| "not set".to_string());
    let github_ref = env::var("GITHUB_REF").unwrap_or_else(|_| "not set".to_string());
    let github_sha = env::var("GITHUB_SHA").unwrap_or_else(|_| "not set".to_string());

    // Log input values
    logger.info(&format!("Version Input: '{}'", version));
    logger.info(&format!("Prerelease Flag: '{}'", prerelease));
    logger.info(&format!("GitHub Ref: '{}'", github_ref));
    logger.info(&format!("GitHub SHA: '{}'", github_sha));
    logger.info("======================");

    Ok(())
}
