const VERSION: &str = env!("CARGO_PKG_VERSION");
const DOCKERFILE_TIMEOUT: u64 = 30;
const DOCKERHUB_TIMEOUT: u64 = 45; 
const TORRENT_TIMEOUT: u64 = 60;

// ...existing code...

async fn get_version() -> String {
    format!("v{}", VERSION)
}
