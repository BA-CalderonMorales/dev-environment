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
            if !files.is_empty() {
                self.logger.info(&format!("✅ Found {} changed files using direct diff", files.len()));
                return Ok(files);
            }
            self.logger.warn("Direct diff returned empty result, trying alternative methods");
        }
        
        // Strategy 2: Try diff with HEAD only (for single commit checkout)
        let head_diff_result = self.try_git_diff(&[
            "diff-tree", "--no-commit-id", "--name-only", "-r", "HEAD"
        ]);
        
        if let Ok(files) = head_diff_result {
            if !files.is_empty() {
                self.logger.info(&format!("✅ Found {} changed files using HEAD diff-tree", files.len()));
                return Ok(files);
            }
            self.logger.warn("HEAD diff-tree returned empty result, trying alternative methods");
        }
        
        // Strategy 3: Compare current branch with develop branch
        let develop_diff_result = self.try_git_diff(&[
            "diff", "--name-only", "origin/develop...HEAD"
        ]);
        
        if let Ok(files) = develop_diff_result {
            if !files.is_empty() {
                self.logger.info(&format!("✅ Found {} changed files comparing with develop branch", files.len()));
                return Ok(files);
            }
            self.logger.warn("Develop branch comparison returned empty result, trying alternative methods");
        }
        
        // Strategy 4: Get all files in Docker directory as fallback
        let ls_result = self.try_git_diff(&[
            "ls-files", "distributions/dockerhub/"
        ]);
        
        if let Ok(files) = ls_result {
            if !files.is_empty() {
                self.logger.info(&format!(
                    "⚠️ Couldn't determine specific changes - using all {} files in Docker directory", 
                    files.len()
                ));
                return Ok(files);
            }
            self.logger.warn("Failed to list Docker files, using fallback");
        }
        
        // Strategy 5: Last resort - return Docker path as a fallback
        self.logger.warn("❌ All git commands failed - assuming Docker files changed as a precaution");
        
        // Return Docker files as a precaution to ensure build
        Ok(vec![
            "distributions/dockerhub/Dockerfile".to_string(),
            "docker-compose.yml".to_string()
        ])
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
        let files: Vec<String> = String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
            
        // Print the files for debugging
        if !files.is_empty() {
            self.logger.info("Changed files found:");
            for file in &files {
                self.logger.info(&format!(" - {}", file));
            }
        } else {
            self.logger.warn("No changed files found with this method");
        }
        
        Ok(files)
    }

    // Check if Docker-related files changed
    fn check_docker_changes(&self, files: &[String]) -> bool {
        self.logger.info("🔍 Checking for Docker-related changes...");
        
        // Enhanced check with more Docker-related paths
        let docker_paths = [
            "distributions/dockerhub/",
            "distributions/docker",
            "docker-compose",
            "Dockerfile",
            "docker/"
        ];
        
        let has_changes = files.iter().any(|file| {
            docker_paths.iter().any(|path| file.contains(path))
        });
        
        // Check for workflow files that might affect Docker builds
        let workflow_changes = files.iter().any(|file| {
            file.contains(".github/workflows/workflow_distribution.yml") || 
            file.contains(".github/actions/dockerhub-build") ||
            file.contains(".github/actions/setup-docker")
        });
        
        let docker_changed = has_changes || workflow_changes;
        
        if docker_changed {
            self.logger.info("✅ Docker-related changes detected");
            if workflow_changes {
                self.logger.info("💡 Changes include workflow files affecting Docker builds");
            }
        } else {
            self.logger.info("❌ No Docker-related changes detected");
        }
        
        // Force Docker build to true for now to ensure we capture all changes
        // Remove this line after testing confirms it works properly
        let force_build = true;
        if force_build && !docker_changed {
            self.logger.warn("⚠️ Forcing Docker build for safety (temporary measure)");
            return true;
        }
        
        docker_changed
    }

    // Check if Dockerfile itself changed
    fn check_dockerfile_changes(&self, files: &[String]) -> bool {
        self.logger.info("🔍 Checking for Dockerfile changes...");
        
        // Check if the Dockerfile changed
        let has_changes = files.iter().any(|file| 
            file.contains("distributions/dockerhub/Dockerfile") || 
            file.contains("Dockerfile")
        );
        
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
