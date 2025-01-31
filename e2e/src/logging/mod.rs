pub trait Logger {
    fn debug(&self, message: &str);
    fn info(&self, message: &str);
    fn warn(&self, message: &str);
    fn error(&self, message: &str);
    fn log(&self, message: &str) {
        self.info(message)
    }
}

impl Logger for Box<dyn Logger> {
    fn debug(&self, message: &str) {
        (**self).debug(message)
    }

    fn info(&self, message: &str) {
        (**self).info(message)
    }

    fn warn(&self, message: &str) {
        (**self).warn(message)
    }

    fn error(&self, message: &str) {
        (**self).error(message)
    }

    fn log(&self, message: &str) {
        (**self).log(message)
    }
}

pub struct TestLogger;

impl Logger for TestLogger {
    fn debug(&self, message: &str) { println!("[TEST-DEBUG] {}", message); }
    fn info(&self, message: &str) { println!("[TEST] {}", message); }
    fn warn(&self, message: &str) { println!("[TEST-WARN] {}", message); }
    fn error(&self, message: &str) { println!("[TEST-ERROR] {}", message); }
}

pub struct ConsoleLogger {
    is_local: bool,
}

impl ConsoleLogger {
    pub fn new(is_local: bool) -> Self {
        Self { is_local }
    }
}

impl Logger for ConsoleLogger {
    fn debug(&self, message: &str) {
        if self.is_local {
            println!("🔍 DEBUG: {}", message);
        } else {
            println!("::debug::{}", message);
        }
    }

    fn info(&self, message: &str) {
        println!("{}", message);
    }

    fn warn(&self, message: &str) {
        if self.is_local {
            println!("⚠️  WARN: {}", message);
        } else {
            println!("::warning::{}", message);
        }
    }

    fn error(&self, message: &str) {
        if self.is_local {
            println!("❌ ERROR: {}", message);
        } else {
            println!("::error::{}", message);
        }
    }
}

pub fn init_logging() -> Box<dyn Logger> {
    Box::new(ConsoleLogger::new(std::env::var("GITHUB_ACTIONS").is_err()))
}

pub fn get_logger() -> Box<dyn Logger> {
    Box::new(TestLogger)
}
