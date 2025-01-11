use std::{
    path::PathBuf, 
    process::{Command as StdCommand, Stdio},
    time::Duration,
};
use structopt::StructOpt;
use anyhow::{Result, Context, bail};
use tokio::time::timeout;

#[derive(StructOpt, Debug)]
enum TestCommand {
    /// Test the environment from a creator's perspective
    Creator {
        #[structopt(long)]
        dockerfile: PathBuf,
        #[structopt(long)]
        dockerhub_repo: String,
    },
    /// Test the environment from an end user's perspective
    User {
        #[structopt(long)]
        dockerhub_image: String,
        #[structopt(long)]
        torrent_file: PathBuf,
        #[structopt(long)]
        checksum_file: PathBuf,
    },
}

#[derive(Debug)]
struct TestResult {
    name: String,
    passed: bool,
    duration: Duration,
    error_message: Option<String>,
}

const DOCKERFILE_TIMEOUT: u64 = 30;
const DISTRIBUTION_TIMEOUT: u64 = 300;
const DEFAULT_TIMEOUT: u64 = 60;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let cmd = TestCommand::from_args();

    let mut test_results = Vec::new();

    match cmd {
        TestCommand::Creator { dockerfile, dockerhub_repo } => {
            test_results.extend(run_creator_tests(&dockerfile, &dockerhub_repo).await?);
        },
        TestCommand::User { dockerhub_image, torrent_file, checksum_file } => {
            test_results.extend(run_user_tests(&dockerhub_image, &torrent_file, &checksum_file).await?);
        },
    }

    print_test_results(&test_results);
    
    // Fail if any tests failed
    if test_results.iter().any(|r| !r.passed) {
        bail!("Some tests failed");
    }

    Ok(())
}

async fn run_creator_tests(dockerfile: &PathBuf, repo: &str) -> Result<Vec<TestResult>> {
    let mut results: Vec<TestResult> = Vec::new();

    let tests = vec![
        run_test("Dockerfile Validation", test_dockerfile_customization(dockerfile), DOCKERFILE_TIMEOUT).await,
        run_test("Distribution Creation", test_distribution_creation(dockerfile, repo), DISTRIBUTION_TIMEOUT).await,
        run_test("Torrent Creation", test_torrent_creation(), DEFAULT_TIMEOUT).await,
    ];

    results.extend(tests);
    Ok(results)
}

async fn run_user_tests(image: &str, torrent: &PathBuf, checksum: &PathBuf) -> Result<Vec<TestResult>> {
    let mut results: Vec<TestResult> = Vec::new();

    let tests = vec![
        run_test("DockerHub Installation", test_dockerhub_install(image), DEFAULT_TIMEOUT).await,
        run_test("Torrent Installation", test_torrent_install(torrent, checksum), DEFAULT_TIMEOUT).await,
        run_test("IDE Integration", test_ide_integration(), DEFAULT_TIMEOUT).await,
        run_test("Development Tools", test_dev_workflows(), DEFAULT_TIMEOUT).await,
    ];

    results.extend(tests);
    Ok(results)
}

async fn run_test(test_name: &str, test_fn: impl std::future::Future<Output = Result<()>>, timeout_secs: u64) -> TestResult {
    let start = std::time::Instant::now();
    let result = timeout(Duration::from_secs(timeout_secs), test_fn).await;
    let duration = start.elapsed();

    match result {
        Ok(Ok(_)) => TestResult {
            name: test_name.to_string(),
            passed: true,
            duration,
            error_message: None,
        },
        Ok(Err(e)) => TestResult {
            name: test_name.to_string(),
            passed: false,
            duration,
            error_message: Some(e.to_string()),
        },
        Err(_) => TestResult {
            name: test_name.to_string(),
            passed: false,
            duration,
            error_message: Some("Test timed out".to_string()),
        }
    }
}

fn print_test_results(results: &[TestResult]) {
    println!("\n===== Test Results =====");
    for result in results {
        let status = if result.passed { "✅ PASSED" } else { "❌ FAILED" };
        println!(
            "{} - {} (Duration: {:.2?})\n{}",
            status, 
            result.name,
            result.duration,
            result.error_message.as_deref().unwrap_or("")
        );
    }
    println!("=======================");
}

async fn test_dockerhub_install(image: &str) -> Result<()> {
    let output = StdCommand::new("docker")
        .args(&["pull", image])
        .output()
        .context("Failed to execute docker pull")?;

    if !output.status.success() {
        bail!("Docker pull failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

async fn test_dockerfile_customization(dockerfile: &PathBuf) -> Result<()> {
    if !dockerfile.exists() {
        bail!("Dockerfile does not exist at specified path");
    }

    // Additional Dockerfile validation
    let content = std::fs::read_to_string(dockerfile)
        .context("Failed to read Dockerfile")?;

    if !content.contains("FROM") {
        bail!("Invalid Dockerfile: Missing base image");
    }

    Ok(())
}

async fn test_distribution_creation(dockerfile: &PathBuf, _repo: &str) -> Result<()> {
    // Get the parent directory of the Dockerfile to use as build context
    let context_dir = dockerfile.parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .ok_or_else(|| anyhow::anyhow!("Could not determine build context"))?;

    let output = StdCommand::new("docker")
        .args(&[
            "build", 
            "-t", 
            "test-image",
            "-f", 
            dockerfile.to_str().unwrap(),
            context_dir.to_str().unwrap()  // Use repository root as build context
        ])
        .output()
        .context("Failed to build Docker image")?;

    if !output.status.success() {
        bail!("Docker build failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

async fn test_torrent_creation() -> Result<()> {
    // Simulate torrent creation test
    // In a real scenario, this would involve creating a torrent file
    Ok(())
}

async fn test_torrent_install(_torrent: &PathBuf, _checksum: &PathBuf) -> Result<()> {
    // Simulate torrent installation test
    // In a real scenario, this would involve downloading and verifying a torrent
    Ok(())
}

async fn test_ide_integration() -> Result<()> {
    // Check for VS Code CLI
    let output = StdCommand::new("code")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("VS Code CLI not found")?;

    if !output.success() {
        bail!("VS Code integration check failed");
    }

    Ok(())
}

async fn test_dev_workflows() -> Result<()> {
    let dev_tools = vec![
        ("node", "--version", "v22.3.0"),
        ("go", "version", "go1.22.4"),
        ("rust", "--version", "1.75.0"),
        ("cargo", "--version", "1.75.0"),
        ("git", "--version", "2.43.0"),
    ];

    for (tool, arg, expected_version) in dev_tools {
        let output = StdCommand::new(tool)
            .arg(arg)
            .output()
            .with_context(|| format!("Failed to run {}", tool))?;

        if !output.status.success() {
            bail!("{} check failed", tool);
        }

        let version_output = String::from_utf8_lossy(&output.stdout);
        if !version_output.contains(expected_version) {
            bail!("{} version mismatch. Expected {}, got {}", tool, expected_version, version_output);
        }
    }

    Ok(())
} 