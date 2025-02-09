# Development Environment Architecture

## Overview
This project provides a flexible development environment with multiple distribution methods, designed to solve common development setup challenges.

## Core Components
```
dev-environment/
├── distributions/          # Distribution Methods
│   ├── dockerhub/         # Container Distribution
│   └── direct_download/   # Direct Download Distribution
├── e2e/                   # End-to-End Tests
│   ├── src/              # Test Implementation
│   └── tests/            # Test Modules
├── startup/               # Environment Setup
│   └── lib/              # Core Libraries
└── docs/                 # Documentation
```

## Distribution Methods
Each distribution method is designed to be:
- Independent and self-contained
- Easily testable
- Well-documented
- Fallback-capable

### DockerHub Distribution
Primary distribution method using standard Docker practices.

### Direct Download Distribution
Secondary distribution method providing direct downloads from our secured endpoints.

## Distribution Method Requirements
New distribution methods must implement:
1. Download mechanism (pull/fetch/build)
2. Image verification (checksum/signature)
3. E2E test suite
4. Fallback behavior
5. Version tracking

## CI/CD Infrastructure
Our pipeline runs on GitHub Actions using the following workflows:

### Primary Workflow (`workflow_distribution.yml`)
- Manages all distribution-related tasks
- Validates branches and tags
- Builds and pushes Docker images
- Runs E2E tests
- Performs security scans
- Creates releases

### Support Workflows
1. `workflow_create_release.yml`
   - Handles versioned releases
   - Updates changelog
   - Creates GitHub releases

2. `workflow_cleanup_dockerhub.yml`
   - Manages DockerHub tag cleanup
   - Supports dry-run testing

3. `workflow_cache_cleanup.yml`
   - Daily cache maintenance
   - Optimizes CI/CD performance

### Runner Specifications
- Uses `ubuntu-22.04` runners exclusively
- Requires periodic review for runner updates
- Major Ubuntu version migrations follow beta testing process

### Version Migration Process
1. Monitor GitHub Actions announcements for runner updates
2. Test new runner versions in develop branch
3. Update workflow files with new runner specifications
4. Verify all jobs execute successfully
5. Document any compatibility issues or required changes
6. Merge to main branch after successful testing

## Testing Strategy
Each distribution method includes:
- Unit tests for distribution scripts
- E2E tests for complete workflow
- Integration tests for fallback mechanism
- Performance benchmarks

## Adding New Distributions
See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines on implementing new distribution methods.