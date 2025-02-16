use anyhow::{Context, Result};
use github_workflow_scripts::{get_logger, init};
use octocrab::Octocrab;
use serde_json::Value;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    init();
    let logger = get_logger(false);

    logger.info("🔄 Creating pull request...");

    // Get required inputs
    let token = env::var("GITHUB_TOKEN").context("Missing GitHub token")?;
    let branch = env::var("QUEUE_BRANCH").context("Missing queue branch")?;
    let base_branch = env::var("INPUT_BRANCH").context("Missing base branch")?;
    let position = env::var("QUEUE_POSITION").context("Missing queue position")?;
    let remaining = env::var("QUEUE_REMAINING").context("Missing remaining items")?;
    let est_time = env::var("QUEUE_ESTIMATED_TIME").context("Missing estimated time")?;
    let sha = env::var("INPUT_SHA").context("Missing SHA input")?;

    let octocrab = Octocrab::builder()
        .personal_token(token)
        .build()
        .context("Failed to create GitHub client")?;

    // Create PR
    logger.info("📦 Creating pull request...");
    let pr = octocrab
        .pulls("BA-CalderonMorales", "dev-environment")
        .create(format!("📦 Update Release Queue: Position {}", position), 
                branch,
                base_branch)
        .body(format!(
            "## Release Queue Update\n\n\
            🎯 Adding commit `{}` to the release queue.\n\n\
            ### Queue Status\n\
            - Current Position: {}\n\
            - Items Needed: {} more\n\
            - Estimated Release Time: {}\n\n\
            > This PR was automatically created by the release queue system.\n\
            > Once merged, this commit will be included in the next release batch.",
            sha, position, remaining, est_time
        ))
        .send()
        .await
        .context("Failed to create PR")?;

    logger.info(&format!("✨ Created PR #{}", pr.number));
    println!("::set-output name=pr_number::{}", pr.number);

    // Add labels
    logger.info("🏷️ Adding labels...");
    octocrab
        .issues("BA-CalderonMorales", "dev-environment")
        .add_labels(
            pr.number,
            &["release-queue".to_string(), "automated-pr".to_string()],
        )
        .await
        .context("Failed to add labels")?;

    // Auto-approve
    
    logger.info("✅ Auto-approving PR...");
    
    let _: Value = octocrab.post(
        format!("/repos/{}/{}/pulls/{}/reviews",
            "BA-CalderonMorales",
            "dev-environment",
            pr.number),
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
