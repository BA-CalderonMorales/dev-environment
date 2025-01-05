# Development Environment Setup

This project provides a consistent and easy-to-use development environment based on Docker.

## Prerequisites

*   Docker
*   Docker Compose
*   GitHub Account
*   Docker Hub Account

## Environment Variables

1.  **Create a `.env` file:**

    ```bash
    cp .env.example .env
    ```

2.  **Set environment variables:** Open `.env` and fill in the required values:

    *   **`REMOTE_REPO_TOKEN`:** Your GitHub personal access token with `repo` and `workflow` scopes.
    *   **`GIT_AUTHOR_NAME`:** Your GitHub username.
    *   **`GIT_AUTHOR_EMAIL`:** Your email associated with your GitHub account.
    *   **`DOCKERHUB_USERNAME`:** Your Docker Hub username.
    *   **`DOCKERHUB_TOKEN`:** Your Docker Hub password or access token.

    **Important:** Do not commit your `.env` file to the repository.

3.  **Source environment variables:**

    ```bash
    source .env
    ```

## Starting the Development Environment

1.  Clone the repository:

    ```bash
    git clone <your-repository-url>
    cd <your-repository-name>
    ```

2.  Set up environment variables (as described above).

3.  Start the development environment:

    ```bash
    docker compose up -d
    ```

    This will pull the pre-built development environment image from Docker Hub and start the container.

4.  Attach to the container:

    ```bash
    docker exec -it dev-environment bash
    ```

## Creating a New Project

To initialize a new project, run the following command from within the container (replace `my-new-project` with your desired project name and `full-stack` with the desired stack types):

```bash
./init-project.sh my-new-project full-stack
