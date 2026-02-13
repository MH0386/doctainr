# Docker Integration

How Doctainr integrates with Docker using the Bollard library.

## Overview

Doctainr uses [Bollard](https://github.com/fussybeaver/bollard) as its Docker client library. Bollard is a pure Rust implementation of the Docker API with async/await support.

## Connection Strategy

### Socket Detection

Docker daemon listens on platform-specific endpoints:

| Platform | Default Path |
|----------|-------------|
| Linux | `unix:///var/run/docker.sock` |
| macOS | `unix:///var/run/docker.sock` |
| Windows | `npipe:////./pipe/docker_engine` |

### Connection Initialization

````rust
pub fn new() -> Result<Self> {
    let docker = Docker::connect_with_local_defaults()?;
    Ok(Self { docker })
}
````

**Process**:
1. Check `DOCKER_HOST` environment variable
2. Fall back to platform default socket/pipe
3. Attempt connection
4. Return error if daemon unreachable

### Error Handling

Connection errors indicate:
- Docker daemon not running
- Socket/pipe permissions incorrect
- Docker not installed

**User-Facing Message**:
````
Failed to connect to Docker daemon at unix:///var/run/docker.sock
Ensure Docker is running and accessible.
````

---

## API Operations

### List Containers

Retrieves all containers (running and stopped).

#### Bollard API Call

````rust
let options = Some(ListContainersOptions::<String> {
    all: true,  // Include stopped containers
    ..Default::default()
});

let containers = self.docker.list_containers(options).await?;
````

#### Data Transformation

Raw Docker API response:
````json
{
  "Id": "abc123...",
  "Names": ["/my-container"],
  "Image": "nginx:latest",
  "State": "running",
  "Ports": [{"PrivatePort": 80, "PublicPort": 8080}],
  "Status": "Up 2 hours"
}
````

Transformed to `ContainerInfo`:
````rust
ContainerInfo {
    id: "abc123def456",      // Truncated to 12 chars
    name: "my-container",     // Leading / removed
    image: "nginx:latest",
    status: "Up 2 hours",
    ports: "8080:80",         // Formatted string
    state: ContainerState::Running,
}
````

#### Transformation Logic

**ID Truncation**:
````rust
let id = container.id
    .map(|s| s.chars().take(12).collect())
    .unwrap_or_else(|| "unknown".to_string());
````

**Name Parsing**:
````rust
let name = container.names
    .and_then(|names| names.first())
    .map(|n| n.trim_start_matches('/').to_string())
    .unwrap_or_else(|| "unnamed".to_string());
````

**Port Formatting**:
````rust
let ports = ports.iter()
    .filter_map(|p| match (p.public_port, p.private_port) {
        (Some(pub_port), priv_port) => Some(format!("{}:{}", pub_port, priv_port)),
        (None, priv_port) => Some(format!("{}", priv_port)),
    })
    .collect::<Vec<_>>()
    .join(", ");
````

---

### Start/Stop Containers

#### Start Container

````rust
pub async fn start_container(&self, id: &str) -> Result<()> {
    self.docker
        .start_container(id, None::<StartContainerOptions<String>>)
        .await?;
    Ok(())
}
````

**Bollard Behavior**:
- Returns immediately (async)
- Container state changes asynchronously
- Requires subsequent list operation to verify state

#### Stop Container

````rust
pub async fn stop_container(&self, id: &str) -> Result<()> {
    self.docker
        .stop_container(id, None::<StopContainerOptions>)
        .await?;
    Ok(())
}
````

**Graceful Shutdown**:
- Sends SIGTERM to container
- Waits for timeout (default 10s)
- Sends SIGKILL if still running

---

### List Images

Retrieves local Docker images.

#### Bollard API Call

````rust
let options = Some(ListImagesOptions::<String> {
    all: false,  // Exclude intermediate layers
    ..Default::default()
});

let images = self.docker.list_images(options).await?;
````

#### Tag Parsing

````rust
let (repository, tag) = if let Some(first) = image.repo_tags.first() {
    let parts: Vec<&str> = first.split(':').collect();
    let repo = parts.first().unwrap_or(&"<none>").to_string();
    let tag = parts.get(1).unwrap_or(&"<none>").to_string();
    (repo, tag)
} else {
    ("<none>".to_string(), "<none>".to_string())
};
````

**Examples**:
- `nginx:latest` → repository: `"nginx"`, tag: `"latest"`
- `ubuntu:22.04` → repository: `"ubuntu"`, tag: `"22.04"`
- Untagged → repository: `"<none>"`, tag: `"<none>"`

#### Size Formatting

````rust
fn format_size(size: i64) -> String {
    const KB: i64 = 1024;
    const MB: i64 = KB * 1024;
    const GB: i64 = MB * 1024;

    if size >= GB {
        format!("{:.1}GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.1}MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.1}KB", size as f64 / KB as f64)
    } else {
        format!("{}B", size)
    }
}
````

---

### List Volumes

Retrieves Docker volumes.

#### Bollard API Call

````rust
let options = ListVolumesOptions::<String> {
    ..Default::default()
};

let volumes_response = self.docker.list_volumes(Some(options)).await?;
````

#### Volume Information

Docker API provides:
- **Name** - Volume identifier (hash or custom name)
- **Driver** - Storage driver (usually "local")
- **Mountpoint** - Filesystem path

**Size Limitation**: Volume size requires additional inspection call per volume (expensive), so it's not included in basic listing.

---

## Performance Considerations

### Request Optimization

**Batching**: All list operations fetch complete datasets (no pagination).

**Caching**: No client-side caching; always fetches fresh data.

**Concurrency**: Multiple operations can run concurrently:
````rust
spawn(async move { service.list_containers().await });
spawn(async move { service.list_images().await });
spawn(async move { service.list_volumes().await });
````

### Network Overhead

| Operation | Typical Latency | Payload Size |
|-----------|----------------|--------------|
| List Containers | 10-50ms | ~1KB per container |
| List Images | 10-50ms | ~500B per image |
| List Volumes | 10-30ms | ~300B per volume |
| Start/Stop | 50-200ms | Minimal |

**Note**: Latencies are for local socket communication (negligible network overhead).

---

## Error Handling

### Connection Errors

````rust
Error: ConnectionFailed { addr: "unix:///var/run/docker.sock" }
````

**Causes**:
- Docker daemon not running
- Socket permissions incorrect
- Docker not installed

**User Action**: Start Docker daemon, check permissions.

### API Errors

````rust
Error: DockerResponseServerError {
    status_code: 404,
    message: "No such container: abc123"
}
````

**Causes**:
- Invalid container/image ID
- Resource was deleted
- Permission denied

**User Action**: Refresh data, verify resource exists.

### Timeout Errors

````rust
Error: Timeout
````

**Causes**:
- Docker daemon overloaded
- Container taking too long to stop

**User Action**: Retry operation, check Docker daemon health.

---

## Type Conversions

### From Bollard to App Types

| Bollard Type | App Type | Conversion |
|-------------|----------|------------|
| `ContainerSummary` | `ContainerInfo` | Extract fields, format ports |
| `ImageSummary` | `ImageInfo` | Parse tags, format size |
| `Volume` | `VolumeInfo` | Direct mapping |

### State Mapping

````rust
let state = match container.state {
    Some(s) if s == "running" => ContainerState::Running,
    _ => ContainerState::Stopped,
};
````

**Docker States**: `created`, `running`, `restarting`, `exited`, `paused`, `dead`

**Simplified App States**: `Running`, `Stopped`

**Rationale**: UI only needs binary state for start/stop actions.

---

## Async Runtime

### Tokio Integration

Bollard requires Tokio runtime:

````toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
bollard = "0.18"
````

### Spawn Pattern

````rust
spawn(async move {
    match service.list_containers().await {
        Ok(data) => /* update UI */,
        Err(e) => /* handle error */,
    }
});
````

**Thread Model**:
- Main thread: UI rendering (Dioxus)
- Tokio thread pool: Async I/O (Bollard)
- No blocking calls on main thread

---

## Security Considerations

### Socket Permissions

Docker socket typically requires group membership:

````bash
# Linux
sudo usermod -aG docker $USER

# Check access
ls -l /var/run/docker.sock
srw-rw---- 1 root docker 0 Feb 13 12:00 /var/run/docker.sock
````

**Risk**: Socket access is root-equivalent. Application does not require elevated privileges itself, but Docker daemon access is powerful.

### Input Sanitization

Container IDs and names are sourced from Docker API (trusted):
- No user-provided IDs executed directly
- All IDs come from list operations
- No shell command injection risk

---

## Testing

### Unit Tests

````rust
#[test]
fn test_format_size() {
    assert_eq!(format_size(100), "100B");
    assert_eq!(format_size(1024), "1.0KB");
    assert_eq!(format_size(1048576), "1.0MB");
}
````

### Integration Tests

Require running Docker daemon:

````rust
#[tokio::test]
async fn test_list_containers() {
    let service = DockerService::new().unwrap();
    let containers = service.list_containers().await;
    assert!(containers.is_ok());
}
````

**CI Considerations**: Integration tests disabled by default (require Docker).

---

## Alternative Approaches

### Docker CLI vs Bollard

| Approach | Pros | Cons |
|----------|------|------|
| Bollard | Type-safe, async, pure Rust | Verbose API |
| Docker CLI | Simple, well-documented | Parsing overhead, subprocess management |

**Choice**: Bollard for type safety and async support.

### Docker SDK for Python/Node vs Rust

| Language | Ecosystem | Performance |
|----------|-----------|-------------|
| Python | Mature, large community | Slower |
| Node.js | Mature, large community | Slower |
| Rust | Growing, native performance | Fastest |

**Choice**: Rust for native desktop performance.

---

## Future Enhancements

### Planned Features

- **Docker Compose Support** - Manage multi-container applications
- **Image Building** - Build images from Dockerfile
- **Container Logs** - Stream container logs
- **Resource Monitoring** - CPU/memory usage
- **Network Management** - List and manage Docker networks

### API Extensions

````rust
// Future methods
pub async fn build_image(&self, context: &Path) -> Result<String>;
pub async fn stream_logs(&self, id: &str) -> Result<LogStream>;
pub async fn inspect_container(&self, id: &str) -> Result<ContainerDetail>;
````

---

## See Also

- [Docker Services API](../api/services.md) - Detailed API reference
- [Bollard Documentation](https://docs.rs/bollard/) - Underlying library
- [Docker Engine API](https://docs.docker.com/engine/api/) - REST API reference
- [System Architecture](overview.md) - Integration context
