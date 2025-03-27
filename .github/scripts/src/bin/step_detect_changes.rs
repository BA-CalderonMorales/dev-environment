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

    // Get changed files between commits with multiple fallback strategies
    fn get_changed_files(&self) -> Result<Vec<String>> {
        self.logger.info(&format!("📄 Getting changed files between {} and {}", 
                          self.before_commit, self.current_commit));
        
        // Strategy 1: Try git diff with the provided commits
        let diff_result = self.try_git_diff(&[
            "diff", "--name-only", &self.before_commit, &self.current_commit
        ]);
        
        if let Ok(files) = diff_result {
            self.logger.info(&format!("✅ Found {} changed files using direct diff", files.len()));
            return Ok(files);
        }
        
        // Strategy 2: Try diff with HEAD only (for single commit checkout)
        let head_diff_result = self.try_git_diff(&[
            "diff-tree", "--no-commit-id", "--name-only", "-r", "HEAD"
        ]);
        
        if let Ok(files) = head_diff_result {
            self.logger.info(&format!("✅ Found {} changed files using HEAD diff-tree", files.len()));
            return Ok(files);
        }
        
        // Strategy 3: Try to get the list of all files in tracked Docker directories
        let ls_result = self.try_git_diff(&[
            "ls-files", "distributions/dockerhub/"
        ]);
        
        if let Ok(files) = ls_result {
            self.logger.info(&format!(
                "⚠️ Couldn't determine specific changes - using all {} files in Docker directory", 
                files.len()
            ));
            return Ok(files);
        }
        
        // Strategy 4: Last resort - return Docker path as a fallback
        self.logger.warn("❌ All git commands failed - assuming Docker files changed as a precaution");
        Ok(vec!["distributions/dockerhub/Dockerfile".to_string()])
    }
    
    // Helper method to try different git commands and handle errors
    fn try_git_diff(&self, args: &[&str]) -> Result<Vec<String>> {
        let cmd_str = format!("git {}", args.join(" "));
        self.logger.info(&format!("Trying: {}", cmd_str));
        
        let output = Command::new("git")
            .args(args)
            .output()
            .context(format!("Failed to execute: {}", cmd_str))?;
            
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            self.logger.warn(&format!("Command failed: {} ({})", cmd_str, error));
            anyhow::bail!("Git command failed: {}", error)
        }
        
        // Parse the output into a vector of strings
        Ok(String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect())
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

    // Run the detection process with better error handling
    async fn run(&self) -> Result<()> {
        self.logger.info("🔎 Starting change detection...");
        
        // Get changed files with graceful fallback
        let changed_files = match self.get_changed_files() {
            Ok(files) => files,
            Err(e) => {
                // Log the error but continue with a fallback assumption
                self.logger.warn(&format!("⚠️ Error getting changed files: {}. Assuming Docker files changed as a precaution.", e));
                vec!["distributions/dockerhub/Dockerfile".to_string()]
            }
        };
        
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
