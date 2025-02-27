use std::sync::Once;
use tracing::{debug, info, warn, Level};
use tracing_subscriber::fmt;

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

pub fn init() {
    INIT.call_once(|| {
        let log_level = std::env::var("RUST_LOG")
            .unwrap_or_else(|_| "info".to_string());
            
        fmt()
            .with_max_level(match log_level.as_str() {
                "debug" => Level::DEBUG,
                "warn" => Level::WARN,
                _ => Level::INFO,
            })
            .with_target(false)
            .with_thread_ids(false)
            .with_file(true)
            .with_line_number(true)
            .pretty()
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
