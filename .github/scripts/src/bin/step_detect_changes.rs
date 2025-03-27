//! Change detection for GitHub Actions
//! Used by: ./.github/actions/detect-changes/action.yml
//! Purpose: Detects changes in relevant Docker-related paths

use anyhow::{Context, Result};
use github_workflow_scripts::{get_logger, init};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

// Struct to hold common dependencies and state
struct ChangeDetector {
    logger: Box<dyn github_workflow_scripts::Logger>,
    before_commit: String,
    current_commit: String,
}

impl ChangeDetector {
    // Initialize with required inputs
    fn new() -> Result<Self> {
        let logger = get_logger(false);
        
        // Get required environment variables
        let before_commit = env::var("GITHUB_EVENT_BEFORE")
            .unwrap_or_else(|_| {
                logger.warn("GITHUB_EVENT_BEFORE not set, using default");
                "HEAD^".to_string()
            });
            
        let current_commit = env::var("GITHUB_SHA")
            .unwrap_or_else(|_| {
                logger.warn("GITHUB_SHA not set, using default");
                "HEAD".to_string()
            });
            
        Ok(Self {
            logger,
            before_commit,
            current_commit,
        })
    }

    // Get changed files between commits
    fn get_changed_files(&self) -> Result<Vec<String>> {
        self.logger.info(&format!("📄 Getting changed files between {} and {}", 
                          self.before_commit, self.current_commit));
        
        // Use git diff to get changed files
        let output = Command::new("git")
            .args(&["diff", "--name-only", &self.before_commit, &self.current_commit])
            .output()
            .context("Failed to execute git diff command")?;
            
        if !output.status.success() {
            self.logger.warn(&format!("Git diff command failed: {}", 
                            String::from_utf8_lossy(&output.stderr)));
                            
            // Try with a simpler approach if the specific commits failed
            let fallback_output = Command::new("git")
                .args(&["diff", "--name-only", "HEAD^", "HEAD"])
                .output()
                .context("Failed to execute fallback git diff command")?;
                
            if !fallback_output.status.success() {
                anyhow::bail!(
                    "All git diff commands failed: {}",
                    String::from_utf8_lossy(&fallback_output.stderr)
                );
            }
            
            return Ok(String::from_utf8_lossy(&fallback_output.stdout)
                .lines()
                .map(|s| s.to_string())
                .collect());
        }
        
        // Parse the output into a vector of strings
        let changed_files = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|s| s.to_string())
            .collect();
            
        Ok(changed_files)
    }

    // Check if Docker-related files changed
    fn check_docker_changes(&self, files: &[String]) -> bool {
        self.logger.info("🔍 Checking for Docker-related changes...");
        
        // Check if any file in the distributions/dockerhub/ directory changed
        let has_changes = files.iter().any(|file| file.contains("distributions/dockerhub/"));
        
        if has_changes {
            self.logger.info("✅ Docker-related changes detected");
        } else {
            self.logger.info("❌ No Docker-related changes detected");
        }
        
        has_changes
    }

    // Check if Dockerfile itself changed
    fn check_dockerfile_changes(&self, files: &[String]) -> bool {
        self.logger.info("🔍 Checking for Dockerfile changes...");
        
        // Check if the Dockerfile changed
        let has_changes = files.iter().any(|file| file.contains("distributions/dockerhub/Dockerfile"));
        
        if has_changes {
            self.logger.info("✅ Dockerfile changes detected");
        } else {
            self.logger.info("❌ No Dockerfile changes detected");
        }
        
        has_changes
    }

    // Set GitHub outputs directly using GITHUB_OUTPUT environment file
    fn set_outputs(&self, docker_changed: bool, dockerfile_changed: bool) -> Result<()> {
        self.logger.info("📤 Setting GitHub outputs...");
        
        // Get GITHUB_OUTPUT environment file path
        let github_output = env::var("GITHUB_OUTPUT").unwrap_or_else(|_| {
            self.logger.warn("GITHUB_OUTPUT not set, outputs will not be saved");
            "/dev/null".to_string()
        });
        
        // Try to open the file for appending
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&github_output)
            .context(format!("Failed to open GITHUB_OUTPUT file: {}", github_output))?;
            
        // Write outputs in GitHub Actions format
        writeln!(file, "docker_changed={}", docker_changed)
            .context("Failed to write docker_changed output")?;
            
        writeln!(file, "dockerfile_changed={}", dockerfile_changed)
            .context("Failed to write dockerfile_changed output")?;
            
        self.logger.info(&format!("✅ Set outputs: docker_changed={}, dockerfile_changed={}", 
                          docker_changed, dockerfile_changed));
                          
        Ok(())
    }

    // Run the detection process
    async fn run(&self) -> Result<()> {
        self.logger.info("🔎 Starting change detection...");
        
        // Get changed files
        let changed_files = self.get_changed_files()?;
        
        // Log the number of changed files
        self.logger.info(&format!("Found {} changed files", changed_files.len()));
        
        // Check for Docker changes
        let docker_changed = self.check_docker_changes(&changed_files);
        
        // Check for Dockerfile changes
        let dockerfile_changed = self.check_dockerfile_changes(&changed_files);
        
        // Set outputs for GitHub Actions
        self.set_outputs(docker_changed, dockerfile_changed)?;
        
        self.logger.info("✅ Change detection completed successfully");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    init();
    let detector = ChangeDetector::new()?;
    detector.run().await
}
