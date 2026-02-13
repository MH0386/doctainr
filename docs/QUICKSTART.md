# Quick Start Guide

Get up and running with Containr in 5 minutes.

## Installation

### Prerequisites

Ensure you have the following installed:

- ✅ **Rust** (1.70+) - [Install via rustup](https://rustup.rs/)
- ✅ **Docker** - [Get Docker](https://docs.docker.com/get-docker/)
- ✅ **Git** - [Install Git](https://git-scm.com/downloads)

### Step 1: Clone the Repository

``````bash
git clone https://github.com/MH0386/containr.git
cd containr
``````

### Step 2: Build the Application

``````bash
cargo build --release
``````

This will download dependencies and compile the application. First build may take a few minutes.

### Step 3: Run Containr

``````bash
cargo run --release
``````

The application will launch as a native desktop window.

## First Launch

### Dashboard Overview

Upon launching, you'll see the **Dashboard** with:

- **Running containers**: Count of active containers
- **Stopped containers**: Containers ready to restart  
- **Images**: Total Docker images on your system
- **Volumes**: Persistent storage volumes

### Navigation

Use the **sidebar** to navigate between views:

- 🏠 **Dashboard** - Overview and metrics
- 📦 **Containers** - Manage containers
- 🖼️ **Images** - Browse images
- 💾 **Volumes** - View volumes
- ⚙️ **Settings** - Configuration

## Basic Usage

### Managing Containers

1. Click **Containers** in the sidebar
2. View all containers with their status
3. Click **Start** to start a stopped container
4. Click **Stop** to stop a running container

**Note**: Currently uses mock data. Real Docker integration coming soon.

### Viewing Images

1. Click **Images** in the sidebar
2. Browse your Docker images
3. See repository, tag, and size information

### Monitoring Volumes

1. Click **Volumes** in the sidebar
2. View all Docker volumes
3. See driver type and storage usage

### Configuring Settings

1. Click **Settings** in the sidebar
2. Update Docker host URL if needed
3. Default: `unix:///var/run/docker.sock`

## Development Mode

For development with hot reload:

### Install Dioxus CLI

``````bash
cargo install dioxus-cli
``````

### Run Development Server

``````bash
dx serve --platform desktop
``````

Changes to source files will automatically reload the app.

## Keyboard Shortcuts

Currently, navigation is mouse-based. Keyboard shortcuts are planned for a future release.

## Troubleshooting

### Application Won't Start

**Check Rust installation:**
``````bash
rustc --version
cargo --version
``````

Should show Rust 1.70 or higher.

**Clean and rebuild:**
``````bash
cargo clean
cargo build --release
``````

### Docker Connection Issues

**Verify Docker is running:**
``````bash
docker ps
``````

Should list running containers without errors.

**Check Docker socket:**
``````bash
ls -la /var/run/docker.sock
``````

On macOS, Docker Desktop must be running.

### Build Errors

**Update dependencies:**
``````bash
cargo update
``````

**Check for conflicting versions:**
``````bash
cargo tree | grep dioxus
``````

### Performance Issues

**Build in release mode:**
``````bash
cargo build --release
cargo run --release
``````

Debug builds are significantly slower.

## Next Steps

- 📖 Read the [Developer Guide](docs/DEVELOPER_GUIDE.md)
- 🔍 Explore the [API Reference](docs/API.md)
- 🤝 Check [Contributing Guidelines](CONTRIBUTING.md)
- 🐛 Report issues on [GitHub](https://github.com/MH0386/containr/issues)

## Tips

- **Status indicators**: Green = Running, Gray = Stopped
- **Last action**: View recent actions in the header
- **Mock data**: The app uses sample data until Docker API integration
- **Cross-platform**: Works on Linux, macOS, and Windows

## Example Workflow

1. **Launch Containr**
   ``````bash
   cargo run --release
   ``````

2. **View your containers**
   - Click "Containers" in sidebar
   - See all containers and their status

3. **Manage a container**
   - Click "Start" on a stopped container
   - Check the header for action confirmation

4. **Browse images**
   - Click "Images" in sidebar
   - Review available Docker images

5. **Check settings**
   - Click "Settings" in sidebar
   - Verify Docker host configuration

## Getting Help

- 📚 **Documentation**: See [docs/](docs/) folder
- 💬 **Discussions**: Use GitHub Discussions
- 🐛 **Bug reports**: Open an issue with details
- 🚀 **Feature requests**: Describe your use case

Happy containerizing! 🐳
