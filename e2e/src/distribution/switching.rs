use anyhow::{bail, Result};
use std::env;
use std::process::Command;
use tokio::time::timeout;

async fn run_test_case(test_name: &str, env_vars: Vec<(&str, &str)>) -> Result<()> {
    println!("📋 Running test case: {}", test_name);

    // Clean any existing containers/images
    let _ = Command::new("docker").args(&["compose", "down"]).output();
    let _ = Command::new("docker").args(&["rmi", "dev-environment:latest"]).output();
    let _ = Command::new("docker").args(&["rmi", "cmoe640/dev-environment:latest"]).output();

    // Set environment variables for the test
    for (key, value) in env_vars {
        env::set_var(key, value);
    }

    // Run start-dev.sh
    let status = Command::new("./start-dev.sh").status()?;

    if !status.success() {
        bail!("❌ Test failed: {}", test_name);
    }

    println!("✅ Test passed: {}", test_name);
    Ok(())
}

async fn cleanup() -> Result<()> {
    println!("🧹 Running cleanup...");
    let _ = Command::new("docker").args(&["compose", "down", "-v"]).output();
    let _ = Command::new("docker").args(&["rmi", "dev-environment:latest"]).output();
    let _ = Command::new("docker").args(&["rmi", "cmoe640/dev-environment:latest"]).output();
    Ok(())
}

pub async fn test_distribution_switching() -> Result<()> {
    println!("🔄 Testing Distribution Switching Mechanism...");

    // Download start-dev.sh if not testing locally
    if env::var("LOCAL_TEST").is_err() {
        let repo = env::var("GITHUB_REPOSITORY")?;
        let url = format!("https://raw.githubusercontent.com/{}/main/startup/start-dev.sh", repo);
        let status = Command::new("curl").arg("-O").arg(url).status()?;
        if !status.success() {
            bail!("Failed to download start-dev.sh");
        }
        let status = Command::new("chmod").arg("+x").arg("start-dev.sh").status()?;
        if !status.success() {
            bail!("Failed to make start-dev.sh executable");
        }
    }

    // Test Cases
    println!("🧪 Running distribution switching tests...");

    let tests = vec![
        ("BitTorrent Fallback", vec![("FORCE_BITTORRENT_FAIL", "true")]),
        ("DockerHub Fallback", vec![("SIMULATE_DOCKERHUB_RATE_LIMIT", "true")]),
        ("Preferred Distribution", vec![("PREFER_BITTORRENT", "true")]),
    ];

    for (name, env_vars) in tests {
        // 5 minute timeout per test
        let test_result = timeout(std::time::Duration::from_secs(300), run_test_case(name, env_vars)).await;
        match test_result {
            Ok(result) => result?,
            Err(_) => bail!("Test timed out: {}", name),
        }
    }

    cleanup().await?;

    println!("✅ Distribution switching tests completed");
    Ok(())
} 