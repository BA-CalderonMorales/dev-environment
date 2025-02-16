use anyhow::{Context, Result};
use chrono::Utc;
use github_workflow_scripts::{get_logger, init};
use serde_json::{json, Value};
use std::{env, fs, path::Path};

#[tokio::main]
async fn main() -> Result<()> {
    init();
    let logger = get_logger(false);

    logger.info("📝 Processing queue update...");

    // Get required inputs
    let branch = env::var("INPUT_BRANCH").context("Missing branch input")?;
    let sha = env::var("INPUT_SHA").context("Missing SHA input")?;
    
    let queue_dir = Path::new(".github/release_queue");
    let queue_file = queue_dir.join(format!("{}.json", branch));

    // Create directory if needed
    fs::create_dir_all(queue_dir)
        .context("Failed to create queue directory")?;

    // Read or initialize queue file
    let queue_content = if queue_file.exists() {
        fs::read_to_string(&queue_file)
            .context("Failed to read queue file")?
    } else {
        logger.info("🆕 Creating new queue file");
        String::from(r#"{"items":[]}"#)
    };

    let mut queue: Value = serde_json::from_str(&queue_content)
        .context("Failed to parse queue JSON")?;
    
    // Add new item
    let items = queue["items"].as_array_mut()
        .context("Invalid queue structure")?;
    items.push(json!({
        "commit": sha,
        "date": Utc::now().to_rfc3339()
    }));

    // Calculate queue metrics
    let position = items.len();
    let (est_days, min_items) = if branch == "beta" {
        (0, 10)
    } else {
        (14, 15)
    };

    // Save updated queue
    fs::write(&queue_file, serde_json::to_string_pretty(&queue)?)
        .context("Failed to write queue file")?;

    logger.info(&format!("✅ Added commit to queue position {}", position));
    
    // Output for GitHub Actions
    println!("::set-output name=position::{}", position);
    println!("::set-output name=estimated_time::{} days", est_days);
    println!("::set-output name=remaining::{}", min_items - position);

    Ok(())
}
