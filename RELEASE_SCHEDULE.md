# Release Schedule & Process

This document explains how releases are determined and automated.

## Branch Roles

- **develop**: Aggregates work from multiple pull requests.
- **beta**: When work from develop is merged into beta, a beta release is triggered.
- **main**: Stable releases are created when code in beta is merged into main.

## Release Flow

1. **Develop to Beta (Beta Releases)**
   - Developers merge features into **develop**.
   - When ready, merge **develop** into **beta**.
   - A beta release is created automatically based on the merge.
   - Scheduled automation ensures releases follow a predictable cadence.

2. **Beta to Main (Stable Releases)**
   - After thorough testing on beta, merge **beta** into **main**.
   - A stable release is automatically generated.

## Advantages

- **Simplicity:** Developers only need to know branch roles.
- **Traceability:** Clear documentation of release flow reduces tech debt.
- **Scheduled Automation:** Releases occur on a fixed schedule once branch merges are complete.

## Next Steps

- Use this document to align on further automation improvements.
- Update and refine workflows as needed to fully automate release sequencing.
