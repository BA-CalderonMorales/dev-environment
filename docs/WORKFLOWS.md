# Workflow Documentation

## Overview
Our GitHub Actions workflows implement a secure, two-phase release process with robust artifact validation and automated rollbacks.

## System Architecture

### Complete Pipeline Overview
```mermaid
graph TB
    subgraph "Phase 1: Distribution"
        Push[Code Push] --> Checks{Path Changes?}
        Checks -->|Yes| Dist[Distribution Workflow]
        Checks -->|No| Skip[Skip Build]
        
        subgraph "Build Process"
            Dist --> DockerB[DockerHub Build]
            Dist --> DirectB[Direct Download Build]
            DockerB --> Tests1[Security Scan]
            DirectB --> Tests1
            Tests1 --> E2E[E2E Tests]
        end
        
        E2E -->|Success| Store[Store Artifacts]
        E2E -->|Failure| Revert[Auto-Revert]
    end
    
    subgraph "Phase 2: Release"
        Store --> RelTrig[Release Trigger]
        RelTrig --> BranchCheck{Branch Type}
        
        BranchCheck -->|main| Stable[Stable Release]
        BranchCheck -->|beta| Beta[Beta Release]
        BranchCheck -->|release-*| RC[Release Candidate]
        
        Stable --> Verify[Artifact Verification]
        Beta --> Verify
        RC --> Verify
        
        Verify -->|Valid| Publish[Create Release]
        Verify -->|Invalid| Fail[Fail Release]
    end

    style Push fill:#f9f,stroke:#333
    style Store fill:#9f9,stroke:#333
    style Fail fill:#f66,stroke:#333
    style Revert fill:#f66,stroke:#333
```

### Artifact Verification Pipeline
```mermaid
flowchart TD
    A[Start Verification] --> B{Artifacts Present?}
    B -->|No| C[Download from Previous Run]
    B -->|Yes| D{Checksums Match?}
    C --> D
    
    D -->|No| E[Fail Release]
    D -->|Yes| F{Size Valid?}
    
    F -->|No| E
    F -->|Yes| G{Content Check}
    
    G -->|Failed| E
    G -->|Passed| H{Docker Image?}
    
    H -->|Yes| I[Pull & Verify Image]
    H -->|No| J[Final Validation]
    
    I -->|Success| J
    I -->|Failure| E
    
    J -->|Pass| K[Release Creation]
    J -->|Fail| E
```

### Error Recovery Flow
```mermaid
sequenceDiagram
    participant CI as GitHub Actions
    participant Git as Git Repo
    participant DH as DockerHub
    participant GHA as GitHub Artifacts
    
    Note over CI: Job Failure Detected
    CI->>Git: Trigger Revert Workflow
    CI->>DH: Remove Tagged Image
    CI->>GHA: Delete Failed Artifacts
    
    Git->>Git: Create Revert Commit
    Git->>Git: Sign with GPG
    Git->>CI: Push Revert
    
    Note over CI: Notify Team
    CI->>CI: Create Issue
    CI->>CI: Add Failure Labels
```

## Workflow States & Transitions

### Distribution Pipeline States
```mermaid
stateDiagram-v2
    [*] --> Triggered
    Triggered --> Building
    Building --> Testing
    Testing --> Storing
    Testing --> Failed
    Storing --> Ready
    Failed --> Reverting
    Reverting --> [*]
    Ready --> [*]
    
    state Building {
        [*] --> DockerHub
        [*] --> DirectDownload
        DockerHub --> [*]
        DirectDownload --> [*]
    }
    
    state Testing {
        [*] --> Security
        Security --> E2E
        E2E --> [*]
    }
```

## Critical Path Analysis

### Success Path
1. Code Push → Path Check
2. Build Distribution
3. Security Scans
4. E2E Tests
5. Artifact Storage
6. Release Creation
7. Documentation Update

### Failure Points & Mitigations

| Stage | Failure | Mitigation |
|-------|---------|------------|
| Build | Docker build fails | Auto-revert, cached layers |
| Tests | Security vulnerabilities | Block release, create issue |
| Artifacts | Storage failure | Retry mechanism, temp storage |
| Release | Missing artifacts | Re-run distribution |
| DockerHub | Rate limits | Authenticated pulls, caching |
| GPG | Signing fails | Fallback keys, manual intervention |

## Developer Guidelines

### Branch Strategy
```mermaid
gitGraph
    commit
    branch develop
    checkout develop
    commit
    branch feature/new-tool
    checkout feature/new-tool
    commit
    commit
    checkout develop
    merge feature/new-tool
    branch release-1.0
    checkout release-1.0
    commit
    checkout main
    merge release-1.0
    branch beta
    checkout beta
    commit
```

### Release Checklist

✅ Pre-Release
- [ ] All tests passing
- [ ] Security scan clear
- [ ] Artifacts validated
- [ ] Documentation updated
- [ ] Version bumped
- [ ] Changelog updated

✅ Release
- [ ] Branch protection rules met
- [ ] Required approvals obtained
- [ ] GPG signing configured
- [ ] Distribution workflow successful
- [ ] Artifacts available

✅ Post-Release
- [ ] Download links verified
- [ ] DockerHub image available
- [ ] Documentation links updated
- [ ] Release notes complete
- [ ] Notifications sent

### Common Pitfalls

❌ **Don't:**
- Push directly to protected branches
- Skip security scans
- Force-push to release branches
- Ignore failed tests
- Bypass branch protection
- Delete release tags

✅ **Do:**
- Use feature branches
- Wait for all checks
- Sign commits and tags
- Follow versioning scheme
- Update documentation
- Verify artifacts

## Emergency Procedures

### Release Rollback
```mermaid
flowchart TD
    A[Issue Detected] -->|Critical| B[Immediate Action]
    A -->|Non-Critical| C[Standard Process]
    
    B --> D[Stop Distribution]
    B --> E[Remove Release]
    B --> F[Revert Tag]
    
    C --> G[Create Issue]
    C --> H[Plan Fix]
    C --> I[Normal Release]
    
    D --> J[Notify Users]
    E --> J
    F --> J
```

### Quick Reference

🚨 **Emergency Contacts**
- GitHub Team: @github-team
- DevOps: @devops-team
- Security: @security-team

🔧 **Quick Commands**
```bash
# Revert last release
git revert $(git rev-list -n 1 $(git tag | sort -V | tail -n 1))

# Remove failed release
git tag -d v1.2.3
git push origin :v1.2.3

# Check artifact status
gh run download --name artifact-name
```

## Monitoring & Alerts

### Key Metrics
- Build Duration: < 15 minutes
- Test Coverage: > 80%
- Security Issues: 0 critical
- Release Time: < 30 minutes
- Artifact Size: < 1GB

### Alert Thresholds
```mermaid
graph LR
    A[Metrics] --> B{Duration}
    A --> C{Coverage}
    A --> D{Security}
    
    B -->|>15min| E[Warning]
    B -->|>30min| F[Critical]
    
    C -->|<80%| G[Warning]
    C -->|<70%| H[Critical]
    
    D -->|Any Critical| I[Block Release]
    D -->|>5 High| J[Review Required]
```

## Future Enhancements

### Planned Improvements
- [ ] Automated dependency updates
- [ ] Advanced artifact caching
- [ ] Release candidate promotion
- [ ] Automated changelog
- [ ] Performance metrics
- [ ] Compliance checks