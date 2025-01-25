use std::process::Command;
use anyhow::{bail, Context, Result};
use crate::Logger;

pub struct IdeTest<'a> {
    logger: &'a dyn Logger,
}

impl<'a> IdeTest<'a> {
    pub fn new(logger: &'a dyn Logger) -> Self {
        Self { logger }
    }

    pub async fn test_vscode_integration(&self) -> Result<()> {
        self.logger.debug("Testing VS Code integration");
        
        // Skip VS Code check in CI environment
        if std::env::var("CI").is_ok() {
            self.logger.info("Skipping VS Code check in CI environment");
            return Ok(());
        }

        let code_path = which::which("code")
            .context("VS Code CLI not found. VS Code must be installed with the 'code' command available in PATH")?;
        
        let version_check = Command::new(code_path)
            .arg("--version")
            .output()
            .context("Failed to execute VS Code CLI")?;

        if !version_check.status.success() {
            bail!("VS Code CLI check failed: {}", 
                String::from_utf8_lossy(&version_check.stderr));
        }

        self.logger.debug("VS Code integration successful");
        Ok(())
    }

    pub async fn test_extensions(&self) -> Result<()> {
        self.logger.debug("Testing VS Code extensions");

        // Skip extension check in CI environment
        if std::env::var("CI").is_ok() {
            self.logger.info("Skipping extension check in CI environment");
            return Ok(());
        }

        let code_path = which::which("code")
            .context("VS Code CLI not found")?;

        // Check for required extensions
        let required_extensions = [
            "rust-lang.rust-analyzer",
            "vadimcn.vscode-lldb",
            "serayuzgur.crates",
        ];

        for ext in required_extensions {
            let output = Command::new(&code_path)
                .args(["--list-extensions"])
                .output()
                .context("Failed to list VS Code extensions")?;

            let extensions = String::from_utf8_lossy(&output.stdout);
            if !extensions.contains(ext) {
                bail!("Required extension not found: {}", ext);
            }
        }

        self.logger.debug("VS Code extensions check successful");
        Ok(())
    }

    pub async fn test_settings(&self) -> Result<()> {
        self.logger.debug("Testing VS Code settings");

        // Check for .vscode directory and settings.json
        let vscode_dir = std::path::Path::new(".vscode");
        if !vscode_dir.exists() {
            std::fs::create_dir_all(vscode_dir)
                .context("Failed to create .vscode directory")?;
        }

        let settings_file = vscode_dir.join("settings.json");
        if !settings_file.exists() {
            std::fs::write(&settings_file, "{}")
                .context("Failed to create settings.json")?;
        }

        self.logger.debug("VS Code settings check successful");
        Ok(())
    }
}
