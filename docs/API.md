# API Reference

Complete API reference for Doctainr's internal modules and types.

## Services

### Docker Service

The `DockerService` provides high-level Docker operations.

#### `DockerService::new()`

Creates a new Docker service instance.

````rust
pub fn new() -> Result<Self>
````

**Returns:** `Result<DockerService, anyhow::Error>`

**Example:**
````rust
let docker = DockerService::new()?;
````

#### `DockerService::list_containers()`

Lists all Docker containers (running and stopped).

````rust
pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>>
````

**Returns:** Vector of `ContainerInfo` structs

**Example:**
````rust
let containers = docker.list_containers().await?;
for container in containers {
    println!("{}: {}", container.name, container.state.label());
}
````

#### `DockerService::list_images()`

Lists all Docker images.

````rust
pub async fn list_images(&self) -> Result<Vec<ImageInfo>>
````

**Returns:** Vector of `ImageInfo` structs

#### `DockerService::list_volumes()`

Lists all Docker volumes.

````rust
pub async fn list_volumes(&self) -> Result<Vec<VolumeInfo>>
````

**Returns:** Vector of `VolumeInfo` structs

#### `DockerService::start_container()`

Starts a stopped container.

````rust
pub async fn start_container(&self, id: &str) -> Result<()>
````

**Parameters:**
- `id`: Container ID or name

**Returns:** `Result<(), anyhow::Error>`

**Example:**
````rust
docker.start_container("my-container").await?;
````

#### `DockerService::stop_container()`

Stops a running container.

````rust
pub async fn stop_container(&self, id: &str) -> Result<()>
````

**Parameters:**
- `id`: Container ID or name

**Returns:** `Result<(), anyhow::Error>`

#### `DockerService::container_stats()`

Retrieves real-time statistics for a container.

````rust
pub async fn container_stats(&self, id: &str) -> Result<ContainerStats>
````

**Parameters:**
- `id`: Container ID or name

**Returns:** `ContainerStats` struct with CPU, memory, and network usage

## Types

### `ContainerState`

Represents the state of a Docker container.

````rust
pub enum ContainerState {
    Running,
    Stopped,
}
````

**Methods:**

- `label() -> &'static str`: Human-readable label ("Running" or "Stopped")
- `css_class() -> &'static str`: CSS class name for styling
- `action_label() -> &'static str`: Label for toggle action button

### `ContainerInfo`

Information about a Docker container.

````rust
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub ports: String,
    pub state: ContainerState,
}
````

**Fields:**
- `id`: Container ID (short format)
- `name`: Container name
- `image`: Image name and tag
- `status`: Human-readable status string
- `ports`: Port mappings (e.g., "80/tcp -> 0.0.0.0:8080")
- `state`: Current state (Running or Stopped)

### `ImageInfo`

Information about a Docker image.

````rust
pub struct ImageInfo {
    pub id: String,
    pub repository: String,
    pub tag: String,
    pub size: String,
    pub created: String,
}
````

**Fields:**
- `id`: Image ID (short format)
- `repository`: Repository name
- `tag`: Image tag
- `size`: Human-readable size (e.g., "150 MB")
- `created`: Creation timestamp

### `VolumeInfo`

Information about a Docker volume.

````rust
pub struct VolumeInfo {
    pub name: String,
    pub driver: String,
    pub mountpoint: String,
    pub created: String,
}
````

**Fields:**
- `name`: Volume name
- `driver`: Volume driver (usually "local")
- `mountpoint`: Filesystem path
- `created`: Creation timestamp

### `ContainerStats`

Real-time statistics for a container.

````rust
pub struct ContainerStats {
    pub cpu_usage: f64,
    pub memory_usage: u64,
    pub memory_limit: u64,
    pub network_rx: u64,
    pub network_tx: u64,
}
````

**Fields:**
- `cpu_usage`: CPU usage percentage
- `memory_usage`: Memory usage in bytes
- `memory_limit`: Memory limit in bytes
- `network_rx`: Bytes received
- `network_tx`: Bytes transmitted

## State Management

### `AppState`

Global application state using Dioxus Signals.

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

**Methods:**

#### `AppState::new()`

Creates a new application state instance.

````rust
pub fn new() -> Self
````

Automatically attempts to connect to Docker daemon on creation.

#### `AppState::refresh_containers()`

Refreshes the container list from Docker.

````rust
pub async fn refresh_containers(&self) -> Result<()>
````

Updates the `containers` Signal with fresh data.

#### `AppState::refresh_images()`

Refreshes the image list from Docker.

````rust
pub async fn refresh_images(&self) -> Result<()>
````

Updates the `images` Signal with fresh data.

#### `AppState::refresh_volumes()`

Refreshes the volume list from Docker.

````rust
pub async fn refresh_volumes(&self) -> Result<()>
````

Updates the `volumes` Signal with fresh data.

#### `AppState::refresh_all()`

Refreshes all data (containers, images, volumes).

````rust
pub fn refresh_all(&self)
````

Spawns async tasks to refresh all data concurrently.

#### `AppState::toggle_container()`

Toggles a container's state (start if stopped, stop if running).

````rust
pub async fn toggle_container(&self, id: &str, current_state: ContainerState) -> Result<()>
````

**Parameters:**
- `id`: Container ID or name
- `current_state`: Current container state

Updates state Signals on success or failure.

## Components

### `MetricCard`

Displays a metric value with an icon.

````rust
#[component]
pub fn MetricCard(title: String, value: String, icon: String) -> Element
````

**Props:**
- `title`: Metric title
- `value`: Metric value (as string)
- `icon`: Icon character or emoji

### `SectionHeader`

Displays a section header with optional action button.

````rust
#[component]
pub fn SectionHeader(title: String, action: Option<String>) -> Element
````

**Props:**
- `title`: Section title
- `action`: Optional action button label

### `StatusPill`

Displays a status indicator pill.

````rust
#[component]
pub fn StatusPill(state: ContainerState) -> Element
````

**Props:**
- `state`: Container state to display

## Views

### `Dashboard`

Main dashboard view showing overview of all resources.

````rust
#[component]
pub fn Dashboard() -> Element
````

### `Containers`

Container management view.

````rust
#[component]
pub fn Containers() -> Element
````

### `Images`

Image management view.

````rust
#[component]
pub fn Images() -> Element
````

### `Volumes`

Volume management view.

````rust
#[component]
pub fn Volumes() -> Element
````

### `Settings`

Application settings view.

````rust
#[component]
pub fn Settings() -> Element
````

### `AppShell`

Layout component providing navigation and page structure.

````rust
#[component]
pub fn AppShell() -> Element
````

Uses Dioxus router's `Outlet<Route>` to render child routes.

## Error Handling

All service methods return `Result<T, anyhow::Error>`. Errors are propagated to the UI layer and displayed to users via the `AppState::error_message` Signal.

**Example:**
````rust
match app_state.toggle_container(&id, state).await {
    Ok(_) => {
        // Success - Signal automatically updated
    }
    Err(e) => {
        // Error stored in app_state.error_message
        eprintln!("Failed to toggle container: {}", e);
    }
}
````

## Configuration

### Environment Variables

- `DOCKER_HOST`: Docker daemon address (default: `unix:///var/run/docker.sock`)
  - Unix socket: `unix:///var/run/docker.sock`
  - TCP: `tcp://localhost:2375`
  - TLS: `tcp://localhost:2376` (requires certificates)

**Example:**
````bash
export DOCKER_HOST=tcp://remote-host:2375
cargo run
````
