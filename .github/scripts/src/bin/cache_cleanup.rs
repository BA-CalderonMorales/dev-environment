use anyhow::Result;
use octocrab::Octocrab;
use serde::Deserialize;
use serde_json::Value;
use std::env;
use github_workflow_scripts::{init_logging, get_logger};

#[derive(Debug, Deserialize)]
struct Cache {
    id: u64,
    key: String,
    size_in_bytes: u64,
}

#[derive(Debug, Deserialize)]
struct ListCachesResponse {
    total_count: u64,
    actions_caches: Vec<Cache>,
}

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();
    let is_local = env::var("GITHUB_ACTIONS").is_err();
    let logger = get_logger(is_local);

    logger.info("🧹 Starting cache cleanup...");

    if is_local {
        logger.info("🏠 Running in local development mode (compilation test only)");
        logger.debug("Local environment detected - skipping GitHub API calls");
        logger.info("✅ Script compiled successfully!");
        return Ok(());
    }

    logger.debug("Loading environment variables");
    let github_token = env::var("GITHUB_TOKEN")
        .map_err(|_| anyhow::anyhow!("GITHUB_TOKEN not found"))?;
    let repository = env::var("GITHUB_REPOSITORY")
        .map_err(|_| anyhow::anyhow!("GITHUB_REPOSITORY not found"))?;
    
    let (owner, repo) = repository
        .split_once('/')
        .ok_or_else(|| anyhow::anyhow!("Invalid repository format"))?;

    logger.info(&format!("Repository: {}/{}", owner, repo));

    logger.info("🔑 Initializing GitHub client");
    let octocrab = Octocrab::builder()
        .personal_token(github_token)
        .build()?;

    logger.info("📦 Fetching repository caches...");
    let route = format!("/repos/{}/{}/actions/caches", owner, repo);
    let response = octocrab
        .get::<Value, _, ()>(&route, None::<&()>)
        .await?;
    
    let caches: ListCachesResponse = serde_json::from_value(response)?;
    logger.info(&format!("Found {} caches", caches.total_count));

    for cache in caches.actions_caches {
        logger.info(&format!(
            "Deleting cache: {} (key: {}, size: {} bytes)",
            cache.id,
            cache.key,
            cache.size_in_bytes
        ));
        let delete_route = format!("/repos/{}/{}/actions/caches/{}", owner, repo, cache.id);
        octocrab
            .delete::<(), _, ()>(&delete_route, None::<&()>)
            .await?;
    }

    logger.info("✅ Cache cleanup completed successfully");
    Ok(())
}