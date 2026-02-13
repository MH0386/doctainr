# Troubleshooting Guide

Common issues and solutions when working with Doctainr.

## Installation Issues

### Cannot Install Dioxus CLI

**Problem**: `curl -sSL http://dioxus.dev/install.sh | sh` fails

**Solutions**:
1. Check internet connectivity
2. Try manual installation via cargo:
   ````bash
   cargo install dioxus-cli
   ````
3. Verify cargo is in PATH: `cargo --version`

### Rust Toolchain Not Found

**Problem**: `rustc: command not found`

**Solutions**:
1. Install Rust from [rustup.rs](https://rustup.rs/)
2. Restart terminal after installation
3. Verify installation: `rustc --version`

### Build Fails with Dependency Errors

**Problem**: Cargo cannot resolve dependencies

**Solutions**:
1. Update Rust: `rustup update`
2. Clear build cache: `cargo clean`
3. Update dependencies: `cargo update`
4. Check internet connection (required for initial build)

## Docker Connection Issues

### "Failed to connect to Docker"

**Problem**: Application cannot reach Docker Engine

**Diagnosis**:
````bash
docker info
````

**Solutions**:

**On Linux**:
1. Start Docker daemon:
   ````bash
   sudo systemctl start docker
   ````
2. Add user to docker group:
   ````bash
   sudo usermod -aG docker $USER
   # Log out and back in
   ````

**On macOS/Windows**:
1. Launch Docker Desktop
2. Wait for "Docker is running" status
3. Verify in system tray/menu bar

**On remote Docker**:
1. Set environment variable:
   ````bash
   export DOCKER_HOST=tcp://remote-host:2375
   ````
2. Restart Doctainr

### Permission Denied on Docker Socket

**Problem**: Cannot access `/var/run/docker.sock`

**Solutions**:

**Temporary** (single session):
````bash
sudo chmod 666 /var/run/docker.sock
````

**Permanent** (recommended):
````bash
sudo usermod -aG docker $USER
newgrp docker  # Or log out and back in
````

Verify:
````bash
docker ps  # Should work without sudo
````

### Docker Socket Not Found

**Problem**: `/var/run/docker.sock` doesn't exist

**Solutions**:
1. Verify Docker is installed: `docker --version`
2. Check if Docker daemon is running:
   ````bash
   sudo systemctl status docker  # Linux
   ````
3. Reinstall Docker if necessary

## Runtime Issues

### Application Won't Start

**Problem**: `dx serve` fails or crashes

**Diagnosis**:
````bash
dx serve --platform desktop --verbose
````

**Common Causes**:

1. **Port conflict**: Another process using the dev server port
   ````bash
   # Check what's using the port
   lsof -i :8080
   # Kill the process or use different port
   dx serve --port 8081
   ````

2. **Missing assets**: CSS or favicon not found
   ````bash
   # Verify files exist
   ls -la assets/
   ````

3. **Compilation errors**: Code syntax issues
   ````bash
   cargo build
   # Fix reported errors
   ````

### Application Crashes on Launch

**Problem**: Window opens then immediately closes

**Diagnosis**: Run via cargo to see error messages:
````bash
cargo run
````

**Solutions**:
1. Check Docker is running: `docker info`
2. Review error output for specific issues
3. Try rebuilding: `cargo clean && cargo build`

### No Containers/Images Shown

**Problem**: Views are empty despite Docker having resources

**Diagnosis**:
````bash
# Verify Docker has resources
docker ps -a
docker images
docker volume ls
````

**Solutions**:
1. Click "Refresh" button in the view
2. Check error message in UI (bottom of screen)
3. Verify Docker socket connectivity
4. Check application logs for errors

### "Docker service not available"

**Problem**: Error message displayed in UI

**Solutions**:
1. Ensure Docker is running
2. Verify socket permissions
3. Restart the application
4. Check `DOCKER_HOST` environment variable

## UI/Display Issues

### Window Too Small/Large

**Problem**: UI doesn't fit screen properly

**Solutions**:
1. Resize window manually
2. Check monitor resolution
3. Restart application

### Styles Not Loading

**Problem**: UI appears unstyled or broken

**Solutions**:
1. Verify `assets/styling/main.css` exists
2. Check file permissions
3. Rebuild: `cargo clean && dx serve`
4. Clear browser cache if using web target

### Hot Reload Not Working

**Problem**: Changes don't appear without full restart

**Solutions**:
1. Ensure using `dx serve` (not `cargo run`)
2. Check file watcher isn't hitting limits:
   ````bash
   # Linux only
   echo fs.inotify.max_user_watches=524288 | sudo tee -a /etc/sysctl.conf
   sudo sysctl -p
   ````
3. Save files in editor before expecting reload

## Performance Issues

### Slow to Load Container List

**Problem**: Long delay when viewing containers

**Possible Causes**:
- Many containers (100+)
- Docker daemon slow to respond
- Network latency (remote Docker)

**Solutions**:
1. Click "Refresh" less frequently
2. Consider filtering in future versions
3. Check Docker daemon performance: `docker system df`

### High CPU Usage

**Problem**: Application using excessive CPU

**Solutions**:
1. Close other resource-intensive apps
2. Check for runaway Docker containers
3. Restart Doctainr
4. File a bug report with details

### Memory Leaks

**Problem**: Memory usage grows over time

**Solutions**:
1. Restart application periodically
2. Note reproduction steps for bug report
3. Update to latest version

## Build Issues

### Clippy Warnings/Errors

**Problem**: `cargo clippy` reports issues

**Solutions**:
````bash
# Fix automatically where possible
cargo clippy --fix

# Review and fix remaining issues manually
cargo clippy
````

### Format Check Fails

**Problem**: CI fails on formatting

**Solutions**:
````bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check
````

### Tests Failing

**Problem**: `cargo test` reports failures

**Solutions**:
1. Run tests with output:
   ````bash
   cargo test -- --nocapture
   ````
2. Run specific test:
   ````bash
   cargo test test_name
   ````
3. Fix failing tests before committing

## Development Environment

### VS Code Extensions

Recommended extensions for development:

- **rust-analyzer**: Rust language support
- **Better TOML**: TOML file editing
- **Error Lens**: Inline error display

Install via:
````bash
code --install-extension rust-lang.rust-analyzer
code --install-extension bungcip.better-toml
code --install-extension usernamehw.errorlens
````

### Environment Variables

Useful variables:

````bash
# Use remote Docker
export DOCKER_HOST=tcp://192.168.1.100:2375

# Rust backtrace for debugging
export RUST_BACKTRACE=1

# Verbose cargo output
export CARGO_TERM_VERBOSE=true
````

## Getting More Help

### Enable Debug Logging

Add to `main.rs` temporarily:
````rust
println!("Debug info: {:?}", value);
eprintln!("Error occurred: {}", error);
````

### Check System Requirements

Minimum requirements:
- Rust 1.70+
- Docker 20.10+
- 4GB RAM
- macOS 10.15+, Windows 10+, or Linux kernel 3.10+

### Search Existing Issues

Before reporting bugs:
1. Search [GitHub Issues](https://github.com/MH0386/doctainr/issues)
2. Check [Discussions](https://github.com/MH0386/doctainr/discussions)
3. Review this troubleshooting guide

### Report a Bug

Include:
- Operating system and version
- Docker version: `docker --version`
- Rust version: `rustc --version`
- Steps to reproduce
- Error messages/screenshots
- Expected vs actual behavior

[Open an Issue](https://github.com/MH0386/doctainr/issues/new)

---

**Still stuck?** [Open a Discussion](https://github.com/MH0386/doctainr/discussions) or [contact support](https://github.com/MH0386/doctainr/issues).
