# Branch Strategy

This document outlines our git branching strategy to ensure consistent and efficient development workflow.

## Core Branches

### Production Branches
- `main` - Production-ready code, always stable
- `beta` - Pre-release testing environment
- `develop` - Primary development integration branch

### Support Branches
These branches are temporary and should be deleted after merging:

#### Feature Branches
- Pattern: `feature/*`
- Purpose: New features and non-emergency enhancements
- Branch from: `develop`
- Merge to: `develop`
- Naming: `feature/descriptive-feature-name`
- Example: `feature/add-bittorrent-distribution`

#### Bugfix Branches
- Pattern: `bugfix/*`
- Purpose: Fixes for bugs found in `develop`
- Branch from: `develop`
- Merge to: `develop`
- Naming: `bugfix/issue-description`
- Example: `bugfix/fix-artifact-detection`

#### Hotfix Branches
- Pattern: `hotfix/*`
- Purpose: Critical production fixes
- Branch from: `main`
- Merge to: `main`, `beta`, and `develop`
- Naming: `hotfix/critical-issue-description`
- Example: `hotfix/fix-security-vulnerability`

#### Documentation Branches
- Pattern: `documentation/*`
- Purpose: Documentation updates only
- Branch from: `develop`
- Merge to: `develop`
- Naming: `documentation/doc-description`
- Example: `documentation/update-installation-guide`

#### Release Branches
- Pattern: `release/*`
- Purpose: Release preparation
- Branch from: `develop`
- Merge to: `beta` and `develop`
- Naming: `release/vX.Y.Z`
- Example: `release/v1.2.0`

#### Pipeline Branches
- Pattern: `pipeline/*`
- Purpose: CI/CD pipeline development and testing
- Branch from: `develop`
- Merge to: `develop`
- Naming: `pipeline/descriptive-change`
- Example: `pipeline/fix-docker-build`
- Note: These branches are specifically for testing and modifying GitHub Actions workflows and related CI/CD infrastructure

## Workflow

### Development Flow
1. `develop` → `feature/*` → `develop` (Feature Development)
2. `develop` → `release/*` → `beta` (Release Preparation)
3. `beta` → `main` (Production Release)

### Hotfix Flow
1. `main` → `hotfix/*` → `main`
2. After merging to `main`, backport to `beta` and `develop`

## Branch Protection Rules

### `main` Branch
- Will create `:latest` tag on DockerHub
- Requires pull request reviews
- Requires all workflow checks to pass
- No direct pushes
- No force pushes
- Maintains linear history

### `beta` Branch
- Will create `:beta` tag on DockerHub
- Requires pull request reviews
- Requires distribution workflow to pass
- No direct pushes
- Force pushes allowed for maintainers only

### `develop` Branch
- Will create `:dev` tag on DockerHub
- Requires pull request reviews
- Requires distribution workflow to pass
- No direct pushes
- Force pushes allowed for maintainers only

### Pipeline Branch Rules
- Creates `:pipeline` tag on DockerHub
- Testing environment for CI/CD changes
- Temporary tags cleaned up after PR closure
- Must pass distribution workflow before merge
- Should include testing results in PR description

## Testing Workflows

### Local Testing
1. Create a branch following the `pipeline/*` pattern
2. Test workflows locally using [act](https://github.com/nektos/act)
3. Push changes to test in GitHub environment
4. Document findings in PR

### Remote Testing
1. Create a `pipeline/*` branch for isolated testing
2. Push changes to trigger workflow
3. Monitor execution in GitHub Actions
4. Iterate as needed before merging to `develop`

### Available Workflows
1. `workflow_distribution.yml`
   - Handles Docker image builds and distribution
   - Runs E2E tests
   - Manages DockerHub tags
   - Performs security scans
   - Creates releases

2. `workflow_create_release.yml`
   - Triggered on version tags
   - Creates GitHub releases
   - Updates changelog

3. `workflow_cleanup_dockerhub.yml`
   - Manual cleanup of DockerHub tags
   - Supports dry-run mode

4. `workflow_cache_cleanup.yml`
   - Daily cleanup of GitHub Actions cache
   - Maintains optimal CI/CD performance