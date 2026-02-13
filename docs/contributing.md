# Contributing to Doctainr

Thank you for your interest in contributing to Doctainr! This guide will help you get started.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Code Standards](#code-standards)
- [Documentation](#documentation)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)

---

## Code of Conduct

### Our Standards

- **Be respectful** - Treat all contributors with respect
- **Be constructive** - Provide helpful feedback
- **Be collaborative** - Work together toward common goals
- **Be inclusive** - Welcome contributors of all backgrounds

### Reporting Issues

If you experience unacceptable behavior, please email the maintainers.

---

## Getting Started

### Prerequisites

1. **Rust** 1.70 or later
   ````bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ````

2. **Docker** (running locally)
   ````bash
   docker info
   ````

3. **Dioxus CLI**
   ````bash
   cargo install dioxus-cli
   ````

### Initial Setup

````bash
# Clone the repository
git clone https://github.com/MH0386/doctainr.git
cd doctainr

# Build the project
cargo build

# Run the application
dx run
````

### Optional: devenv Setup

If you have [devenv](https://devenv.sh/) installed:

````bash
devenv shell
devenv tasks run dx:run
````

---

## Development Workflow

### 1. Create a Branch

````bash
git checkout -b feature/your-feature-name
````

**Branch Naming**:
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation only
- `refactor/` - Code refactoring
- `test/` - Test additions/changes

### 2. Make Changes

Edit code following our [code standards](#code-standards).

### 3. Test Changes

````bash
# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run linter
cargo clippy

# Build to verify
cargo build
````

### 4. Commit Changes

````bash
git add .
git commit -m "feat: add container restart functionality"
````

**Commit Message Format**:
````
<type>: <description>

[optional body]

[optional footer]
````

**Types**:
- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation
- `style` - Formatting
- `refactor` - Code restructuring
- `test` - Test changes
- `chore` - Build/tooling

### 5. Push and Create PR

````bash
git push origin feature/your-feature-name
````

Then create a Pull Request on GitHub.

---

## Code Standards

### Rust Style Guide

Follow the official [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/).

**Key Points**:
- Use `cargo fmt` for automatic formatting
- Max line length: 100 characters
- Use descriptive variable names
- Prefer explicit types when clarity improves
- Use `?` for error propagation

### Code Organization

````
src/
├── main.rs              # Entry point and routing
├── components/          # Reusable UI components
│   ├── mod.rs
│   └── component_name.rs
├── views/               # Page-level components
│   ├── mod.rs
│   └── view_name.rs
├── services/            # External integrations
│   ├── mod.rs
│   └── docker.rs
└── utils/               # Shared utilities
    ├── mod.rs
    └── app_state.rs
````

### Naming Conventions

| Item | Convention | Example |
|------|------------|---------|
| Files | snake_case | `app_state.rs` |
| Functions | snake_case | `refresh_containers()` |
| Types | PascalCase | `ContainerInfo` |
| Constants | SCREAMING_SNAKE_CASE | `MAX_RETRIES` |
| Modules | snake_case | `mod utils` |

### Documentation Comments

Use doc comments for public APIs:

````rust
/// Starts a Docker container by ID.
///
/// # Arguments
///
/// * `id` - Container ID (short or full form)
///
/// # Returns
///
/// * `Ok(())` on success
/// * `Err` if container not found or already running
///
/// # Examples
///
/// ```rust
/// service.start_container("abc123").await?;
/// ```
pub async fn start_container(&self, id: &str) -> Result<()> {
    // Implementation
}
````

---

## Documentation

### Documentation Philosophy

> **Documentation gaps are treated like failing tests.**

If you add a feature, document it. If you find missing documentation, add it.

### What to Document

1. **Public APIs** - All public functions, structs, enums
2. **Architecture** - Design decisions and rationale
3. **User Guides** - How to accomplish tasks
4. **Examples** - Working code samples

### Documentation Structure

See [docs/README.md](docs/README.md) for the full structure.

**When to update**:
- New features → Update API docs + user guide
- Bug fixes → Update if behavior changed
- Refactoring → Update architecture docs if design changed

### Writing Style

- **Active voice**: "The function returns..." not "The value is returned..."
- **Present tense**: "Updates the state" not "Will update the state"
- **Concise**: Remove unnecessary words
- **Examples**: Include working code samples

---

## Testing

### Running Tests

````bash
# All tests
cargo test

# Specific test
cargo test test_format_size

# With output
cargo test -- --nocapture
````

### Writing Tests

#### Unit Tests

````rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_state_labels() {
        assert_eq!(ContainerState::Running.label(), "Running");
        assert_eq!(ContainerState::Stopped.label(), "Stopped");
    }
}
````

#### Integration Tests

````rust
#[tokio::test]
async fn test_list_containers() {
    let service = DockerService::new().unwrap();
    let containers = service.list_containers().await;
    assert!(containers.is_ok());
}
````

**Note**: Integration tests require Docker daemon running.

### Test Coverage

Aim for:
- **90%+** - Core business logic (services, state management)
- **70%+** - UI components (where feasible)
- **100%** - Critical paths (data transformations, error handling)

---

## Pull Request Process

### Before Submitting

- [ ] Code formatted (`cargo fmt`)
- [ ] Lints pass (`cargo clippy`)
- [ ] Tests pass (`cargo test`)
- [ ] Documentation updated
- [ ] Commit messages follow convention

### PR Template

````markdown
## Description

Brief description of changes.

## Type of Change

- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing

Describe testing performed.

## Screenshots (if applicable)

Include screenshots for UI changes.

## Checklist

- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] Tests added/updated
````

### Review Process

1. **Automated Checks** - CI runs tests and lints
2. **Code Review** - Maintainer reviews code
3. **Feedback** - Address review comments
4. **Approval** - Maintainer approves PR
5. **Merge** - PR merged to main branch

### Review Criteria

- ✅ **Functionality** - Does it work as intended?
- ✅ **Code Quality** - Is it readable and maintainable?
- ✅ **Tests** - Are there adequate tests?
- ✅ **Documentation** - Is it documented?
- ✅ **Performance** - Is it efficient?

---

## Project Structure

### Module Responsibilities

| Module | Responsibility | Dependencies |
|--------|---------------|--------------|
| `main.rs` | App initialization, routing | All modules |
| `components/` | Reusable UI elements | None |
| `views/` | Page-level components | components, utils |
| `services/` | External integrations | None |
| `utils/` | State management | services |

### Dependency Rules

- **No circular dependencies**
- **UI doesn't import services** (goes through state layer)
- **Services are self-contained**

---

## Issue Reporting

### Bug Reports

Include:
- **Description** - What went wrong?
- **Steps to Reproduce** - How to recreate the issue?
- **Expected Behavior** - What should happen?
- **Actual Behavior** - What actually happens?
- **Environment** - OS, Rust version, Docker version
- **Screenshots** - If applicable

### Feature Requests

Include:
- **Problem Statement** - What problem does this solve?
- **Proposed Solution** - How should it work?
- **Alternatives** - Other approaches considered?
- **Additional Context** - Any other relevant info?

---

## Development Tips

### Hot Reload

Use `dx serve` for hot reload during development:

````bash
dx serve --platform desktop
````

Changes to RSX will reload automatically.

### Debugging

**Console Logging**:
````rust
println!("Debug: {:?}", value);
eprintln!("Error: {}", error);
````

**Logging Crate** (future enhancement):
````rust
use log::{info, warn, error};

info!("Container started: {}", id);
error!("Failed to connect: {}", err);
````

### Common Issues

**Issue**: Docker connection fails  
**Solution**: Ensure Docker daemon is running, check socket permissions

**Issue**: Compilation errors with signals  
**Solution**: Verify using Dioxus 0.7 patterns (no `use_state`, no `cx`)

**Issue**: Hot reload not working  
**Solution**: Restart `dx serve`, check `Dioxus.toml` config

---

## Resources

### Documentation

- [Doctainr Docs](docs/README.md) - Project documentation
- [Dioxus 0.7 Guide](https://dioxuslabs.com/learn/0.7/) - Framework docs
- [Bollard Docs](https://docs.rs/bollard/) - Docker client library

### Community

- [GitHub Issues](https://github.com/MH0386/doctainr/issues) - Bug reports and features
- [GitHub Discussions](https://github.com/MH0386/doctainr/discussions) - Questions and ideas

### Tools

- [Rust Book](https://doc.rust-lang.org/book/) - Learn Rust
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Rust examples
- [Docker Docs](https://docs.docker.com/) - Docker reference

---

## License

By contributing, you agree that your contributions will be licensed under the same license as the project.

---

## Questions?

- **General questions**: Open a [Discussion](https://github.com/MH0386/doctainr/discussions)
- **Bug reports**: Open an [Issue](https://github.com/MH0386/doctainr/issues)
- **Security issues**: Email maintainers directly

Thank you for contributing! 🎉
