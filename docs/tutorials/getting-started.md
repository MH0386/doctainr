# Getting Started with Doctainr

This tutorial will guide you through installing and running Doctainr for the first time.

## What You'll Learn

By the end of this tutorial, you will:

- Have Doctainr installed and running on your system
- Understand the main features of the application
- Know how to view and manage Docker containers
- Be able to navigate between different views

## Prerequisites

Before starting, ensure you have:

1. **Docker installed and running**
   ````bash
   docker --version
   # Should output: Docker version 20.x.x or higher
   ````

2. **Rust toolchain** (1.70 or higher)
   ````bash
   rustc --version
   # Should output: rustc 1.70.x or higher
   ````

3. **Dioxus CLI** (optional, for hot reload)
   ````bash
   curl -sSL http://dioxus.dev/install.sh | sh
   dx --version
   ````

## Installation

### Step 1: Clone the Repository

````bash
git clone https://github.com/MH0386/doctainr.git
cd doctainr
````

### Step 2: Build the Application

Using Dioxus CLI (recommended for development):

````bash
dx build --release
````

Or using Cargo directly:

````bash
cargo build --release
````

### Step 3: Run Doctainr

With Dioxus CLI:

````bash
dx serve --platform desktop
````

With Cargo:

````bash
cargo run --release
````

The application window should open automatically.

## First Steps

### Exploring the Dashboard

When you first launch Doctainr, you'll see the **Dashboard** view:

1. **Metrics Cards**: Display counts for running containers, stopped containers, images, and volumes
2. **Engine Information**: Shows your Docker host configuration
3. **Refresh All Button**: Updates all data from the Docker daemon

### Viewing Containers

1. Click **Containers** in the navigation menu
2. You'll see a list of all containers (running and stopped)
3. Each container shows:
   - Container name
   - Status (Running/Stopped)
   - Image used
   - Port mappings

### Managing Container State

To start a stopped container:

1. Navigate to the **Containers** view
2. Find a stopped container (status shows "Stopped")
3. Click the **Start** button

To stop a running container:

1. Find a running container (status shows "Running")
2. Click the **Stop** button

### Browsing Images

1. Click **Images** in the navigation menu
2. View all Docker images on your system
3. See repository name, tag, image ID, and size

### Viewing Volumes

1. Click **Volumes** in the navigation menu
2. Browse all Docker volumes
3. See volume name, driver, and mount point

## Refreshing Data

Doctainr doesn't auto-refresh by default. To update information:

- Click **Refresh All** on the Dashboard to update everything
- Click **Refresh** on individual views (Containers, Images, Volumes)

## Troubleshooting

### "Failed to connect to Docker"

**Problem**: Application can't connect to the Docker daemon.

**Solution**:
1. Ensure Docker is running: `docker info`
2. Check Docker socket permissions (Unix/Linux):
   ````bash
   sudo chmod 666 /var/run/docker.sock
   ````
3. Verify DOCKER_HOST environment variable (if using remote Docker)

### Application Won't Start

**Problem**: Cargo build or runtime errors.

**Solution**:
1. Update Rust: `rustup update`
2. Clean build artifacts: `cargo clean`
3. Rebuild: `cargo build --release`

### Empty Container/Image Lists

**Problem**: No containers or images appear.

**Solution**:
1. Verify Docker has containers: `docker ps -a`
2. Click the **Refresh** button
3. Check error messages at the top of the view

## Next Steps

Now that you have Doctainr running:

- Learn more about [managing containers](../how-to/manage-containers.md)
- Explore [image operations](../how-to/work-with-images.md)
- Understand the [architecture](../reference/architecture.md)
- Contribute to development by following the [build guide](../how-to/build-from-source.md)

## Summary

You've successfully:

✅ Installed Doctainr  
✅ Launched the application  
✅ Explored the Dashboard  
✅ Viewed containers, images, and volumes  
✅ Learned how to refresh data  

Continue to the [How-To guides](../how-to/) for specific tasks, or read the [Architecture Overview](../reference/architecture.md) to understand how Doctainr works.
