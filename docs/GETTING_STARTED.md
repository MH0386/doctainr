# Getting Started with Containr

This guide will help you get up and running with Containr, a modern Docker desktop application built with Rust.

## Prerequisites

Before you begin, ensure you have:

- **Docker**: Docker daemon installed and running
  - [Install Docker Desktop](https://docs.docker.com/get-docker/) OR
  - [Install Docker Engine](https://docs.docker.com/engine/install/)
- **Rust**: Version 1.70 or later
  - Install via [rustup](https://rustup.rs/): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## Quick Start

### 1. Install Dioxus CLI

The Dioxus CLI (`dx`) provides development tools and build capabilities:

``````bash
curl -sSL http://dioxus.dev/install.sh | sh
``````

Verify installation:

``````bash
dx --version
``````

### 2. Clone the Repository

``````bash
git clone https://github.com/MH0386/containr.git
cd containr
``````

### 3. Build and Run

#### Using Dioxus CLI (Recommended)

``````bash
dx serve --platform desktop
``````

This will:
- Build the project in development mode
- Launch the desktop application
- Enable hot reloading for rapid development

#### Using Cargo

``````bash
cargo run --features desktop
``````

For release build:

``````bash
cargo build --release --features desktop
./target/release/containr
``````

## Verifying Your Installation

### Check Docker Connection

1. Launch Containr
2. Navigate to **Dashboard**
3. You should see:
   - Docker host connection string
   - Container counts
   - Image and volume statistics

### Test Container Management

1. Go to **Containers** view
2. See list of all containers (running and stopped)
3. Try starting/stopping a container

If you see an error about Docker connection, see [Troubleshooting](#troubleshooting).

## Application Overview

### Main Views

#### Dashboard

The landing page showing:
- Running and stopped container counts
- Total images and volumes
- Docker engine information
- Quick refresh button

#### Containers

Manage Docker containers:
- View all containers with status
- Start/stop containers with one click
- See container details (ID, image, ports, status)
- Filter and search containers

#### Images

Browse local Docker images:
- Image repository and tag information
- Image sizes
- Image IDs

#### Volumes

Inspect Docker volumes:
- Volume names
- Driver types
- Mount points

#### Settings

Configure Containr:
- Docker host connection
- Application preferences

## Basic Workflows

### Managing Containers

#### Start a Stopped Container

1. Navigate to **Containers**
2. Find the container you want to start
3. Click the **Start** button
4. Status updates automatically

#### Stop a Running Container

1. Navigate to **Containers**
2. Find the running container
3. Click the **Stop** button
4. Container status changes to "Stopped"

### Viewing Container Details

Each container displays:
- **ID**: Short container ID (12 characters)
- **Name**: Container name
- **Image**: Source image
- **Status**: Current status text
- **Ports**: Exposed port mappings
- **State**: Visual indicator (Running/Stopped)

### Refreshing Data

Click **Refresh** or **Refresh All** to update data from Docker daemon.

## Configuration

### Docker Host

By default, Containr connects to the local Docker socket:

``````
unix:///var/run/docker.sock
``````

#### Connecting to Remote Docker

Set the `DOCKER_HOST` environment variable before launching:

``````bash
export DOCKER_HOST=tcp://192.168.1.100:2375
cargo run
``````

Or for persistent configuration, add to your shell profile (`~/.bashrc`, `~/.zshrc`):

``````bash
echo 'export DOCKER_HOST=tcp://192.168.1.100:2375' >> ~/.bashrc
source ~/.bashrc
``````

#### Docker over SSH

For secure remote connections:

``````bash
ssh -L 2375:localhost:2375 user@remote-host
export DOCKER_HOST=tcp://localhost:2375
cargo run
``````

### Development vs Production

#### Development Mode

- Hot reloading enabled
- Debug assertions active
- Larger binary size
- Slower performance

``````bash
dx serve --platform desktop
``````

#### Production Build

- Optimized binary
- No debug overhead
- Smaller size
- Maximum performance

``````bash
cargo build --release --features desktop
``````

## Troubleshooting

### "Failed to connect to Docker"

**Symptoms**: Error message on Dashboard, no containers/images shown

**Solutions**:

1. **Verify Docker is running**:
   ``````bash
   docker ps
   ``````

2. **Check Docker socket permissions**:
   ``````bash
   ls -l /var/run/docker.sock
   ``````
   
   If permission denied, add your user to `docker` group:
   ``````bash
   sudo usermod -aG docker $USER
   newgrp docker
   ``````

3. **Verify DOCKER_HOST**:
   ``````bash
   echo $DOCKER_HOST
   ``````

### "Permission denied" on Socket

**Linux/macOS**:
``````bash
sudo chmod 666 /var/run/docker.sock
``````

Or permanently via group:
``````bash
sudo usermod -aG docker $USER
``````

Then log out and back in.

### Application Won't Start

1. **Check Rust version**:
   ``````bash
   rustc --version
   ``````
   Should be 1.70 or later.

2. **Update dependencies**:
   ``````bash
   cargo clean
   cargo update
   ``````

3. **Rebuild from scratch**:
   ``````bash
   cargo clean
   cargo build
   ``````

### Containers Not Updating

If container status doesn't update after start/stop:

1. Click **Refresh** button
2. Check Docker daemon is responding:
   ``````bash
   docker ps
   ``````
3. Check application logs for errors

### Build Errors

**"linker not found"**:
- Linux: `sudo apt install build-essential`
- macOS: `xcode-select --install`

**"SSL/TLS errors"**:
``````bash
cargo clean
cargo build
``````

## Next Steps

### Learn More

- Read [Architecture Guide](../docs/ARCHITECTURE.md) to understand the codebase
- Check [API Reference](../docs/API.md) for technical details
- See [Contributing Guide](../docs/CONTRIBUTING.md) to contribute

### Customize Containr

- Modify CSS in `assets/styling/` for custom themes
- Add new views by creating components in `src/views/`
- Extend Docker functionality in `src/services/docker.rs`

### Join the Community

- Report bugs: [GitHub Issues](https://github.com/MH0386/containr/issues)
- Request features: [GitHub Issues](https://github.com/MH0386/containr/issues)
- Discuss ideas: [GitHub Discussions](https://github.com/MH0386/containr/discussions)

## Getting Help

If you encounter issues:

1. Check this guide's [Troubleshooting](#troubleshooting) section
2. Search [existing issues](https://github.com/MH0386/containr/issues)
3. Ask in [GitHub Discussions](https://github.com/MH0386/containr/discussions)
4. Open a new issue with details about your problem

## Performance Tips

### Reduce Docker API Calls

- Use **Refresh** button instead of auto-refresh
- Close unused views to reduce background updates

### Optimize Build Times

Development builds with hot reload:
``````bash
dx serve --hot-reload --platform desktop
``````

Use incremental compilation (default in dev mode):
``````toml
[profile.dev]
incremental = true
``````

### System Resources

Containr is lightweight and uses minimal resources:
- **Memory**: ~20-50 MB
- **CPU**: Minimal when idle
- **Disk**: ~10 MB binary (release build)

## What's Next?

Now that you're up and running:

- Explore all views (Dashboard, Containers, Images, Volumes)
- Try managing your containers through Containr
- Customize the application to fit your workflow
- Consider contributing improvements back to the project

Happy containerizing! 🚀
