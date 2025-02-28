//! GitHub Workflow Scripts
//! 
//! This crate provides Rust utilities for GitHub Actions workflow scripts
//! with proper logging, error handling, and GitHub Actions integration.

// Re-export modules for ease of use
pub mod logger;
pub mod github;

use chrono::Utc;

// Custom log level enum to replace external dependency
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Error => "ERROR",
            LogLevel::Warn  => "WARN ",
            LogLevel::Info  => "INFO ",
            LogLevel::Debug => "DEBUG",
            LogLevel::Trace => "TRACE",
        }
    }
    
    fn from_env() -> Self {
        match std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()).to_lowercase().as_str() {
            "error" => LogLevel::Error,
            "warn"  => LogLevel::Warn,
            "info"  => LogLevel::Info, 
            "debug" => LogLevel::Debug,
            "trace" => LogLevel::Trace,
            _       => LogLevel::Info, // Default to Info
        }
    }
}

// Current global log level (static variable)
static mut CURRENT_LOG_LEVEL: LogLevel = LogLevel::Info;

/// Initialize logging for workflow scripts
pub fn init() {
    // Set log level from environment
    unsafe {
        CURRENT_LOG_LEVEL = LogLevel::from_env();
    }
}

/// Log a message at the specified level
pub fn log(level: LogLevel, message: &str) {
    // Skip if message level is higher than current level
    if !should_log(level) {
        return;
    }
    
    // Format timestamp
    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3f");
    
    // Print to stdout with formatting
    println!(
        "{:>20}Z  {}  {}",
        timestamp,
        level.as_str(),
        message
    );
    
    // Also emit as GitHub annotation for warnings and errors
    match level {
        LogLevel::Error => println!("::error::{}", message),
        LogLevel::Warn => println!("::warning::{}", message),
        _ => {}
    }
}

// Check if a message at the given level should be logged
fn should_log(level: LogLevel) -> bool {
    unsafe {
        (level as u8) <= (CURRENT_LOG_LEVEL as u8)
    }
}

// Logger trait and implementations

/// Common logger trait for consistent logging interface
pub trait Logger {
    fn info(&self, message: &str);
    fn warn(&self, message: &str);
    fn error(&self, message: &str);
    fn debug(&self, message: &str);
}

/// Standard logger implementation
struct StandardLogger {
    verbose: bool,
}

impl Logger for StandardLogger {
    fn info(&self, message: &str) {
        log(LogLevel::Info, message);
    }

    fn warn(&self, message: &str) {
        log(LogLevel::Warn, message);
    }

    fn error(&self, message: &str) {
        log(LogLevel::Error, message);
    }

    fn debug(&self, message: &str) {
        if self.verbose {
            log(LogLevel::Debug, message);
        }
    }
}

/// Get a logger instance for workflow scripts
pub fn get_logger(verbose: bool) -> Box<dyn Logger> {
    // If verbose, set log level to Debug when requested
    if verbose {
        unsafe {
            CURRENT_LOG_LEVEL = LogLevel::Debug;
        }
    }
    
    Box::new(StandardLogger { verbose })
}