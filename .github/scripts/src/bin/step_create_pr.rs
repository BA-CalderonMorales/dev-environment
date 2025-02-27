use anyhow::{Context, Result};
use github_workflow_scripts::{get_logger, init};
use octocrab::Octocrab;
use std::env;

// Struct to hold common dependencies and state
struct PrCreator {
    logger: Box<dyn github_workflow_scripts::Logger>,
    github_token: String,
    queue_branch: String,
    input_branch: String,
    sha: String,
    position: String,
    remaining: String,
    est_time: String,
    owner: String,
    repo: String,
}

impl PrCreator {
    // Initialize with required inputs
    fn new() -> Result<Self> {
        let logger = get_logger(false);

        // Retrieve inputs
        let github_token = env::var("GITHUB_TOKEN").context("Missing GITHUB_TOKEN")?;
        let queue_branch = env::var("QUEUE_BRANCH").context("Missing QUEUE_BRANCH")?;
        let input_branch = env::var("INPUT_BRANCH").context("Missing INPUT_BRANCH")?;
        let sha = env::var("INPUT_SHA").context("Missing INPUT_SHA")?;
        let position = env::var("QUEUE_POSITION").context("Missing QUEUE_POSITION")?;
        let remaining = env::var("QUEUE_REMAINING").context("Missing QUEUE_REMAINING")?;
        let est_time = env::var("QUEUE_ESTIMATED_TIME").context("Missing QUEUE_ESTIMATED_TIME")?;

        // Extract owner and repo from GITHUB_REPOSITORY
        let github_repo = env::var("GITHUB_REPOSITORY").context("Missing GITHUB_REPOSITORY")?;
        let parts: Vec<&str> = github_repo.split('/').collect();
        if parts.len() != 2 {
            anyhow::bail!("Invalid GITHUB_REPOSITORY format");
        }
        let owner = parts[0].to_string();
        let repo = parts[1].to_string();

        Ok(Self {
            logger,
            github_token,
            queue_branch,
            input_branch,
            sha,
            position,
            remaining,
            est_time,
            owner,
            repo,
        })
    }

    // Create Octocrab instance
    fn create_octocrab(&self) -> Result<Octocrab> {
        self.logger.info("Initializing Octocrab...");
        Octocrab::builder()
            .personal_token(self.github_token.clone())
            .build()
            .context("Failed to build Octocrab instance")
    }

    // Add method to validate branch names
    fn validate_branches(&self) -> Result<()> {
        self.logger.info("🔍 Validating branch names...");
        self.logger.info(&format!("Queue Branch: {}", self.queue_branch));
        self.logger.info(&format!("Input Branch: {}", self.input_branch));

        if self.queue_branch.is_empty() {
            anyhow::bail!("Queue branch name is empty");
        }

        if self.input_branch.is_empty() {
            anyhow::bail!("Input branch name is empty");
        }

        // Remove any 'refs/heads/' prefix from input branch
        if self.input_branch.starts_with("refs/heads/") {
            self.logger.info("Removing 'refs/heads/' prefix from input branch");
            let cleaned_branch = self.input_branch.replace("refs/heads/", "");
            self.logger.info(&format!("Cleaned Input Branch: {}", cleaned_branch));
        }

        Ok(())
    }

    // Add method to check branch protection
    async fn check_branch_protection(&self, octocrab: &Octocrab, branch: &str) -> Result<bool> {
        self.logger.info(&format!("🛡️ Checking protection for branch: {}", branch));

        // Check if branch exists first
        match octocrab
            .repos(&self.owner, &self.repo)
            .list_branches()
            .send()  // Add send() before await
            .await {
                Ok(branches) => {
                    // Log found branches
                    self.logger.info("Found branches:");
                    for branch_info in &branches {
                        self.logger.info(&format!("- {}", branch_info.name));
                    }

                    // Rest of the branch checking logic...
                    match branches.items.into_iter().find(|b| b.name == branch) {
                        Some(branch_info) => {
                            let is_protected = branch_info.protected;
                            if is_protected {
                                self.logger.info(&format!("✅ Branch '{}' is protected", branch));
                            } else {
                                self.logger.warn(&format!("⚠️ Branch '{}' is not protected", branch));
                            }
                            Ok(is_protected)
                        },
                        None => {
                            self.logger.warn(&format!("❌ Branch '{}' not found", branch));
                            Ok(false)
                        }
                    }
                },
                Err(e) => {
                    self.logger.warn(&format!("Failed to list branches: {}", e));
                    Ok(false) // Assume not protected on error
                }
            }
    }

