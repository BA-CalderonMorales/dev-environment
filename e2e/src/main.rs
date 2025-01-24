use std::path::{Path, PathBuf};
use std::env;
use std::time::Duration;
use anyhow::{anyhow, bail, Context, Result};
use structopt::StructOpt;

// Import Logger and test_utils from our crate
use e2e_tests::{Logger, test_utils};

// Remove common module since we're using the lib version
mod distribution;
use distribution::DistributionTest;

mod ide;
use ide::IdeTest;

mod startup;
use startup::StartupTest;

// Constants for timeouts
const DOCKERFILE_TIMEOUT: u64 = 30;
const DISTRIBUTION_TIMEOUT: u64 = 300;
const TORRENT_TIMEOUT: u64 = 60;
const DOCKERHUB_TIMEOUT: u64 = 300;
const IDE_TIMEOUT: u64 = 30;
const DEV_TOOLS_TIMEOUT: u64 = 60;

#[derive(StructOpt, Debug)]
#[structopt(name = "e2e-tests")]
enum Cli {
    #[structopt(name = "creator")]
    Creator {
        #[structopt(long)]
        dockerfile: PathBuf,
        #[structopt(long)]
        dockerhub_repo: String,
    },
    #[structopt(name = "user")]
    User {
        #[structopt(long)]
        dockerhub_image: String,
        #[structopt(long)]
        torrent_file: PathBuf,
        #[structopt(long)]
        checksum_file: PathBuf,
    },
}

struct ConsoleLogger {
    is_local: bool,
}

impl ConsoleLogger {
    fn new(is_local: bool) -> Self {
        Self { is_local }
    }
}

impl Logger for ConsoleLogger {
    fn debug(&self, message: &str) {
        if self.is_local {
            println!("🔍 DEBUG: {}", message);
        } else {
            println!("::debug::{}", message);
        }
    }

    fn info(&self, message: &str) {
        println!("{}", message);
    }

    fn warn(&self, message: &str) {
        if self.is_local {
            println!("⚠️  WARN: {}", message);
        } else {
            println!("::warning::{}", message);
        }
    }

    fn error(&self, message: &str) {
        if self.is_local {
            println!("❌ ERROR: {}", message);
        } else {
            println!("::error::{}", message);
        }
    }
}

fn init_logging() -> Box<dyn Logger> {
    let is_local = env::var("GITHUB_ACTIONS").is_err();
    Box::new(ConsoleLogger::new(is_local))
}

struct TestResult {
    name: String,
    success: bool,
    duration: Duration,
    error: Option<String>,
}

// Main function update
#[tokio::main]
async fn main() -> Result<()> {
    let logger = init_logging();
    logger.info("🚀 Starting E2E tests...");

    // Remove &** dereferencing, just use &* for Box<dyn Logger>
        let warnings = test_utils::validate_test_environment(logger.as_ref());
        if !warnings.is_empty() {
            println!("\n⚠️  Environment Warnings:");
            for warning in warnings {
            println!("  - {}", warning);
        }
        println!();
    }

    let cli = Cli::from_args();
    match cli {
        Cli::Creator { dockerfile, dockerhub_repo } => {
            logger.info("🏗️  Running creator workflow tests...");
            // Use &* for Box<dyn Logger>
            let results = run_creator_tests(&dockerfile, &dockerhub_repo, logger.as_ref()).await?;
            print_test_results(&results, logger.as_ref());
        },
        Cli::User { dockerhub_image, torrent_file, checksum_file } => {
            logger.info("👤 Running user workflow tests...");
            // Use &* for Box<dyn Logger>
            let results = run_user_tests(&dockerhub_image, &torrent_file, &checksum_file, logger.as_ref()).await?;
            print_test_results(&results, logger.as_ref());
        }
    }

    Ok(())
}

async fn run_creator_tests(dockerfile: &Path, repo: &str, logger: &dyn Logger) -> Result<Vec<TestResult>> {
    logger.info("Running creator workflow tests...");
    let mut results = Vec::new();
    
    let distribution_test = DistributionTest::new(logger);

    let tests = vec![
        run_test("Dockerfile Validation", 
            test_dockerfile_customization(dockerfile, logger).await, 
            DOCKERFILE_TIMEOUT).await,
        run_test("Distribution Creation", 
            distribution_test.test_distribution_creation(dockerfile, repo).await, 
            DISTRIBUTION_TIMEOUT).await,
        run_test("Torrent Creation", 
            distribution_test.test_torrent_creation().await, 
            TORRENT_TIMEOUT).await,
    ];

    results.extend(tests);
    Ok(results)
}

async fn run_user_tests(image: &str, torrent: &Path, checksum: &Path, logger: &dyn Logger) -> Result<Vec<TestResult>> {
    logger.info("Running user workflow tests...");
    let mut results = Vec::new();

    let distribution_test = DistributionTest::new(logger);

    let tests = vec![
        run_test("DockerHub Installation", 
            distribution_test.test_dockerhub_install(image).await, 
            DOCKERHUB_TIMEOUT).await,
        run_test("Torrent Installation", 
            distribution_test.test_torrent_install(torrent, checksum).await, 
            TORRENT_TIMEOUT).await,
        run_test("IDE Integration", 
            test_ide_integration(logger).await, 
            IDE_TIMEOUT).await,
        run_test("Development Tools", 
            test_dev_workflows(logger).await, 
            DEV_TOOLS_TIMEOUT).await,
    ];

    results.extend(tests);
    Ok(results)
}

async fn run_test(name: &str, test: Result<()>, timeout_secs: u64) -> TestResult {
    let start = std::time::Instant::now();
    let result = match tokio::time::timeout(Duration::from_secs(timeout_secs), async { test }).await {
        Ok(test_result) => test_result,
        Err(_) => Err(anyhow!("Test timed out after {} seconds", timeout_secs)),
    };

    TestResult {
        name: name.to_string(),
        success: result.is_ok(),
        duration: start.elapsed(),
        error: result.err().map(|e| e.to_string()),
    }
}

fn print_test_results(results: &[TestResult], logger: &dyn Logger) {
    logger.info("\n===== Test Results =====");
    
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
    
    logger.info("=======================");
    
    if failed {
        logger.error("Some tests failed");
    } else {
        logger.info("✅ All tests passed!");
    }
}

async fn test_dockerfile_customization(dockerfile: &Path, logger: &dyn Logger) -> Result<()> {
    logger.debug(&format!("Validating Dockerfile: {:?}", dockerfile));
    
    // Read and validate Dockerfile
    let content = std::fs::read_to_string(dockerfile)
        .context("Failed to read Dockerfile")?;

    // Required base components
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
            bail!("Dockerfile validation failed: {}", error_msg);
        }
    }

    logger.debug("Dockerfile validation passed");
    Ok(())
}

async fn test_ide_integration(logger: &dyn Logger) -> Result<()> {
    let ide_test = IdeTest::new(logger);
    
    // Run all IDE integration tests
    ide_test.test_vscode_integration().await?;
    ide_test.test_extensions().await?;
    ide_test.test_settings().await?;
    
    Ok(())
}

async fn test_dev_workflows(logger: &dyn Logger) -> Result<()> {
    let startup_test = StartupTest::new(logger);
    
    // Run all startup tests
    startup_test.test_dev_tools().await?;
    startup_test.test_environment_variables().await?;
    startup_test.test_workspace_structure().await?;
    
    Ok(())
}