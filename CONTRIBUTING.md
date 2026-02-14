# Contributing to Doctainr

Thank you for your interest in contributing to Doctainr! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

Be respectful, inclusive, and constructive. We aim to create a welcoming environment for all contributors.

## How to Contribute

### Reporting Issues

If you find a bug or have a feature request:

1. **Search existing issues** to avoid duplicates
2. **Open a new issue** with a clear title and description
3. **Provide details**:
   - For bugs: Steps to reproduce, expected vs actual behavior, system info
   - For features: Use case, proposed solution, alternatives considered

### Submitting Changes

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/your-feature-name`
3. **Make your changes** following our coding standards
4. **Write or update tests** if applicable
5. **Run checks** (see below)
6. **Commit with clear messages** (see commit guidelines)
7. **Push to your fork**: `git push origin feature/your-feature-name`
8. **Open a Pull Request** with a clear description

## Development Setup

See [Building from Source](docs/how-to/build-from-source.md) for detailed setup instructions.

Quick start:

````bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/doctainr.git
cd doctainr

# Install dependencies (Rust must be installed)
cargo build

# Run in development mode
dx serve
````

## Code Standards

### Formatting

All code must be formatted with `rustfmt`:

````bash
cargo fmt
````

Configuration is in the default `rustfmt.toml` (if present).

### Linting

Code must pass `clippy` without warnings:

````bash
cargo clippy -- -D warnings
````

Configuration is in `clippy.toml`.

### Testing

Run existing tests before submitting:

````bash
cargo test
````

Add tests for new functionality when appropriate.

## Commit Guidelines

Follow conventional commit format:

````
<type>(<scope>): <subject>

<body>

<footer>
````

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, no logic change)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Build process, dependencies, tooling

**Examples**:

````
feat(containers): add container removal functionality

Adds a delete button to the container view that calls the Docker
API to remove stopped containers.

Closes #123
````

````
fix(docker): handle connection timeout gracefully

Previously the app would hang if Docker didn't respond. Now it
shows an error message after 5 seconds.
````

````
docs: add architecture documentation

Creates comprehensive architecture docs following Diátaxis framework.
````

## Pull Request Guidelines

### PR Description Template

````markdown
## Description
Brief description of what this PR does.

## Motivation
Why is this change necessary? What problem does it solve?

## Changes Made
- List of key changes
- Another change
- And another

## Testing
How was this tested? What scenarios were covered?

## Screenshots (if applicable)
Add screenshots for UI changes.

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex logic
- [ ] Documentation updated
- [ ] Tests pass
- [ ] No new warnings from clippy
````

### Review Process

1. **Automated checks** must pass (formatting, linting, tests)
2. **Code review** by maintainer(s)
3. **Requested changes** addressed
4. **Approval** and merge

## Architecture Principles

When contributing, keep these principles in mind:

1. **Simplicity**: Prefer simple, readable code over clever optimizations
2. **Safety**: Leverage Rust's type system and ownership model
3. **Performance**: Avoid unnecessary allocations, prefer references
4. **Modularity**: Keep components focused and reusable
5. **Error Handling**: Always handle errors gracefully, never panic in production code

## Component Structure

When creating new components:

````rust
use dioxus::prelude::*;

/// Brief description of what this component does.
///
/// # Props
/// - `title`: Description of this prop
/// - `value`: Description of this prop
#[component]
pub fn MyComponent(
    title: String,
    value: ReadOnlySignal<i32>,
) -> Element {
    rsx! {
        div { class: "my-component",
            h2 { "{title}" }
            p { "Value: {value}" }
        }
    }
}
````

## Adding New Features

### UI Features

1. **Design first**: Consider UX and existing patterns
2. **Create component**: Add to `src/components/` or `src/views/`
3. **Update state**: Modify `AppState` if needed
4. **Add route**: Update `Route` enum if new page
5. **Style**: Add CSS classes to `assets/styling/main.css`
6. **Document**: Update relevant documentation

### Docker Features

1. **Service method**: Add to `DockerService` in `src/services/docker.rs`
2. **Data model**: Create or update struct in `docker.rs`
3. **AppState method**: Add wrapper in `src/utils/app_state.rs`
4. **UI integration**: Call from component
5. **Error handling**: Ensure errors are caught and displayed
6. **Test**: Add unit tests if possible

## Documentation

### Updating Documentation

Follow the Diátaxis framework:

- **Tutorials** (`docs/tutorials/`): Step-by-step learning guides
- **How-To** (`docs/how-to/`): Task-oriented instructions
- **Reference** (`docs/reference/`): Technical descriptions
- **Explanation** (`docs/explanation/`): Conceptual discussions

Use 4 backticks for code blocks to avoid formatting issues:

`````markdown
````rust
fn example() {
    println!("code here");
}
````
`````

### Code Comments

- **Document public APIs**: All public functions, structs, and modules
- **Explain complex logic**: When code isn't self-explanatory
- **Don't state the obvious**: Avoid redundant comments
- **Use rustdoc format**: `///` for docs, `//` for inline comments

````rust
/// Lists all Docker containers on the local machine.
///
/// # Returns
/// A `Result` containing a vector of `ContainerInfo` on success,
/// or an error if the Docker API call fails.
///
/// # Errors
/// Returns an error if:
/// - Docker daemon is not running
/// - Connection to Docker socket fails
pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>> {
    // Implementation
}
````

## Licensing

By contributing, you agree that your contributions will be licensed under the same license as the project (to be determined).

## Getting Help

- **Questions**: Open a GitHub Discussion
- **Issues**: Open a GitHub Issue
- **Chat**: [To be determined - Discord/Matrix?]

## Recognition

Contributors will be acknowledged in the README and release notes.

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Dioxus Documentation](https://dioxuslabs.com/learn/0.7/)
- [Bollard Documentation](https://docs.rs/bollard/)
- [Docker Engine API](https://docs.docker.com/engine/api/)

Thank you for contributing to Doctainr! 🎉
