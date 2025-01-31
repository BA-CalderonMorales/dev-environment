use anyhow::{Context, Result};
use github_workflow_scripts::{get_logger, init_logging};
use std::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();
    let logger = get_logger(false);

    // Get image tag from environment
    let image = std::env::var("INPUT_IMAGE").context("INPUT_IMAGE not set")?;
    logger.info(&format!("Checking for image: {}", image));

    // Try local image first
    let local_check = Command::new("docker")
        .args(["image", "inspect", &image])
        .output()
        .context("Failed to check local image")?;

    if local_check.status.success() {
        logger.info("Found image locally");
        println!("::set-output name=image_exists::true");
        println!("::set-output name=image_location::local");
        return Ok(());
    }

    // Try pulling from registry
    logger.info("Image not found locally, attempting to pull...");
    let pull_result = Command::new("docker")
        .args(["pull", &image])
        .output()
        .context("Failed to pull image")?;

    if pull_result.status.success() {
        logger.info("Successfully pulled image from registry");
        println!("::set-output name=image_exists::true");
        println!("::set-output name=image_location::registry");
        Ok(())
    } else {
        logger.info("Failed to find or pull image");
        println!("::set-output name=image_exists::false");
        println!("::error::Docker image '{}' not found locally or in registry", image);
        std::process::exit(1);
    }
}
