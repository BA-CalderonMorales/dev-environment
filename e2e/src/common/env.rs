use crate::Logger;

pub fn validate_environment(logger: &dyn Logger) -> Vec<String> {
    let mut warnings = Vec::new();
    
    let required_vars = vec![
        ("GITHUB_TOKEN", "GitHub token for API access"),
        ("GOPATH", "Go workspace path"),
        ("NODE_PATH", "Node.js modules path")
    ];

    for (var, description) in required_vars {
        if std::env::var(var).is_err() {
            let warning = format!("Environment variable {} not set ({})", var, description);
            logger.warn(&warning);
            warnings.push(warning);
        }
    }

    warnings
}
