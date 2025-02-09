use std::path::Path;
use std::process::Command;
use anyhow::{anyhow, bail, Context, Result};
use crate::logging::Logger;

pub struct DistributionTest<'a> {
    logger: &'a dyn Logger,
}

impl<'a> DistributionTest<'a> {
    pub fn new(logger: &'a dyn Logger) -> Self {
        Self { logger }
    }

    pub async fn test_distribution_creation(&self, dockerfile: &Path, repo: &str) -> Result<()> {
        self.logger.debug(&format!(
            "Testing distribution creation with repo: {} and dockerfile: {}",
            repo,
            dockerfile.display()
        ));
        
        if !dockerfile.exists() {
            bail!("Dockerfile not found at: {}", dockerfile.display());
        }

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
        
        // Validate image format
        if !image.contains('/') || !image.contains(':') {
            bail!("Invalid image format. Expected format: repository/image:tag");
        }

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

    pub async fn test_direct_download(&self, tarfile: &Path, checksum: &Path) -> Result<()> {
        self.logger.debug(&format!(
            "Testing direct download verification for file: {} with checksum: {}", 
            tarfile.display(),
            checksum.display()
        ));
        
        if !tarfile.exists() {
            bail!("Tar file not found at: {}", tarfile.display());
        }

        if !checksum.exists() {  // Fixed extra parenthesis here
            bail!("Checksum file not found at: {}", checksum.display());
        }

        // Verify checksum
        let checksum_output = Command::new("sha256sum")
            .arg("--check")
            .arg(checksum)
            .current_dir(tarfile.parent().unwrap())
            .output()
            .context("Failed to verify checksum")?;

        if !checksum_output.status.success() {
            bail!("Checksum verification failed: {}", 
                String::from_utf8_lossy(&checksum_output.stderr));
        }

        self.logger.debug("Direct download verification successful");
        Ok(())
    }
}