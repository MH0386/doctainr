# Contributing to Containr

Thank you for your interest in contributing to Containr! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers and help them learn
- Focus on constructive feedback
- Prioritize project goals over personal preferences

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Docker daemon installed and running
- Basic familiarity with Rust and Docker
- Git for version control

### Setting Up Development Environment

1. **Fork and clone the repository**:

``````bash
git clone https://github.com/YOUR_USERNAME/containr.git
cd containr
``````

2. **Install Dioxus CLI**:

``````bash
curl -sSL http://dioxus.dev/install.sh | sh
``````

3. **Build and run**:

``````bash
dx serve --platform desktop
``````

### Development Workflow

1. Create a feature branch from `main`:

``````bash
git checkout -b feature/your-feature-name
``````

2. Make your changes following our coding standards
3. Test your changes thoroughly
4. Commit with clear, descriptive messages
5. Push to your fork
6. Open a Pull Request

## Coding Standards

### Rust Style

Follow standard Rust conventions:

- Use `cargo fmt` for automatic formatting
- Run `cargo clippy` and fix all warnings
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

### Naming Conventions

- **Types**: PascalCase (e.g., `ContainerInfo`, `DockerService`)
- **Functions**: snake_case (e.g., `list_containers`, `format_size`)
- **Constants**: SCREAMING_SNAKE_CASE (e.g., `FAVICON`, `MAIN_CSS`)
- **Private fields**: Prefix with underscore if unused (e.g., `_unused`)

### Code Organization

- **Services**: Pure business logic, no UI dependencies
- **Components**: Presentational, accept props, minimal logic
- **Views**: Page-level components, orchestrate interactions
- **Utils**: Shared helpers and state management

### Documentation

- Add doc comments for public APIs:

``````rust
/// Lists all Docker containers.
///
/// # Returns
/// 
/// A vector of `ContainerInfo` with details about each container.
///
/// # Errors
///
/// Returns an error if the Docker daemon is unreachable.
pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>>
``````

- Update relevant documentation files when adding features
- Include examples in doc comments when helpful

### Testing

#### Unit Tests

Add unit tests for utility functions and data transformations:

``````rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(1024), "1.0KB");
    }
}
``````

#### Running Tests

``````bash
cargo test
``````

For verbose output:

``````bash
cargo test -- --nocapture
``````

#### Test Coverage

Aim for:
- 80%+ coverage for services
- Tests for all public APIs
- Edge case validation

### Error Handling

- Use `anyhow::Result` for service methods
- Store user-facing errors in `AppState.error_message`
- Log detailed errors with context:

``````rust
Err(e) => {
    eprintln!("Failed to list containers: {:#}", e);
    error_message.set(Some("Failed to load containers".to_string()));
}
``````

## Pull Request Process

### Before Submitting

- [ ] Code builds without errors: `cargo build`
- [ ] All tests pass: `cargo test`
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] Code is formatted: `cargo fmt`
- [ ] Documentation is updated
- [ ] Commit messages are clear and descriptive

### PR Description Template

``````markdown
## Description
Brief description of changes

## Motivation
Why is this change needed?

## Changes
- List of specific changes
- Breaking changes (if any)

## Testing
How was this tested?

## Screenshots
If UI changes, include before/after screenshots

## Checklist
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] No breaking changes (or documented)
``````

### Review Process

1. Automated CI checks must pass
2. At least one maintainer review required
3. Address review comments promptly
4. Squash commits if requested
5. Maintainer will merge when approved

## Issue Guidelines

### Reporting Bugs

Use the bug report template:

``````markdown
**Describe the bug**
Clear description of the issue

**To Reproduce**
Steps to reproduce:
1. Go to '...'
2. Click on '...'
3. See error

**Expected behavior**
What should happen

**Environment**
- OS: [e.g., Ubuntu 22.04]
- Rust version: [e.g., 1.75.0]
- Docker version: [e.g., 24.0.7]

**Additional context**
Screenshots, logs, etc.
``````

### Feature Requests

Include:
- Clear description of the feature
- Use case / problem it solves
- Proposed implementation (if applicable)
- Willingness to contribute

### Questions

For questions:
- Check existing documentation first
- Search existing issues
- Use GitHub Discussions for general questions

## Development Tips

### Debugging

Enable debug logs:

``````bash
RUST_LOG=debug cargo run
``````

### Hot Reloading

Dioxus CLI supports hot reloading:

``````bash
dx serve --hot-reload
``````

### Docker Testing

For local testing without Docker daemon:

``````bash
export DOCKER_HOST=tcp://192.168.1.100:2375
``````

### VSCode Setup

Recommended extensions:
- rust-analyzer
- CodeLLDB (for debugging)
- Better TOML

Settings:

``````json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "editor.formatOnSave": true
}
``````

## Project Architecture

Review these documents before contributing:

- [Architecture Guide](./docs/ARCHITECTURE.md) - System design and patterns
- [API Reference](./docs/API.md) - Technical API documentation

## Component Guidelines

### Creating New Components

1. **Presentational components** go in `src/components/`
2. Use props for all inputs
3. Keep components focused and reusable
4. Document props with doc comments

Example:

``````rust
/// Displays a metric card with title, value, and optional hint.
#[component]
pub fn MetricCard(
    /// Card title displayed prominently
    title: String,
    /// Main metric value
    value: String,
    /// Optional hint text shown below value
    hint: Option<String>,
) -> Element {
    // Implementation
}
``````

### Creating New Views

1. **Page views** go in `src/views/`
2. Add route to `Route` enum in `main.rs`
3. Use `AppState` via context
4. Handle loading and error states

## Service Guidelines

### Adding Docker Operations

1. Add method to `DockerService`
2. Add corresponding method to `AppState` that spawns async task
3. Update relevant signals
4. Handle errors gracefully

Example flow:

``````rust
// 1. In DockerService
pub async fn remove_container(&self, id: &str) -> Result<()> {
    self.docker.remove_container(id, None).await?;
    Ok(())
}

// 2. In AppState
pub fn remove_container(&self, id: String) {
    if let Some(service) = &self.docker_service {
        let service = service.clone();
        let app_state = self.clone();
        
        spawn(async move {
            match service.remove_container(&id).await {
                Ok(_) => app_state.refresh_containers(),
                Err(e) => app_state.error_message.set(Some(e.to_string())),
            }
        });
    }
}
``````

## Release Process

Maintainers follow this process for releases:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create git tag: `git tag v0.x.0`
4. Push tag: `git push origin v0.x.0`
5. GitHub Actions builds and creates release

## Getting Help

- **GitHub Discussions**: General questions and ideas
- **GitHub Issues**: Bug reports and feature requests
- **Pull Request Comments**: Specific code feedback

## Recognition

Contributors will be recognized in:
- `CONTRIBUTORS.md` file
- Release notes
- Project README

Thank you for contributing to Containr! 🎉
