# Contributing to Doctainr

Thank you for your interest in contributing to Doctainr! This document provides guidelines and instructions for contributing to the project.

## Getting Started

1. **Fork the Repository**: Click the "Fork" button on GitHub
2. **Clone Your Fork**:
   ````bash
   git clone https://github.com/YOUR_USERNAME/containr.git
   cd containr
   ````
3. **Set Up Development Environment**: See [Development Setup](#development-setup)

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Docker daemon running locally
- Dioxus CLI (recommended for development)

### Environment Setup

We recommend using `devenv` for a consistent development environment:

````bash
# Install devenv (if not already installed)
curl -fsSL https://get.jetpack.io/devenv | sh

# Enter the development environment
devenv shell
````

This will automatically install all required dependencies including Rust, Dioxus CLI, and development tools.

### Manual Setup

If not using devenv:

````bash
# Install Dioxus CLI
curl -sSL http://dioxus.dev/install.sh | sh

# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
````

## Development Workflow

### 1. Create a Feature Branch

````bash
git checkout -b feature/your-feature-name
````

Branch naming conventions:
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation updates
- `refactor/` - Code refactoring
- `test/` - Test additions or improvements

### 2. Make Your Changes

- Write clear, concise code following Rust conventions
- Add tests for new functionality
- Update documentation as needed
- Follow the [Code Style Guidelines](#code-style-guidelines)

### 3. Run Tests and Linters

````bash
# Run tests
cargo test

# Run clippy
cargo clippy -- -D warnings

# Format code
cargo fmt
````

### 4. Commit Your Changes

We follow conventional commit messages:

````bash
git commit -m "feat: add container log viewer"
git commit -m "fix: resolve Docker connection timeout"
git commit -m "docs: update installation instructions"
````

Commit types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Test additions or changes
- `chore`: Maintenance tasks

### 5. Push and Create Pull Request

````bash
git push origin feature/your-feature-name
````

Then create a pull request on GitHub with:
- Clear title describing the change
- Detailed description of what and why
- Reference any related issues
- Screenshots (if UI changes)

## Code Style Guidelines

### Rust Code

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for consistent formatting
- Address all `cargo clippy` warnings
- Keep functions small and focused
- Write descriptive variable names

### Dioxus Components

````rust
#[component]
fn ComponentName(prop_name: Type) -> Element {
    // Use descriptive component names (PascalCase)
    // Props should be owned types, not references
    // Use Signal<T> for mutable state
    
    rsx! {
        // Use semantic HTML elements
        // Keep RSX readable and well-indented
        // Use CSS classes for styling
    }
}
````

### Documentation

- Add doc comments for public functions and types
- Use `///` for doc comments, not `//`
- Include examples in doc comments when helpful
- Keep comments up-to-date with code changes

Example:
````rust
/// Lists all running Docker containers.
///
/// # Returns
///
/// A vector of `ContainerInfo` structs representing running containers.
///
/// # Errors
///
/// Returns an error if the Docker daemon is unreachable or the API call fails.
pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>> {
    // Implementation
}
````

## Testing Guidelines

### Unit Tests

Place unit tests in the same file as the code they test:

````rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_state_label() {
        assert_eq!(ContainerState::Running.label(), "Running");
        assert_eq!(ContainerState::Stopped.label(), "Stopped");
    }
}
````

### Integration Tests

Place integration tests in the `tests/` directory:

````rust
// tests/docker_service_tests.rs
#[tokio::test]
async fn test_list_containers() {
    let service = DockerService::new().unwrap();
    let containers = service.list_containers().await.unwrap();
    assert!(containers.len() >= 0);
}
````

## Pull Request Guidelines

### Before Submitting

- [ ] Code builds without errors
- [ ] All tests pass
- [ ] Clippy produces no warnings
- [ ] Code is formatted with `cargo fmt`
- [ ] Documentation is updated
- [ ] Commit messages follow conventions
- [ ] Branch is up-to-date with main

### PR Description Template

````markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
Describe how you tested your changes

## Screenshots (if applicable)
Add screenshots for UI changes

## Related Issues
Closes #123
````

## Issue Reporting

### Bug Reports

Include:
- Clear, descriptive title
- Steps to reproduce
- Expected vs actual behavior
- Environment details (OS, Rust version, Docker version)
- Error messages and logs
- Screenshots if applicable

### Feature Requests

Include:
- Clear, descriptive title
- Problem description
- Proposed solution
- Alternative solutions considered
- Additional context

## Code Review Process

1. Maintainers review PRs within 48 hours
2. Address feedback and requested changes
3. Keep discussions respectful and constructive
4. Once approved, a maintainer will merge your PR

## Community Guidelines

- Be respectful and inclusive
- Provide constructive feedback
- Help others learn and grow
- Follow the [Code of Conduct](CODE_OF_CONDUCT.md)

## Need Help?

- Open a [GitHub Issue](https://github.com/MH0386/containr/issues)
- Join our [Discussions](https://github.com/MH0386/containr/discussions)
- Check the [Documentation](https://github.com/MH0386/containr/tree/main/docs)

Thank you for contributing to Doctainr! 🎉
