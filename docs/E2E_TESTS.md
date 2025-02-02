# E2E Testing Documentation

## Overview

End-to-End (E2E) tests ensure our development environment works correctly across different distribution methods. Our tests validate both creator and user workflows with detailed reporting and timeouts.

## Test Structure

All E2E tests are now located in the `e2e/` directory at the root of the repository. The tests are implemented in Rust and organized into the following modules:

- `distribution/`: Tests for different distribution methods (DockerHub, Direct Download)
- `startup/`: Tests for the environment startup process
- `ide/`: Tests for IDE integrations

This centralized structure allows us to maintain all E2E tests in one place, using a consistent language and framework.

## Running Tests

### Prerequisites
- Rust toolchain
- Docker
- Internet connection
- VS Code (for IDE tests)

### Locally
To run the tests locally for debugging and development:

```bash
# Navigate to the e2e directory
cd e2e

# Run all tests
cargo test

# Run a specific test module
cargo test distribution
```

### CI
The E2E tests run automatically on every pull request and merge to main as part of our CI pipeline. The tests run in GitHub Actions, providing a consistent, isolated environment.

See [distribution.yml](../.github/workflows/distribution.yml) for the full workflow definition.

## CI Integration

Tests run automatically in these scenarios:
- All PRs to main/beta/develop
- Pushes to these branches
- Manual workflow dispatch

### Pipeline Integration
Tests are part of the `workflow_distribution.yml` pipeline:
1. Builds test image with `:pipeline` tag
2. Runs full E2E test suite
3. Reports results to PR
4. Required for merge approval

### Test Environment
- Runs on `ubuntu-22.04` runners
- Uses isolated Docker containers
- Cleaned up after each run
- Includes security scanning

## Adding New Tests

To add a new test:

1. Identify the appropriate module for your test (`distribution`, `startup`, `ide`)
2. Create a new Rust file in that module (or add to an existing one)
3. Write your test as an async function that returns `Result<()>`
4. Add your test to the module's `mod.rs` file
5. Update this documentation to describe your new test

Remember to:
- Handle errors and use the `bail!` macro to fail the test if needed
- Use a timeout (default is 60 seconds) to prevent tests from hanging
- Clean up any resources created during the test
- Provide clear error messages and logging

## Error Handling
- All tests must handle cleanup
- Clear error messages required
- Automatic timeout after 60 seconds
- Structured error reporting

## Intent
The intent of this E2E testing framework is to:
- Validate the end-to-end functionality of our development environment
- Ensure consistency across different distribution methods
- Catch regressions and integration issues
- Provide a safety net for refactoring and changes
- Serve as living documentation of key workflows