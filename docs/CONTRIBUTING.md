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

## Development Workflow

### Branching Strategy
1. Create feature branch from `develop`:
   ```bash
   git checkout develop
   git checkout -b feature/your-feature
   ```

2. Make changes and test locally:
   ```bash
   # Run E2E tests
   cd e2e
   cargo test
   ```

3. Push changes and create PR:
   - PRs to `develop` create `:dev` tag
   - PRs to `beta` create `:beta` tag
   - PRs to `main` create `:latest` tag

4. CI/CD Pipeline:
   - Automated testing
   - Docker image builds
   - Security scans
   - Documentation checks

See [WORKFLOWS.md](WORKFLOWS.md) for detailed CI/CD documentation.

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