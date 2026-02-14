# Building for Production

This guide covers building Doctainr for production deployment.

## Quick Start

```bash
# Build optimized release binary
cargo build --release

# Binary location
./target/release/doctainr
```

## Build Types

### Development Build

Fast compilation, includes debug symbols, slower runtime.

```bash
cargo build
./target/debug/doctainr
```

**Use for**: Active development, debugging

### Release Build

Slower compilation, optimized for performance, smaller binary.

```bash
cargo build --release
./target/release/doctainr
```

**Use for**: Production deployment, distribution

## Build Configuration

### Cargo.toml Profile Settings

Release builds use these optimizations:

```toml
[profile.release]
opt-level = 3        # Maximum optimization
lto = true          # Link-time optimization
codegen-units = 1   # Better optimization, slower compile
strip = true        # Strip symbols for smaller binary
```

### Custom Profile

Create a custom profile for specific needs:

```toml
[profile.production]
inherits = "release"
opt-level = "z"     # Optimize for size
lto = "fat"         # Aggressive LTO
```

Build with custom profile:
```bash
cargo build --profile production
```

## Build Steps

### 1. Clean Previous Builds

```bash
cargo clean
```

### 2. Update Dependencies

```bash
cargo update
```

### 3. Run Tests

```bash
cargo test --release
```

### 4. Run Linter

```bash
cargo clippy --release -- -D warnings
```

### 5. Build Release

```bash
cargo build --release
```

### 6. Verify Binary

```bash
./target/release/doctainr --version
ls -lh ./target/release/doctainr
```

## Optimization Techniques

### Size Optimization

Reduce binary size:

```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true
strip = true
panic = "abort"     # Don't unwind on panic
```

Additional size reduction:

```bash
# Install UPX (optional)
upx --best --lzma ./target/release/doctainr
```

### Performance Optimization

Maximum performance:

```toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
```

Enable CPU-specific optimizations:

```bash
# Build for native CPU
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

## Cross-Compilation

### Install Cross-Compilation Tools

```bash
# Install cross
cargo install cross

# Or use rustup
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

### Build for Different Platforms

#### Linux (x86_64)

```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

#### Windows (x86_64)

```bash
# On Linux with MinGW
sudo apt-get install mingw-w64
cargo build --release --target x86_64-pc-windows-gnu

# Or use cross
cross build --release --target x86_64-pc-windows-gnu
```

#### macOS (Intel)

```bash
cargo build --release --target x86_64-apple-darwin
```

#### macOS (Apple Silicon)

```bash
cargo build --release --target aarch64-apple-darwin
```

### Universal macOS Binary

Create universal binary for both Intel and Apple Silicon:

```bash
# Build for both architectures
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Combine with lipo
lipo -create \
    target/x86_64-apple-darwin/release/doctainr \
    target/aarch64-apple-darwin/release/doctainr \
    -output doctainr-universal
```

## Using Dioxus CLI

The Dioxus CLI provides additional build features.

### Install Dioxus CLI

```bash
cargo install dioxus-cli
```

### Build with dx

```bash
# Standard release build
dx build --release --platform desktop

# Output location
ls -la dist/
```

### dx Build Features

```bash
# Build for specific platform
dx build --release --platform desktop

# Build with custom features
dx build --release --features "desktop"

# Verbose output
dx build --release --platform desktop --verbose
```

## Asset Bundling

Assets are bundled automatically by Dioxus.

### Asset Configuration

In `Dioxus.toml`:

```toml
[bundle]
identifier = "com.mh0386.doctainr"
publisher = "Doctainr"
icon = ["assets/icon.svg"]
```

### Asset Optimization

Optimize assets before building:

```bash
# Optimize SVG icons
npx svgo assets/icon.svg

# Optimize CSS (if using PostCSS)
npx postcss assets/styling/main.css -o assets/styling/main.min.css
```

## Build Scripts

### Automated Build Script

Create `scripts/build.sh`:

```bash
#!/bin/bash
set -e

echo "Building Doctainr for production..."

# Clean
echo "Cleaning previous builds..."
cargo clean

# Update dependencies
echo "Updating dependencies..."
cargo update

# Tests
echo "Running tests..."
cargo test --release

# Lint
echo "Running linter..."
cargo clippy --release -- -D warnings

# Build
echo "Building release binary..."
cargo build --release

# Verify
echo "Build complete!"
ls -lh target/release/doctainr
./target/release/doctainr --version || echo "Binary created successfully"
```

Make executable:
```bash
chmod +x scripts/build.sh
./scripts/build.sh
```

### Multi-Platform Build Script

Create `scripts/build-all.sh`:

```bash
#!/bin/bash
set -e

TARGETS=(
    "x86_64-unknown-linux-gnu"
    "x86_64-pc-windows-gnu"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
)

for target in "${TARGETS[@]}"; do
    echo "Building for $target..."
    cargo build --release --target "$target"
done

echo "All builds complete!"
ls -lh target/*/release/doctainr*
```

## Continuous Integration Builds

### GitHub Actions

Example workflow in `.github/workflows/release.yml`:

```yaml
name: Release Build

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: ${{ matrix.os }}
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          
      - name: Build
        run: cargo build --release
        
      - name: Upload artifact
        uses: actions/upload-artifact@v3
        with:
          name: doctainr-${{ matrix.os }}
          path: target/release/doctainr*
```

## Build Verification

### Verify Binary

```bash
# Check binary size
ls -lh target/release/doctainr

# Check dependencies
ldd target/release/doctainr  # Linux
otool -L target/release/doctainr  # macOS

# Test run
./target/release/doctainr
```

### Performance Benchmarks

```bash
# Measure startup time
time ./target/release/doctainr

# Memory usage
valgrind --tool=massif ./target/release/doctainr

# CPU profiling
perf record ./target/release/doctainr
perf report
```

## Troubleshooting

### Build Fails

```bash
# Clean and retry
cargo clean
cargo build --release

# Update Rust
rustup update

# Check dependencies
cargo check
```

### Binary Too Large

```bash
# Strip symbols
strip target/release/doctainr

# Use size optimization
cargo build --release --config profile.release.opt-level='z'

# Compress with UPX
upx --best target/release/doctainr
```

### Missing Dependencies

```bash
# Linux: Install build dependencies
sudo apt-get install build-essential libwebkit2gtk-4.1-dev

# macOS: Install Xcode Command Line Tools
xcode-select --install
```

## Best Practices

1. **Always test before building release**
   ```bash
   cargo test --release
   cargo clippy --release
   ```

2. **Use clean builds for releases**
   ```bash
   cargo clean
   cargo build --release
   ```

3. **Verify binary on target platform**
   Test on actual target OS before distribution

4. **Keep build artifacts**
   Save binaries for each release

5. **Document build process**
   Maintain build instructions in repository

## Next Steps

- [Platform-Specific Builds](platforms.md) - Platform-specific considerations
- [Distribution Guide](distribution.md) - Package and distribute binaries
- [Development Guide](../guides/development.md) - Development setup

## Related Documentation

- [Installation Guide](../guides/installation.md)
- [Deployment Overview](platforms.md)
- [CI/CD Workflows](../../.github/workflows/ci.yaml)
