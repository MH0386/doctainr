# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Real Docker API integration
- Container logs viewer
- Container shell access
- Docker Compose support
- Image pull/push functionality
- Volume creation and deletion
- Network management
- Statistics and resource monitoring
- Keyboard shortcuts
- Search and filtering

## [0.1.0] - 2026-02-13

### Added
- Initial release of Containr
- Desktop application built with Dioxus 0.7
- Dashboard with Docker engine overview
- Container management view with start/stop controls
- Image browser with repository, tag, and size information
- Volume management view
- Settings page for Docker host configuration
- Modern dark theme UI
- Sidebar navigation
- Mock Docker data for development
- Application state management with signals
- Routing with Dioxus Router
- Reusable UI components (MetricCard, SectionHeader, StatusPill)
- Unit tests for core functionality
- Development environment with devenv.nix
- CI/CD workflows (GitHub Actions)
- Code quality tools (clippy, rustfmt)
- Documentation structure

### UI Components
- MetricCard: Displays metrics with title, value, and hint
- SectionHeader: Page headers with title and subtitle
- StatusPill: Visual container status indicators
- AppShell: Application layout with navigation

### Styling
- Dark theme with custom CSS
- Responsive layout
- Action buttons with hover states
- Error message styling
- Navigation sidebar with hover effects

### Technical
- Rust 2021 edition
- Dioxus 0.7.1 with desktop support
- Tokio async runtime
- Serde for serialization
- Context API for state management
- Signal-based reactive state

### Developer Experience
- Dioxus CLI support with hot reload
- Custom clippy configuration
- Devenv.nix for reproducible development environment
- Comprehensive documentation

## [0.0.1] - Initial Commit

### Added
- Project initialization
- Basic project structure
- Cargo configuration
- Dioxus setup

---

## Release Notes

### Version 0.1.0 Notes

This is the initial release of Containr, focusing on establishing the UI/UX foundation and application architecture. The application currently uses mock Docker data to demonstrate functionality.

**Key Features:**
- Clean, modern interface for Docker management
- Reactive state management with Dioxus signals
- Modular component architecture
- Comprehensive documentation

**Current Limitations:**
- Mock data only (no real Docker API integration)
- Container actions update UI but don't interact with Docker
- Local Docker daemon connections only

**Upcoming:**
The next major release will focus on integrating the real Docker API using the `bollard` crate, enabling full container lifecycle management.

---

## How to Upgrade

### From Source

``````bash
git pull origin main
cargo build --release
``````

### Clean Install

If you encounter issues after upgrading:

``````bash
cargo clean
cargo update
cargo build --release
``````

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on contributing to this project.

## License

This project is licensed under the MIT License.
