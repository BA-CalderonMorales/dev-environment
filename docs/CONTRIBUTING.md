# Maintainer Documentation

This guide is for developers who want to contribute to or maintain the dev-environment project.

## Project Structure
```
dev-environment/
├── cli/                   # Setup CLI tool
├── distributions/         # Distribution methods
├── e2e/                  # End-to-end tests
└── startup/              # Environment scripts
```

## Development Requirements
- Rust toolchain (stable)
- Docker
- Git

## Setting Up Development Environment
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone repository
git clone https://github.com/yourusername/dev-environment.git
cd dev-environment

# Run tests
cd e2e
cargo test
```

## Workflow
1. Create feature branch
2. Make changes
3. Run E2E tests
4. Submit PR

## CI/CD Pipeline
- Automated testing
- Version management
- Release creation

See [WORKFLOWS.md](WORKFLOWS.md) for detailed CI/CD documentation.

## Testing
See [E2E_TESTS.md](E2E_TESTS.md) for test suite documentation. 