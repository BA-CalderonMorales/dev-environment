//! Release queue management script
//! 
//! Used by: ./.github/actions/queue-release/action.yml
//! Purpose: Adds a release to the queue for processing

use anyhow::{Context, Result};
use github_workflow_scripts::{init, get_logger, github};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

/// Queue entry structure for releases
#[derive(Debug, Serialize, Deserialize)]
struct QueueEntry {
    sha: String,
    branch: String,
    timestamp: u64,
    status: String,
    estimated_time: String,
}

/// Queue manager to encapsulate queue operations
struct QueueManager {
    logger: Box<dyn github_workflow_scripts::Logger>,
    // Removed unused queue_dir field
    queue_file: String,
}

impl QueueManager {
    /// Create a new queue manager
    fn new() -> Result<Self> {
        let logger = get_logger(false);
        
        // Ensure queue directory exists
        let queue_dir = ".github/release_queue";
        if !Path::new(queue_dir).exists() {
            fs::create_dir_all(queue_dir).context("Failed to create queue directory")?;
            logger.info("Created queue directory");
        }
        
        // Queue file path
        let queue_file = format!("{}/queue.json", queue_dir);
        
        // Create queue file if it doesn't exist
        if !Path::new(&queue_file).exists() {
            File::create(&queue_file).context("Failed to create queue file")?;
            logger.info("Created new release queue file");
        }
        
        Ok(Self {
            logger,
            // Removed queue_dir field
            queue_file,
        })
    }
    
    /// Load the queue from disk
    fn load_queue(&self) -> Result<Vec<QueueEntry>> {
        // Check if file exists and is not empty
        let file = File::open(&self.queue_file)?;
        let reader = BufReader::new(file);
        
        let mut entries = Vec::new();
        
        // Try to read as JSON array
        if let Ok(json_entries) = serde_json::from_reader::<_, Vec<QueueEntry>>(reader) {
            entries = json_entries;
        } else {
            // If not valid JSON, try line by line
            let file = File::open(&self.queue_file)?;
            let reader = BufReader::new(file);
            
            for line in reader.lines() {
                let line = line?;
                if line.trim().is_empty() {
                    continue;
                }
                
                if let Ok(entry) = serde_json::from_str(&line) {
                    entries.push(entry);
                }
            }
        }
        
        Ok(entries)
    }
    
    /// Save the queue to disk
    fn save_queue(&self, queue: &[QueueEntry]) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.queue_file)?;
            
        let json = serde_json::to_string_pretty(queue)?;
        file.write_all(json.as_bytes())?;
        
        Ok(())
    }
    
    /// Add or update an entry in the queue
    fn process_entry(&self, sha: &str, branch: &str) -> Result<(usize, String)> {
        // Load existing queue
        let mut queue = self.load_queue()?;
        
        let current_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Check if entry already exists
        if queue.iter().any(|entry| entry.sha == sha) {
            self.logger.info(&format!("Commit {} already in queue, updating timestamp", sha));
            // Update entry
            for entry in &mut queue {
                if entry.sha == sha {
                    entry.timestamp = current_timestamp;
                }
            }
        } else {
            // Create new entry
            let new_entry = QueueEntry {
                sha: sha.to_string(),
                branch: branch.to_string(),
                timestamp: current_timestamp,
                status: "pending".to_string(),
                estimated_time: "".to_string(),
            };
            
            // Add to queue
            queue.push(new_entry);
            self.logger.info(&format!("Added commit {} to release queue in position {}", 
                                     sha, queue.len()));
        }
        
        // Re-calculate estimated times
        self.update_estimated_times(&mut queue);
        
        // Save updated queue
        self.save_queue(&queue)?;
        
        // Calculate position
        let position = queue.iter()
            .position(|entry| entry.sha == sha)
            .unwrap_or(0) + 1;
        
        // Calculate estimated time
        let estimated_time = if position > 1 {
            format!("{} minutes", (position - 1) * 15)
        } else {
            "Next in queue".to_string()
        };
        
        Ok((position, estimated_time))
    }
    
    /// Update estimated times for all entries in the queue
    fn update_estimated_times(&self, queue: &mut Vec<QueueEntry>) {
        let mut position = 1;
        for entry in queue {
            // Simple estimation: 15 minutes per release
            let estimated_minutes = (position - 1) * 15;
            entry.estimated_time = format_time(estimated_minutes);
            position += 1;
        }
    }
}

/// Formats minutes into a human-readable time string
fn format_time(minutes: usize) -> String {
    if minutes == 0 {
        return "Next in queue".to_string();
    }
    
    if minutes < 60 {
        return format!("{} minutes", minutes);
    }
    
    let hours = minutes / 60;
    let remaining_minutes = minutes % 60;
    
    if remaining_minutes == 0 {
        return format!("{} hour{}", hours, if hours > 1 { "s" } else { "" });
    }
    
    format!("{} hour{} {} minute{}", 
        hours, 
        if hours > 1 { "s" } else { "" },
        remaining_minutes,
        if remaining_minutes > 1 { "s" } else { "" }
    )
}

/// Validate the branch name
fn validate_branch(branch: &str, logger: &Box<dyn github_workflow_scripts::Logger>) -> bool {
    if branch != "beta" && branch != "main" {
        logger.warn(&format!("Invalid branch: {}. Queue is only for beta/main branches.", branch));
        return false;
    }
    true
}

/// Read required input parameters
fn read_input_parameters() -> Result<(String, String)> {
    let sha = env::var("INPUT_SHA").context("Missing INPUT_SHA")?;
    let branch = env::var("INPUT_BRANCH").context("Missing INPUT_BRANCH")?;
    
    Ok((sha, branch))
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    init();
    let logger = get_logger(false);
    
    logger.info("🔄 Processing release queue request...");
    
    // Get required parameters
    let (sha, branch) = read_input_parameters()?;
    
    // Validate branch
    if !validate_branch(&branch, &logger) {
        return Ok(());
    }
    
    // Initialize queue manager
    let queue_manager = QueueManager::new()?;
    
    // Process the queue entry
    let (position, estimated_time) = queue_manager.process_entry(&sha, &branch)?;
    
    // Set outputs for GitHub Actions
    github::set_output("queue_position", &position.to_string());
    github::set_output("estimated_time", &estimated_time);
    
    logger.info(&format!("✅ Release queued at position {} with estimated time: {}", 
        position, estimated_time));
    
    Ok(())
}
