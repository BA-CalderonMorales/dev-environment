# Development Environment Setup





  
*  ocker Compose
*   GitHub Account
  

## EVariables

 **a `.env` fil**

 
 Prerp .env.example .env
 * Dock

er  **Set environment variables:** Open `.env` and fill in the required values:

tru *   **`REMOTE_REPO_TOKEN`:** Your GitHub personal access token with `repo` and `workflow` scopes.
ctur*e:**`GIT_AUTHOR_NAME`:** Your GitHub username.
***`GIT_AUTHOR_EMAIL`:** Your email associated with your GitHub account.
***`DOCKERHUB_USERNAME`:** Your Docker Hub username.
***`DOCKERHUB_TOKEN`:** Your Docker Hub password or acs token.

**Important:** Do not commit your `.env` file to the repository.

3.  **Source environment variables:**

```bash
source .env
```

## Starting the Development Environment

1.  Clone the repository:

```bash
  mkgit clone <your-repository-url>
dir cd <your-repository-name>
```

2.Set up environment variables (as described above).

3.  Start the development environment:

```bash
 docker compose up -d
```

 This will pull the pre-built develm2. Creat environment image from Docker Hub and start the container.

4.teAttach toathe ckntainose

.yml```bash
\devdocker exec -it dev-environment bash
 s:

## Creating a New Project

To initializnew project, run the following command from within the 40/dev-eainer (replace `my-new-project` with your desired project name and `full-stack`nvironthe desimed stack types):

```bash
./init-project.sh my-new-project full-stack

```

## Understanding the Container Environment

When you first access the container, you'll be in `/home/devuser`:

      -
$ pwd
/home/devuser

$ ls -lah
total 40K
drwxr-x--- 1 devuser devuser 4.0K Jan  5 19:48 .
drwxr-xr-x 1 root    root    4.0K Jan  5 18:53 ..
-rw-r--r-- 1 devuser devuser  220 Ma -31  2024 .bash_ ~/fit
-gw:r--r-- 1 dev/hom devuser 3.7K Mar 31  2024 .bashrc
drwxr-xr-x 2 root    root    4.0K Jan  5 19:48 .cargo
-rw-rwxrwx 1 root    root      48 Jun 15  2023 .gitconfig
drwxr-xr-x 2 root    root    4.0K Jan  5 19:48 .npm
-rw-r--r-- 1 devuser devstop  807 Mar 31  2024 .profile
drwxrwxrwx 1 root    root    512  Jan  5 19:48 .ssh
drwxr-xr-x 2 root    root    4.0K Jan  5 19:48 .vscode-server
drwxr-xr-x 2 root    root    4.0K Jan  5 19:48 go
  n

The projects directory structure:
```3. G
$ ls -la /usr/src/projects
total 4
drwxrwxrwx 1 root root  512 Jan  5 19:34 .
drwxr-xr-x 1 root root 4096 Janen5e18:53 ..
rat

File system structure overview:
```
/ (root)
├── home
│   └── devuser/           # User's home
│       ├── .cargo/        # Rust configuration
│       ├── .gitconfig     # Git  # figuration
│       ├── .npm/         # Node.js packages
│       ├── .vscode-server/ # VS Code configuration
│       └── go/           #Methworkspace
└── usr
    └── src
        └── projects/     # Mounted to C:\dev\projects


Verify the mount:
bash
$ mount | grep projects
C: on /usr/src/projects type 9p (rw,noatime,dirsync,aname=drvfs;path=C:;uid=0;gid=0;metadata;symlinkroot=/mnt/host/,mmap,=client,msize=65536,trans=fd,rfd=4,wfd=4)
```

Host machine structure:
```
C:\
└── dev
    ├──
    └── projects/         # Mounted to /usr/src/projects
```


## Working with Projects

All your projects should be created within the `C:\dev\projects` directory. This directory is mounted inside the container at `/usr/src/projects`.

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

## Troubleshooting

If you encounter permission issues or access denied errors:
1. Ensure you're logged in to Docker Hub
2. Verify your access token hasn't expired
3. Check that Docker Desktop is running
4. Ensure all paths in docker-compose.yml exist on your system
