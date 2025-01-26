use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::{anyhow, bail, Context, Result};
use crate::Logger;

pub struct DistributionTest<'a> {
    logger: &'a dyn Logger,
}

impl<'a> DistributionTest<'a> {
    pub fn new(logger: &'a dyn Logger) -> Self {
        Self { logger }
    }

    pub async fn test_distribution_creation(&self, dockerfile: &Path, repo: &str) -> Result<()> {
        self.logger.debug(&format!("Testing distribution creation with repo: {}", repo));
        
        // Get the project root directory (one level up from e2e)
        let project_root = std::env::current_dir()?
            .parent()
            .ok_or_else(|| anyhow!("Could not find project root"))?
            .to_path_buf();
            
        // Change to project root for Docker build context
        std::env::set_current_dir(project_root)
            .context("Failed to change to project root directory")?;
        
        let output = Command::new("docker")
            .args([
                "build",
                "-t", &format!("{}:latest", repo),
                "-f", dockerfile.to_str().unwrap(),
                "."
            ])
            .output()
            .context("Failed to build Docker image")?;

        if !output.status.success() {
            bail!("Docker build failed: {}", String::from_utf8_lossy(&output.stderr));
        }

        self.logger.debug("Distribution creation successful");
        Ok(())
    }

    pub async fn test_dockerhub_install(&self, image: &str) -> Result<()> {
        self.logger.debug(&format!("Testing DockerHub installation for image: {}", image));
        
        let output = Command::new("docker")
            .args(["pull", image])
            .output()
            .context("Failed to pull Docker image")?;

        if !output.status.success() {
            bail!("Failed to pull image: {}", String::from_utf8_lossy(&output.stderr));
        }

        self.logger.debug("DockerHub installation successful");
        Ok(())
    }

    pub async fn test_direct_download(&self, version: &str) -> Result<()> {
        self.logger.debug(&format!("Testing direct download for version: {}", version));
        
        let download_url = match version {
            "latest" => "https://github.com/BA-CalderonMorales/dev-environment/releases/latest/download/dev-environment-latest.tar",
            "beta" => "https://github.com/BA-CalderonMorales/dev-environment/releases/download/beta/dev-environment-beta.tar",
            _ => bail!("Invalid version specified")
        };

        let output = Command::new("curl")
            .args(["-L", "-o", &format!("dev-environment-{}.tar", version), download_url])
            .output()
            .context("Failed to download image")?;

        if !output.status.success() {
            bail!("Download failed: {}", String::from_utf8_lossy(&output.stderr));
        }

        self.logger.debug("Direct download successful");
        Ok(())
    }
}