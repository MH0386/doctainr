# Building from Source

This guide explains how to build Doctainr from source code for development or contribution.

## Prerequisites

### Required Tools

1. **Rust toolchain** (1.70 or later)
   ````bash
   # Install Rust via rustup
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Verify installation
   rustc --version
   cargo --version
   ````

2. **Docker daemon**
   - Must be installed and running
   - Your user must have permission to access the Docker socket

3. **Git**
   ````bash
   git --version
   ````

### Recommended Tools

1. **Dioxus CLI** - Provides hot reload and development server
   ````bash
   curl -sSL http://dioxus.dev/install.sh | sh
   ````

2. **rust-analyzer** - For IDE integration (VS Code, etc.)

## Clone the Repository

````bash
git clone https://github.com/MH0386/doctainr.git
cd doctainr
````

## Development Build

### Using Dioxus CLI (Recommended)

The Dioxus CLI provides hot reload, which automatically rebuilds when you change files:

````bash
# Run in development mode with hot reload
dx serve --platform desktop

# Or use the shorter alias
dx serve
````

This will:
1. Compile the application
2. Open the desktop window
3. Watch for file changes
4. Automatically rebuild and reload on changes

### Using Cargo

If you prefer standard Cargo:

````bash
# Build and run in debug mode
cargo run

# Or build without running
cargo build
````

**Note**: Debug builds are significantly slower than release builds but include debug symbols.

## Production Build

### Create Optimized Release Build

````bash
# Using Dioxus CLI
dx build --release --platform desktop

# Or using Cargo
cargo build --release
````

The compiled binary will be located at:
- **Linux/macOS**: `target/release/doctainr`
- **Windows**: `target\release\doctainr.exe`

### Run Release Build

````bash
./target/release/doctainr
````

## Project Structure

Understanding the codebase layout:

````
doctainr/
├── src/
│   ├── main.rs              # Entry point, routing
│   ├── components/          # Reusable UI components
│   │   ├── mod.rs
│   │   ├── metric_card.rs
│   │   ├── section_header.rs
│   │   └── status_pill.rs
│   ├── services/            # Docker API integration
│   │   ├── mod.rs
│   │   └── docker.rs
│   ├── utils/               # App state, helpers
│   │   ├── mod.rs
│   │   └── app_state.rs
│   └── views/               # Page components
│       ├── mod.rs
│       ├── dashboard.rs
│       ├── containers.rs
│       ├── images.rs
│       ├── volumes.rs
│       ├── settings.rs
│       └── shell.rs
├── assets/                  # Static resources
│   ├── styling/
│   │   └── main.css
│   ├── favicon.ico
│   └── icon.svg
├── Cargo.toml               # Rust dependencies
├── Dioxus.toml             # Dioxus configuration
└── README.md
````

## Code Quality Checks

### Format Code

````bash
cargo fmt
````

### Run Linter

````bash
cargo clippy
````

Configuration is in `clippy.toml`.

### Run Tests

````bash
cargo test
````

Currently includes unit tests for Docker service utilities.

## Troubleshooting Build Issues

### "cannot find crate for bollard"

**Problem**: Missing dependencies

**Solution**:
````bash
cargo clean
cargo build
````

### "failed to connect to Docker"

**Problem**: Docker daemon not running or inaccessible

**Solution**:
1. Start Docker: `systemctl start docker` (Linux) or start Docker Desktop
2. Verify: `docker info`
3. Check socket permissions: `ls -l /var/run/docker.sock`

### "linking with cc failed"

**Problem**: Missing system libraries

**Solution** (Debian/Ubuntu):
````bash
sudo apt-get install build-essential pkg-config libssl-dev
````

**Solution** (Fedora/RHEL):
````bash
sudo dnf install gcc openssl-devel
````

### Dioxus CLI not found

**Problem**: `dx` command not in PATH

**Solution**:
````bash
# Reinstall Dioxus CLI
cargo install dioxus-cli

# Or add to PATH manually
export PATH="$HOME/.cargo/bin:$PATH"
````

## IDE Setup

### Visual Studio Code

1. Install **rust-analyzer** extension
2. Install **Dioxus** extension (for RSX syntax highlighting)

Recommended `settings.json`:
````json
{
    "rust-analyzer.cargo.features": ["desktop"],
    "rust-analyzer.checkOnSave.command": "clippy"
}
````

### IntelliJ IDEA / RustRover

1. Install **Rust plugin**
2. Configure Cargo features: `desktop`

## Build Configuration

### Cargo.toml Features

````toml
[features]
default = ["desktop"]
web = ["dioxus/web"]
desktop = ["dioxus/desktop"]
````

**To build for web** (experimental):
````bash
dx build --platform web
````

### Dioxus.toml

Configuration for Dioxus CLI:

````toml
[application]
name = "doctainr"

[web.app]
title = "Doctainr"

[bundle]
identifier = "com.doctainr.app"
````

## Contributing

Before submitting changes:

1. **Format**: `cargo fmt`
2. **Lint**: `cargo clippy`
3. **Test**: `cargo test`
4. **Build**: `cargo build --release`

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for more details.

## Platform-Specific Notes

### Linux

- Requires GTK3 development libraries
- Docker socket typically at `/var/run/docker.sock`

### macOS

- Uses system WebKit for rendering
- Docker Desktop creates socket at `/var/run/docker.sock`

### Windows

- Requires Microsoft Edge WebView2 runtime
- Docker Desktop uses named pipe: `//./pipe/docker_engine`

## Related Documentation

- [Architecture Overview](../reference/architecture.md) - Understanding the codebase
- [Getting Started](../tutorials/getting-started.md) - Running the application
- [Configuration Reference](../reference/configuration.md) - Build options
