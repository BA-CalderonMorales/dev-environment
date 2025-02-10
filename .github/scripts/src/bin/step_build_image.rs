use anyhow::{Context, Result};
use github_workflow_scripts::{get_logger, init};
use std::{env, process::Command};

#[tokio::main]
async fn main() -> Result<()> {
    init();
    let logger = get_logger(false);

    // Get environment variables
    let tag = env::var("INPUT_TAG").context("INPUT_TAG not set")?;
    let environment = env::var("INPUT_ENVIRONMENT").context("INPUT_ENVIRONMENT not set")?;
    let base_image = env::var("DOCKER_IMAGE").context("DOCKER_IMAGE not set")?;

    logger.info(&format!("Building image for environment: {}", environment));

    // Determine tags based on branch/environment
    let version_tag = match environment.as_str() {
        "main" => format!("{}:latest", base_image),
        "beta" => format!("{}:beta", base_image),
        "develop" => format!("{}:dev", base_image),
        _ => format!("{}:{}", base_image, tag)
    };

    // Store formatted strings to extend their lifetime
    let stable_tag = format!("{}:stable", base_image);

    // Prepare build context
    std::fs::create_dir_all("distributions/dockerhub")
        .context("Failed to create build directory")?;
    
    Command::new("cp")
        .args(["-r", "startup", "distributions/dockerhub/"])
        .status()
        .context("Failed to copy build context")?;

    // Build image with appropriate tags
    let mut build_args = vec![
        "build",
        "--no-cache",
        "-t", &version_tag,
    ];

    // Add additional tag if main branch
    if environment == "main" {
        build_args.extend(&["-t", &stable_tag]);
    }

    Command::new("docker")
        .args(&build_args)
        .current_dir("distributions/dockerhub")
        .status()
        .context("Failed to build image")?;

    // Push images for protected branches
    if ["main", "beta", "develop"].contains(&environment.as_str()) {
        logger.info("Pushing image to DockerHub");
        Command::new("docker")
            .args(["push", &version_tag])
            .status()
            .context("Failed to push image")?;

        if environment == "main" {
            Command::new("docker")
                .args(["push", &stable_tag])
                .status()
                .context("Failed to push stable tag")?;
        }
    }

    // Cleanup
    std::fs::remove_dir_all("distributions/dockerhub/startup")
        .context("Failed to cleanup build context")?;

    println!("::set-output name=image_tag::{}", version_tag);
    Ok(())
}
