#!/bin/bash
# init-project.sh

# --- Configuration ---
# Define constants for the GitHub API endpoint, the directory containing project templates, and the CI/CD workflow templates.
GITHUB_API="https://api.github.com"
TEMPLATES_DIR="$(dirname "$0")/templates"
WORKFLOWS_DIR="$(dirname "$0")/templates/workflows"

# --- Helper Functions ---

# --- Function: create_github_repo ---
# Creates a new private GitHub repository with the given project name.
# Uses the REMOTE_REPO_TOKEN environment variable for authentication.
#
# Arguments: None (uses global variable PROJECT_NAME)
# Returns: 0 if successful, 1 otherwise.
create_github_repo() {
    # Construct the API request to create a new repository.
    # The repository is created as private by default.
    response=$(curl -s -X POST \
        -H "Authorization: token $REMOTE_REPO_TOKEN" \
        -H "Accept: application/vnd.github.v3+json" \
        "$GITHUB_API/user/repos" \
        -d "{\"name\":\"$PROJECT_NAME\",\"private\":true}")

    # Check if the repository creation was successful.
    # The GitHub API returns a JSON response containing an "id" field if successful.
    if [[ "$response" == *"\"id\":"* ]]; then
        echo "GitHub repository '$PROJECT_NAME' created successfully."
        return 0
    else
        echo "Error creating GitHub repository:"
        echo "$response"
        return 1
    fi
}

# --- Function: init_project_structure ---
# Initializes the directory structure for a new project based on the selected stack types.
# Creates directories for frontend, backend (with sub-directories for each language), database, and CI/CD workflows.
# Copies relevant CI/CD workflow templates based on the chosen stack.
#
# Arguments: None (uses global variables PROJECT_NAME and STACK_TYPES)
# Returns: 0 (always successful in this simplified version).
init_project_structure() {
    # Create the base directory for the new project.
    mkdir -p "projects/$PROJECT_NAME"

    # Create project subdirectories based on the chosen stack types.
    for type in "${STACK_TYPES[@]}"; do
        case "$type" in
            "full-stack")
                mkdir -p "projects/$PROJECT_NAME"/{frontend,backend/{rust,go,node,python},database,.github/workflows}
                ;;
            "frontend")
                mkdir -p "projects/$PROJECT_NAME"/{frontend,.github/workflows}
                ;;
            "backend")
                mkdir -p "projects/$PROJECT_NAME"/{backend/{rust,go,node,python},.github/workflows}
                ;;
            "database")
                mkdir -p "projects/$PROJECT_NAME"/{database}
                ;;
        esac
    done

    # Copy CI/CD workflow templates based on selected stack types.
    for type in "${STACK_TYPES[@]}"; do
        if [[ "$type" == "full-stack" || "$type" == "frontend" ]]; then
            cp "$WORKFLOWS_DIR/frontend-ci.yml" "projects/$PROJECT_NAME/.github/workflows/"
        fi
        if [[ "$type" == "full-stack" || "$type" == "backend" ]]; then
            cp "$WORKFLOWS_DIR/backend-ci.yml" "projects/$PROJECT_NAME/.github/workflows/"
        fi
        # Always copy the deploy workflow (you can adjust this logic).
        cp "$WORKFLOWS_DIR/deploy.yml" "projects/$PROJECT_NAME/.github/workflows/"
    done

    # Create a project-specific .env file, populating it with default values and potentially
    # overriding them with environment variables if they are set.
    cat > "projects/$PROJECT_NAME/.env" << EOL
