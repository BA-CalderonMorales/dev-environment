use anyhow::{Context, Result};
use github_workflow_scripts::{get_logger, init};
use std::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
    init();
    let logger = get_logger(false);

    // Get environment variables
    let tag = std::env::var("INPUT_TAG").context("INPUT_TAG not set")?;
    let branch = std::env::var("GITHUB_REF").context("GITHUB_REF not set")?;
    let branch = branch.trim_start_matches("refs/heads/");

    logger.info(&format!("Checking image: {}", tag));

    // Check if image exists in DockerHub
    let output = Command::new("docker")
        .args(["manifest", "inspect", &tag])
        .output()
        .context("Failed to execute docker manifest inspect")?;

    if output.status.success() {
        logger.info("Image exists in DockerHub");
        println!("::set-output name=exists::true");
        println!("::set-output name=image_tag::{}", tag);
        Ok(())
    } else {
        // Image doesn't exist, determine base tag
        let base_tag = if branch == "main" || branch == "beta" {
            format!("cmoe640/dev-environment:{}", branch)
        } else {
            "cmoe640/dev-environment:develop".to_string()
        };

        logger.info(&format!("Image not found, will use base: {}", base_tag));
        println!("::set-output name=exists::false");
        println!("::set-output name=base_tag::{}", base_tag);
        Ok(())
    }
}
