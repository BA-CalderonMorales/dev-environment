use anyhow::{Context, Result};
use github_workflow_scripts::{get_logger, init};
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::Path;

// Data models for the queue
#[derive(Debug, Serialize, Deserialize)]
struct QueueItem {
    commit: String,
    date: String,
    pr: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct QueueFile {
    items: Vec<QueueItem>,
}

// Main manager for queue operations
struct QueueManager {
    logger: Box<dyn github_workflow_scripts::Logger>,
    branch: String,
    // Removed unused github_token field
}

impl QueueManager {
    // Create a new queue manager instance
    fn new() -> Result<Self> {
        let logger = get_logger(false);
        
        // Get required environment variables
        let branch = env::var("INPUT_BRANCH").context("Missing branch input")?;
        // We still read the token to validate it exists, but don't store it since we don't use it
        let _github_token = env::var("GITHUB_TOKEN").context("Missing GitHub token")?;
        
        Ok(Self {
            logger,
            branch,
        })
    }
    
    // Get the file path for the queue based on branch
    fn get_queue_file_path(&self) -> String {
        format!(".github/release_queue/{}.json", self.branch)
    }
    
    // Ensure the directory exists for the queue file
    fn ensure_directory_exists(&self) -> Result<()> {
        let path = Path::new(".github/release_queue");
        if !path.exists() {
            create_dir_all(path).context("Failed to create release_queue directory")?;
        }
        Ok(())
    }
    
    // Load the queue from the file system
    fn load_queue(&self) -> Result<QueueFile> {
        let path = self.get_queue_file_path();
        let file_path = Path::new(&path);
        
        // If file doesn't exist, return an empty queue
        if !file_path.exists() {
            return Ok(QueueFile { items: Vec::new() });
        }
        
        // Read and parse the file
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        let queue: QueueFile = serde_json::from_str(&contents)
            .context("Failed to parse queue file")?;
            
        Ok(queue)
    }
    
    // Save the queue to file
    fn save_queue(&self, queue: &QueueFile) -> Result<()> {
        self.ensure_directory_exists()?;
        let path = self.get_queue_file_path();
        let serialized = serde_json::to_string_pretty(queue)
            .context("Failed to serialize queue data")?;
            
        let mut file = File::create(path)?;
        file.write_all(serialized.as_bytes())?;
        
        Ok(())
    }
    
    // Clear processed commits from the queue
    fn clear_processed_commits(&self, processed_sha: &str) -> Result<()> {
        let mut queue = self.load_queue()?;
        
        // Find the position of the processed commit
        if let Some(pos) = queue.items.iter().position(|item| item.commit == processed_sha) {
            self.logger.info(&format!("Removing processed commit {} and all older items", processed_sha));
            
            // Remove this commit and all older ones
            queue.items.drain(0..=pos);
            
            self.save_queue(&queue)?;
            self.logger.info(&format!("{} items remain in the queue", queue.items.len()));
        } else {
            self.logger.warn(&format!("Processed commit {} not found in queue", processed_sha));
        }
        
        Ok(())
    }
    
    // Add a commit to the queue
    fn add_commit_to_queue(&self, sha: &str, pr_number: Option<u64>) -> Result<()> {
        let mut queue = self.load_queue()?;
        
        // Check if SHA already exists
        if queue.items.iter().any(|item| item.commit == sha) {
            self.logger.info(&format!("Commit {} already in queue, skipping", sha));
            return Ok(());
        }
        
        // Add new item
        let timestamp = chrono::Utc::now().to_rfc3339();
        queue.items.push(QueueItem {
            commit: sha.to_string(),
            date: timestamp,
            pr: pr_number,
        });
        
        self.save_queue(&queue)?;
        self.logger.info(&format!("Added commit {} to queue position {}", sha, queue.items.len()));
        
        Ok(())
    }
    
    // Get status information about the queue
    fn get_queue_status(&self) -> Result<(usize, String)> {
        let queue = self.load_queue()?;
        
        // Get queue position
        let position = queue.items.len();
        
        // Create an empty string to use as default
        let empty_date = String::new();
        
        // Fix the borrowing issue by creating a longer-lived value
        let oldest = queue.items
            .first()
            .map(|item| &item.date)
            .unwrap_or(&empty_date);
        
        Ok((position, oldest.clone()))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    init();
    
    let manager = QueueManager::new()?;
    let action = env::var("INPUT_ACTION").unwrap_or_else(|_| "status".to_string());
    
    // Process the requested action
    match action.as_str() {
        "add" => {
            let sha = env::var("INPUT_SHA").context("Missing SHA input for add action")?;
            let pr_number = env::var("INPUT_PR_NUMBER").ok().and_then(|s| s.parse().ok());
            manager.add_commit_to_queue(&sha, pr_number)?;
        },
        "clear" => {
            let processed_sha = env::var("INPUT_PROCESSED_SHA").context("Missing processed SHA")?;
            manager.clear_processed_commits(&processed_sha)?;
        },
        "status" | _ => {
            let (count, oldest) = manager.get_queue_status()?;
            println!("::set-output name=count::{}", count);
            println!("::set-output name=oldest::{}", oldest);
        }
    }
    
    Ok(())
}
