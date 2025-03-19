use anyhow::{Context, Result};
use github_workflow_scripts::{get_logger, init};
use std::{env, process::Command};
use regex::Regex;
use std::str::FromStr;

/// VersionDeterminer manages the version numbering logic for releases
struct VersionDeterminer {
    logger: Box<dyn github_workflow_scripts::Logger>,
    source_branch: String,
    default_version: String,
}

impl VersionDeterminer {
    /// Create a new VersionDeterminer with inputs from environment
    fn new() -> Result<Self> {
        let logger = get_logger(true); // Enable verbose logging for debugging
        
        // Get the source branch for versioning with better error handling
        let source_branch = env::var("INPUT_SOURCE_BRANCH")
            .unwrap_or_else(|_| {
                logger.warn("INPUT_SOURCE_BRANCH environment variable not set, defaulting to 'beta'");
                "beta".to_string()
            });
            
        // Get the default version in case no tags exist
        let default_version = env::var("INPUT_INITIAL_VERSION")
            .unwrap_or_else(|_| {
                logger.warn("INPUT_INITIAL_VERSION environment variable not set, defaulting to 'beta-v0.0.1'");
                "beta-v0.0.1".to_string()
            });
        
        logger.info(&format!("Initializing VersionDeterminer: branch='{}', default='{}'", 
            &source_branch, &default_version));
        
        Ok(Self {
            logger,
            source_branch,
            default_version,
        })
    }
    
    /// Run a git command and return its output
    fn run_git_command(&self, args: &[&str]) -> Result<String> {
        self.logger.debug(&format!("Running git command: git {}", args.join(" ")));
        
        let output = Command::new("git")
            .args(args)
            .output()
            .context("Failed to execute git command")?;
            
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            self.logger.warn(&format!("Git command failed: {}", stderr));
            anyhow::bail!("Git command failed: {}", stderr);
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(stdout)
    }
    
    /// Fetch all tags from the repository
    fn fetch_tags(&self) -> Result<()> {
        self.logger.info("Fetching all git tags...");
        self.run_git_command(&["fetch", "--tags", "--force"])?;
        Ok(())
    }
    
    /// Find the latest tag with a specific prefix
    fn find_latest_tag_with_prefix(&self, prefix: &str) -> Result<Option<String>> {
        self.logger.info(&format!("Looking for tags with prefix: {}", prefix));
        
        // List all tags with the prefix
        let tags_output = self.run_git_command(&["tag", "-l", &format!("{}*", prefix)])?;
        
        if tags_output.is_empty() {
            self.logger.info(&format!("No tags found with prefix: {}", prefix));
            return Ok(None);
        }
        
        // Split into lines and sort by version
        let mut tags: Vec<&str> = tags_output.lines().collect();
        
        // Log all found tags for debugging
        self.logger.debug(&format!("Found {} tags with prefix {}: {:?}", tags.len(), prefix, tags));
        
        // Custom sort function that understands semantic versioning
        tags.sort_by(|a, b| {
            // Extract version parts for comparison
            let extract_version = |tag: &str| -> Vec<i32> {
                let re = Regex::new(&format!(r"{}-v(\d+)\.(\d+)\.(\d+)", prefix)).unwrap();
                if let Some(caps) = re.captures(tag) {
                    return vec![
                        i32::from_str(&caps[1]).unwrap_or(0),
                        i32::from_str(&caps[2]).unwrap_or(0),
                        i32::from_str(&caps[3]).unwrap_or(0),
                    ];
                }
                vec![0, 0, 0]
            };
            
            let a_ver = extract_version(a);
            let b_ver = extract_version(b);
            
            // Compare version components
            for i in 0..3 {
                let a_comp = a_ver.get(i).unwrap_or(&0);
                let b_comp = b_ver.get(i).unwrap_or(&0);
                
                match a_comp.cmp(b_comp) {
                    std::cmp::Ordering::Equal => continue,
                    other => return other,
                }
            }
            
            std::cmp::Ordering::Equal
        });
        
        // Get the last (highest) tag
        let latest_tag = tags.last().map(|&s| s.to_string());
        
        if let Some(tag) = &latest_tag {
            self.logger.info(&format!("Found latest tag with prefix {}: {}", prefix, tag));
        }
        
        Ok(latest_tag)
    }
    
    /// Extract version components from a tag
    fn extract_version(&self, tag: &str) -> Result<String> {
        self.logger.debug(&format!("Extracting version from tag: {}", tag));
        
        let re = Regex::new(r"(?:stable|beta)-v([0-9]+\.[0-9]+\.[0-9]+)").unwrap();
        
        if let Some(caps) = re.captures(tag) {
            let version = caps.get(1).unwrap().as_str().to_string();
            self.logger.debug(&format!("Extracted version: {}", version));
            return Ok(version);
        }
        
        // Try alternative pattern with just 'v'
        let re_simple = Regex::new(r"v([0-9]+\.[0-9]+\.[0-9]+)").unwrap();
        if let Some(caps) = re_simple.captures(tag) {
            let version = caps.get(1).unwrap().as_str().to_string();
            self.logger.debug(&format!("Extracted version (alt pattern): {}", version));
            return Ok(version);
        }
        
        self.logger.warn(&format!("Could not extract version from tag: {}", tag));
        Ok("0.0.1".to_string())  // Default if no match
    }
    
