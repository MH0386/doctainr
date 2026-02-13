# Contributing to Containr

Thank you for your interest in contributing to Containr! This document provides guidelines and information for contributors.

## Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please be respectful and considerate in all interactions.

## How Can I Contribute?

### Reporting Bugs

Before submitting a bug report:
- Check existing [issues](https://github.com/MH0386/containr/issues) to avoid duplicates
- Ensure you're using the latest version
- Verify Docker is running correctly

When submitting a bug report, include:
- **Description**: Clear description of the problem
- **Steps to reproduce**: Detailed steps to reproduce the issue
- **Expected behavior**: What you expected to happen
- **Actual behavior**: What actually happened
- **Environment**: OS, Rust version, Docker version
- **Logs**: Relevant error messages or logs

**Example:**

``````markdown
**Description**
Application crashes when clicking "Start" on stopped container

**Steps to Reproduce**
1. Open Containr
2. Navigate to Containers page
3. Click "Start" button on stopped container

**Expected Behavior**
Container should start

**Actual Behavior**
Application crashes with panic message

**Environment**
- OS: Ubuntu 22.04
- Rust: 1.75.0
- Docker: 24.0.5

**Logs**
```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value'
```
``````

### Suggesting Features

We welcome feature suggestions! When proposing a feature:

- **Use case**: Explain the problem it solves
- **Description**: Describe the proposed solution
- **Alternatives**: Consider alternative approaches
- **Mockups**: Include UI mockups if applicable

**Example:**

``````markdown
**Use Case**
As a developer, I want to view container logs to debug application issues.

**Description**
Add a "Logs" view that displays real-time container logs with filtering and search.

**Proposed Solution**
- New route `/containers/:id/logs`
- Stream logs using Docker API
- Support log filtering by level
- Search functionality

**Alternatives**
- Open external terminal
- Integration with external log viewers
``````

### Pull Requests

#### Before Submitting

1. **Search existing PRs** to avoid duplicates
2. **Discuss major changes** in an issue first
3. **Follow code style** (run `cargo fmt`)
4. **Add tests** for new features
5. **Update documentation** as needed

#### PR Process

1. **Fork** the repository
2. **Create a branch** from `main`:
   ``````bash
   git checkout -b feature/my-feature
   ``````

3. **Make your changes** following code style guidelines

4. **Write tests** for new functionality:
   ``````rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_new_feature() {
           // Test implementation
       }
   }
   ``````

5. **Run tests and lints**:
   ``````bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ``````

6. **Commit your changes** with clear messages:
   ``````bash
   git commit -m "Add feature: container log viewer"
   ``````

7. **Push to your fork**:
   ``````bash
   git push origin feature/my-feature
   ``````

8. **Open a Pull Request** with:
   - Clear title describing the change
   - Description of what changed and why
   - Reference to related issues
   - Screenshots for UI changes

#### PR Review

- Be responsive to feedback
- Make requested changes promptly
- Discuss disagreements constructively
- Be patient during review process

## Development Guidelines

### Code Style

#### Rust Conventions

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting: `cargo fmt`
- Use `clippy` for linting: `cargo clippy`
- Document public APIs with doc comments

**Example:**

``````rust
/// Fetches all running containers from Docker daemon.
///
/// # Returns
///
/// A vector of `ContainerInfo` structs representing running containers.
///
/// # Errors
///
/// Returns an error if Docker connection fails or API call fails.
pub async fn fetch_containers() -> Result<Vec<ContainerInfo>, Error> {
    // Implementation
}
``````

#### Component Guidelines

**✅ Good:**
``````rust
#[component]
pub fn MetricCard(title: String, value: String, hint: Option<String>) -> Element {
    rsx! {
        div { class: "metric-card",
            h3 { "{title}" }
            p { class: "value", "{value}" }
            if let Some(hint_text) = hint {
                p { class: "hint", "{hint_text}" }
            }
        }
    }
}
``````

**❌ Bad:**
``````rust
// Missing #[component] annotation
// Using &str instead of String
pub fn metric_card(title: &str, value: &str) -> Element {
    // Missing proper structure
}
``````

#### State Management

**✅ Good:**
``````rust
// Use signals for reactive state
let mut count = use_signal(|| 0);

// Use memo for derived state
let double = use_memo(move || count() * 2);

// Clone signals when passing to closures
let mut count_clone = count.clone();
``````

**❌ Bad:**
``````rust
// Don't use RefCell or Rc manually
let count = Rc::new(RefCell::new(0));

// Don't mutate state directly without signals
let mut count = 0;
count += 1; // Won't trigger re-render
``````

### Testing

#### Unit Tests

Test individual functions and components:

``````rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_state_transitions() {
        let state = ContainerState::Stopped;
        assert_eq!(state.action_label(), "Start");
    }
}
``````

#### Integration Tests

Test feature workflows:

``````rust
// tests/container_management.rs
#[test]
fn test_container_lifecycle() {
    let containers = mock_containers();
    assert!(!containers.is_empty());
    
    let running = containers.iter()
        .filter(|c| c.state == ContainerState::Running)
        .count();
    assert!(running > 0);
}
``````

#### Test Coverage

Aim for good test coverage:
- Core business logic: 80%+
- UI components: Basic smoke tests
- Integration points: Critical paths covered

### Documentation

#### Code Comments

- Document **why**, not **what**
- Explain complex algorithms
- Note edge cases and gotchas

**Good:**
``````rust
// We clone the signal here to avoid holding a mutable borrow
// across the async boundary, which would violate Rust's borrow rules
let mut containers = self.containers.clone();
``````

**Bad:**
``````rust
// Clone containers
let mut containers = self.containers.clone();
``````

#### Markdown Documentation

- Use clear headings
- Include code examples
- Add links to references
- Keep it up-to-date

### Commit Messages

Follow conventional commit format:

``````
<type>(<scope>): <subject>

<body>

<footer>
``````

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**

``````
feat(containers): add log viewer

Implement real-time container log streaming using Docker API.
Includes filtering by log level and search functionality.

Closes #42
``````

``````
fix(state): prevent concurrent state updates

Use mutex to prevent race condition when multiple components
update container state simultaneously.

Fixes #58
``````

## Project-Specific Guidelines

### Dioxus Best Practices

1. **Components must be `#[component]` annotated**
2. **Props must be owned types** (String, not &str)
3. **Use signals for reactive state**
4. **Memo expensive calculations**
5. **Keep components small and focused**

### Docker Integration

When adding Docker API calls:

1. **Use async/await** with tokio runtime
2. **Handle errors gracefully** with Result types
3. **Provide mock data fallback** for testing
4. **Cache responses** when appropriate
5. **Rate limit API calls** to avoid overwhelming Docker

### UI/UX Guidelines

1. **Maintain dark theme consistency**
2. **Use existing CSS classes** where possible
3. **Ensure responsive layout**
4. **Provide loading states**
5. **Show error messages clearly**

## Release Process

Releases are managed by maintainers:

1. **Version bump** in `Cargo.toml`
2. **Update CHANGELOG.md**
3. **Create GitHub release**
4. **Tag with semantic version**

Version scheme: `MAJOR.MINOR.PATCH`

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes

## Getting Help

- **Documentation**: Check [docs/](docs/) directory
- **Issues**: Search existing issues or create new one
- **Discussions**: Use GitHub Discussions for questions
- **Code**: Read existing code for examples

## Recognition

Contributors will be recognized in:
- GitHub contributors list
- CHANGELOG.md for significant contributions
- README.md acknowledgments section

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT License).

Thank you for contributing to Containr! 🎉
