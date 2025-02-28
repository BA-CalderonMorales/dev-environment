//! GitHub Actions helper functions for workflow scripts
//! 
//! This module provides utilities for interacting with GitHub Actions environment,
//! such as setting outputs and managing workflow commands.

use std::io::Write;
use crate::log;
use crate::LogLevel;

/// Sets an output parameter for GitHub Actions
/// 
/// This writes to GITHUB_OUTPUT environment file according to GitHub Actions workflow command format
/// See: https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions#setting-an-output-parameter
/// 
/// # Arguments
///
/// * `name` - The name of the output parameter
/// * `value` - The value to set for the output parameter
///
/// # Examples
///
/// ```
/// use github_workflow_scripts::github;
/// 
/// // Set an output parameter named "result" with value "success"
/// github::set_output("result", "success");
/// ```
pub fn set_output(name: &str, value: &str) {
    if let Ok(path) = std::env::var("GITHUB_OUTPUT") {
        // Use GitHub Actions output file approach (preferred)
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(path)
            .unwrap_or_else(|_| panic!("Failed to open GITHUB_OUTPUT file"));
            
        writeln!(file, "{}={}", name, value)
            .unwrap_or_else(|_| panic!("Failed to write to GITHUB_OUTPUT file"));
            
        // Also log the action
        log(LogLevel::Debug, &format!("Setting output parameter {}={}", name, value));
    } else {
        // Fallback to deprecated echo approach
        println!("::set-output name={}::{}", name, value);
    }
}

/// Sets an environment variable for the current and future steps in a workflow
///
/// # Arguments
///
/// * `name` - The name of the environment variable
/// * `value` - The value to set for the environment variable
pub fn set_env(name: &str, value: &str) {
    if let Ok(path) = std::env::var("GITHUB_ENV") {
        // Use GitHub Actions environment file approach (preferred)
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(path)
            .unwrap_or_else(|_| panic!("Failed to open GITHUB_ENV file"));
            
        writeln!(file, "{}={}", name, value)
            .unwrap_or_else(|_| panic!("Failed to write to GITHUB_ENV file"));
            
        // Also log the action
        log(LogLevel::Debug, &format!("Setting environment variable {}={}", name, value));
    } else {
        // Fallback to deprecated echo approach
        println!("::set-env name={}::{}", name, value);
    }
}

/// Outputs a debug message to the GitHub Actions log
///
/// # Arguments
///
/// * `message` - The debug message to output
pub fn debug(message: &str) {
    log(LogLevel::Debug, message);
    println!("::debug::{}", message);
}

/// Outputs a warning message to the GitHub Actions log
///
/// # Arguments
///
/// * `message` - The warning message to output
pub fn warning(message: &str) {
    log(LogLevel::Warn, message);
    println!("::warning::{}", message);
}

/// Outputs an error message to the GitHub Actions log
///
/// # Arguments
///
/// * `message` - The error message to output
pub fn error(message: &str) {
    log(LogLevel::Error, message);
    println!("::error::{}", message);
}

/// Groups log output under an expandable section in the GitHub Actions log
///
/// # Arguments
///
/// * `title` - The title of the group
pub fn group_start(title: &str) {
    println!("::group::{}", title);
    log(LogLevel::Info, &format!("Group: {}", title));
}

/// Ends the current output group
pub fn group_end() {
    println!("::endgroup::");
}