    /// Increment a specific component of a version string
    fn increment_version(&self, version: &str, position: usize) -> Result<String> {
        self.logger.debug(&format!("Incrementing version {} at position {}", version, position));
        
        // Parse the version into components
        let mut parts: Vec<i32> = version
            .split('.')
            .map(|s| i32::from_str(s).unwrap_or(0))
            .collect();
            
        // Ensure we have at least 3 parts
        while parts.len() < 3 {
            parts.push(0);
        }
        
        // Increment the specified position (0-indexed in the function)
        if position <= parts.len() {
            parts[position - 1] += 1;
            
            // Reset subsequent components
            for i in position..parts.len() {
                parts[i] = 0;
            }
        }
        
        // Rebuild the version string
        let new_version = parts.iter().map(|&n| n.to_string()).collect::<Vec<String>>().join(".");
        self.logger.debug(&format!("Incremented version: {}", new_version));
        
        Ok(new_version)
    }
    
    fn get_normalized_branch(&self) -> Result<String> {
        let branch = self.source_branch.to_lowercase();
        
        // Debug branch name to ensure we're working with correct value
        self.logger.info(&format!("Normalizing branch name: '{}'", branch));
        
        // Only allow main and beta branches (with normalization)
        if branch == "main" {
            return Ok("main".to_string());
        } else if branch == "beta" {
            return Ok("beta".to_string());
        }
        
        // Any other branch name is invalid and should prevent releases
        self.logger.error(&format!("⛔ Invalid branch: '{}'. Only 'main' and 'beta' are allowed for releases", branch));
        anyhow::bail!("Invalid branch for release: '{}'", branch)
    }
    
    /// Determine the next version based on branch and existing tags
    async fn determine_version(&self) -> Result<(String, bool)> {
        // Get normalized branch with strict validation
        let normalized_branch = self.get_normalized_branch()?;
        self.logger.info(&format!("Determining version for branch: '{}'", normalized_branch));
        
        // Make sure we have all tags
        self.fetch_tags()?;
        
        // Find latest tags for each prefix
        let latest_beta_tag = self.find_latest_tag_with_prefix("beta-v")?;
        let latest_stable_tag = self.find_latest_tag_with_prefix("stable-v")?;
        
        self.logger.info(&format!("Latest beta tag: {:?}", latest_beta_tag));
        self.logger.info(&format!("Latest stable tag: {:?}", latest_stable_tag));
        
        // Determine the next version based on branch and tags
        // Since we've validated the branch above, we know it's either "main" or "beta"
        let (new_version, is_beta) = if normalized_branch == "beta" {
            self.logger.info("Using beta branch versioning logic");
            if let Some(tag) = &latest_beta_tag {
                // Increment patch version of latest beta
                let version = self.extract_version(tag)?;
                let incremented = self.increment_version(&version, 3)?;
                (format!("beta-v{}", incremented), true)
            } else if let Some(tag) = &latest_stable_tag {
                // Derive from stable by incrementing minor
                let version = self.extract_version(tag)?;
                let incremented = self.increment_version(&version, 2)?;
                (format!("beta-v{}", incremented), true)
            } else {
                // No tags at all
                (self.default_version.clone(), true)
            }
        } else {
            // This must be main branch due to our validation
            self.logger.info("Using main branch versioning logic");
            if let Some(tag) = &latest_beta_tag {
                // Promote beta to stable
                let version = self.extract_version(tag)?;
                (format!("stable-v{}", version), false)
            } else if let Some(tag) = &latest_stable_tag {
                // Increment patch version of latest stable
                let version = self.extract_version(tag)?;
                let incremented = self.increment_version(&version, 3)?;
                (format!("stable-v{}", incremented), false)
            } else {
                // No tags at all
                let default_stable = self.default_version.replace("beta-v", "stable-v");
                self.logger.info(&format!("No tags found, using default stable version: {}", default_stable));
                (default_stable, false)
            }
        };
        
        self.logger.info(&format!("Determined new version: {} (is_beta: {})", &new_version, is_beta));
        Ok((new_version, is_beta))
    }
    
    /// Execute the version determination process
    async fn run(&self) -> Result<()> {
        self.logger.info("Starting version determiner execution");
        
        // Get the new version and beta status
        let (version, is_beta) = self.determine_version().await?;
        
        // Set outputs for GitHub Actions
        github_workflow_scripts::github::set_output("version", &version);
        github_workflow_scripts::github::set_output("is_beta", if is_beta { "true" } else { "false" });
        
        self.logger.info(&format!("Successfully determined version: {} (beta: {})", version, is_beta));
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    init();
    let determiner = VersionDeterminer::new()?;
    determiner.run().await
}
