# Docker Services API

The services module provides a high-level interface to Docker daemon operations using the [Bollard](https://github.com/fussybeaver/bollard) library.

## Module: `services::docker`

**Location**: `src/services/docker.rs`

### Overview

The Docker service layer provides:
- Container lifecycle management (list, start, stop)
- Image listing and inspection
- Volume enumeration
- Automatic connection to local Docker daemon

## Core Types

### `DockerService`

Main service object for Docker operations.

````rust
pub struct DockerService {
    docker: Docker,
}
````

#### Constructor

##### `new() -> Result<Self>`

Creates a new Docker service instance by connecting to the local Docker daemon.

**Connection Priority**:
1. Uses `DOCKER_HOST` environment variable if set
2. Falls back to platform defaults:
   - Unix: `unix:///var/run/docker.sock`
   - Windows: Named pipe `//./pipe/docker_engine`

**Returns**: `Result<DockerService, anyhow::Error>`

**Example**:
````rust
use crate::services::DockerService;

let service = DockerService::new()?;
````

**Errors**: Returns error if:
- Docker daemon is not running
- Connection socket/pipe is not accessible
- Insufficient permissions

---

### Container Operations

#### `list_containers() -> Result<Vec<ContainerInfo>>`

Lists all containers (both running and stopped).

**Returns**: `Result<Vec<ContainerInfo>, anyhow::Error>`

**Example**:
````rust
let containers = service.list_containers().await?;
for container in containers {
    println!("{}: {} ({})", 
        container.name, 
        container.status, 
        container.state.label()
    );
}
````

**Note**: Container IDs are truncated to 12 characters for display.

#### `start_container(id: &str) -> Result<()>`

Starts a stopped container.

**Parameters**:
- `id` - Container ID (full or short form accepted)

**Returns**: `Result<(), anyhow::Error>`

**Example**:
````rust
service.start_container("abc123def456").await?;
````

**Errors**: Returns error if:
- Container ID is invalid
- Container is already running
- Docker daemon error occurs

#### `stop_container(id: &str) -> Result<()>`

Stops a running container gracefully.

**Parameters**:
- `id` - Container ID (full or short form accepted)

**Returns**: `Result<(), anyhow::Error>`

**Example**:
````rust
service.stop_container("abc123def456").await?;
````

**Errors**: Returns error if:
- Container ID is invalid
- Container is already stopped
- Docker daemon error occurs

---

### Image Operations

#### `list_images() -> Result<Vec<ImageInfo>>`

Lists all images in the local Docker cache.

**Returns**: `Result<Vec<ImageInfo>, anyhow::Error>`

**Example**:
````rust
let images = service.list_images().await?;
for image in images {
    println!("{}:{} - {}", 
        image.repository, 
        image.tag, 
        image.size
    );
}
````

**Note**: Excludes dangling images (untagged). Image sizes are formatted as human-readable strings.

---

### Volume Operations

#### `list_volumes() -> Result<Vec<VolumeInfo>>`

Lists all Docker volumes on the system.

**Returns**: `Result<Vec<VolumeInfo>, anyhow::Error>`

**Example**:
````rust
let volumes = service.list_volumes().await?;
for volume in volumes {
    println!("{} ({}) at {}", 
        volume.name, 
        volume.driver, 
        volume.mountpoint
    );
}
````

**Note**: Volume size information requires additional inspection calls and is not included in basic listing.

---

## Data Structures

### `ContainerInfo`

Represents container metadata.

````rust
pub struct ContainerInfo {
    pub id: String,           // Short ID (12 chars)
    pub name: String,         // Container name without leading /
    pub image: String,        // Source image name
    pub status: String,       // Human-readable status
    pub ports: String,        // Port mappings (formatted)
    pub state: ContainerState, // Typed state enum
}
````

**Port Formatting**:
- `"8080:80, 8443:443"` - Public:private mappings
- `"80"` - Private port only
- `"--"` - No ports exposed

### `ImageInfo`

Represents image metadata.

````rust
pub struct ImageInfo {
    pub id: String,       // Full SHA256 ID
    pub repository: String, // Repository name
    pub tag: String,      // Image tag
    pub size: String,     // Human-readable size (e.g., "1.2GB")
}
````

**Tag Parsing**:
- `"nginx:latest"` → repository: `"nginx"`, tag: `"latest"`
- Untagged images show `"<none>"` for both fields

### `VolumeInfo`

Represents volume metadata.

````rust
pub struct VolumeInfo {
    pub name: String,      // Volume name/hash
    pub driver: String,    // Storage driver (usually "local")
    pub mountpoint: String, // Filesystem path
    pub size: String,      // Currently "--" (requires inspection)
}
````

### `ContainerState`

Type-safe container state enumeration.

````rust
pub enum ContainerState {
    Running,
    Stopped,
}
````

#### Methods

##### `label() -> &'static str`

Returns display label for UI.

**Returns**:
- `"Running"` for `ContainerState::Running`
- `"Stopped"` for `ContainerState::Stopped`

##### `css_class() -> &'static str`

Returns CSS class name for styling.

**Returns**:
- `"running"` for `ContainerState::Running`
- `"stopped"` for `ContainerState::Stopped`

##### `action_label() -> &'static str`

Returns label for state toggle button.

**Returns**:
- `"Stop"` for `ContainerState::Running`
- `"Start"` for `ContainerState::Stopped`

---

## Helper Functions

### `format_size(size: i64) -> String`

Formats byte size as human-readable string.

**Parameters**:
- `size` - Size in bytes

**Returns**: Formatted string with appropriate unit

**Examples**:
- `format_size(100)` → `"100B"`
- `format_size(1024)` → `"1.0KB"`
- `format_size(1048576)` → `"1.0MB"`
- `format_size(1073741824)` → `"1.0GB"`

---

## Error Handling

All async methods return `Result<T, anyhow::Error>`. Common error scenarios:

### Connection Errors
````rust
// Docker daemon not running
Failed to connect to Docker daemon at unix:///var/run/docker.sock

// Permission denied
Permission denied while trying to connect to the Docker daemon socket
````

### Operation Errors
````rust
// Invalid container ID
No such container: abc123

// Container already in desired state
Container already stopped
````

### Best Practices

Always handle errors in the UI layer:
````rust
match docker_service.start_container(&id).await {
    Ok(_) => app_state.record_action("Container started"),
    Err(e) => app_state.set_error(&format!("Failed to start: {}", e)),
}
````

---

## Thread Safety

`DockerService` is `Clone` and can be safely shared across async tasks. The underlying Bollard client uses `Arc` internally for thread-safe operations.

## Performance Considerations

- **List operations** are relatively fast (<100ms for typical workloads)
- **Start/stop operations** are async and may take several seconds
- **Connection pooling** is handled automatically by Bollard
- Consider implementing caching for frequently accessed data

## Testing

Basic unit tests verify:
- State label consistency
- Size formatting correctness

Integration tests require a running Docker daemon and are not included in the default test suite.

---

## See Also

- [Application State](state.md) - How Docker service integrates with app state
- [Views Documentation](views.md) - How data is displayed in the UI
- [Bollard Documentation](https://docs.rs/bollard/) - Underlying Docker client library
