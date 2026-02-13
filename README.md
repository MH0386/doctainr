# Doctainr

A modern, lightweight Docker container management desktop application built with [Dioxus 0.7](https://dioxuslabs.com) and Rust.

## Overview

Doctainr provides an intuitive graphical interface for managing Docker containers, images, and volumes. Built using the latest Dioxus 0.7 framework, it offers a native desktop experience with the power and safety of Rust.

## Features

- 🐳 **Container Management**: View, start, stop, and inspect Docker containers
- 🖼️ **Image Management**: Browse and manage Docker images
- 💾 **Volume Management**: Monitor Docker volumes
- 📊 **Dashboard**: Real-time overview of your Docker environment
- ⚙️ **Settings**: Configure Docker host and application preferences
- 🚀 **Native Performance**: Built with Rust for speed and reliability

## Quick Start

### Prerequisites

- Rust 1.70 or later
- Docker daemon running locally or remotely
- Dioxus CLI (optional, for development)

### Installation

1. Clone the repository:
````bash
git clone https://github.com/MH0386/containr.git
cd containr
````

2. Build and run:
````bash
cargo run --release
````

Or with Dioxus CLI:
````bash
dx serve --platform desktop
````

## Usage

### Connecting to Docker

By default, Doctainr connects to the local Docker daemon at `unix:///var/run/docker.sock`. You can customize this by setting the `DOCKER_HOST` environment variable:

````bash
export DOCKER_HOST=tcp://remote-docker-host:2375
cargo run
````

### Navigation

- **Dashboard**: Overview of containers, images, and volumes
- **Containers**: List, start, stop, and inspect containers
- **Images**: Browse available Docker images
- **Volumes**: View Docker volumes
- **Settings**: Configure application preferences

## Development

### Project Structure

````
src/
├── components/      # Reusable UI components
├── services/        # Docker API integration
├── utils/          # Application state and helpers
├── views/          # Page components and layouts
└── main.rs         # Application entry point
````

### Building

````bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test
````

### Development with Dioxus CLI

The Dioxus CLI provides hot-reloading and development tools:

````bash
# Install Dioxus CLI
curl -sSL http://dioxus.dev/install.sh | sh

# Run in development mode
dx serve --platform desktop

# Build for production
dx build --release
````

## Architecture

Doctainr is built using modern Rust patterns and the Dioxus 0.7 framework:

- **Dioxus 0.7**: Modern reactive UI framework
- **Bollard**: Async Docker API client
- **Tokio**: Async runtime
- **Signal-based State**: Reactive state management

See [ARCHITECTURE.md](docs/ARCHITECTURE.md) for detailed architectural documentation.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](docs/CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Dioxus](https://dioxuslabs.com)
- Docker integration via [Bollard](https://github.com/fussybeaver/bollard)
- Inspired by Docker Desktop and Portainer
