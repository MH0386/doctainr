# Architecture Documentation

This document describes the architecture and design patterns used in Containr.

## Overview

Containr is built as a desktop application using Dioxus 0.7, a React-like UI framework for Rust. The application follows a component-based architecture with unidirectional data flow.

## Architecture Layers

### 1. Presentation Layer (`views/`)

The presentation layer contains page-level components that represent distinct routes in the application.

#### Components

- **Dashboard**: Overview page showing system metrics
- **Containers**: List and manage Docker containers
- **Images**: Browse local Docker images
- **Volumes**: View Docker volumes
- **Settings**: Application configuration
- **AppShell**: Root layout with navigation sidebar

#### Responsibilities

- Render UI based on application state
- Handle user interactions
- Trigger state updates via AppState methods
- No direct Docker API calls

### 2. Component Layer (`components/`)

Reusable UI components shared across views.

#### Components

- **MetricCard**: Display numeric metrics with title and hint
- **SectionHeader**: Page section titles with optional subtitles
- **StatusPill**: Visual indicator for container status

#### Design Principles

- Pure presentational components
- Accept data via props
- No internal state or side effects
- Maximize reusability

### 3. Service Layer (`services/`)

Integration layer for external services and APIs.

#### DockerService

Wraps the Bollard Docker client and provides high-level operations:

``````rust
pub struct DockerService {
    docker: Docker,
}

impl DockerService {
    pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>>;
    pub async fn list_images(&self) -> Result<Vec<ImageInfo>>;
    pub async fn list_volumes(&self) -> Result<Vec<VolumeInfo>>;
    pub async fn start_container(&self, id: &str) -> Result<()>;
    pub async fn stop_container(&self, id: &str) -> Result<()>;
}
``````

#### Design Decisions

- **Async API**: All Docker operations are async to prevent blocking the UI
- **Error Handling**: Uses `anyhow::Result` for flexible error propagation
- **Data Transformation**: Converts Bollard types to domain models
- **Connection Management**: Single persistent connection to Docker daemon

### 4. State Management Layer (`utils/`)

#### AppState

Central state container managing all application data and operations.

``````rust
pub struct AppState {
    // Connection
    pub docker_host: Signal<String>,
    
    // Data
    pub containers: Signal<Vec<ContainerInfo>>,
    pub images: Signal<Vec<ImageInfo>>,
    pub volumes: Signal<Vec<VolumeInfo>>,
    
    // UI State
    pub last_action: Signal<Option<String>>,
    pub error_message: Signal<Option<String>>,
    pub is_loading: Signal<bool>,
    
    // Service
    docker_service: Option<DockerService>,
}
``````

#### State Update Pattern

1. User action triggers method on AppState
2. AppState spawns async task
3. Task calls DockerService
4. Result updates Signal state
5. UI automatically re-renders

#### Example Flow

``````rust
pub fn start_container(&self, id: String) {
    let service = self.docker_service.clone();
    let mut last_action = self.last_action.clone();
    
    spawn(async move {
        match service.start_container(&id).await {
            Ok(_) => {
                last_action.set(Some(format!("Started {}", id)));
                self.refresh_containers();
            }
            Err(e) => {
                error_message.set(Some(format!("Error: {}", e)));
            }
        }
    });
}
``````

## Data Flow

### Unidirectional Data Flow

```
User Action
    ↓
AppState Method
    ↓
Spawn Async Task
    ↓
DockerService Call
    ↓
Update Signal
    ↓
UI Re-render
```

### State Propagation

```
App (Root)
    ↓ use_context_provider(|| app_state)
AppShell
    ↓ use_context::<AppState>()
Views (Dashboard, Containers, etc.)
    ↓ Read signals, call methods
User Actions
```

## Reactivity Model

### Signals

Containr uses Dioxus signals for fine-grained reactivity:

- **Automatic tracking**: Components re-render when reading signals
- **Efficient updates**: Only components reading changed signals re-render
- **Clone semantics**: Signals can be cloned cheaply for async tasks

### Signal Usage Patterns

#### Reading State

``````rust
let containers = (app_state.containers)();  // Clone current value
let count = containers.len();
``````

#### Writing State

``````rust
let mut signal = app_state.containers.clone();
signal.set(new_containers);  // Triggers re-render
``````

#### Async Updates

``````rust
spawn(async move {
    let result = async_operation().await;
    signal.set(result);  // Update from async context
});
``````

## Routing

### Route Definition

Routes are defined as a Rust enum using Dioxus router:

``````rust
#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(AppShell)]
        #[route("/")]
        Dashboard {},
        #[route("/containers")]
        Containers {},
        // ... more routes
}
``````

### Navigation

- **Declarative**: `Link { to: Route::Dashboard {} }`
- **Programmatic**: `navigator.push(Route::Containers {})`
- **Layout nesting**: AppShell wraps all routes via `#[layout]`

## Error Handling

### Strategies

1. **Service Layer**: `Result<T, anyhow::Error>`
2. **State Layer**: Store errors in `error_message: Signal<Option<String>>`
3. **UI Layer**: Conditional rendering of error messages

### Example

``````rust
if let Some(error) = (app_state.error_message)() {
    div { class: "error-message", "⚠️ {error}" }
}
``````

## Async Execution

### Tokio Runtime

- Background async runtime for Docker API calls
- Spawned tasks via `spawn()` macro
- Non-blocking UI updates

### Patterns

#### Fire-and-forget

``````rust
spawn(async move {
    let _ = docker_service.refresh_data().await;
});
``````

#### Update on completion

``````rust
spawn(async move {
    match docker_service.list_containers().await {
        Ok(data) => signal.set(data),
        Err(e) => error_signal.set(Some(e.to_string())),
    }
});
``````

## Asset Pipeline

### Static Assets

- **Location**: `assets/` directory
- **Access**: `asset!("/assets/path/to/file")`
- **Processing**: Dioxus CLI handles bundling and minification

### CSS

- **Main styles**: `assets/styling/main.css`
- **Component styles**: `assets/styling/<component>.css`
- **Injection**: Via `document::Link` or `document::Stylesheet`

## Testing Strategy

### Unit Tests

- Test data transformations in `services/docker.rs`
- Test state logic patterns
- Example: `format_size()` function tests

### Integration Points

- Mock DockerService for testing AppState
- Test routing configuration
- Verify component props handling

## Performance Considerations

### Optimization Techniques

1. **Signal granularity**: Separate signals for independent state
2. **Async spawning**: Non-blocking Docker operations
3. **Efficient re-renders**: Only components reading changed signals update
4. **Connection pooling**: Single persistent Docker connection

### Potential Bottlenecks

- Large container/image lists (pagination not yet implemented)
- Frequent polling for status updates
- No data caching layer

## Security Considerations

### Docker Socket Access

- Requires access to `/var/run/docker.sock`
- Effectively root-level access to Docker daemon
- Users must trust the application

### Remote Docker

- Supports TCP connections via `DOCKER_HOST`
- No built-in TLS/authentication (relies on Docker daemon config)
- Consider SSH tunneling for remote connections

## Future Architecture Improvements

### Planned Enhancements

1. **Pagination**: Handle large datasets efficiently
2. **Caching**: Reduce redundant Docker API calls
3. **WebSocket**: Real-time container status updates
4. **Plugins**: Extensible architecture for custom integrations
5. **Multi-engine**: Support multiple Docker hosts simultaneously

### Technical Debt

- Add comprehensive error types instead of `anyhow::Error`
- Implement proper logging framework
- Add telemetry/metrics collection
- Improve test coverage