    // Add method to stage and commit changes
    fn commit_queue_changes(&self) -> Result<()> {
        self.logger.info("📝 Committing queue changes...");

        // Create queue directory if it doesn't exist
        std::fs::create_dir_all(".github/release_queue")
            .context("Failed to create queue directory")?;

        // Configure git
        let git_configs = [
            ("user.name", "GitHub Actions"),
            ("user.email", "actions@github.com"),
        ];

        for (key, value) in git_configs {
            let output = std::process::Command::new("git")
                .args(["config", key, value])
                .output()
                .context("Failed to configure git")?;

            if !output.status.success() {
                anyhow::bail!("Failed to set git config {}: {}", key, 
                    String::from_utf8_lossy(&output.stderr));
            }
        }

        // Stage changes
        let output = std::process::Command::new("git")
            .args(["add", ".github/release_queue/"])
            .output()
            .context("Failed to stage changes")?;

        if !output.status.success() {
            anyhow::bail!("Failed to stage changes: {}", 
                String::from_utf8_lossy(&output.stderr));
        }

        // Commit changes
        let commit_msg = format!("📦 Queue update for {}", self.sha);
        let output = std::process::Command::new("git")
            .args(["commit", "-m", &commit_msg])
            .output()
            .context("Failed to commit changes")?;

        if !output.status.success() {
            anyhow::bail!("Failed to commit changes: {}", 
                String::from_utf8_lossy(&output.stderr));
        }

        // Push changes
        let output = std::process::Command::new("git")
            .args(["push", "origin", &self.queue_branch])
            .output()
            .context("Failed to push changes")?;

        if !output.status.success() {
            anyhow::bail!("Failed to push changes: {}", 
                String::from_utf8_lossy(&output.stderr));
        }

        Ok(())
    }

    // Modified to make token verification optional
    async fn verify_token_permissions(&self, octocrab: &Octocrab) -> Result<()> {
        self.logger.info("🔑 Verifying GitHub token permissions...");
        
        // Get current user info to check token validity
        match octocrab.current().user().await {
            Ok(user) => {
                self.logger.info(&format!("✅ Authenticated as: {}", user.login));
                Ok(())
            },
            Err(e) => {
                self.logger.warn(&format!("⚠️ Token verification warning: {}", e));
                self.logger.info("Proceeding with PR creation anyway, as GITHUB_TOKEN might have limited API access but still have PR creation permissions");
                Ok(()) // Return Ok() instead of an error to continue execution
            }
        }
    }

