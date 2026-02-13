# User Guide

Complete guide for using Doctainr to manage your Docker containers.

## Table of Contents

- [Getting Started](#getting-started)
- [Dashboard Overview](#dashboard-overview)
- [Managing Containers](#managing-containers)
- [Managing Images](#managing-images)
- [Managing Volumes](#managing-volumes)
- [Settings](#settings)
- [Troubleshooting](#troubleshooting)

## Getting Started

### First Launch

When you first launch Doctainr, it will automatically attempt to connect to your local Docker daemon at `unix:///var/run/docker.sock`.

**Connection Success:**
- You'll see the dashboard with your Docker resources
- Metrics will display container, image, and volume counts

**Connection Failure:**
- Check that Docker is running: `docker ps`
- Verify Docker socket permissions
- See [Troubleshooting](#troubleshooting) for common issues

### Understanding the Interface

Doctainr uses a sidebar navigation with the following sections:

- **🏠 Dashboard**: Overview of all resources
- **📦 Containers**: Manage containers
- **🖼️ Images**: View images
- **💾 Volumes**: View volumes
- **⚙️ Settings**: Application settings

## Dashboard Overview

The dashboard provides a high-level view of your Docker environment:

### Metrics

- **Total Containers**: Number of containers (running and stopped)
- **Running Containers**: Currently active containers
- **Images**: Total number of Docker images
- **Volumes**: Total number of Docker volumes

### Quick Actions

- **Refresh All**: Updates all data from Docker daemon
- Each metric card links to its respective management view

## Managing Containers

The Containers view displays all containers with detailed information.

### Container List

Each container shows:
- **Name**: Container name
- **Image**: Source image name and tag
- **Status**: Current status (e.g., "Up 2 hours", "Exited (0) 5 minutes ago")
- **Ports**: Port mappings (e.g., "0.0.0.0:8080->80/tcp")
- **State**: Visual indicator (green = running, red = stopped)

### Actions

#### Start a Container

1. Find a stopped container (red indicator)
2. Click the **Start** button
3. Wait for the operation to complete
4. Container state updates to Running (green)

#### Stop a Container

1. Find a running container (green indicator)
2. Click the **Stop** button
3. Confirm the action (if prompted)
4. Wait for the operation to complete
5. Container state updates to Stopped (red)

### Container Information

Click on a container name to view detailed information:
- Full container ID
- Image details
- Network settings
- Volume mounts
- Environment variables
- Resource limits

## Managing Images

The Images view displays all Docker images on your system.

### Image List

Each image shows:
- **Repository**: Image repository name
- **Tag**: Image tag (e.g., "latest", "v1.2.3")
- **ID**: Short image ID
- **Size**: Image size on disk
- **Created**: When the image was created

### Actions

#### Pull an Image

(Feature coming soon)

1. Click **Pull Image** button
2. Enter image name and tag
3. Wait for download to complete

#### Remove an Image

(Feature coming soon)

1. Select an unused image
2. Click **Remove** button
3. Confirm deletion

**Note:** Images in use by containers cannot be removed.

## Managing Volumes

The Volumes view displays all Docker volumes.

### Volume List

Each volume shows:
- **Name**: Volume name or ID
- **Driver**: Volume driver (usually "local")
- **Mountpoint**: Filesystem path
- **Created**: When the volume was created

### Volume Information

Volumes persist data between container restarts and are essential for:
- Database storage
- Application data
- Configuration files
- Log files

### Actions

#### Create a Volume

(Feature coming soon)

#### Remove a Volume

(Feature coming soon)

**Warning:** Removing a volume deletes all its data permanently.

## Settings

Configure Doctainr and Docker connection settings.

### Docker Host

Change the Docker daemon connection:

1. Navigate to **Settings**
2. Find **Docker Host** field
3. Enter new connection string:
   - Local: `unix:///var/run/docker.sock`
   - Remote: `tcp://hostname:2375`
   - TLS: `tcp://hostname:2376`
4. Click **Save**
5. Application reconnects to new host

### Application Preferences

(Feature coming soon)

- Theme selection (light/dark)
- Auto-refresh interval
- Default view on startup
- Notification settings

## Troubleshooting

### Cannot Connect to Docker

**Symptoms:**
- "Failed to connect to Docker" message
- Empty dashboard
- No containers/images/volumes displayed

**Solutions:**

1. **Verify Docker is Running:**
   ````bash
   docker ps
   ````
   If this fails, start Docker:
   - **Linux**: `sudo systemctl start docker`
   - **macOS**: Open Docker Desktop
   - **Windows**: Start Docker Desktop

2. **Check Socket Permissions:**
   ````bash
   ls -l /var/run/docker.sock
   ````
   Add your user to the docker group:
   ````bash
   sudo usermod -aG docker $USER
   newgrp docker
   ````

3. **Verify DOCKER_HOST:**
   ````bash
   echo $DOCKER_HOST
   ````
   If set, ensure it points to the correct Docker daemon.

### Container Start/Stop Fails

**Symptoms:**
- Error message when starting/stopping container
- Container state doesn't change

**Solutions:**

1. **Check Container Logs:**
   ````bash
   docker logs <container-name>
   ````

2. **Verify Container Exists:**
   ````bash
   docker ps -a | grep <container-name>
   ````

3. **Check for Port Conflicts:**
   - Another container may be using the same port
   - Stop conflicting containers first

### Slow Performance

**Symptoms:**
- Slow loading times
- Delayed UI updates
- High CPU usage

**Solutions:**

1. **Reduce Container Count:**
   - Stop unused containers
   - Remove old containers: `docker container prune`

2. **Clear Docker Cache:**
   ````bash
   docker system prune
   ````

3. **Check Docker Resource Limits:**
   - Adjust Docker Desktop memory/CPU limits
   - Check system resources

### Application Won't Start

**Symptoms:**
- Application crashes on launch
- Error messages in terminal

**Solutions:**

1. **Check Rust Installation:**
   ````bash
   rustc --version
   cargo --version
   ````

2. **Rebuild Application:**
   ````bash
   cargo clean
   cargo build --release
   ````

3. **Check Dependencies:**
   ````bash
   cargo update
   ````

## Keyboard Shortcuts

(Feature coming soon)

- `Ctrl/Cmd + R`: Refresh current view
- `Ctrl/Cmd + 1-5`: Navigate between views
- `Ctrl/Cmd + Q`: Quit application

## Tips and Best Practices

### Resource Management

- Regularly remove unused containers: `docker container prune`
- Clean up unused images: `docker image prune`
- Monitor disk space: `docker system df`

### Container Organization

- Use descriptive container names
- Apply labels for categorization
- Group related containers with Docker Compose

### Security

- Don't expose Docker daemon over network without TLS
- Use read-only mounts when possible
- Limit container resources
- Keep images updated

## Getting Help

- **Documentation**: [docs/](.)
- **Issues**: [GitHub Issues](https://github.com/MH0386/containr/issues)
- **Discussions**: [GitHub Discussions](https://github.com/MH0386/containr/discussions)

## Feedback

We'd love to hear your feedback! Please:
- Report bugs via GitHub Issues
- Request features via GitHub Issues
- Share your experience in Discussions
- Contribute improvements via Pull Requests
