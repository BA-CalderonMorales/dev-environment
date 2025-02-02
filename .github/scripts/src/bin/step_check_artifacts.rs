use anyhow::Result;
use octocrab::Octocrab;
use octocrab::models::RunId;
use std::env;
use github_workflow_scripts::{Logger, init_logging, get_logger};

struct ArtifactCheck<'a> {
    docker: bool,
    direct_download: bool,  // Renamed from bittorrent
    logger: &'a Box<dyn Logger>,
}

impl<'a> ArtifactCheck<'a> {
    fn new(logger: &'a Box<dyn Logger>) -> Self {
        Self {
            docker: false,
            direct_download: false,  // Renamed from bittorrent
            logger,
        }
    }

    async fn check_artifacts(
        &mut self,
        owner: &str,
        repo: &str,
        run_id: u64,
        token: &str
    ) -> Result<()> {
        self.logger.debug(&format!("Starting artifact check for run_id: {}", run_id));
        
        // If running locally, return mock success
        if env::var("GITHUB_ACTIONS").is_err() {
            self.logger.info("🏠 Local development mode - mocking successful artifact check");
            self.docker = true;
            self.direct_download = true;
            return Ok(());
        }

        self.logger.info("🔑 Initializing GitHub client");
        let octocrab = Octocrab::builder()
            .personal_token(token.to_string())
            .build()?;
        
        self.logger.debug(&format!(
            "Fetching artifacts for {}/{} (Run ID: {})",
            owner, repo, run_id
        ));

        let artifacts = octocrab
            .actions()
            .list_workflow_run_artifacts(owner, repo, RunId(run_id))
            .send()
            .await?;

        let items = artifacts.value.unwrap_or_default().items;
        
        self.logger.info(&format!("Found {} artifacts", items.len()));
        for item in items.iter() {
            self.logger.debug(&format!(
                "Artifact details:\n  Name: {}\n  ID: {}\n  Size: {} bytes\n  Created: {}",
                item.name, item.id, item.size_in_bytes, item.created_at
            ));
        }

        self.docker = items.iter().any(|a| a.name == "dockerhub-artifacts");
        self.direct_download = items.iter().any(|a| a.name == "direct-download-artifacts"); // Updated artifact name

        self.logger.debug(&format!(
            "Artifact presence check:\n  DockerHub: {}\n  Direct Download: {}",
            self.docker, self.direct_download
        ));

        Ok(())
    }

    fn print_status(&self) {
        self.logger.info("📊 Detailed Artifact Status:");
        self.logger.info(&format!(
            "  - DockerHub: {} ({})", 
            if self.docker { "✅" } else { "❌" },
            if self.docker { "Found" } else { "Not Found" }
        ));
        self.logger.info(&format!(
            "  - Direct Download: {} ({})",
            if self.direct_download { "✅" } else { "❌" },
            if self.direct_download { "Found" } else { "Not Found" }
        ));
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();
    let is_local = env::var("GITHUB_ACTIONS").is_err();
    let logger = get_logger(is_local);

    logger.info("🔍 Starting distribution workflow state check...");
    
    if is_local {
        logger.info("🏠 Running in local development mode (compilation test only)");
        logger.debug("Local environment detected - skipping GitHub API calls");
        logger.info("✅ Script compiled successfully!");
        
        let mut checker = ArtifactCheck::new(&logger);
        checker.check_artifacts(
            "example-owner",
            "example-repo",
            12345,
            "mock-token"
        ).await?;
        
        logger.info("\n📝 Example CI output would look like:");
        checker.print_status();
        return Ok(());
    }

    // Real CI execution
    logger.debug("Loading environment variables");
    let github_token = env::var("GITHUB_TOKEN")
        .map_err(|_| anyhow::anyhow!("GITHUB_TOKEN not found"))?;
    let repository = env::var("GITHUB_REPOSITORY")
        .map_err(|_| anyhow::anyhow!("GITHUB_REPOSITORY not found"))?;
    let run_id = env::var("GITHUB_RUN_ID")
        .map_err(|_| anyhow::anyhow!("GITHUB_RUN_ID not found"))?;
    let head_sha = env::var("GITHUB_SHA")
        .map_err(|_| anyhow::anyhow!("GITHUB_SHA not found"))?;
    
    let (owner, repo) = repository
        .split_once('/')
        .ok_or_else(|| anyhow::anyhow!("Invalid repository format"))?;
    
    logger.info(&format!("Repository: {}/{}", owner, repo));
    logger.debug(&format!("Workflow Run ID: {}", run_id));
    logger.debug(&format!("Head SHA: {}", head_sha));
    
    logger.info("📦 Checking for artifacts...");
    let mut checker = ArtifactCheck::new(&logger);
    checker.check_artifacts(
        owner,
        repo,
        run_id.parse()?,
        &github_token
    ).await?;

    checker.print_status();

    if !checker.docker || !checker.direct_download {
        checker.logger.warn("❌ Required artifacts are missing");
        anyhow::bail!("Required artifacts are missing");
    }

    checker.logger.info("✅ All required artifacts found");
    Ok(())
}
}