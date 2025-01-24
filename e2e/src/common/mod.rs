pub trait Logger: Send + Sync {

    fn debug(&self, message: &str);

    fn info(&self, message: &str);

    #[allow(dead_code)]
    fn warn(&self, message: &str);

    fn error(&self, message: &str);
}
