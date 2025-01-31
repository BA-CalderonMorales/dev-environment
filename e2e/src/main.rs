//! End-to-End Testing Framework
//! 
//! Main orchestration module for the development environment testing framework.
//! Provides high-level test workflows for both creators and users.

mod cli;
mod distribution;
mod ide;
mod logging;
mod startup;
mod test_runner;
mod common;

use anyhow::Result;
use structopt::StructOpt;

use cli::Cli;
use logging::init_logging;
use test_runner::{run_creator_workflow, run_user_workflow};

/// Main entry point for the testing framework
#[tokio::main]
async fn main() -> Result<()> {
    let logger = init_logging();
    logger.info("🚀 Starting E2E tests...");

    let cli = Cli::from_args();
    let success = match &cli {
        Cli::Creator { dockerfile, dockerhub_repo, download_url } => {
            run_creator_workflow(dockerfile, dockerhub_repo, download_url).await?
        },
        Cli::User { dockerhub_image, download_url } => {
            run_user_workflow(dockerhub_image, download_url).await?
        },
    };

    std::process::exit(if success { 0 } else { 1 });
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup() -> Result<()> {
        // Setup test environment
        Ok(())
    }

    #[tokio::test]
    async fn test_creator_workflow() -> Result<()> {
        setup().await?;
        // Your existing test implementation
        Ok(())
    }

    #[tokio::test]
    async fn test_user_workflow() -> Result<()> {
        setup().await?;
        // Your existing test implementation
        Ok(())
    }
}