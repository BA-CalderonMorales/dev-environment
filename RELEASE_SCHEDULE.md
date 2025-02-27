# Release Schedule and Process

This document describes the release process and schedule for the Development Environment.

## Release Workflow Overview

Our project uses a structured release workflow to ensure quality and stability:

1. **Develop → Beta**: Changes merged to `develop` branch automatically create PRs to `beta`
2. **Beta Releases**: Weekly releases from `beta` branch (every Saturday)
3. **Beta → Main**: Stable releases from `beta` to `main` every 2 weeks

## Release Queue System

### How It Works

1. When code is merged to `develop`, a PR is automatically created to the `beta` branch
2. These PRs are tagged with `release-queue` label
3. The release automation processes these queued PRs in batches

### Queue Processing Rules

- **Beta Releases**: Process when 3+ PRs are queued OR on weekly schedule (Saturday)
- **Main Releases**: Process when 5+ PRs are in queue AND minimum 7 days since last release

### Manual Intervention

To force-process a release queue:

1. Go to Actions → "Create Release" workflow
2. Click "Run workflow" 
3. Select the branch (`beta` or `main`)
4. Check "Force queue processing" option
5. Click "Run workflow"

## Troubleshooting

### Common Issues

- **PRs not being processed**: Ensure PRs have the `release-queue` label
- **Empty queue warning**: Check if PRs are properly labeled and targeting the correct branch
- **Failed releases**: Check the Actions log for details and errors

### Fixing Queue Issues

If the queue gets stuck or contains invalid entries:

1. Go to `.github/release_queue/{branch}.json` file
2. Edit the file to remove problematic entries
3. Commit changes directly to the branch

## Release Schedule

| Branch | Schedule | Min Items | Age Requirement |
|--------|----------|-----------|----------------|
| Beta   | Weekly (Sat) | 3 | None |
| Main   | Bi-weekly | 5 | 7 days |
