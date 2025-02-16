# Release Schedule & Process

## Branch Roles & Automation

- **develop**: Aggregates work from multiple pull requests
- **beta**: Beta release branch
  - Triggers release queue on merges from develop
  - Creates beta releases after queue processing
- **main**: Production release branch
  - Triggers release queue on merges from beta
  - Creates stable releases after queue processing

## Automated Release Flow

1. **Develop to Beta (Beta Releases)**
   - When develop is merged into beta:
     1. Queue-release job triggers automatically
     2. Creates PR to update release queue
     3. Adds commit to beta release queue
     4. Weekly automation processes beta queue

2. **Beta to Main (Stable Releases)**
   - When beta is merged into main:
     1. Queue-release job triggers automatically
     2. Creates PR to update release queue
     3. Adds commit to main release queue
     4. Bi-weekly automation processes main queue

## Queue Processing

- **Beta Queue**:
  - Processed weekly (Saturday 10:00 AM CST)
  - Minimum 10 items needed for release
  - Creates beta release tags

- **Main Queue**:
  - Processed bi-weekly
  - Minimum 15 items needed for release
  - Creates stable release tags

## Manual Triggers

- Workflow dispatch on beta/main branches will:
  1. Queue release if conditions met
  2. Skip optional jobs (dockerhub_build, etc.)
  3. Process queue based on branch rules

## Queue Validation

Each queue addition:
1. Validates branch conditions
2. Verifies essential tests passed
3. Creates signed commit
4. Opens PR for queue update

## Next Steps

- [ ] Implement queue size notifications
- [ ] Add release schedule dashboard
- [ ] Automate changelog generation
