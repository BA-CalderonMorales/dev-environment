//! Debug inputs script
//! 
//! Purpose: Prints all environment variables for debugging purposes

use github_workflow_scripts::{init, get_logger};
use std::env;
use anyhow::Result;

/// Struct to handle debug operations
struct DebugHelper {
    logger: Box<dyn github_workflow_scripts::Logger>,
    verbose: bool,
}

impl DebugHelper {
    /// Create a new debug helper
    fn new(verbose: bool) -> Self {
        let logger = get_logger(verbose);
        Self { logger, verbose }
    }
    
    /// Display all input environment variables
    fn display_input_variables(&self) -> Result<()> {
        self.logger.info("📥 Checking for INPUT_ environment variables...");
        
        // Find all environment variables that start with INPUT_
        let vars: Vec<(String, String)> = env::vars()
            .filter(|(key, _)| key.starts_with("INPUT_"))
            .collect();
            
        if vars.is_empty() {
            self.logger.warn("No INPUT_ environment variables found!");
            return Ok(());
        }
        
        self.logger.info(&format!("Found {} input variables:", vars.len()));
        
        // Print each variable with masking for sensitive values
        for (key, value) in vars {
            let display_value = self.mask_sensitive_value(&key, &value);
            self.logger.info(&format!("  {}: {}", key, display_value));
        }
        
        Ok(())
    }
    
    /// Display GitHub context variables
    fn display_github_context(&self) -> Result<()> {
        self.logger.info("📋 GitHub Context Variables:");
        
        let github_vars = [
            "GITHUB_ACTOR", "GITHUB_REF", "GITHUB_SHA", "GITHUB_REPOSITORY", 
            "GITHUB_WORKFLOW", "GITHUB_EVENT_NAME", "GITHUB_JOB", 
            "GITHUB_RUN_ID", "GITHUB_RUN_NUMBER"
        ];
        
        let mut found = false;
        
        for var in &github_vars {
            if let Ok(value) = env::var(var) {
                found = true;
                self.logger.info(&format!("  {}: {}", var, value));
            }
        }
        
        if !found {
            self.logger.warn("No GitHub context variables found!");
        }
        
        Ok(())
    }
    
    /// Display action environment variables
    fn display_action_variables(&self) -> Result<()> {
        self.logger.info("🔧 Action-specific Environment Variables:");
        
        let action_vars = [
            "ACTIONS_RUNNER_DEBUG", "ACTIONS_STEP_DEBUG", 
            "GITHUB_ACTION", "GITHUB_ACTION_PATH", "RUNNER_OS", 
            "RUNNER_TOOL_CACHE", "RUNNER_TEMP"
        ];
        
        let mut found = false;
        
        for var in &action_vars {
            if let Ok(value) = env::var(var) {
                found = true;
                self.logger.info(&format!("  {}: {}", var, value));
            }
        }
        
        if !found && self.verbose {
            self.logger.warn("No action-specific environment variables found!");
        }
        
        Ok(())
    }
    
    /// Mask sensitive values in environment variables
    fn mask_sensitive_value(&self, key: &str, value: &str) -> String {
        // Check if this is likely a sensitive value that should be masked
        if key.contains("TOKEN") || 
           key.contains("KEY") || 
           key.contains("PASSWORD") || 
           key.contains("SECRET") ||
           key.contains("PASSPHRASE") {
            return "[REDACTED]".to_string();
        }
        
        // Check for potential secrets in the value itself
        if value.len() > 20 && (
            value.contains("eyJ") || // Potential JWT token
            value.contains("-----BEGIN") || // PEM key 
            value.contains("ghp_") || // GitHub Personal Access Token
            value.contains("xox") // Slack token
        ) {
            return "[POTENTIAL SECRET REDACTED]".to_string();
        }
        
        value.to_string()
    }
    
    /// Run all debug checks
    fn run_all_checks(&self) -> Result<()> {
        self.logger.info("🔍 Starting environment variable debugging");
        
        // Display input variables
        self.display_input_variables()?;
        
        // Display GitHub context
        self.display_github_context()?;
        
        // Display action variables if verbose
        if self.verbose {
            self.display_action_variables()?;
        }
        
        self.logger.info("✅ Debug information complete");
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    init();
    
    // Create debug helper with verbose mode enabled
    let debug_helper = DebugHelper::new(true);
    
    // Run all debug checks
    debug_helper.run_all_checks()?;
    
    Ok(())
}
