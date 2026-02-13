# Getting Started with Doctainr

This tutorial will guide you through installing, running, and using Doctainr for the first time.

## Prerequisites

Before you begin, ensure you have:

1. **Docker** installed and running
   ````bash
   docker info
   ````
   If this command fails, install Docker Desktop from [docker.com](https://www.docker.com/products/docker-desktop/)

2. **Rust toolchain** (1.70 or later)
   ````bash
   rustc --version
   ````
   If not installed, get it from [rustup.rs](https://rustup.rs/)

3. **Dioxus CLI**
   ````bash
   curl -sSL http://dioxus.dev/install.sh | sh
   ````

## Installation

### Step 1: Clone the Repository

````bash
git clone https://github.com/MH0386/doctainr.git
cd doctainr
````

### Step 2: Build Dependencies

The project will automatically download and compile dependencies on first run.

### Step 3: Run Doctainr

````bash
dx serve --platform desktop
````

The application window should open automatically.

## First Steps

### Understanding the Interface

When you launch Doctainr, you'll see:

1. **Navigation Sidebar** (left): Switch between views
   - Dashboard: Overview of your Docker environment
   - Containers: Manage running and stopped containers
   - Images: Browse Docker images
   - Volumes: View Docker volumes
   - Settings: Configure application preferences

2. **Main Content Area** (right): Dynamic content based on selected view

### Your First Action: View Containers

1. Click **Containers** in the sidebar
2. You'll see a list of all containers (running and stopped)
3. Try starting a stopped container:
   - Find a stopped container (gray status pill)
   - Click the **Start** button
   - Watch the status change to "Running" (green pill)

### Refreshing Data

Doctainr displays real-time data, but you can manually refresh:

- Click **Refresh** button in any view
- Or click **Refresh All** in the Dashboard

## Understanding Container States

Containers can be in two states:

- **Running** (green): Container is active
- **Stopped** (gray): Container is inactive but preserved

You can toggle between states using Start/Stop buttons.

## Next Steps

Now that you're familiar with the basics:

- Explore the [API Reference](../reference/api.md) to understand the codebase
- Learn about [State Management](../guides/state-management.md)
- Try [Building Custom Views](./building-views.md)

## Troubleshooting

**Application won't start?**
- Verify Docker is running: `docker info`
- Check Rust version: `rustc --version`
- See [Troubleshooting Guide](../guides/troubleshooting.md)

**Can't see your containers?**
- Ensure Docker socket is accessible
- Check `DOCKER_HOST` environment variable
- Try restarting the application

---

**Next**: [Building Your First View](./building-views.md)
