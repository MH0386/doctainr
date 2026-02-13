# API Reference

This section provides detailed technical reference documentation for Doctainr's public APIs, data structures, and modules.

## Module Overview

Doctainr is organized into four main modules:

### 📦 [Services](services.md)
External integrations and business logic layer.
- **Docker Service** - Interface to Docker daemon via Bollard
- Container, image, and volume operations
- Connection management

### 🎨 [Components](components.md)
Reusable UI building blocks.
- **MetricCard** - Dashboard metric display
- **SectionHeader** - Page title and subtitle
- **StatusPill** - Visual status indicators

### 📄 [Views](views.md)
Page-level components and layouts.
- **AppShell** - Application layout and navigation
- **Dashboard** - System overview
- **Containers** - Container management
- **Images** - Image browser
- **Volumes** - Volume manager
- **Settings** - Configuration panel

### 🔄 [State](state.md)
Application state management using Dioxus signals.
- **AppState** - Global application state
- Reactive data flow
- State update patterns

## Quick Navigation

| Topic | Module | Documentation |
|-------|--------|---------------|
| Docker operations | `services::docker` | [services.md](services.md) |
| UI components | `components` | [components.md](components.md) |
| Pages and layouts | `views` | [views.md](views.md) |
| State management | `utils::app_state` | [state.md](state.md) |

## Core Data Structures

### Container Information
```rust
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub ports: String,
    pub state: ContainerState,
}
```

### Image Information
```rust
pub struct ImageInfo {
    pub id: String,
    pub repository: String,
    pub tag: String,
    pub size: String,
}
```

### Volume Information
```rust
pub struct VolumeInfo {
    pub name: String,
    pub driver: String,
    pub mountpoint: String,
    pub size: String,
}
```

### Container State
```rust
pub enum ContainerState {
    Running,
    Stopped,
}
```

## Common Patterns

### Accessing Application State
```rust
#[component]
fn MyComponent() -> Element {
    let app_state = use_context::<AppState>();
    let containers = (app_state.containers)();
    // ... use containers
}
```

### Triggering State Updates
```rust
// Refresh all data
app_state.refresh_all();

// Refresh specific resource
app_state.refresh_containers();
app_state.refresh_images();
app_state.refresh_volumes();

// Container operations
app_state.start_container("container_id");
app_state.stop_container("container_id");
```

### Recording Actions
```rust
app_state.record_action("Container started");
```

## Error Handling

All Docker operations return `Result<T, anyhow::Error>`. Errors are captured and displayed in the UI via `app_state.error_message`.

```rust
if let Err(e) = docker_service.start_container(&container_id) {
    app_state.set_error(&format!("Failed to start container: {}", e));
}
```

## Type Conventions

- **IDs**: `String` (container ID, image ID)
- **Names**: `String` (container name, volume name)
- **Sizes**: `String` (human-readable, e.g., "1.2 GB")
- **Ports**: `String` (comma-separated, e.g., "80/tcp, 443/tcp")
- **States**: `enum` (type-safe state representation)

## Next Steps

- Dive into [Services Documentation](services.md) for Docker integration details
- Review [Components Documentation](components.md) for UI building blocks
- Explore [Views Documentation](views.md) for page structure
- Understand [State Documentation](state.md) for reactive data flow
