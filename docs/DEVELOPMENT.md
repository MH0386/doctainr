# Development Guide

This guide covers the development workflow, tools, and best practices for working on Doctainr.

## Table of Contents

- [Setting Up Your Environment](#setting-up-your-environment)
- [Development Workflow](#development-workflow)
- [Building the Project](#building-the-project)
- [Running the Application](#running-the-application)
- [Testing](#testing)
- [Code Quality](#code-quality)
- [Debugging](#debugging)
- [Hot Reload](#hot-reload)
- [Common Tasks](#common-tasks)

## Setting Up Your Environment

### Prerequisites

1. **Rust Toolchain** (1.70+)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **Docker**
   - Linux: Install via package manager
   - macOS: Install Docker Desktop
   - Windows: Install Docker Desktop

3. **Dioxus CLI**
   ```bash
   cargo install dioxus-cli
   ```

4. **(Optional) Nix with devenv**
   ```bash
   # Install Nix
   curl -L https://nixos.org/nix/install | sh
   
   # Install devenv
   nix profile install nixpkgs#devenv
   ```

### Platform-Specific Setup

#### Linux

Install GTK and WebKit dependencies:

```bash
# Debian/Ubuntu
sudo apt-get install libgtk-3-dev libwebkit2gtk-4.1-dev

# Fedora
sudo dnf install gtk3-devel webkit2gtk4.1-devel

# Arch
sudo pacman -S gtk3 webkit2gtk
```

#### macOS

No additional dependencies required with Dioxus desktop.

#### Windows

No additional dependencies required with Dioxus desktop.

### Clone and Setup

```bash
# Clone the repository
git clone https://github.com/MH0386/doctainr.git
cd doctainr

# Option 1: Using devenv (recommended)
devenv shell

# Option 2: Manual setup
cargo build
```

### Verify Installation

```bash
# Check Rust
rustc --version
cargo --version

# Check Dioxus CLI
dx --version

# Check Docker
docker info

# Run tests
cargo test
```

## Development Workflow

### Using devenv (Recommended)

devenv provides a reproducible development environment with all tools configured:

```bash
# Enter the development shell
devenv shell

# Available tasks
devenv tasks run dx:build      # Build the project
devenv tasks run dx:test       # Run Dioxus checks
devenv tasks run dx:run        # Run the application
devenv tasks run dx:serve      # Serve with hot reload
devenv tasks run cargo:test    # Run Rust tests
```

### Git Hooks

devenv automatically sets up git hooks for:
- Code formatting (rustfmt)
- Linting (clippy)
- TOML validation
- JSON validation
- YAML validation

Hooks run automatically on commit.

### Manual Workflow

Without devenv:

```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Run tests
cargo test

# Build
cargo build

# Run
cargo run
# or
dx run
```

## Building the Project

### Development Build

```bash
# Using Dioxus CLI (faster)
dx build

# Using Cargo
cargo build
```

### Release Build

```bash
# Optimized release build
dx build --release

# Or with Cargo
cargo build --release
```

The release binary will be at:
- Cargo: `target/release/doctainr`
- Dioxus: `target/dx/doctainr/release/bundle/`

### Build Features

The project supports multiple build targets:

```bash
# Desktop (default)
cargo build --features desktop

# Web (experimental)
cargo build --features web
```

## Running the Application

### Development Mode

```bash
# Using Dioxus CLI (with hot reload)
dx serve --platform desktop

# Using Cargo
cargo run
```

### Release Mode

```bash
# Run optimized binary
cargo run --release
```

### Docker Requirements

Ensure Docker daemon is running:

```bash
# Check Docker status
docker info

# Start Docker (if needed)
# Linux
sudo systemctl start docker

# macOS/Windows
# Start Docker Desktop application
```

### Environment Variables

Configure Docker connection:

```bash
# Default: Unix socket
export DOCKER_HOST=unix:///var/run/docker.sock

# TCP connection
export DOCKER_HOST=tcp://localhost:2375

# Run with custom DOCKER_HOST
DOCKER_HOST=tcp://localhost:2375 dx run
```

## Testing

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture

# Integration tests only
cargo test --test '*'

# Documentation tests
cargo test --doc
```

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        // Synchronous test
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn async_test() {
        // Asynchronous test
        let result = async_function().await;
        assert!(result.is_ok());
    }
}
```

### Writing Tests

1. **Unit Tests**: Test individual functions
   ```rust
   #[test]
   fn test_format_size() {
       assert_eq!(format_size(1024), "1.0KB");
   }
   ```

2. **Integration Tests**: Test module interactions
   ```rust
   #[tokio::test]
   async fn test_docker_integration() {
       let service = DockerService::new().unwrap();
       let containers = service.list_containers().await;
       assert!(containers.is_ok());
   }
   ```

3. **Component Tests**: Test UI components
   ```rust
   #[test]
   fn test_metric_card_renders() {
       // Component testing with Dioxus testing utilities
   }
   ```

## Code Quality

### Formatting

```bash
# Format all code
cargo fmt

# Check formatting without modifying
cargo fmt -- --check
```

### Linting

```bash
# Run Clippy
cargo clippy

# Strict mode
cargo clippy -- -D warnings

# Fix automatic issues
cargo clippy --fix
```

### Dioxus Checks

```bash
# Check Dioxus-specific issues
dx check

# Format RSX
dx fmt
```

### Pre-commit Checks

Run before committing:

```bash
# Format, lint, and test
cargo fmt && cargo clippy && cargo test

# Or with devenv
devenv test
```

## Debugging

### Logging

Add debug logging:

```rust
// In functions
println!("Debug: {:?}", value);
eprintln!("Error: {:?}", error);

// In async blocks
spawn(async move {
    println!("Async operation started");
    let result = operation().await;
    println!("Result: {:?}", result);
});
```

### Rust Debugger

Using rust-gdb or rust-lldb:

```bash
# Build with debug info
cargo build

# Debug
rust-gdb target/debug/doctainr
# or
rust-lldb target/debug/doctainr
```

### VS Code Debugging

Create `.vscode/launch.json`:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug Doctainr",
      "cargo": {
        "args": ["build", "--bin=doctainr"]
      },
      "args": [],
      "cwd": "${workspaceFolder}"
    }
  ]
}
```

### Docker Issues

Debug Docker connection:

```rust
// In DockerService::new()
match Docker::connect_with_local_defaults() {
    Ok(docker) => {
        println!("Connected to Docker");
        Ok(Self { docker })
    }
    Err(e) => {
        eprintln!("Failed to connect to Docker: {}", e);
        Err(e.into())
    }
}
```

## Hot Reload

### Desktop Hot Reload

```bash
# Start with hot reload
dx serve --platform desktop
```

Changes to Rust code or assets will trigger rebuilds.

### Asset Hot Reload

Assets in the `assets/` directory are reloaded automatically:
- CSS changes reflect immediately
- Images update on reload

### Limitations

Hot reload doesn't support:
- Macro changes (requires full rebuild)
- Dependency updates (requires rebuild)
- Some state changes (may need app restart)

## Common Tasks

### Adding a New View

1. Create view file in `src/views/`:
   ```rust
   // src/views/my_view.rs
   use dioxus::prelude::*;
   
   #[component]
   pub fn MyView() -> Element {
       rsx! {
           div { "My View" }
       }
   }
   ```

2. Export in `src/views/mod.rs`:
   ```rust
   mod my_view;
   pub use my_view::MyView;
   ```

3. Add route in `src/main.rs`:
   ```rust
   #[derive(Routable, Clone, PartialEq)]
   enum Route {
       #[layout(AppShell)]
           // ... existing routes
           #[route("/myview")]
           MyView {},
   }
   ```

4. Add navigation link in `src/views/shell.rs`

### Adding a Component

1. Create component file in `src/components/`:
   ```rust
   // src/components/my_component.rs
   use dioxus::prelude::*;
   
   #[component]
   pub fn MyComponent(title: String) -> Element {
       rsx! {
           div { class: "my-component",
               "{title}"
           }
       }
   }
   ```

2. Export in `src/components/mod.rs`:
   ```rust
   mod my_component;
   pub use my_component::MyComponent;
   ```

3. Use in views:
   ```rust
   use crate::components::MyComponent;
   
   rsx! {
       MyComponent { title: "Hello".to_string() }
   }
   ```

### Adding a Service Method

1. Add method to `DockerService`:
   ```rust
   // src/services/docker.rs
   impl DockerService {
       pub async fn my_operation(&self) -> Result<()> {
           // Implementation
           Ok(())
       }
   }
   ```

2. Add state method in `AppState`:
   ```rust
   // src/utils/app_state.rs
   impl AppState {
       pub fn trigger_operation(&self) {
           if let Some(service) = &self.docker_service {
               let service = service.clone();
               spawn(async move {
                   match service.my_operation().await {
                       Ok(_) => println!("Success"),
                       Err(e) => eprintln!("Error: {}", e),
                   }
               });
           }
       }
   }
   ```

### Updating Dependencies

```bash
# Check for updates
cargo outdated

# Update dependencies
cargo update

# Update specific dependency
cargo update -p dioxus

# Update Cargo.lock
cargo generate-lockfile
```

### Running CI Locally

Simulate CI checks:

```bash
# All checks
devenv test

# Or manually
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test
dx check
```

### Profiling Performance

```bash
# Build with profiling
cargo build --release --features profiling

# Use cargo-flamegraph
cargo install flamegraph
cargo flamegraph

# Use perf (Linux)
perf record -g target/release/doctainr
perf report
```

## Troubleshooting

### Build Errors

```bash
# Clean and rebuild
cargo clean
cargo build

# Update toolchain
rustup update
```

### Docker Connection Issues

```bash
# Check Docker socket permissions
ls -l /var/run/docker.sock

# Add user to docker group (Linux)
sudo usermod -aG docker $USER
newgrp docker
```

### GTK/WebKit Issues (Linux)

```bash
# Reinstall dependencies
sudo apt-get install --reinstall libgtk-3-dev libwebkit2gtk-4.1-dev
```

## Best Practices

1. **Run tests frequently**: `cargo test`
2. **Format before commit**: `cargo fmt`
3. **Check with clippy**: `cargo clippy`
4. **Write documentation**: Add doc comments
5. **Keep commits small**: Focused, atomic changes
6. **Update tests**: Test new features
7. **Handle errors**: Don't unwrap in production code
8. **Use types**: Leverage Rust's type system

## Resources

- [Dioxus Documentation](https://dioxuslabs.com)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Bollard Documentation](https://docs.rs/bollard)
- [Docker API Reference](https://docs.docker.com/engine/api/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
