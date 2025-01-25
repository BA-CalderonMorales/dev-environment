use std::sync::Once;
use tracing::{debug, info, warn, Level};

static INIT: Once = Once::new();

pub trait Logger {
    fn debug(&self, message: &str);
    fn info(&self, message: &str);
    fn warn(&self, message: &str);
}

pub struct TracingLogger;

impl Logger for TracingLogger {
    fn debug(&self, message: &str) {
        debug!("{}", message);
    }

    fn info(&self, message: &str) {
        info!("{}", message);
    }

    fn warn(&self, message: &str) {
        warn!("{}", message);
    }
}

pub struct MockLogger;

impl Logger for MockLogger {
    fn debug(&self, message: &str) {
        println!("DEBUG: {}", message);
    }

    fn info(&self, message: &str) {
        println!("INFO: {}", message);
    }

    fn warn(&self, message: &str) {
        println!("WARN: {}", message);
    }
}

pub fn init_logging() {
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();
    });
}

pub fn get_logger(is_local: bool) -> Box<dyn Logger> {
    if is_local {
        Box::new(MockLogger)
    } else {
        Box::new(TracingLogger)
    }
} 