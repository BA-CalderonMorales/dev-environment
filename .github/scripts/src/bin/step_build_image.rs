use anyhow::{Context, Result};
use github_workflow_scripts::{get_logger, init};
use std::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
    init();
    let logger = get_logger(false);

    // Get environment variables
    let tag = std::env::var("INPUT_TAG").context("INPUT_TAG not set")?;
    let environment = std::env::var("INPUT_ENVIRONMENT").context("INPUT_ENVIRONMENT not set")?;

    logger.info(&format!("Building image: {}", tag));

    // Prepare build context
    std::fs::create_dir_all("distributions/dockerhub")
        .context("Failed to create build directory")?;
    
    Command::new("cp")
        .args(["-r", "startup", "distributions/dockerhub/"])
        .status()
        .context("Failed to copy build context")?;

    // Build image
    let mut build_args = vec![
        "build",
        "--no-cache",
        "-t", &tag,
    ];

    // Add additional tags
    if environment == "develop" {
        build_args.extend(&["-t", "cmoe640/dev-environment:latest"]);
    }

    Command::new("docker")
        .args(&build_args)
        .current_dir("distributions/dockerhub")
        .status()
        .context("Failed to build image")?;

    // Push images based on branch rules
    if ["main", "beta", "develop"].contains(&environment.as_str()) {
        logger.info("Pushing image to DockerHub");
        Command::new("docker")
            .args(["push", &tag])
            .status()
            .context("Failed to push image")?;

        if environment == "develop" {
            Command::new("docker")
                .args(["push", "cmoe640/dev-environment:latest"])
                .status()
                .context("Failed to push latest tag")?;
        }
    }

    // Cleanup
    std::fs::remove_dir_all("distributions/dockerhub/startup")
        .context("Failed to cleanup build context")?;

    println!("::set-output name=image_tag::{}", tag);
    Ok(())
}
