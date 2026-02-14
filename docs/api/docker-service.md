# Docker Service API Reference

Complete API reference for the Docker service layer.

## Module: `services::docker`

**Location**: `src/services/docker.rs`

### Overview

The Docker service provides type-safe methods for interacting with the Docker API via Bollard.

## Types

### DockerService

Main service struct for Docker operations.

```rust
#[derive(Clone)]
pub struct DockerService {
    client: Docker,
}
```

**Methods**:

#### `new() -> Result<Self>`

Creates a new Docker service instance.

**Returns**: `Result<DockerService>` - Service instance or connection error

**Example**:
```rust
let service = DockerService::new()?;
```

**Errors**:
- Docker daemon not running
- Permission denied accessing Docker socket
- Invalid Docker host configuration

---

#### `list_containers() -> Result<Vec<ContainerInfo>>`

Lists all Docker containers (running and stopped).

**Returns**: `Result<Vec<ContainerInfo>>` - List of container information

**Example**:
```rust
let containers = service.list_containers().await?;
for container in containers {
    println!("{}: {}", container.name, container.state.label());
}
```

**Errors**:
- Docker daemon communication failure
- Permission issues

---

#### `list_images() -> Result<Vec<ImageInfo>>`

Lists all local Docker images.

**Returns**: `Result<Vec<ImageInfo>>` - List of image information

**Example**:
```rust
let images = service.list_images().await?;
for image in images {
    println!("{} ({})", image.repository, image.size);
}
```

---

#### `list_volumes() -> Result<Vec<VolumeInfo>>`

Lists all Docker volumes.

**Returns**: `Result<Vec<VolumeInfo>>` - List of volume information

**Example**:
```rust
let volumes = service.list_volumes().await?;
for volume in volumes {
    println!("{}: {}", volume.name, volume.mountpoint);
}
```

---

#### `start_container(id: &str) -> Result<()>`

Starts a stopped container.

**Parameters**:
- `id`: Container ID or name

**Returns**: `Result<()>` - Success or error

**Example**:
```rust
service.start_container("my_container").await?;
```

**Errors**:
- Container not found
- Container already running
- Permission denied
- Resource conflicts (port already in use)

---

#### `stop_container(id: &str) -> Result<()>`

Stops a running container.

**Parameters**:
- `id`: Container ID or name

**Returns**: `Result<()>` - Success or error

**Example**:
```rust
service.stop_container("my_container").await?;
```

**Errors**:
- Container not found
- Container already stopped
- Permission denied

---

### ContainerState

Enum representing container lifecycle states.

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContainerState {
    Running,
    Stopped,
}
```

**Methods**:

#### `label() -> &'static str`

Returns human-readable label.

**Returns**:
- `"Running"` for `ContainerState::Running`
- `"Stopped"` for `ContainerState::Stopped`

**Example**:
```rust
let state = ContainerState::Running;
assert_eq!(state.label(), "Running");
```

---

#### `css_class() -> &'static str`

Returns CSS class name for styling.

**Returns**:
- `"running"` for `ContainerState::Running`
- `"stopped"` for `ContainerState::Stopped`

**Example**:
```rust
let css = ContainerState::Running.css_class();
// Use in UI: <div class="status {css}">
```

---

#### `action_label() -> &'static str`

Returns label for action button.

**Returns**:
- `"Stop"` for `ContainerState::Running`
- `"Start"` for `ContainerState::Stopped`

**Example**:
```rust
let action = container.state.action_label();
// Display as button label
```

---

### ContainerInfo

Container metadata structure.

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub ports: String,
    pub state: ContainerState,
}
```

**Fields**:

- `id`: Container ID (short format, first 12 characters)
- `name`: Container name (without leading `/`)
- `image`: Image name used to create container
- `status`: Status text from Docker (e.g., "Up 2 hours")
- `ports`: Formatted port mappings (e.g., "8080→80")
- `state`: Current state (`Running` or `Stopped`)

**Example**:
```rust
let container = ContainerInfo {
    id: "abc123def456".to_string(),
    name: "my_app".to_string(),
    image: "nginx:latest".to_string(),
    status: "Up 2 hours".to_string(),
    ports: "8080→80".to_string(),
    state: ContainerState::Running,
};
```

---

### ImageInfo

Docker image metadata structure.

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageInfo {
    pub id: String,
    pub repository: String,
    pub tag: String,
    pub size: String,
}
```

