# API Reference

This document provides technical reference for Containr's internal APIs.

## Table of Contents

- [Services](#services)
  - [DockerService](#dockerservice)
- [State Management](#state-management)
  - [AppState](#appstate)
- [Data Models](#data-models)
- [Components](#components)

## Services

### DockerService

Docker daemon integration service using Bollard.

#### Creation

``````rust
use crate::services::DockerService;

let service = DockerService::new()?;
``````

Connects to Docker daemon via:
- Environment variable `DOCKER_HOST`
- Default: `unix:///var/run/docker.sock`

#### Methods

##### `list_containers`

Lists all Docker containers.

``````rust
pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>>
``````

**Returns**: Vector of container information including:
- ID (truncated to 12 characters)
- Name (without leading `/`)
- Image name
- Status text
- Port mappings
- Current state (Running/Stopped)

**Errors**: Returns error if Docker daemon is unreachable or API fails.

**Example**:

``````rust
let containers = service.list_containers().await?;
for container in containers {
    println!("{}: {}", container.name, container.status);
}
``````

##### `list_images`

Lists all Docker images.

``````rust
pub async fn list_images(&self) -> Result<Vec<ImageInfo>>
``````

**Returns**: Vector of image information including:
- Image ID
- Repository name
- Tag
- Size (formatted as human-readable string)

**Example**:

``````rust
let images = service.list_images().await?;
for image in images {
    println!("{}: {} ({})", image.repository, image.tag, image.size);
}
``````

##### `list_volumes`

Lists all Docker volumes.

``````rust
pub async fn list_volumes(&self) -> Result<Vec<VolumeInfo>>
``````

**Returns**: Vector of volume information including:
- Volume name
- Driver type
- Mount point path
- Size (currently shows "--" as Docker API doesn't provide this directly)

**Note**: Volume size requires additional inspection calls, not yet implemented.

##### `start_container`

Starts a stopped container.

``````rust
pub async fn start_container(&self, id: &str) -> Result<()>
``````

**Parameters**:
- `id`: Container ID (full or short form)

**Returns**: `Ok(())` on success

**Example**:

``````rust
service.start_container("abc123def456").await?;
``````

##### `stop_container`

Stops a running container.

``````rust
pub async fn stop_container(&self, id: &str) -> Result<()>
``````

**Parameters**:
- `id`: Container ID (full or short form)

**Returns**: `Ok(())` on success

**Example**:

``````rust
service.stop_container("abc123def456").await?;
``````

## State Management

### AppState

Global application state managed through Dioxus signals.

#### Creation

``````rust
use crate::utils::AppState;

let app_state = AppState::new();
use_context_provider(|| app_state);
``````

Automatically:
- Connects to Docker daemon
- Initializes all signals
- Triggers initial data refresh

#### Accessing State

From any component:

``````rust
let app_state = use_context::<AppState>();
``````

#### Signals

##### `docker_host: Signal<String>`

Current Docker host connection string.

**Read**: `let host = (app_state.docker_host)();`

##### `containers: Signal<Vec<ContainerInfo>>`

List of all containers.

**Read**: `let containers = (app_state.containers)();`

##### `images: Signal<Vec<ImageInfo>>`

List of all images.

**Read**: `let images = (app_state.images)();`

##### `volumes: Signal<Vec<VolumeInfo>>`

List of all volumes.

**Read**: `let volumes = (app_state.volumes)();`

##### `last_action: Signal<Option<String>>`

Description of last user action.

**Read**: `let action = (app_state.last_action)();`

##### `error_message: Signal<Option<String>>`

Current error message, if any.

**Read**: `let error = (app_state.error_message)();`

##### `is_loading: Signal<bool>`

Whether data is currently loading.

**Read**: `let loading = (app_state.is_loading)();`

#### Methods

##### `refresh_all`

Refreshes all data (containers, images, volumes).

``````rust
pub fn refresh_all(&self)
``````

Spawns async tasks to fetch fresh data from Docker daemon.

**Example**:

``````rust
app_state.refresh_all();
``````

##### `refresh_containers`

Refreshes only container data.

``````rust
pub fn refresh_containers(&self)
``````

##### `refresh_images`

Refreshes only image data.

``````rust
pub fn refresh_images(&self)
``````

##### `refresh_volumes`

Refreshes only volume data.

``````rust
pub fn refresh_volumes(&self)
``````

##### `start_container`

Starts a container and refreshes container list.

``````rust
pub fn start_container(&self, id: String)
``````

**Parameters**:
- `id`: Container ID

Updates `last_action` on success, `error_message` on failure.

##### `stop_container`

Stops a container and refreshes container list.

``````rust
pub fn stop_container(&self, id: String)
``````

**Parameters**:
- `id`: Container ID

##### `set_container_state`

Changes container state (start or stop).

``````rust
pub fn set_container_state(&self, id: &str, next_state: ContainerState)
``````

**Parameters**:
- `id`: Container ID
- `next_state`: Desired state (Running or Stopped)

Convenience method that calls either `start_container` or `stop_container`.

##### `record_action`

Records a user action message.

``````rust
pub fn record_action(&self, message: impl Into<String>)
``````

Updates the `last_action` signal.

## Data Models

### ContainerInfo

``````rust
pub struct ContainerInfo {
    pub id: String,           // Container ID (12 chars)
    pub name: String,         // Container name
    pub image: String,        // Image name
    pub status: String,       // Status text from Docker
    pub ports: String,        // Port mappings
    pub state: ContainerState, // Running or Stopped
}
``````

### ImageInfo

``````rust
pub struct ImageInfo {
    pub id: String,         // Image ID
    pub repository: String, // Repository name
    pub tag: String,        // Tag name
    pub size: String,       // Human-readable size
}
``````

### VolumeInfo

``````rust
pub struct VolumeInfo {
    pub name: String,       // Volume name
    pub driver: String,     // Driver type
    pub mountpoint: String, // Mount path
    pub size: String,       // Size (currently "--")
}
``````

### ContainerState

``````rust
pub enum ContainerState {
    Running,
    Stopped,
}
``````

**Methods**:

``````rust
pub fn label(&self) -> &'static str        // "Running" or "Stopped"
pub fn css_class(&self) -> &'static str    // "running" or "stopped"
pub fn action_label(&self) -> &'static str // "Stop" or "Start"
``````

## Components

### MetricCard

Displays a metric with title and optional hint.

**Props**:

``````rust
#[component]
fn MetricCard(
    title: String,
    value: String,
    hint: Option<String>,
) -> Element
``````

**Example**:

``````rust
rsx! {
    MetricCard {
        title: "Running Containers".to_string(),
        value: "5".to_string(),
        hint: Some("Across all projects".to_string()),
    }
}
``````

### SectionHeader

Page section header with title and optional subtitle.

**Props**:

``````rust
#[component]
fn SectionHeader(
    title: String,
    subtitle: Option<String>,
) -> Element
``````

**Example**:

``````rust
rsx! {
    SectionHeader {
        title: "Containers".to_string(),
        subtitle: Some("Manage your containers".to_string()),
    }
}
``````

### StatusPill

Visual indicator for container status.

**Props**:

``````rust
#[component]
fn StatusPill(
    state: ContainerState,
) -> Element
``````

**Example**:

``````rust
rsx! {
    StatusPill { state: ContainerState::Running }
}
``````

## Type Conversions

### Size Formatting

Internal utility for formatting byte sizes:

``````rust
fn format_size(size: i64) -> String
``````

**Output Examples**:
- `100` → `"100B"`
- `1024` → `"1.0KB"`
- `1048576` → `"1.0MB"`
- `1073741824` → `"1.0GB"`

## Error Handling

All service methods return `Result<T, anyhow::Error>`.

**Common error scenarios**:
- Docker daemon not running
- Permission denied on socket
- Container/image not found
- Invalid container ID
- Network timeout

**Handling in AppState**:

``````rust
match service.operation().await {
    Ok(data) => {
        signal.set(data);
        error_message.set(None);
    }
    Err(e) => {
        error_message.set(Some(format!("Error: {}", e)));
    }
}
``````

## Threading and Async

### Task Spawning

All async operations use `spawn()`:

``````rust
spawn(async move {
    // Async work here
});
``````

- Runs on Tokio runtime
- Non-blocking UI thread
- Can update signals from async context

### Signal Updates from Async

``````rust
let mut signal = app_state.containers.clone();

spawn(async move {
    let data = fetch_data().await;
    signal.set(data); // Safe from async context
});
``````

## Configuration

### Environment Variables

- `DOCKER_HOST`: Docker daemon connection string
  - Default: `unix:///var/run/docker.sock`
  - Remote example: `tcp://192.168.1.100:2375`

### Feature Flags

From `Cargo.toml`:

``````toml
[features]
default = ["desktop"]
desktop = ["dioxus/desktop"]
web = ["dioxus/web"]
``````

**Build for desktop**:
``````bash
cargo build --features desktop
``````

**Build for web**:
``````bash
cargo build --features web
``````
