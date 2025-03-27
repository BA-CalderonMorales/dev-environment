# DockerHub Distribution

The fastest way to get started with good internet connectivity.

<hr/>

## Prerequisites
- Docker Desktop (see [main guide](README.md))
- Internet connection

<hr/>

## Steps
<details>
1. Ensure Docker Desktop is running
    - Literally click on the icon and open it.
    - If you don't have it, then you'll have to download it.

2. Pull and run the environment:
```bash
# Pull the latest image
docker pull cmoe640/dev-environment:latest

# Run the environment
docker run -it cmoe640/dev-environment:latest
```

3. You should now be within the "dev-environment"
    - At this point, you should see a list of techologies available to use and their versions

4. To exit the dev-environment, literally type in the word:
    - "exit" in the terminal.

5. Check to see if it's still running.
    - To do this, type in the command: docker ps
    - Shouldn't see anything running. Can also verify it's "down" within Docker Desktop.

</details>

<hr/>

## Troubleshooting
If you encounter any issues:
- Check the [Troubleshooting Guide](../TROUBLESHOOTING.md#docker-issues)
- Specifically [DockerHub Rate Limit](../TROUBLESHOOTING.md#dockerhub-rate-limit) issues
