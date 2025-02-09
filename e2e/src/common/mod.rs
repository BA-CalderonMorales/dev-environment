use std::path::Path;
use anyhow::{Result, bail, Context};

// Download utilities
pub mod download {

    use super::*;
    
    pub async fn download_file(url: &str, path: &Path) -> Result<()> {
        let response = reqwest::get(url).await
            .context("Failed to download file")?;
            
        if !response.status().is_success() {
            bail!("Download failed with status: {}", response.status());
        }
        
        let content = response.bytes().await
            .context("Failed to read response body")?;
            
        std::fs::write(path, content)
            .context("Failed to write downloaded file")?;
        
        Ok(())
    }

}

// Environment validation
pub mod environment {

}
