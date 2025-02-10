//! GitHub release creation script for create-release action
//! Used by: ./.github/actions/create-release/action.yml
//! Purpose: Creates GitHub release with assets

use anyhow::{Context, Result};
use chrono::Utc;
use octocrab::Octocrab;
use std::{env, fs};

#[derive(Debug)]
struct ReleaseInfo {
    tag_name: String,
    name: String,
    prerelease: bool,
    body: String,
}

impl ReleaseInfo {
    async fn new() -> Result<Self> {
        // Try multiple sources for version info
        let version = env::var("VALIDATED_VERSION")
            .or_else(|_| env::var("INPUT_VERSION"))
            .or_else(|_| env::var("INITIAL_VERSION"))
            .context("No version information found")?;

        // Default to true for prerelease if on beta branch
        let prerelease = env::var("INPUT_PRERELEASE")
            .map(|v| v.parse::<bool>().unwrap_or(false))
            .unwrap_or_else(|_| {
                env::var("GITHUB_REF")
                    .map(|r| r.contains("/beta"))
                    .unwrap_or(false)
            });

        let release_type = if prerelease { "Beta" } else { "Stable" };
        
        // Read checksum file
        let checksum = fs::read_to_string("checksum.txt")
            .context("Failed to read checksum.txt")?;

        let body = format!(
            r#"## Release Notes
            
### Distributions
This release includes:
- Complete development environment configuration
- Docker setup files for containerized usage
- Direct deployment scripts

### Installation
```bash
# Clone the repository
git clone https://github.com/user/dev-environment
cd dev-environment

# Run setup script
./setup.sh
```

### SHA-256 Checksums
```
{checksum}
```

### Release Details
- Type: {release_type}
- Version: {version}
- Release Date: {}"#, 
            Utc::now().format("%Y-%m-%d")
        );

        Ok(Self {
            tag_name: format!("v{version}"),
            name: format!("{release_type} Release v{version}"),
            prerelease,
            body,
        })
    }

    async fn create_release(&self) -> Result<()> {
        let token = env::var("GITHUB_TOKEN").context("GITHUB_TOKEN not set")?;
        let octocrab = Octocrab::builder()
            .personal_token(token)
            .build()
            .context("Failed to create GitHub client")?;

        // In GitHub Actions, we can use the current repository context
        let release = octocrab
            .repos(env::var("GITHUB_REPOSITORY_OWNER")?, env::var("GITHUB_REPOSITORY")?)
            .releases()
            .create(&self.tag_name)
            .name(&self.name)
            .body(&self.body)
            .draft(false)
            .prerelease(self.prerelease)
            .send()
            .await
            .context("Failed to create release")?;

        println!("Created release {} ({})", self.name, release.id);

        // Note: For uploading assets in GitHub Actions, it's better to use
        // the actions/upload-release-asset action instead of doing it here
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let release_info = ReleaseInfo::new().await?;
    println!("Creating release with info: {:?}", release_info);
    release_info.create_release().await?;
    Ok(())
}
