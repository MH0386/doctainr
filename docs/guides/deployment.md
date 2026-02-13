# Deployment Guide

Instructions for building and deploying Doctainr in various environments.

## Build Modes

### Development Build

For local development with debug symbols:

````bash
# Using Dioxus CLI (recommended)
dx serve --platform desktop

# Or with cargo
cargo run
````

**Features**:
- Hot reload enabled
- Debug symbols included
- Faster compilation
- Larger binary size
- Development server for assets

### Release Build

Optimized production build:

````bash
# Using Dioxus CLI
dx build --platform desktop --release

# Or with cargo
cargo build --release
````

**Output**: Binary at `target/release/doctainr`

**Features**:
- Optimizations enabled
- Stripped debug symbols
- Smaller binary size
- Faster runtime performance

## Platform-Specific Builds

### Linux

**Build**:
````bash
cargo build --release --target x86_64-unknown-linux-gnu
````

**Dependencies** (Debian/Ubuntu):
````bash
sudo apt-get install -y \
    libwebkit2gtk-4.0-dev \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
````

**Distribution**:
````bash
# Create tarball
tar -czf doctainr-linux-x64.tar.gz \
    -C target/release doctainr \
    -C ../../assets .

# Or create .deb package (requires cargo-deb)
cargo install cargo-deb
cargo deb
````

### macOS

**Build**:
````bash
cargo build --release --target x86_64-apple-darwin      # Intel
cargo build --release --target aarch64-apple-darwin     # Apple Silicon
````

**Universal Binary** (both architectures):
````bash
# Build both targets
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Create universal binary
lipo -create \
    target/x86_64-apple-darwin/release/doctainr \
    target/aarch64-apple-darwin/release/doctainr \
    -output doctainr-universal

# Create .app bundle
cargo install cargo-bundle
cargo bundle --release
````

**Code Signing** (optional):
````bash
codesign --force --deep --sign "Developer ID" \
    target/release/bundle/osx/Doctainr.app
````

### Windows

**Build**:
````bash
# On Windows
cargo build --release --target x86_64-pc-windows-msvc

# Cross-compile from Linux
cargo build --release --target x86_64-pc-windows-gnu
````

**Dependencies**:
- Visual Studio Build Tools (MSVC) or MinGW

**Create Installer** (with cargo-wix):
````bash
cargo install cargo-wix
cargo wix --nocapture
````

Output: `target/wix/doctainr-*.msi`

## Docker Containerization

While Doctainr manages Docker, you can also package it in a container for distribution.

### Dockerfile

````dockerfile
FROM rust:1.70-slim AS builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev

# Copy source
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY assets ./assets

# Build release
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /app/target/release/doctainr /usr/local/bin/

# Note: Requires Docker socket mount at runtime
CMD ["doctainr"]
````

**Build and Run**:
````bash
docker build -t doctainr:latest .

docker run --rm \
    -v /var/run/docker.sock:/var/run/docker.sock \
    -e DISPLAY=$DISPLAY \
    -v /tmp/.X11-unix:/tmp/.X11-unix \
    doctainr:latest
````

⚠️ **Note**: Desktop apps in containers have limitations (no native window manager integration).

## Distribution Strategies

### Direct Binary

**Pros**:
- Simple distribution
- No dependencies (mostly)
- Small download size

**Cons**:
- Platform-specific builds required
- Manual updates

**Process**:
1. Build release binary
2. Bundle with assets
3. Create archive (`.tar.gz`, `.zip`)
4. Distribute via GitHub Releases

### Package Managers

#### Homebrew (macOS)

Create formula:
````ruby
class Doctainr < Formula
  desc "Docker Desktop UI built with Rust and Dioxus"
  homepage "https://github.com/MH0386/doctainr"
  url "https://github.com/MH0386/doctainr/archive/v0.1.0.tar.gz"
  sha256 "..."

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/doctainr", "--version"
  end
end
````

#### Cargo (Rust ecosystem)

````bash
# Publish to crates.io
cargo publish
````

Users install with:
````bash
cargo install doctainr
````

#### APT (Debian/Ubuntu)

Create `.deb` package:
````bash
cargo install cargo-deb
cargo deb
````

Distribute via PPA or direct download.

## CI/CD Pipeline

### GitHub Actions

`.github/workflows/release.yml`:

````yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    runs-on: ${{ matrix.os }}
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Build release
        run: cargo build --release
      
      - name: Create archive
        shell: bash
        run: |
          cd target/release
          tar -czf ../../doctainr-${{ matrix.os }}.tar.gz doctainr
      
      - name: Upload artifact
        uses: actions/upload-artifact@v3
        with:
          name: doctainr-${{ matrix.os }}
          path: doctainr-${{ matrix.os }}.tar.gz
  
  release:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/download-artifact@v3
      
      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            doctainr-*/doctainr-*.tar.gz
````

## Configuration

### Environment Variables

Configure at runtime:

````bash
# Docker host
export DOCKER_HOST=unix:///var/run/docker.sock

# Or TCP
export DOCKER_HOST=tcp://192.168.1.100:2375

# TLS
export DOCKER_TLS_VERIFY=1
export DOCKER_CERT_PATH=/path/to/certs
````

### Config File (Future)

Planned `~/.config/doctainr/config.toml`:

````toml
[docker]
host = "unix:///var/run/docker.sock"
timeout = 30

[ui]
theme = "dark"
refresh_interval = 5
````

## Performance Optimization

### Build Optimizations

`Cargo.toml`:

````toml
[profile.release]
opt-level = 3              # Maximum optimization
lto = true                 # Link-time optimization
codegen-units = 1          # Better optimization, slower build
strip = true               # Strip symbols
panic = "abort"            # Smaller binary
````

### Binary Size Reduction

````bash
# Strip symbols manually
strip target/release/doctainr

# Use cargo-bloat to analyze size
cargo install cargo-bloat
cargo bloat --release

# Use cargo-strip
cargo install cargo-strip
cargo strip
````

## Testing Deployment

### Pre-release Checklist

- [ ] Builds successfully on all target platforms
- [ ] Application launches without errors
- [ ] Docker connectivity works
- [ ] All features functional
- [ ] No debug/test code in release
- [ ] Version number updated
- [ ] CHANGELOG.md updated
- [ ] Documentation current

### Smoke Tests

````bash
# Binary exists
test -f target/release/doctainr

# Binary is executable
test -x target/release/doctainr

# Version check
./target/release/doctainr --version

# Help text
./target/release/doctainr --help
````

## Rollback Strategy

If release has issues:

1. **Tag previous stable version**:
   ````bash
   git tag -a v0.1.0-stable -m "Stable release"
   ````

2. **Revert GitHub release**: Mark as pre-release or delete

3. **Notify users**: Update README with known issues

4. **Fix and re-release**: Increment patch version

## Update Mechanism

### Manual Updates

Users download new release and replace binary.

### Automated Updates (Future)

Planned self-update mechanism:

````bash
# Check for updates
doctainr update check

# Apply update
doctainr update apply
````

## Monitoring

Post-deployment monitoring:

- GitHub release download stats
- Issue tracker for bug reports
- User feedback via Discussions
- Crash reports (if telemetry added)

---

**See Also**: [Contributing Guide](./contributing.md) | [Architecture](../reference/architecture.md)
