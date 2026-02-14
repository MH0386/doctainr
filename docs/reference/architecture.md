# Architecture Overview

This document describes the high-level architecture of Doctainr, including its components, data flow, and design decisions.

## System Architecture

Doctainr is built as a single-process desktop application with the following layers:

````
┌─────────────────────────────────────────┐
│         User Interface (Dioxus)         │
│  ┌──────────┬──────────┬─────────────┐  │
│  │Dashboard │Containers│Images/Volumes│  │
│  └──────────┴──────────┴─────────────┘  │
├─────────────────────────────────────────┤
│        Application State (Signals)      │
│  ┌────────────────────────────────────┐ │
│  │    AppState (Context Provider)     │ │
│  └────────────────────────────────────┘ │
├─────────────────────────────────────────┤
│       Service Layer (Business Logic)    │
│  ┌────────────────────────────────────┐ │
│  │        DockerService (Bollard)     │ │
│  └────────────────────────────────────┘ │
├─────────────────────────────────────────┤
│       Docker Engine (Unix Socket/TCP)   │
└─────────────────────────────────────────┘
````

## Core Components

### 1. User Interface Layer

**Technology**: Dioxus 0.7 (React-like framework for Rust)

**Location**: `src/views/` and `src/components/`

The UI layer consists of:

- **Views**: Full-page components for different routes
  - `Dashboard`: Overview of Docker resources
  - `Containers`: Container list and management
  - `Images`: Image browser
  - `Volumes`: Volume browser
  - `Settings`: Application configuration
  - `AppShell`: Navigation layout wrapper

- **Components**: Reusable UI elements
  - `MetricCard`: Dashboard metric display
  - `SectionHeader`: Page header with title/subtitle
  - `StatusPill`: Container state indicator

**Key Design Decisions**:

- Uses Dioxus 0.7 reactive signals (`Signal<T>`) instead of older `use_state`
- RSX syntax for declarative UI definition
- Component-based architecture for reusability
- CSS-based styling (no CSS-in-Rust)

### 2. State Management

**Technology**: Dioxus Signals + Context API

**Location**: `src/utils/app_state.rs`

The `AppState` struct holds all application state:

````rust
pub struct AppState {
    pub docker_host: Signal<String>,
    pub containers: Signal<Vec<ContainerInfo>>,
    pub images: Signal<Vec<ImageInfo>>,
    pub volumes: Signal<Vec<VolumeInfo>>,
    pub last_action: Signal<Option<String>>,
    pub error_message: Signal<Option<String>>,
    pub is_loading: Signal<bool>,
    docker_service: Option<DockerService>,
}
````

**State Access Pattern**:

1. `AppState` is provided at the root via `use_context_provider`
2. Child components consume it with `use_context::<AppState>()`
3. Signals are reactive—updating them triggers re-renders

**State Lifecycle**:

- **Initialization**: `AppState::new()` connects to Docker and loads initial data
- **Updates**: User actions trigger async operations that update signals
- **Refresh**: Manual refresh or automatic refresh after state-changing operations

### 3. Service Layer

**Technology**: Bollard (async Docker API client)

**Location**: `src/services/docker.rs`

The `DockerService` wraps Bollard and provides:

- **Container Operations**:
  - `list_containers()` → `Vec<ContainerInfo>`
  - `start_container(id)` → `Result<()>`
  - `stop_container(id)` → `Result<()>`

- **Image Operations**:
  - `list_images()` → `Vec<ImageInfo>`

- **Volume Operations**:
  - `list_volumes()` → `Vec<VolumeInfo>`

**Connection Details**:

- Defaults to Unix socket: `/var/run/docker.sock`
- Respects `DOCKER_HOST` environment variable
- Uses Bollard's `connect_with_local_defaults()`

**Error Handling**:

- Service methods return `Result<T, anyhow::Error>`
- Errors are caught in `AppState` and stored in `error_message` signal
- UI displays error messages to users

### 4. Data Models

**Location**: `src/services/docker.rs`

