use std::path::{Path, PathBuf};
use std::time::Duration;
use anyhow::{anyhow, Result};
use tokio::time::timeout;

use crate::distribution::DistributionTest;
use crate::ide::IdeTest;
use crate::startup::StartupTest;
use crate::logging::get_logger;
use crate::common::download::download_file;

// Test timeout constants (in seconds)
const DOCKERFILE_TIMEOUT: u64 = 30;
const DISTRIBUTION_TIMEOUT: u64 = 300;
const DOCKERHUB_TIMEOUT: u64 = 300;
const DIRECT_DOWNLOAD_TIMEOUT: u64 = 180;
const IDE_TIMEOUT: u64 = 30;
const DEV_TOOLS_TIMEOUT: u64 = 60;

#[derive(Debug)]
pub struct TestResult {
    pub name: String,
    pub success: bool,
    pub duration: Duration,
    pub error: Option<String>,
}

async fn test_with_timeout(
    name: &str,
    test: impl std::future::Future<Output = Result<()>>,
    timeout_secs: u64
) -> Result<TestResult> {
    let start = std::time::Instant::now();
    let result = match timeout(Duration::from_secs(timeout_secs), test).await {
        Ok(test_result) => test_result,
        Err(_) => Err(anyhow!("Test timed out after {} seconds", timeout_secs)),
    };

    Ok(TestResult {
        name: name.to_string(),
        success: result.is_ok(),
        duration: start.elapsed(),
        error: result.err().map(|e| e.to_string()),
    })
}

async fn validate_dockerfile(dockerfile: &Path) -> Result<()> {
    let logger = get_logger();
    let content = std::fs::read_to_string(dockerfile)?;
    
    let required_components = vec![
        ("FROM", "Base image not specified"),
        ("RUN apt-get update", "Package update missing"),
        ("nodejs", "Node.js installation missing"),
        ("go", "Go installation missing"),
        ("rustup", "Rust installation missing"),
        ("git", "Git installation missing"),
    ];

    for (component, error_msg) in required_components {
        if !content.contains(component) {
            return Err(anyhow!(error_msg));
        }
    }

    logger.debug("Dockerfile validation passed");
    Ok(())
}

async fn run_ide_tests(ide_test: &IdeTest<'_>) -> Result<()> {
    ide_test.test_vscode_integration().await?;
    ide_test.test_extensions().await?;
    ide_test.test_settings().await?;
    Ok(())
}

async fn run_startup_tests(startup_test: &StartupTest<'_>) -> Result<()> {
    startup_test.test_dev_tools().await?;
    startup_test.test_environment_variables().await?;
    startup_test.test_workspace_structure().await?;
    Ok(())
}

async fn prepare_test_env(download_url: &str) -> Result<(PathBuf, PathBuf)> {
    let mut download_path = PathBuf::from("target/tmp");
    std::fs::create_dir_all(&download_path)?;
    download_path.push("dev-environment.tar");
    
    let mut checksum_path = download_path.clone();
    checksum_path.set_extension("checksum");
    
    download_file(download_url, &download_path).await?;
    
    Ok((download_path, checksum_path))
}

fn report_results(results: &[TestResult]) -> Result<bool> {
    let logger = get_logger();
    let mut failed = false;

    for result in results {
        let status = if result.success { "✅ PASSED" } else { "❌ FAILED" };
        let duration = format!("Duration: {:.2?}", result.duration);
        
        logger.info(&format!("{} - {} ({})", status, result.name, duration));
        if !result.success {
            logger.error(&result.error.clone().unwrap_or_default());
            failed = true;
        }
    }

    Ok(!failed)
}

pub async fn run_creator_workflow(
    dockerfile: &Path,
    repo: &str,
    download_url: &str,
) -> Result<bool> {
    let logger = get_logger();
    let distribution_test = DistributionTest::new(logger.as_ref());
    
    let (download_path, checksum_path) = prepare_test_env(download_url).await?;
    
    let results = vec![
        test_with_timeout(
            "Dockerfile Validation", 
            validate_dockerfile(dockerfile), 
            DOCKERFILE_TIMEOUT
        ).await?,
        test_with_timeout(
            "Distribution Creation", 
            distribution_test.test_distribution_creation(dockerfile, repo), 
            DISTRIBUTION_TIMEOUT
        ).await?,
        test_with_timeout(
            "Direct Download Package", 
            distribution_test.test_direct_download(&download_path, &checksum_path), 
            DIRECT_DOWNLOAD_TIMEOUT
        ).await?,
    ];

    report_results(&results)
}

pub async fn run_user_workflow(
    image: &str,
    download_url: &str,
) -> Result<bool> {
    let logger = get_logger();
    let distribution_test = DistributionTest::new(logger.as_ref());
    let ide_test = IdeTest::new(logger.as_ref());
    let startup_test = StartupTest::new(logger.as_ref());
    
    let (download_path, checksum_path) = prepare_test_env(download_url).await?;
    
    let results = vec![
        test_with_timeout(
            "DockerHub Installation", 
            distribution_test.test_dockerhub_install(image), 
            DOCKERHUB_TIMEOUT
        ).await?,
        test_with_timeout(
            "Direct Download Installation", 
            distribution_test.test_direct_download(&download_path, &checksum_path), 
            DIRECT_DOWNLOAD_TIMEOUT
        ).await?,
        test_with_timeout(
            "IDE Integration", 
            run_ide_tests(&ide_test), 
            IDE_TIMEOUT
        ).await?,
        test_with_timeout(
            "Development Tools", 
            run_startup_tests(&startup_test), 
            DEV_TOOLS_TIMEOUT
        ).await?,
    ];

    report_results(&results)
}
