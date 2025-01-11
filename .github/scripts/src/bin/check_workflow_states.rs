use anyhow::{Context, Result};
use octocrab::{models, Octocrab};
use serde::Deserialize;
use std::env;
use tracing::{debug, info, warn};

// Define our expected types
#[derive(Debug, Deserialize)]
struct WorkflowArtifact {
    name: String,
    #[serde(skip)]
    _other: (),
}

#[derive(Debug, Deserialize)]
struct WorkflowRun {
    conclusion: Option<String>,
    #[serde(skip)]
    _other: (),
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging with debug level
    tracing_subscriber::fmt()
        .init();
    
    let env_vars = get_required_env_vars().context("Failed to get required environment variables")?;
    debug!("Environment variables loaded successfully");

    let octocrab = Octocrab::builder()
        .personal_token(env_vars.token.clone())
        .build()
        .context("Failed to initialize GitHub client")?;

    info!("🔍 Checking workflow state for {}/{}", env_vars.owner, env_vars.repo);
    debug!("Run ID: {}", env_vars.run_id);

    // Check artifacts with type validation
    let artifacts = get_workflow_artifacts(&octocrab, &env_vars).await
        .context("Failed to fetch workflow artifacts")?;

    let artifact_status = check_artifact_status(&artifacts);
    log_artifact_status(&artifact_status);

    if !artifact_status.all_present {
        set_failure_output("Required artifacts are missing")?;
        return Ok(());
    }

    // Check workflow runs with type validation
    let workflow_status = check_workflow_runs(&octocrab, &env_vars).await
        .context("Failed to check workflow runs")?;
    
    log_workflow_status(&workflow_status);

    if !workflow_status.all_successful {
        set_failure_output(&workflow_status.failure_reason)?;
        return Ok(());
    }

    set_success_output()?;
    Ok(())
}

#[derive(Clone)]
struct EnvVars {
    token: String,
    owner: String,
    repo: String,
    run_id: u64,
    branch: String,
}

fn get_required_env_vars() -> Result<EnvVars> {
    Ok(EnvVars {
        token: env::var("GITHUB_TOKEN").context("GITHUB_TOKEN not found")?,
        owner: env::var("GITHUB_REPOSITORY_OWNER").context("GITHUB_REPOSITORY_OWNER not found")?,
        repo: env::var("GITHUB_REPOSITORY")
            .context("GITHUB_REPOSITORY not found")?
            .split('/')
            .nth(1)
            .context("Invalid repository format")?
            .to_string(),
        run_id: env::var("GITHUB_RUN_ID")
            .context("GITHUB_RUN_ID not found")?
            .parse()
            .context("Invalid GITHUB_RUN_ID format")?,
        branch: env::var("GITHUB_REF_NAME").context("GITHUB_REF_NAME not found")?,
    })
}

struct ArtifactStatus {
    has_dockerhub: bool,
    has_bittorrent: bool,
    all_present: bool,
}

async fn get_workflow_artifacts(octocrab: &Octocrab, env_vars: &EnvVars) -> Result<Vec<WorkflowArtifact>> {
    info!("📦 Fetching artifacts for run ID: {}", env_vars.run_id);
    
    let response = octocrab
        .actions()
        .list_workflow_run_artifacts(&env_vars.owner, &env_vars.repo, models::RunId(env_vars.run_id))
        .send()
        .await
        .context("Failed to fetch artifacts")?;

    let artifacts = response.value.unwrap().items;
    
    // Log all found artifacts
    info!("Found {} artifacts:", artifacts.len());
    for artifact in artifacts.iter() {
        info!("  - Name: {}", artifact.name);
        debug!("    ID: {}", artifact.id);
        debug!("    Size: {} bytes", artifact.size_in_bytes);
        debug!("    Created: {}", artifact.created_at);
    }

    Ok(artifacts.into_iter().map(|a| WorkflowArtifact {
        name: a.name,
        _other: (),
    }).collect())
}

