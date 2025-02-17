use anyhow::{Context, Result};
use github_workflow_scripts::{get_logger, init};
use octocrab::Octocrab;
use serde_json::Value;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    init();
    let logger = get_logger(false);

    logger.info("Creating pull request...");

    // Retrieve inputs
    let github_token = env::var("GITHUB_TOKEN").context("Missing GITHUB_TOKEN")?;
    let queue_branch = env::var("QUEUE_BRANCH").context("Missing QUEUE_BRANCH")?;
    let input_branch = env::var("INPUT_BRANCH").context("Missing INPUT_BRANCH")?;
    let sha = env::var("INPUT_SHA").context("Missing INPUT_SHA")?;
    let position = env::var("QUEUE_POSITION").context("Missing QUEUE_POSITION")?;
    let remaining = env::var("QUEUE_REMAINING").context("Missing QUEUE_REMAINING")?;
    let est_time = env::var("QUEUE_ESTIMATED_TIME").context("Missing QUEUE_ESTIMATED_TIME")?;

    // Initialize Octocrab
    let octocrab = Octocrab::builder()
        .personal_token(github_token)
        .build()?;

    // Extract owner and repo from GITHUB_REPOSITORY
    let github_repo = env::var("GITHUB_REPOSITORY").context("Missing GITHUB_REPOSITORY")?;
    let parts: Vec<&str> = github_repo.split('/').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid GITHUB_REPOSITORY format");
    }
    let owner = parts[0];
    let repo = parts[1];

    // Create pull request
    let title = format!("📦 Queue Update: Release {} (Position: {})", sha, position);
    let body = format!(
        "This PR updates the release queue for commit {}.\n\nQueue Status:\n- Position: {}\n- Items needed: {} more\n- Estimated time: {}",
        sha, position, remaining, est_time
    );

    let new_pr = octocrab
        .pulls(owner, repo)
        .create(queue_branch.as_str(), input_branch.as_str(), title.as_str())
        .body(body)
        .send()
        .await?;

    logger.info(&format!("✅ Created pull request: {}", new_pr.html_url.unwrap()));

    // Set output for the PR number
    println!("::set-output name=pr_number::{}", new_pr.number);

    // Add labels
    logger.info("🏷️ Adding labels...");
    octocrab
        .issues(owner, repo)
        .add_labels(
            new_pr.number,
            &["release-queue".to_string(), "automated-pr".to_string()],
        )
        .await
        .context("Failed to add labels")?;

    // Auto-approve
    logger.info("✅ Auto-approving PR...");
    let _: Value = octocrab.post(
        format!("/repos/{}/{}/pulls/{}/reviews",
            owner,
            repo,
            new_pr.number),
        Some(&serde_json::json!({
            "event": "APPROVE",
            "body": "Automatically approved by release queue system"
        })),
    )
    .await
    .context("Failed to approve PR")?;

    logger.info("🎉 Pull request workflow completed successfully!");
    Ok(())
}
