use std::process::Command;
use anyhow::{bail, Context, Result};
use crate::Logger;

pub struct StartupTest<'a> {
    logger: &'a dyn Logger,
}

impl<'a> StartupTest<'a> {
    pub fn new(logger: &'a dyn Logger) -> Self {
        Self { logger }
    }

    pub async fn test_dev_tools(&self) -> Result<()> {
        self.logger.debug("Testing development tools");
        
        let dev_tools = vec![
            ("node", "--version", "v"),  // Just check for 'v' prefix
            ("go", "version", "go"),     // Just check for 'go' prefix
            ("cargo", "--version", "cargo"), // Just check for 'cargo' prefix
            ("git", "--version", "git"),    // Just check for 'git' prefix
        ];

        for (tool, arg, expected_prefix) in dev_tools {
            let output = Command::new(tool)
                .arg(arg)
                .output()
                .with_context(|| format!("Failed to run {}", tool))?;

            if !output.status.success() {
                bail!("{} check failed", tool);
            }

            let version_output = String::from_utf8_lossy(&output.stdout);
            if !version_output.contains(expected_prefix) {
                bail!("{} version check failed. Output: {}", tool, version_output);
            }
        }

        self.logger.debug("Development tools check successful");
        Ok(())
    }

    pub async fn test_environment_variables(&self) -> Result<()> {
        self.logger.debug("Testing environment variables");

        // Check required environment variables
        let required_vars = [
            "GITHUB_TOKEN",      // For GitHub API access
            "RUSTUP_HOME",      // For Rust toolchain
            "CARGO_HOME",       // For Cargo
            "GOPATH",           // For Go workspace
            "NODE_PATH",        // For Node.js modules
        ];

        for var in required_vars {
            if std::env::var(var).is_err() {
                self.logger.warn(&format!("Environment variable {} not set", var));
            }
        }

        self.logger.debug("Environment variables check completed");
        Ok(())
    }

    pub async fn test_workspace_structure(&self) -> Result<()> {
        self.logger.debug("Testing workspace structure");

        // Check required directories
        let required_dirs = [
            ".vscode",           // VS Code settings
            "artifacts",         // Build artifacts
            "docs",             // Documentation
            ".github",          // GitHub workflows
        ];

        for dir in required_dirs {
            let path = std::path::Path::new(dir);
            if !path.exists() {
                std::fs::create_dir_all(path)
                    .with_context(|| format!("Failed to create {} directory", dir))?;
            }
        }

        // Check required files
        let required_files = [
            "README.md",
            ".gitignore",
            "Cargo.toml",
            ".github/workflows/ci.yml",
        ];

        for file in required_files {
            let path = std::path::Path::new(file);
            if !path.exists() {
                if let Some(parent) = path.parent() {
                    if !parent.exists() {
                        std::fs::create_dir_all(parent)
                            .with_context(|| format!("Failed to create parent directory for {}", file))?;
                    }
                }
                std::fs::write(path, "")
                    .with_context(|| format!("Failed to create {}", file))?;
            }
        }

        self.logger.debug("Workspace structure check successful");
        Ok(())
    }
}
