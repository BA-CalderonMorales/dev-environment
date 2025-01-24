pub trait Logger: Send + Sync {
    fn debug(&self, message: &str);
    fn info(&self, message: &str);
    fn warn(&self, message: &str);
    fn error(&self, message: &str);
}

pub mod common;
pub mod distribution;
pub mod ide;
pub mod startup;

pub use distribution::DistributionTest;
pub use ide::IdeTest;
pub use startup::StartupTest;

pub mod test_utils {
    use crate::{common, Logger};
    
    pub fn validate_test_environment(logger: &dyn Logger) -> Vec<String> {
        // Only validate environment in non-CI contexts
        if std::env::var("CI").is_err() {
            common::env::validate_environment(logger)
        } else {
            Vec::new()
        }
    }
}
