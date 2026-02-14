# Managing Containers

This how-to guide covers common container management tasks in Doctainr.

## Prerequisites

- Doctainr installed and running
- Docker daemon running with at least one container

## View All Containers

1. Navigate to the **Containers** view using the sidebar
2. The list displays all containers (both running and stopped)

Each container entry shows:
- Container name
- Current status (Running/Stopped)
- Image the container is based on
- Port mappings
- Container ID (short form)

## Start a Stopped Container

**Steps**:

1. Navigate to **Containers** view
2. Locate the container you want to start (look for "Stopped" status)
3. Click the **Start** button next to the container

**Expected result**: The container status updates to "Running" after a brief moment. If the container fails to start, an error message appears at the top of the view.

**Tip**: After starting a container, Doctainr automatically refreshes the container list to show the updated state.

## Stop a Running Container

**Steps**:

1. Navigate to **Containers** view
2. Find the running container (status shows "Running")
3. Click the **Stop** button

**Expected result**: The container status changes to "Stopped". The container's processes are gracefully terminated.

**Note**: Docker sends SIGTERM to the main process, then SIGKILL after a timeout (default 10 seconds).

## Refresh Container List

Doctainr doesn't auto-refresh the container list. To see the latest state:

**Method 1** (Containers view):
- Click the **Refresh** button at the top of the Containers view

**Method 2** (Dashboard):
- Navigate to the Dashboard
- Click **Refresh All**

## Understanding Container Status

Containers can be in several states:

- **Running**: Container is actively executing
- **Stopped**: Container has exited (may have stopped gracefully or crashed)
- **Exited**: Same as Stopped
- **Created**: Container created but never started
- **Restarting**: Docker is attempting to restart the container

Currently, Doctainr simplifies this to two states: Running and Stopped.

## View Container Details

Currently, Doctainr displays:
- **Name**: Container identifier
- **Status**: Human-readable status string from Docker
- **Image**: Source image (e.g., `nginx:latest`)
- **Ports**: Port mappings (e.g., `8080:80`)

**Limitations**: 
- No detailed inspection view yet
- No logs viewing
- No resource usage metrics

## Common Issues

### "Failed to start container"

**Possible causes**:
- Port conflict (another process using the same port)
- Volume mount issues
- Insufficient system resources

**Solution**: Check Docker logs with `docker logs <container-name>` in your terminal.

### Container list is empty

**Possible causes**:
- No containers on system
- Connection issue with Docker

**Solution**:
1. Verify containers exist: `docker ps -a`
2. Click **Refresh** in Doctainr
3. Check error messages at the top of the view

### Action buttons don't respond

**Possible causes**:
- Docker daemon not responding
- Permission issues

**Solution**:
1. Verify Docker is running: `docker info`
2. Check Docker socket permissions
3. Look for error messages in Doctainr

## Tips and Best Practices

1. **Refresh regularly**: Manually refresh to see changes from other tools (Docker CLI, other applications)
2. **Check error messages**: Error messages appear at the top of the view when operations fail
3. **Use descriptive names**: Easier to identify containers in the list
4. **Port conflicts**: Before starting a container, ensure ports aren't in use

## What You Can't Do (Yet)

Current limitations of Doctainr:

- Cannot create new containers
- Cannot remove containers
- Cannot view container logs
- Cannot execute commands inside containers
- Cannot attach to running containers
- Cannot inspect detailed container configuration
- No real-time status updates (manual refresh required)

These features may be added in future versions.

## Related Documentation

- [Getting Started](../tutorials/getting-started.md) - Initial setup
- [Architecture: Container State](../reference/architecture.md#data-models) - Technical details of container management
- [Docker Integration](../explanation/docker-integration.md) - How Doctainr communicates with Docker
