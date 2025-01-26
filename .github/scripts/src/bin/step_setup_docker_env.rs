use anyhow::{Context, Result};
use github_workflow_scripts::{get_logger, init_logging};
use std::process::Command;

fn run_command(cmd: &str, args: &[&str]) -> Result<()> {
    Command::new(cmd)
        .args(args)
        .status()
        .with_context(|| format!("Failed to execute: {} {:?}", cmd, args))?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();
    let logger = get_logger(false);

    logger.info("Setting up Docker environment...");

    // Clean previous installations
    run_command("sudo", &["apt-get", "remove", "-y", "docker", "docker-engine", "docker.io", "containerd", "runc"])?;
    run_command("sudo", &["apt-get", "autoremove", "-y"])?;
    
    // Install prerequisites
    run_command("sudo", &["apt-get", "update"])?;
    run_command("sudo", &["apt-get", "install", "-y", "ca-certificates", "curl", "gnupg"])?;

    // Add Docker's GPG key
    run_command("sudo", &["install", "-m", "0755", "-d", "/etc/apt/keyrings"])?;
    
    let gpg_cmd = "curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg";
    Command::new("bash")
        .arg("-c")
        .arg(gpg_cmd)
        .status()
        .context("Failed to add Docker's GPG key")?;

    // Set up repository
    let repo_cmd = r#"echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null"#;
    Command::new("bash")
        .arg("-c")
        .arg(repo_cmd)
        .status()
        .context("Failed to set up Docker repository")?;

    // Install Docker Engine
    run_command("sudo", &["apt-get", "update"])?;
    let packages = ["docker-ce", "docker-ce-cli", "containerd.io", "docker-buildx-plugin"];
    for pkg in packages {
        logger.info(&format!("Installing {}", pkg));
        run_command("sudo", &["apt-get", "install", "-y", pkg])?;
    }

    // Verify installation
    let version = Command::new("docker")
        .arg("--version")
        .output()
        .context("Failed to get Docker version")?;
    
    logger.info(&format!("Docker setup complete: {}", 
        String::from_utf8_lossy(&version.stdout).trim()));
    Ok(())
}
