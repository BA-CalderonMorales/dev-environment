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
- Requires pull request reviews
- Requires status checks to pass
- No direct pushes
- No force pushes
- Maintain linear history

### `beta` Branch
- Requires pull request reviews
- Requires status checks to pass
- No direct pushes
- Force pushes allowed for maintainers only

### `develop` Branch
- Requires pull request reviews
- Requires status checks to pass
- No direct pushes
- Force pushes allowed for maintainers only