**ContainerInfo**:
````rust
pub struct ContainerInfo {
    pub id: String,           // Short ID (12 chars)
    pub name: String,         // Container name
    pub image: String,        // Image reference
    pub status: String,       // Status text
    pub ports: String,        // Port mappings (formatted)
    pub state: ContainerState, // Running/Stopped
}
````

**ImageInfo**:
````rust
pub struct ImageInfo {
    pub id: String,         // Full image ID
    pub repository: String, // Repository name
    pub tag: String,        // Image tag
    pub size: String,       // Formatted size (GB/MB/KB)
}
````

**VolumeInfo**:
````rust
pub struct VolumeInfo {
    pub name: String,       // Volume name
    pub driver: String,     // Storage driver
    pub mountpoint: String, // Filesystem path
    pub size: String,       // Size (not available from API)
}
````

## Data Flow

### Reading Data (Example: Loading Containers)

1. User opens Containers view
2. Component calls `app_state.refresh_containers()`
3. `AppState` spawns async task:
   ````rust
   spawn(async move {
       match service.list_containers().await {
           Ok(data) => containers.set(data),
           Err(e) => error_message.set(Some(e)),
       }
   });
   ````
4. Signal update triggers UI re-render
5. Component displays updated container list

### Modifying State (Example: Starting a Container)

1. User clicks "Start" button on stopped container
2. Button's `onclick` handler calls `app_state.start_container(id)`
3. `AppState` spawns async task:
   ````rust
   spawn(async move {
       service.start_container(&id).await?;
       app_state.refresh_containers(); // Refresh after action
   });
   ````
4. Container state updates
5. UI shows updated status

## Routing

**Technology**: Dioxus Router

**Location**: `src/main.rs`

Routes are defined as an enum:

````rust
#[derive(Routable)]
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

- All routes share the `AppShell` layout (navigation sidebar)
- Router handles URL synchronization and component rendering
- No dynamic route parameters (yet)

## Async Execution

**Technology**: Tokio (via Dioxus `spawn`)

- All Docker API calls are async
- Dioxus `spawn()` schedules tasks on Tokio runtime
- Signals bridge async operations to reactive UI

**Example**:
````rust
spawn(async move {
    let result = async_operation().await;
    signal.set(result);
});
````

## Styling

**Technology**: Custom CSS

**Location**: `assets/styling/main.css`

- Traditional CSS stylesheet
- Class-based styling (e.g., `.card`, `.button`)
- No inline styles or CSS-in-JS
- Loaded via `document::Link` in root component

## Build Configuration

**Files**: `Cargo.toml`, `Dioxus.toml`

**Key Dependencies**:
- `dioxus = "0.7.1"` with `router` and `desktop` features
- `bollard = "0.18"` for Docker API
- `tokio` with `full` features
- `serde` and `serde_json` for serialization
- `anyhow` for error handling

**Build Targets**:
- Default: Desktop (native binary)
- Optional: Web (WebAssembly)

## Testing

**Location**: Inline tests in `src/services/docker.rs`

Currently includes:
- Unit tests for `ContainerState` methods
- Unit tests for size formatting

**Future**: Integration tests for Docker operations (requires Docker daemon)

## Security Considerations

1. **Socket Access**: Requires permission to `/var/run/docker.sock`
2. **Privilege**: No special privileges required beyond Docker socket access
3. **Input Validation**: Container IDs are passed directly to Docker API (API validates)

## Performance

- **Native Performance**: Rust compiled to machine code
- **Efficient Rendering**: Dioxus virtual DOM minimizes updates
- **Lazy Loading**: Data loaded on-demand, not continuously polled
- **Memory Safety**: Rust's ownership system prevents leaks

## Future Architecture Considerations

Potential enhancements:

1. **Auto-refresh**: Background polling for real-time updates
2. **Container Logs**: Stream logs from running containers
3. **Container Exec**: Interactive shell access
4. **Multi-host**: Manage multiple Docker hosts
5. **Compose Support**: Docker Compose stack management
6. **Persistence**: Save window position, preferences

## Related Documentation

- [State Management](state-management.md) - Detailed signal usage
- [API Documentation](api.md) - Function signatures and types
- [Design Principles](../explanation/design-principles.md) - Why these choices were made
