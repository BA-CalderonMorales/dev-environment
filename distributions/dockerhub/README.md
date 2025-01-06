# Development Environment Setup

This project provides a consistent development environment using Docker.

## Prerequisites

* Docker Desktop installed
* Docker Hub Account (request access to the dev-environment image)

## Available Image Tags

- `latest` - Current stable version
- `latest-<commit-hash>` - Specific version tied to a commit

Example using specific version:
```bash
docker pull cmoe640/dev-environment:latest-8325b1a411ad382a64fd6c69ad2f5f50084d2dcc
```

## Initial Setup

1. Create your development directory structure:
   ```bash
   mkdir C:\dev
   cd C:\dev
   mkdir projects
   ```

2. Create a docker-compose.yml file in C:\dev with the following content:
   ```yaml
   version: '3.8'
   services:
     dev:
       image: cmoe640/dev-environment:latest
       container_name: dev-environment
       ports:
         - "8080:8080"    # VS Code
         - "3000-3010:3000-3010"  # Frontend apps
         - "8000-8010:8000-8010"  # Backend services
       volumes:
         - ./projects:/usr/src/projects
         - ~/.ssh:/home/devuser/.ssh:ro
         - ~/.gitconfig:/home/devuser/.gitconfig:ro
         - vscode-server:/home/devuser/.vscode-server
         - npm-cache:/home/devuser/.npm
         - cargo-cache:/home/devuser/.cargo
         - go-cache:/home/devuser/go
       restart: unless-stopped
       stdin_open: true
       tty: true

   volumes:
     vscode-server:
     npm-cache:
     cargo-cache:
     go-cache:
   ```

3. Log in to Docker Hub:
   ```bash
   # For Windows Git Bash:
   docker login -u your-username -p your-access-token
   ```
   Note: Create an access token in Docker Hub → Account Settings → Security → New Access Token

4. Start the development environment:
   ```bash
   cd C:\dev
   docker compose up -d
   ```

5. Access the container:
   ```bash
   # For Windows Git Bash (MinGW) users:
   winpty docker exec -it dev-environment bash

   # For Windows CMD or PowerShell users:
   docker exec -it dev-environment bash
   ```

## Understanding the Container Environment

When you first access the container, you'll be in `/home/devuser`:

```bash
$ pwd
/home/devuser

$ ls -lah
total 40K
drwxr-x--- 1 devuser devuser 4.0K Jan  5 19:48 .
drwxr-xr-x 1 root    root    4.0K Jan  5 18:53 ..
-rw-r--r-- 1 devuser devuser  220 Mar 31  2024 .bash_logout
-rw-r--r-- 1 devuser devuser 3.7K Mar 31  2024 .bashrc
drwxr-xr-x 2 root    root    4.0K Jan  5 19:48 .cargo
-rw-rwxrwx 1 root    root      48 Jun 15  2023 .gitconfig
drwxr-xr-x 2 root    root    4.0K Jan  5 19:48 .npm
-rw-r--r-- 1 devuser devuser  807 Mar 31  2024 .profile
drwxrwxrwx 1 root    root    512  Jan  5 19:48 .ssh
drwxr-xr-x 2 root    root    4.0K Jan  5 19:48 .vscode-server
drwxr-xr-x 2 root    root    4.0K Jan  5 19:48 go
```

Directory Structure Overview:
```
/ (root)
├── home
│   └── devuser/           # User's home directory
│       ├── .cargo/        # Rust configuration
│       ├── .gitconfig     # Git configuration
│       ├── .npm/          # Node.js packages
│       ├── .vscode-server/ # VS Code configuration
│       └── go/            # Go workspace
└── usr
    └── src
        └── projects/      # Mounted to C:\dev\projects
```

Host Machine Structure:
```
C:\
└── dev
    ├── docker-compose.yml
    └── projects/          # Mounted to /usr/src/projects
```

## Working with Projects

All development work should be done in `/usr/src/projects` (mounted to `C:\dev\projects` on your host machine).

To verify the mount:
```bash
$ mount | grep projects
C: on /usr/src/projects type 9p (rw,noatime,dirsync,aname=drvfs;path=C:;uid=0;gid=0;metadata;symlinkroot=/mnt/host/,mmap,access=client,msize=65536,trans=fd,rfd=4,wfd=4)
```

## Available Development Tools

The development environment comes with:
- Node.js v22.3.0
- Go v1.21.7
- Rust (latest stable)
- Git
- SQLite3

## Stopping the Environment

To stop the development environment:
```bash
cd C:\dev
docker compose down
```

## Image Maintenance

Clean up unused images:
```bash
# Remove unused images
docker image prune

# Remove specific version
docker rmi cmoe640/dev-environment:tag-name

# Remove dangling images
docker image prune --filter="dangling=true"
```

Best Practices:
- Keep `latest` for current stable version
- Maintain last 2-3 versions for potential rollback
- Regularly prune older versions
- Use specific tags for production deployments

## Troubleshooting

If you encounter issues:
1. Ensure you're logged in to Docker Hub
2. Verify your access token hasn't expired
3. Check that Docker Desktop is running
4. Ensure all paths in docker-compose.yml exist on your system
5. For Git Bash users, remember to use `winpty` when executing interactive commands

Additional considerations:
1. Rate Limits
   - DockerHub has pull rate limits
   - Authenticate to increase limits: `docker login`
   - Consider using specific tags to avoid frequent pulls

2. Version Management
   - Use `latest` for development
   - Use specific tags for stability
   - Keep track of working versions