# Project: $PROJECT_NAME
DATABASE_URL=\${DATABASE_URL:-sqlite:///usr/src/projects/$PROJECT_NAME/database/db.sqlite}
$(if [[ " ${STACK_TYPES[@]} " =~ " frontend " ]]; then
    echo "FRONTEND_PORT=\${FRONTEND_PORT:-3000}"
fi)
$(if [[ " ${STACK_TYPES[@]} " =~ " backend " ]]; then
    echo "BACKEND_PORT=\${BACKEND_PORT:-8000}"
fi)
EOL

    # Create a README.md file with basic project information and instructions.
    cat > "projects/$PROJECT_NAME/README.md" << EOL
# $PROJECT_NAME

## Development Setup
1. \`docker compose up -d\`
$(for type in "${STACK_TYPES[@]}"; do
    case "$type" in
        "full-stack")
            echo "2. \`./start.sh fe\` for frontend"
            echo "3. \`./start.sh be\` for backend"
            echo "4. \`./start.sh db\` for database"
            ;;
        "frontend")
            echo "2. \`./start.sh fe\` for frontend"
            ;;
        "backend")
            echo "2. \`./start.sh be\` for backend"
            echo "3. \`./start.sh db\` for database"
            ;;
        "database")
            echo "2. \`./start.sh db\` for database"
            ;;
    esac
done)

## CI/CD
- Frontend CI: Auto-runs on PR to main (if frontend is included)
- Backend CI: Auto-runs on PR to main (if backend is included)
- Deployment: Auto-deploys on merge to main
EOL

    # Create a start.sh script to simplify starting project components.
    # It uses docker compose exec to run commands within the container.
    cat > "projects/$PROJECT_NAME/start.sh" << EOL
#!/bin/bash
source .env

case "\$1" in
$(if [[ " ${STACK_TYPES[@]} " =~ " frontend " ]]; then
    echo '    "fe") docker compose exec dev bash -c "cd /usr/src/projects/'"$PROJECT_NAME"'/frontend && npm start" ;;'
fi)
$(if [[ " ${STACK_TYPES[@]} " =~ " backend " ]]; then
    echo '    "be") docker compose exec dev bash -c "cd /usr/src/projects/'"$PROJECT_NAME"'/backend && ./run.sh" ;;'
fi)
$(if [[ " ${STACK_TYPES[@]} " =~ " database " ]]; then
    echo '    "db") docker compose exec dev bash -c "cd /usr/src/projects/'"$PROJECT_NAME"'/database && sqlite3 db.sqlite" ;;'
fi)
    *) echo "Usage: start [$(for type in "${STACK_TYPES[@]}"; do
        case "$type" in
            "full-stack") echo -n "fe|be|db" ;;
            "frontend") echo -n "fe" ;;
            "backend") echo -n "be|db" ;;
            "database") echo -n "db" ;;
        esac
        echo -n "|"
    done | sed 's/.$//')]
    " ;;
esac
EOL
    chmod +x "projects/$PROJECT_NAME/start.sh"

    return 0 # Indicate successful execution
}

# --- Function: init_git ---
# Initializes a new Git repository within the project directory, adds all files,
# creates an initial commit, and pushes the repository to GitHub.
# It uses docker compose exec to execute Git commands within the running container,
# leveraging the mounted SSH keys and Git configuration.
#
# Arguments: None (uses global variables PROJECT_NAME and GIT_AUTHOR_NAME)
# Returns: 0 if successful, 1 otherwise.
init_git() {
    # Change the current directory to the new project directory.
    cd "projects/$PROJECT_NAME"

    # Initialize a new Git repository.
    git init

    # Add all files in the current directory to the Git staging area.
    git add .

    # Create an initial commit with a message.
    git commit -m "Initial project setup"

    # Rename the default branch to "main".
    git branch -M main

    # Add a remote named "origin" pointing to the GitHub repository.
    git remote add origin "git@github.com:${GIT_AUTHOR_NAME}/${PROJECT_NAME}.git"

    # Push the "main" branch to the "origin" remote, setting it as the upstream branch.
    # The command is executed within the running Docker container using `docker compose exec`.
    docker compose exec dev git push -u origin main

    # Change back to the parent directory.
    cd ../..

    return 0
}

# --- Main Script ---

# --- Process Command Line Arguments ---
# Get the project name from the first command line argument.
PROJECT_NAME=$1

# Get the stack types from the second command line argument, splitting by commas.
STACK_TYPES=(${2//,/ })

# --- Input Validation ---
# Check if a project name was provided and if it's valid (alphanumeric and hyphens).
if [ -z "$PROJECT_NAME" ] || ! [[ "$PROJECT_NAME" =~ ^[a-zA-Z0-9-]+$ ]]; then
    echo "Usage: ./init-project.sh <project-name> [stack-types]"
    echo "  <project-name> must contain only letters, numbers, and hyphens."
    echo "  [stack-types] (optional) is a comma-separated list. Available: full-stack, frontend, backend, database"
    exit 1
fi

# Check if the REMOTE_REPO_TOKEN environment variable is set.
if [ -z "$REMOTE_REPO_TOKEN" ]; then
    echo "Error: REMOTE_REPO_TOKEN environment variable not set"
    exit 1
fi

# Validate the provided stack types.
for type in "${STACK_TYPES[@]}"; do
    case "$type" in
        "full-stack" | "frontend" | "backend" | "database") ;;
        *)
            echo "Error: Invalid stack type: $type"
            exit 1
            ;;
    esac
done

# --- Execute Main Functions ---
# Call the functions to create the GitHub repository, initialize the project structure, and set up Git.
echo "Creating GitHub repository..."
create_github_repo

echo "Initializing project structure..."
init_project_structure

echo "Setting up Git and pushing to GitHub..."
init_git

# --- Completion Message ---
echo "Project initialized successfully!"
echo "To start development:"
echo "1. cd projects/$PROJECT_NAME"
for type in "${STACK_TYPES[@]}"; do
    case "$type" in
        "full-stack")
            echo "2. ./start.sh fe # Start frontend"
            echo "3. ./start.sh be # Start backend"
            echo "4. ./start.sh db # Access database"
            ;;
        "frontend")
            echo "2. ./start.sh fe # Start frontend"
            ;;
        "backend")
            echo "2. ./start.sh be # Start backend"
            echo "3. ./start.sh db # Access database"
            ;;
        "database")
            echo "2. ./start.sh db # Access database"
            ;;
    esac
done
