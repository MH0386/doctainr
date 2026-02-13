# Contributing to Doctainr

Thank you for considering contributing to Doctainr! This guide will help you get started.

## Code of Conduct

Be respectful, inclusive, and constructive in all interactions. We're building a welcoming community.

## Getting Started

### Prerequisites

1. Rust toolchain (1.70+)
2. Docker Desktop or Engine
3. Dioxus CLI: `curl -sSL http://dioxus.dev/install.sh | sh`
4. Git

### Fork and Clone

````bash
# Fork the repository on GitHub, then:
git clone https://github.com/YOUR_USERNAME/doctainr.git
cd doctainr
git remote add upstream https://github.com/MH0386/doctainr.git
````

### Development Workflow

1. **Create a branch**
   ````bash
   git checkout -b feature/my-new-feature
   ````

2. **Make your changes**
   - Follow the existing code style
   - Add tests for new functionality
   - Update documentation

3. **Test your changes**
   ````bash
   cargo test
   cargo clippy
   cargo fmt --check
   ````

4. **Run the application**
   ````bash
   dx serve --platform desktop
   ````

5. **Commit your changes**
   ````bash
   git add .
   git commit -m "feat: add new feature"
   ````
   
   Follow [Conventional Commits](https://www.conventionalcommits.org/):
   - `feat:` New feature
   - `fix:` Bug fix
   - `docs:` Documentation changes
   - `style:` Code style changes (formatting)
   - `refactor:` Code refactoring
   - `test:` Adding or updating tests
   - `chore:` Maintenance tasks

6. **Push and create Pull Request**
   ````bash
   git push origin feature/my-new-feature
   ````
   Then open a Pull Request on GitHub.

## Project Structure

````
src/
├── main.rs           # Entry point, routing
├── components/       # Reusable UI components
├── services/         # External integrations (Docker)
├── utils/           # Shared utilities (AppState)
└── views/           # Page components
````

## Coding Standards

### Rust Style

- Run `cargo fmt` before committing
- Fix all `cargo clippy` warnings
- Use meaningful variable names
- Add doc comments to public APIs

````rust
/// Starts a Docker container by ID.
///
/// # Arguments
///
/// * `id` - The container ID or name
///
/// # Errors
///
/// Returns an error if Docker API call fails
pub async fn start_container(&self, id: &str) -> Result<()> {
    // Implementation
}
````

### Component Guidelines

1. **Use Dioxus 0.7 patterns**
   - `use_signal` for local state
   - `use_context` for global state
   - No `cx` or `Scope` (deprecated in 0.7)

2. **Component structure**
   ````rust
   #[component]
   pub fn MyComponent(prop1: String, prop2: i32) -> Element {
       let mut local_state = use_signal(|| 0);
       
       rsx! {
           div { "Content" }
       }
   }
   ````

3. **Async operations**
   ````rust
   spawn(async move {
       match service.operation().await {
           Ok(data) => signal.set(data),
           Err(e) => error_signal.set(Some(format!("{}", e))),
       }
   });
   ````

### Testing

Write tests for:
- Data transformation functions
- State management logic
- Docker service methods (with mocks)

````rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(1024), "1.0KB");
    }
}
````

## Adding New Features

### Adding a New View

1. Create view file in `src/views/my_view.rs`
2. Add to `src/views/mod.rs`: `pub use my_view::*;`
3. Add route to `Route` enum in `main.rs`
4. Implement component following existing patterns

### Adding Docker Operations

1. Add method to `DockerService` in `src/services/docker.rs`
2. Use Bollard API (see [docs](https://docs.rs/bollard/))
3. Add corresponding state management to `AppState`
4. Update views to use new functionality

### Adding UI Components

1. Create in `src/components/my_component.rs`
2. Export from `src/components/mod.rs`
3. Document props and usage
4. Use consistent styling with existing components

## Documentation

Update documentation when:
- Adding new features
- Changing APIs
- Fixing bugs that affect usage

Documentation locations:
- `README.md` - Overview and quick start
- `docs/` - Comprehensive documentation
- Code comments - Implementation details

## Pull Request Process

1. **Title**: Use conventional commit format
   - ✅ `feat: add volume inspection view`
   - ✅ `fix: container refresh race condition`
   - ❌ `updated stuff`

2. **Description**: Include
   - What changed and why
   - Screenshots for UI changes
   - Breaking changes (if any)
   - Related issues

3. **Checklist**
   - [ ] Tests pass (`cargo test`)
   - [ ] No clippy warnings (`cargo clippy`)
   - [ ] Code formatted (`cargo fmt`)
   - [ ] Documentation updated
   - [ ] Application runs (`dx serve`)

4. **Review**: Respond to feedback constructively

## Issue Guidelines

### Reporting Bugs

Include:
- Operating system
- Docker version (`docker --version`)
- Rust version (`rustc --version`)
- Steps to reproduce
- Expected vs actual behavior
- Error messages/logs

### Feature Requests

Describe:
- Use case (what problem does it solve?)
- Proposed solution
- Alternatives considered
- Willingness to implement

## Development Tips

### Hot Reload

Use `dx serve --platform desktop` for development with hot reload.

### Debugging

1. **Add logging**
   ````rust
   println!("Debug: {:?}", value);
   eprintln!("Error: {}", error);
   ````

2. **Check Docker connectivity**
   ````bash
   docker ps  # Verify Docker is accessible
   ````

3. **Inspect state**
   Display state values in UI temporarily for debugging.

### Common Issues

**"Failed to connect to Docker"**
- Ensure Docker is running
- Check `DOCKER_HOST` environment variable
- Verify socket permissions

**Build errors**
- Clear build cache: `cargo clean`
- Update dependencies: `cargo update`
- Check Rust version: `rustup update`

## Resources

- [Dioxus 0.7 Documentation](https://dioxuslabs.com/learn/0.7)
- [Bollard (Docker API)](https://docs.rs/bollard/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Project Issues](https://github.com/MH0386/doctainr/issues)

## Questions?

- Open a [Discussion](https://github.com/MH0386/doctainr/discussions)
- Check [existing issues](https://github.com/MH0386/doctainr/issues)
- Review [architecture docs](../reference/architecture.md)

---

Thank you for contributing! 🎉
