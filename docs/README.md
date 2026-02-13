# Containr Documentation

Welcome to the Containr documentation! This guide will help you understand, use, and contribute to Containr.

## Documentation Index

### For Users

- **[Getting Started](./GETTING_STARTED.md)** - Installation, setup, and first steps
  - Quick start guide
  - Basic workflows
  - Troubleshooting common issues

### For Developers

- **[Architecture](./ARCHITECTURE.md)** - System design and technical architecture
  - Component hierarchy
  - State management patterns
  - Data flow and reactivity
  - Service layer design

- **[API Reference](./API.md)** - Technical API documentation
  - DockerService methods
  - AppState interface
  - Component props
  - Data models

- **[Contributing](./CONTRIBUTING.md)** - How to contribute to Containr
  - Development setup
  - Coding standards
  - Pull request process
  - Testing guidelines

## Quick Links

### New to Containr?

Start with [Getting Started](./GETTING_STARTED.md) to install and run the application.

### Want to Contribute?

1. Read the [Architecture](./ARCHITECTURE.md) guide to understand the codebase
2. Check the [Contributing](./CONTRIBUTING.md) guide for guidelines
3. Browse [open issues](https://github.com/MH0386/containr/issues) for tasks

### Need Technical Details?

Refer to the [API Reference](./API.md) for complete technical documentation.

## Project Overview

Containr is a modern, lightweight Docker desktop application built with:

- **[Dioxus 0.7](https://dioxuslabs.com/)** - React-like UI framework for Rust
- **[Bollard](https://github.com/fussybeaver/bollard)** - Async Docker API client
- **[Tokio](https://tokio.rs/)** - Async runtime
- **Rust** - Systems programming language

### Key Features

- **Container Management**: Start, stop, and monitor containers
- **Image Browser**: View local Docker images
- **Volume Inspector**: Manage Docker volumes
- **Real-time Updates**: Reactive UI with instant feedback
- **Native Performance**: Fast and lightweight desktop app

## Architecture at a Glance

``````
┌─────────────────────────────────────────┐
│             Views (UI)                  │
│  Dashboard │ Containers │ Images │...   │
└──────────────────┬──────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────┐
│          AppState (State)               │
│  Signals + Methods + Event Handlers     │
└──────────────────┬──────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────┐
│       DockerService (API Client)        │
│  list_containers │ start │ stop │...    │
└──────────────────┬──────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────┐
│          Docker Daemon                  │
│      /var/run/docker.sock               │
└─────────────────────────────────────────┘
``````

### Data Flow

1. **User Action** → View component
2. **Method Call** → AppState
3. **Async Task** → Spawned on Tokio runtime
4. **API Call** → DockerService → Docker daemon
5. **Update Signal** → AppState signal updated
6. **Re-render** → View automatically updates

## Documentation Structure

This documentation follows the [Diátaxis framework](https://diataxis.fr/):

### 📚 Tutorials (Learning-oriented)

- [Getting Started](./GETTING_STARTED.md) - Hands-on introduction

### 🔧 How-to Guides (Problem-oriented)

- [CONTRIBUTING.md](./CONTRIBUTING.md) - Step-by-step contribution guide

### 📖 Reference (Information-oriented)

- [API.md](./API.md) - Complete API reference

### 💡 Explanation (Understanding-oriented)

- [ARCHITECTURE.md](./ARCHITECTURE.md) - Design principles and patterns

## Community

### Getting Help

- **Questions**: [GitHub Discussions](https://github.com/MH0386/containr/discussions)
- **Bug Reports**: [GitHub Issues](https://github.com/MH0386/containr/issues)
- **Feature Requests**: [GitHub Issues](https://github.com/MH0386/containr/issues)

### Contributing

We welcome contributions! See [CONTRIBUTING.md](./CONTRIBUTING.md) for:

- Code of conduct
- Development workflow
- Coding standards
- Testing requirements
- Pull request process

## Additional Resources

### Dioxus Resources

- [Dioxus Documentation](https://dioxuslabs.com/learn/0.7)
- [Dioxus Examples](https://github.com/DioxusLabs/example-projects)
- [Dioxus Community](https://discord.gg/XgGxMSkvUM)

### Docker Resources

- [Docker Documentation](https://docs.docker.com/)
- [Docker API Reference](https://docs.docker.com/engine/api/)
- [Bollard Docs](https://docs.rs/bollard/)

### Rust Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Async Book](https://rust-lang.github.io/async-book/)

## Version Information

- **Dioxus**: 0.7.1
- **Rust Edition**: 2021
- **Minimum Rust**: 1.70+

## License

See [LICENSE](../LICENSE) file for details.

## Acknowledgments

Built with amazing open source projects:
- Dioxus Labs for the reactive UI framework
- Bollard team for the Docker client
- Tokio team for the async runtime
- Rust community for the language and ecosystem
