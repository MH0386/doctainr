# Containr

A modern desktop application for managing Docker containers, images, and volumes built with [Dioxus 0.7](https://dioxuslabs.com).

![Dioxus](https://img.shields.io/badge/Dioxus-0.7.1-blue)
![Rust](https://img.shields.io/badge/Rust-2021-orange)
![License](https://img.shields.io/badge/license-MIT-green)

## Overview

Containr is a lightweight, native desktop application that provides a clean interface for managing your local Docker environment. Built with Rust and Dioxus, it offers fast performance and a modern UI for viewing and controlling containers, images, and volumes.

## Features

- 📊 **Dashboard**: Overview of your Docker engine with real-time metrics
- 🐳 **Container Management**: View, start, and stop containers
- 🖼️ **Image Browser**: Browse local Docker images
- 💾 **Volume Management**: Monitor Docker volumes and storage
- ⚙️ **Settings**: Configure Docker host connection
- 🎨 **Modern UI**: Dark theme with intuitive navigation

## Prerequisites

- **Rust** (latest stable): [Install Rust](https://rustup.rs/)
- **Docker** (running locally): [Install Docker](https://docs.docker.com/get-docker/)
- **Dioxus CLI** (optional, for development): Install with `cargo install dioxus-cli`

## Installation

### From Source

1. Clone the repository:
   ``````bash
   git clone https://github.com/MH0386/containr.git
   cd containr
   ``````

2. Build the application:
   ``````bash
   cargo build --release
   ``````

3. Run the application:
   ``````bash
   cargo run --release
   ``````

### Using Dioxus CLI (Development)

``````bash
dx serve --platform desktop
``````

## Usage

### Dashboard

The dashboard provides an at-a-glance view of your Docker environment:

- **Running containers**: Number of active containers
- **Stopped containers**: Containers ready to restart
- **Images**: Total local Docker images
- **Volumes**: Persistent data volumes

### Managing Containers

Navigate to the **Containers** page to:

- View all containers with their status
- Start stopped containers with one click
- Stop running containers
- See container details (ID, image, ports, status)

### Browsing Images

The **Images** page displays:

- Image repository and tag
- Image ID
- Size on disk

### Working with Volumes

Monitor your Docker volumes:

- Volume names and drivers
- Mount points
- Storage usage

### Settings

Configure your Docker connection:

- Docker host URL (default: `unix:///var/run/docker.sock`)
- Context selection

## Architecture

Containr is built with a modular architecture:

``````
src/
├── main.rs              # Application entry point and routing
├── components/          # Reusable UI components
│   ├── metric_card.rs   # Dashboard metric display
│   ├── section_header.rs# Page headers
│   └── status_pill.rs   # Container status indicators
├── services/            # Business logic and data
│   └── docker.rs        # Docker API integration (mock data)
├── utils/               # Shared utilities
│   └── app_state.rs     # Global application state
└── views/               # Page-level components
    ├── containers.rs    # Container management view
    ├── dashboard.rs     # Dashboard overview
    ├── images.rs        # Image browser
    ├── settings.rs      # Configuration
    ├── shell.rs         # App shell layout
    └── volumes.rs       # Volume management
``````

### State Management

The application uses Dioxus signals for reactive state management:

- **AppState**: Global state provided via Context API
- **Signals**: Reactive containers for Docker data
- **Local state**: Component-specific UI state

### Routing

Routing is handled by Dioxus Router with the following routes:

- `/` - Dashboard
- `/containers` - Container management
- `/images` - Image browser
- `/volumes` - Volume management
- `/settings` - Configuration

## Development

### Project Setup

1. Install dependencies:
   ``````bash
   cargo fetch
   ``````

2. Run in development mode:
   ``````bash
   dx serve --platform desktop
   ``````

3. Run tests:
   ``````bash
   cargo test
   ``````

4. Run lints:
   ``````bash
   cargo clippy
   ``````

### Code Style

This project follows standard Rust formatting:

``````bash
cargo fmt
``````

Custom clippy rules are defined in `clippy.toml`.

### Adding Features

The application is structured for easy extension:

1. **New pages**: Add a route variant in `src/main.rs` and create a view component
2. **New components**: Add to `src/components/` and export in `mod.rs`
3. **Docker integration**: Extend `src/services/docker.rs` with real Docker API calls

## Technologies

- **[Dioxus 0.7](https://dioxuslabs.com)**: Rust GUI framework with React-like paradigm
- **[Tokio](https://tokio.rs)**: Async runtime for Docker API calls
- **[Serde](https://serde.rs)**: Serialization framework
- **[reqwest](https://docs.rs/reqwest)**: HTTP client for Docker API

## Current Limitations

- **Mock Data**: Currently uses mock Docker data. Real Docker API integration is planned.
- **Read-only**: Container actions (start/stop) update UI state but don't interact with Docker yet.
- **Local only**: Designed for local Docker daemon connections.

## Roadmap

- [ ] Real Docker API integration
- [ ] Container logs viewer
- [ ] Container shell access
- [ ] Docker Compose support
- [ ] Image pull/push
- [ ] Volume creation and deletion
- [ ] Network management
- [ ] Statistics and resource monitoring

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [Dioxus](https://dioxuslabs.com) - A portable, performant, and pragmatic framework for building cross-platform user interfaces in Rust
- Inspired by Docker Desktop and other container management tools

## Support

For questions, issues, or feature requests, please [open an issue](https://github.com/MH0386/containr/issues) on GitHub.
