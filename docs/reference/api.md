# API Reference

Complete reference for Doctainr's core APIs and data structures.

## DockerService

`src/services/docker.rs`

Primary interface to Docker Engine via Bollard.

### Methods

#### `new() -> Result<Self>`

Creates a new Docker service connected to the local Docker socket.

**Returns**: `Result<DockerService, anyhow::Error>`

**Example**:
````rust
let service = DockerService::new()?;
````

#### `list_containers() -> Result<Vec<ContainerInfo>>`

Lists all Docker containers (running and stopped).

**Returns**: Vector of `ContainerInfo` structs

**Example**:
````rust
let containers = service.list_containers().await?;
for container in containers {
    println!("{}: {}", container.name, container.state.label());
}
````

#### `list_images() -> Result<Vec<ImageInfo>>`

Lists all local Docker images.

**Returns**: Vector of `ImageInfo` structs

**Example**:
````rust
let images = service.list_images().await?;
for image in images {
    println!("{}:{} - {}", image.repository, image.tag, image.size);
}
````

#### `list_volumes() -> Result<Vec<VolumeInfo>>`

Lists all Docker volumes.

**Returns**: Vector of `VolumeInfo` structs

**Example**:
````rust
let volumes = service.list_volumes().await?;
for volume in volumes {
    println!("{} ({})", volume.name, volume.driver);
}
````

#### `start_container(&self, id: &str) -> Result<()>`

Starts a stopped container.

**Parameters**:
- `id`: Container ID or name

**Example**:
````rust
service.start_container("my-container").await?;
````

#### `stop_container(&self, id: &str) -> Result<()>`

Stops a running container.

**Parameters**:
- `id`: Container ID or name

**Example**:
````rust
service.stop_container("my-container").await?;
````

## Data Structures

### ContainerInfo

Represents a Docker container's metadata.

````rust
pub struct ContainerInfo {
    pub id: String,           // Short container ID (12 chars)
    pub name: String,         // Container name
    pub image: String,        // Image name
    pub status: String,       // Status text (e.g., "Up 2 hours")
    pub ports: String,        // Port mappings (e.g., "80:8080, 443:8443")
    pub state: ContainerState,// Running or Stopped
}
````

### ImageInfo

Represents a Docker image.

````rust
pub struct ImageInfo {
    pub id: String,           // Full image ID (sha256:...)
    pub repository: String,   // Repository name
    pub tag: String,          // Image tag
    pub size: String,         // Formatted size (e.g., "125.3MB")
}
````

### VolumeInfo

Represents a Docker volume.

````rust
pub struct VolumeInfo {
    pub name: String,         // Volume name
    pub driver: String,       // Volume driver (usually "local")
    pub mountpoint: String,   // Host filesystem path
    pub size: String,         // Size (currently "--")
}
````

### ContainerState

Enum representing container runtime state.

````rust
pub enum ContainerState {
    Running,
    Stopped,
}
````

**Methods**:
- `label() -> &'static str`: Returns "Running" or "Stopped"
- `css_class() -> &'static str`: Returns CSS class name
- `action_label() -> &'static str`: Returns "Stop" or "Start"

## AppState

`src/utils/app_state.rs`

Global application state using Dioxus signals.

### Structure

````rust
pub struct AppState {
    pub docker_host: Signal<String>,
    pub containers: Signal<Vec<ContainerInfo>>,
    pub images: Signal<Vec<ImageInfo>>,
    pub volumes: Signal<Vec<VolumeInfo>>,
    pub last_action: Signal<Option<String>>,
    pub error_message: Signal<Option<String>>,
    pub is_loading: Signal<bool>,
}
````

### Methods

#### `new() -> Self`

Creates new app state and initializes Docker service. Automatically refreshes all data on creation.

**Example**:
````rust
let app_state = AppState::new();
use_context_provider(|| app_state);
````

#### `refresh_all()`

Refreshes containers, images, and volumes simultaneously.

#### `refresh_containers()`

Reloads container list from Docker.

#### `refresh_images()`

Reloads image list from Docker.

#### `refresh_volumes()`

Reloads volume list from Docker.

#### `start_container(id: String)`

Starts a container and refreshes the container list.

**Parameters**:
- `id`: Container ID

#### `stop_container(id: String)`

Stops a container and refreshes the container list.

**Parameters**:
- `id`: Container ID

#### `set_container_state(&self, id: &str, next_state: ContainerState)`

Transitions a container to the specified state.

**Parameters**:
- `id`: Container ID
- `next_state`: Target state (Running or Stopped)

#### `record_action(&self, message: impl Into<String>)`

Records an action message for display to the user.

## Components

### MetricCard

`src/components/metric_card.rs`

Displays a metric with label and value.

**Props**:
````rust
#[component]
pub fn MetricCard(
    label: String,
    value: String,
    icon: Option<String>,
) -> Element
````

**Example**:
````rust
rsx! {
    MetricCard {
        label: "Containers".to_string(),
        value: "12".to_string(),
    }
}
````

### StatusPill

`src/components/status_pill.rs`

Displays container state as a colored badge.

**Props**:
````rust
#[component]
pub fn StatusPill(
    state: ContainerState,
) -> Element
````

**Example**:
````rust
rsx! {
    StatusPill { state: ContainerState::Running }
}
````

### SectionHeader

`src/components/section_header.rs`

Renders a styled section heading.

**Props**:
````rust
#[component]
pub fn SectionHeader(
    title: String,
) -> Element
````

**Example**:
````rust
rsx! {
    SectionHeader { title: "Docker Containers".to_string() }
}
````

## Routing

### Route Enum

Defines all application routes.

````rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(AppShell)]
        #[route("/")]
        Dashboard {},
        #[route("/containers")]
        Containers {},
        #[route("/images")]
        Images {},
        #[route("/volumes")]
        Volumes {},
        #[route("/settings")]
        Settings {},
}
````

## Utility Functions

### `format_size(size: i64) -> String`

Formats byte size into human-readable format.

**Parameters**:
- `size`: Size in bytes

**Returns**: Formatted string (e.g., "125.3MB")

**Example**:
````rust
let size = format_size(1073741824); // "1.0GB"
````

---

**See Also**: [Architecture](./architecture.md) | [Components Reference](./components.md)
