# Doctainr Documentation

Welcome to the Doctainr documentation. This Docker desktop application is built with Rust and Dioxus 0.7, providing real-time Docker management with a native desktop interface.

## Documentation Structure

This documentation follows the [Diátaxis framework](https://diataxis.fr/) for systematic documentation:

### 📚 [Tutorials](./tutorials/)
**Learning-oriented**: Step-by-step lessons for getting started
- [Getting Started](./tutorials/getting-started.md) - Your first steps with Doctainr
- [Building Your First View](./tutorials/building-views.md) - Create custom UI components

### 📖 [How-To Guides](./guides/)
**Problem-oriented**: Practical solutions for specific tasks
- [Contributing Guide](./guides/contributing.md) - How to contribute to the project
- [Deployment Guide](./guides/deployment.md) - Build and deploy Doctainr
- [Troubleshooting](./guides/troubleshooting.md) - Common issues and solutions

### 📋 [Technical Reference](./reference/)
**Information-oriented**: Precise technical descriptions
- [API Reference](./reference/api.md) - Complete API documentation
- [Architecture](./reference/architecture.md) - System design and structure
- [Components Reference](./reference/components.md) - UI component library

### 💡 [Explanation](./guides/)
**Understanding-oriented**: Background and design decisions
- [Why Dioxus?](./guides/why-dioxus.md) - Technology choices explained
- [State Management](./guides/state-management.md) - How app state works

## Quick Links

- **[GitHub Repository](https://github.com/MH0386/doctainr)**
- **[Report Issues](https://github.com/MH0386/doctainr/issues)**
- **[Contributing Guidelines](./guides/contributing.md)**

## Prerequisites

- Docker Desktop or Docker Engine running
- Rust toolchain 1.70 or later
- Dioxus CLI (`dx`)

## Quick Start

````bash
# Clone the repository
git clone https://github.com/MH0386/doctainr.git
cd doctainr

# Run the application
dx serve --platform desktop
````

## Getting Help

- Check the [Troubleshooting Guide](./guides/troubleshooting.md)
- Search [existing issues](https://github.com/MH0386/doctainr/issues)
- Open a [new issue](https://github.com/MH0386/doctainr/issues/new)

---

**Built with** 🦀 Rust + 🎨 Dioxus 0.7