**Fields**:

- `id`: Image ID (short format, first 12 characters)
- `repository`: Repository name (e.g., "nginx", "postgres")
- `tag`: Image tag (e.g., "latest", "15.2")
- `size`: Human-readable size (e.g., "125.5 MB")

**Example**:
```rust
let image = ImageInfo {
    id: "sha256:abc123".to_string(),
    repository: "nginx".to_string(),
    tag: "latest".to_string(),
    size: "125.5 MB".to_string(),
};
```

---

### VolumeInfo

Docker volume metadata structure.

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VolumeInfo {
    pub name: String,
    pub driver: String,
    pub mountpoint: String,
}
```

**Fields**:

- `name`: Volume name
- `driver`: Storage driver (usually "local")
- `mountpoint`: Filesystem path where volume data is stored

**Example**:
```rust
let volume = VolumeInfo {
    name: "my_data".to_string(),
    driver: "local".to_string(),
    mountpoint: "/var/lib/docker/volumes/my_data/_data".to_string(),
};
```

---

## Error Handling

All async methods return `anyhow::Result<T>`, allowing flexible error handling.

### Common Error Types

- **Connection Errors**: Docker daemon not accessible
- **Permission Errors**: Insufficient permissions to access Docker
- **Not Found Errors**: Container/image/volume doesn't exist
- **State Errors**: Invalid state transition (e.g., starting running container)
- **Resource Errors**: Port conflicts, volume in use, etc.

### Error Handling Examples

```rust
use anyhow::Context;

// Basic error handling
match service.list_containers().await {
    Ok(containers) => {
        // Process containers
    }
    Err(e) => {
        eprintln!("Failed to list containers: {}", e);
    }
}

// With context
service.start_container(id)
    .await
    .context("Failed to start container")?;

// Detailed error handling
match service.start_container(id).await {
    Ok(_) => println!("Container started"),
    Err(e) => {
        if e.to_string().contains("already running") {
            println!("Container is already running");
        } else if e.to_string().contains("not found") {
            println!("Container not found");
        } else {
            println!("Error: {}", e);
        }
    }
}
```

## Usage Patterns

### Basic Usage

```rust
use doctainr::services::DockerService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create service
    let service = DockerService::new()?;
    
    // List containers
    let containers = service.list_containers().await?;
    println!("Found {} containers", containers.len());
    
    // Start a container
    if let Some(stopped) = containers.iter()
        .find(|c| c.state == ContainerState::Stopped) 
    {
        service.start_container(&stopped.id).await?;
        println!("Started {}", stopped.name);
    }
    
    Ok(())
}
```

### With State Management

```rust
use dioxus::prelude::*;

spawn(async move {
    let service = match DockerService::new() {
        Ok(s) => s,
        Err(e) => {
            *error_signal.write() = Some(format!("Docker error: {}", e));
            return;
        }
    };
    
    match service.list_containers().await {
        Ok(containers) => {
            *containers_signal.write() = containers;
            *error_signal.write() = None;
        }
        Err(e) => {
            *error_signal.write() = Some(format!("Failed to list: {}", e));
        }
    }
});
```

### Error Recovery

```rust
async fn fetch_with_retry(service: &DockerService, max_retries: u32) -> Result<Vec<ContainerInfo>> {
    let mut retries = 0;
    
    loop {
        match service.list_containers().await {
            Ok(containers) => return Ok(containers),
            Err(e) if retries < max_retries => {
                retries += 1;
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            Err(e) => return Err(e),
        }
    }
}
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_state_transitions() {
        assert_eq!(ContainerState::Running.action_label(), "Stop");
        assert_eq!(ContainerState::Stopped.action_label(), "Start");
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_docker_operations() {
    let service = DockerService::new()
        .expect("Docker must be running");
    
    let containers = service.list_containers().await
        .expect("Failed to list containers");
    
    assert!(containers.len() >= 0);
}
```

## Related Documentation

- [Docker Service Architecture](../architecture/docker-service.md)
- [State Management](../architecture/state-management.md)
- [Extending Docker Integration](../examples/docker-integration.md)
