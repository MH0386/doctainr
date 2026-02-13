# Containr

A modern, lightweight Docker desktop application built with [Dioxus 0.7](https://dioxuslabs.com/) and Rust.

[![CI](https://github.com/MH0386/containr/workflows/ci/badge.svg)](https://github.com/MH0386/containr/actions)

## Overview

Containr provides a clean, native desktop interface for managing Docker containers, images, and volumes. Built with Rust for performance and reliability, it offers a fast alternative to Docker Desktop.

## Features

- **Container Management**: View, start, and stop containers with real-time status updates
- **Image Browser**: Browse local Docker images with size information
- **Volume Inspector**: View and manage Docker volumes
- **Dashboard**: At-a-glance overview of your Docker environment
- **Native Performance**: Built with Rust and Dioxus for speed and low resource usage

## Prerequisites

- Rust 1.70+ (with `cargo`)
- Docker daemon running locally
- [Dioxus CLI](https://dioxuslabs.com/learn/0.7/CLI) (`dx`)

## Installation

### Install Dioxus CLI

```bash
curl -sSL http://dioxus.dev/install.sh | sh
```

### Clone and Build

```bash
git clone https://github.com/MH0386/containr.git
cd containr
cargo build --release
```

### Run

```bash
dx serve --platform desktop
```

Or using cargo:

```bash
cargo run --features desktop
```

## Development

### Project Structure

```
containr/
├── src/
│   ├── main.rs              # Application entry point and routing
│   ├── components/          # Reusable UI components
│   │   ├── metric_card.rs   # Dashboard metric cards
│   │   ├── section_header.rs # Page section headers
│   │   └── status_pill.rs   # Container status indicators
│   ├── services/            # External service integrations
│   │   └── docker.rs        # Docker API client
│   ├── utils/               # Utilities and shared logic
│   │   └── app_state.rs     # Global application state
│   └── views/               # Page views/routes
│       ├── dashboard.rs     # Dashboard overview
│       ├── containers.rs    # Container management
│       ├── images.rs        # Image browser
│       ├── volumes.rs       # Volume inspector
│       ├── settings.rs      # Settings page
│       └── shell.rs         # App shell layout
├── assets/                  # Static assets (CSS, images)
└── Cargo.toml              # Dependencies and metadata
```

### Technology Stack

- **Framework**: [Dioxus 0.7](https://dioxuslabs.com/) - React-like library for Rust
- **Docker API**: [Bollard](https://github.com/fussybeaver/bollard) - Async Rust Docker daemon API
- **Async Runtime**: [Tokio](https://tokio.rs/) - Async runtime for Rust
- **Serialization**: [Serde](https://serde.rs/) - Serialization framework

### Development Environment

This project uses [devenv](https://devenv.sh/) for reproducible development environments:

```bash
# Enter development shell
devenv shell

# Or use direnv for automatic environment loading
direnv allow
```

### Running Tests

```bash
cargo test
```

### Linting

```bash
cargo clippy -- -D warnings
```

### Code Formatting

```bash
cargo fmt
```

## Architecture

### State Management

Containr uses Dioxus signals for reactive state management:

- **Global State**: Shared via Context API using `AppState`
- **Docker Service**: Singleton service for Docker API communication
- **Async Operations**: Background tasks spawned for Docker operations

### Component Hierarchy

```
App (Root)
└── Router
    └── AppShell (Layout)
        ├── Sidebar (Navigation)
        └── Main Content
            └── Outlet (Route Views)
                ├── Dashboard
                ├── Containers
                ├── Images
                ├── Volumes
                └── Settings
```

### Docker Integration

The `DockerService` uses Bollard to communicate with the Docker daemon via the Unix socket (`/var/run/docker.sock`). All operations are async and run on Tokio runtime.

## Configuration

Set the Docker host via environment variable:

```bash
export DOCKER_HOST=unix:///var/run/docker.sock
```

Or for remote Docker:

```bash
export DOCKER_HOST=tcp://remote-host:2375
```

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Coding Standards

- Follow Rust naming conventions
- Use `cargo fmt` for formatting
- Ensure `cargo clippy` passes without warnings
- Add tests for new functionality
- Update documentation as needed

## License

This project is open source. See LICENSE file for details.

## Acknowledgments

- [Dioxus](https://dioxuslabs.com/) - The reactive UI framework
- [Bollard](https://github.com/fussybeaver/bollard) - Docker API client
- Docker community for API documentation

## Support

- **Issues**: [GitHub Issues](https://github.com/MH0386/containr/issues)
- **Discussions**: [GitHub Discussions](https://github.com/MH0386/containr/discussions)

## Roadmap

- [ ] Container logs viewer
- [ ] Container shell/exec integration
- [ ] Network management
- [ ] Docker Compose support
- [ ] Multi-host Docker management
- [ ] Container resource monitoring