    // Create pull request with robust error handling and diagnostics
    async fn create_pull_request(&self, octocrab: &Octocrab) -> Result<octocrab::models::pulls::PullRequest> {
        self.logger.info("🚀 Starting pull request creation process...");
        
        // Step 1: Verify token has sufficient permissions
        self.verify_token_permissions(octocrab).await?;
        
        // Step 2: Validate branch names and format them appropriately
        self.validate_branches()?;
        
        // Clean branch names with various formatting options to try
        let base_branch = self.input_branch.replace("refs/heads/", "");
        let head_branch = self.queue_branch.replace("refs/heads/", "");
        
        // Step 3: Check branch protection status to ensure we're targeting protected branches properly
        self.logger.info("🛡️ Validating branch protection status...");
        let is_protected = self.check_branch_protection(octocrab, &base_branch).await?;
        self.logger.info(&format!("Base branch '{}' protection status: {}", base_branch, if is_protected { "protected" } else { "not protected" }));
        
        // Generate all possible branch format combinations for robust testing
        let branch_formats = [
            // Strategy 1: Standard format (most common)
            (base_branch.clone(), head_branch.clone()),
            
            // Strategy 2: With owner qualification on head only
            (base_branch.clone(), format!("{}:{}", self.owner, head_branch)),
            
            // Strategy 3: With refs/heads/ prefix (sometimes needed)
            (format!("refs/heads/{}", base_branch), format!("refs/heads/{}", head_branch)),
            
            // Strategy 4: Fully qualified branch names
            (format!("refs/heads/{}", base_branch), format!("{}:refs/heads/{}", self.owner, head_branch)),
        ];
        
        // Step 4: Log detailed diagnostics for debugging
        self.logger.info("📊 Diagnostic Information:");
        self.logger.info(&format!("- Repository owner: {}", self.owner));
        self.logger.info(&format!("- Repository name: {}", self.repo));
        self.logger.info(&format!("- Raw base branch: {}", self.input_branch));
        self.logger.info(&format!("- Raw head branch: {}", self.queue_branch));
        
        // Log all available branches for debugging
        let branches = octocrab
            .repos(&self.owner, &self.repo)
            .list_branches()
            .send()
            .await?;
            
        self.logger.info("📋 Available repository branches:");
        for branch in &branches.items {
            self.logger.info(&format!("- {} (protected: {})", branch.name, branch.protected));
        }
        
        // Step 4: Try multiple strategies until one works
        let title = format!("📦 Queue Update: Release {} (Position: {})", self.sha, self.position);
        let body = format!(
            "This PR updates the release queue for commit {}.\n\nQueue Status:\n- Position: {}\n- Items needed: {} more\n- Estimated time: {}",
            self.sha, self.position, self.remaining, self.est_time
        );
        
        // Try each branch format strategy
        for (i, (base, head)) in branch_formats.iter().enumerate() {
            self.logger.info(&format!("🔄 Trying PR creation strategy {}/{}...", i+1, branch_formats.len()));
            self.logger.info(&format!("  Base: {}", base));
            self.logger.info(&format!("  Head: {}", head));
            
            match octocrab
                .pulls(&self.owner, &self.repo)
                .create(base, head, title.clone())
                .body(body.clone())
                .send()
                .await {
                    Ok(pr) => {
                        self.logger.info(&format!("✅ PR created successfully using strategy {}: #{}", i+1, pr.number));
                        return Ok(pr);
                    },
                    Err(e) => {
                        self.logger.warn(&format!("⚠️ Strategy {} failed: {}", i+1, e));
                        // Continue with next strategy
                    }
                }
        }
        
        // Step 5: If all strategies fail, try direct API call with raw JSON
        self.logger.info("⚡ Attempting direct API call as last resort...");
        
        // Construct direct request with raw JSON payload
        let client = reqwest::Client::new();
        let url = format!("https://api.github.com/repos/{}/{}/pulls", self.owner, self.repo);
        
        let payload = serde_json::json!({
            "title": title,
            "body": body,
            "head": format!("{}:{}", self.owner, head_branch),
            "base": base_branch
        });
        
        self.logger.info(&format!("Direct API request to: {}", url));
        self.logger.info(&format!("Payload: {}", payload));
        
        let response = client
            .post(&url)
            .header("Authorization", format!("token {}", self.github_token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "OctocrabClient")
            .json(&payload)
            .send()
            .await
            .context("Failed to send direct API request")?;
        
        // Capture status code first before consuming the response
        let status = response.status();
        
        // Parse response
        if status.is_success() {
            let pr: octocrab::models::pulls::PullRequest = response
                .json()
                .await
                .context("Failed to parse PR response")?;
                
            self.logger.info(&format!("✅ PR created successfully via direct API: #{}", pr.number));
            return Ok(pr);
        }
        
        // Log the full error response for debugging
        let error_text = response.text().await.unwrap_or_else(|_| "Failed to get response text".to_string());
        self.logger.warn(&format!("❌ Direct API call failed with status code: {}", status));
        self.logger.warn(&format!("Error response: {}", error_text));
        
        anyhow::bail!("Failed to create pull request after trying all strategies")
    }

    // Add labels to pull request
    async fn add_labels_to_pr(&self, octocrab: &Octocrab, pr_number: u64) -> Result<()> {
        self.logger.info("Adding labels to pull request...");
        let _labels = octocrab  // Capture and discard the Vec<Label>
            .issues(&self.owner, &self.repo)
            .add_labels(
                pr_number,
                &["release-queue".to_string(), "automated-pr".to_string()],
            )
            .await
            .context("Failed to add labels")?;

        Ok(())
    }

    // Run the complete process
    async fn run(&self) -> Result<()> {
        let octocrab = self.create_octocrab()?;
        
        // Commit and push changes before creating PR
        self.commit_queue_changes()?;
        
        // Create PR and add labels
        let new_pr = self.create_pull_request(&octocrab).await?;
        self.logger.info(&format!("✅ Created pull request: {}", new_pr.html_url.unwrap()));
        println!("::set-output name=pr_number::{}", new_pr.number);
        self.add_labels_to_pr(&octocrab, new_pr.number).await?;

        self.logger.info("🎉 Pull request workflow completed successfully!");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    init();
    let pr_creator = PrCreator::new()?;
    pr_creator.run().await
}
