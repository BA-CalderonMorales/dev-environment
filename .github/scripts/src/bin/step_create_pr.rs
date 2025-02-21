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

    // Create pull request
    async fn create_pull_request(&self, octocrab: &Octocrab) -> Result<octocrab::models::pulls::PullRequest> {
        self.logger.info("Creating pull request...");

        // Validate branches first
        self.validate_branches()?;

        // Clean branch names (remove refs/heads/ if present)
        let base_branch = self.input_branch.replace("refs/heads/", "");
        let head_branch = self.queue_branch.replace("refs/heads/", "");

        self.logger.info(&format!("Creating PR from '{}' into '{}'", head_branch, base_branch));

        // Prepare pull request details
        let title = format!("📦 Queue Update: Release {} (Position: {})", self.sha, self.position);
        let body = format!(
            "This PR updates the release queue for commit {}.\n\nQueue Status:\n- Position: {}\n- Items needed: {} more\n- Estimated time: {}",
            self.sha, self.position, self.remaining, self.est_time
        );

        // Create the pull request
        octocrab
            .pulls(&self.owner, &self.repo)
            .create(&head_branch, &base_branch, title.as_str())
            .body(body)
            .send()
            .await
            .context("Failed to create pull request")
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

        Ok(()) // Return Ok(()) explicitly
    }

    // Run the complete process
    async fn run(&self) -> Result<()> {
        let octocrab = self.create_octocrab()?;
        let new_pr = self.create_pull_request(&octocrab).await?;

        self.logger.info(&format!("✅ Created pull request: {}", new_pr.html_url.unwrap()));

        // Set output for the PR number
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