fn check_artifact_status(artifacts: &[WorkflowArtifact]) -> ArtifactStatus {
    let has_dockerhub = artifacts.iter().any(|a| a.name == "dockerhub-artifacts");
    let has_bittorrent = artifacts.iter().any(|a| a.name == "bittorrent-artifacts");
    
    ArtifactStatus {
        has_dockerhub,
        has_bittorrent,
        all_present: has_dockerhub && has_bittorrent,
    }
}

struct WorkflowStatus {
    all_successful: bool,
    failure_reason: String,
}

async fn check_workflow_runs(octocrab: &Octocrab, env_vars: &EnvVars) -> Result<WorkflowStatus> {
    info!("🔍 Checking workflow runs on branch: {}", env_vars.branch);
    
    let dockerhub = get_workflow_run(octocrab, env_vars, "dockerhub-build-and-push.yml").await?;
    let bittorrent = get_workflow_run(octocrab, env_vars, "bittorrent-build-and-seed.yml").await?;

    // Log detailed workflow status
    match &dockerhub {
        Some(run) => {
            info!("DockerHub workflow status:");
            info!("  - Conclusion: {:?}", run.conclusion);
        }
        None => warn!("⚠️ No DockerHub workflow run found"),
    }

    match &bittorrent {
        Some(run) => {
            info!("BitTorrent workflow status:");
            info!("  - Conclusion: {:?}", run.conclusion);
        }
        None => warn!("⚠️ No BitTorrent workflow run found"),
    }

    Ok(validate_workflow_status(dockerhub, bittorrent))
}

async fn get_workflow_run(octocrab: &Octocrab, env_vars: &EnvVars, workflow: &str) -> Result<Option<WorkflowRun>> {
    info!("📋 Checking {} workflow", workflow);
    
    let runs = octocrab
        .workflows(&env_vars.owner, &env_vars.repo)
        .list_runs(workflow)
        .branch(&env_vars.branch)
        .per_page(1)
        .send()
        .await
        .with_context(|| format!("Failed to fetch {} workflow runs", workflow))?;

    let total_runs = runs.items.len();
    debug!("Found {} run(s) for {}", total_runs, workflow);

    if total_runs == 0 {
        warn!("⚠️ No runs found for workflow: {}", workflow);
    }

    Ok(runs.items.into_iter().next().map(|r| {
        debug!("Latest run conclusion: {:?}", r.conclusion);
        WorkflowRun {
            conclusion: r.conclusion,
            _other: (),
        }
    }))
}

fn validate_workflow_status(dockerhub: Option<WorkflowRun>, bittorrent: Option<WorkflowRun>) -> WorkflowStatus {
    match (dockerhub, bittorrent) {
        (Some(d), Some(b)) => {
            let d_success = d.conclusion.as_deref() == Some("success");
            let b_success = b.conclusion.as_deref() == Some("success");
            
            WorkflowStatus {
                all_successful: d_success && b_success,
                failure_reason: if !d_success || !b_success {
                    "One or more workflows failed".to_string()
                } else {
                    String::new()
                },
            }
        }
        _ => WorkflowStatus {
            all_successful: false,
            failure_reason: "Waiting for workflows to complete".to_string(),
        },
    }
}

fn set_success_output() -> Result<()> {
    set_output("can_proceed", "true")?;
    set_output("reason", "All checks passed")?;
    Ok(())
}

fn set_failure_output(reason: &str) -> Result<()> {
    set_output("can_proceed", "false")?;
    set_output("reason", reason)?;
    Ok(())
}

fn set_output(name: &str, value: &str) -> Result<()> {
    if let Ok(path) = env::var("GITHUB_OUTPUT") {
        std::fs::write(path, format!("{}={}\n", name, value))
            .with_context(|| format!("Failed to write {} output", name))?;
    }
    Ok(())
}

fn log_artifact_status(status: &ArtifactStatus) {
    info!(
        "📊 Artifact Status:\n- DockerHub: {}\n- BitTorrent: {}",
        if status.has_dockerhub { "✅" } else { "❌" },
        if status.has_bittorrent { "✅" } else { "❌" }
    );
}

fn log_workflow_status(status: &WorkflowStatus) {
    if status.all_successful {
        info!("✅ All workflows completed successfully");
    } else {
        warn!("⚠️ {}", status.failure_reason);
    }
}
