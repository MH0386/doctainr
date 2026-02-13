# Architecture

Doctainr is built using modern Rust patterns with Dioxus 0.7 for reactive UI.

## System Overview

````
┌─────────────────────────────────────────────┐
│          Dioxus Desktop Application         │
│  ┌────────────────────────────────────────┐ │
│  │         Router & Navigation            │ │
│  └────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────┐ │
│  │      Views (Dashboard, Containers,     │ │
│  │       Images, Volumes, Settings)       │ │
│  └────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────┐ │
│  │    Components (MetricCard, Status)     │ │
│  └────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────┐ │
│  │         AppState (Signals)             │ │
│  └────────────────────────────────────────┘ │
│  ┌────────────────────────────────────────┐ │
│  │       DockerService (Bollard)          │ │
│  └────────────────────────────────────────┘ │
└─────────────────────────────────────────────┘
                     │
                     ▼
         ┌──────────────────────┐
         │   Docker Engine      │
         │  (Unix Socket/TCP)   │
         └──────────────────────┘
````

## Project Structure

````
doctainr/
├── src/
│   ├── main.rs              # Application entry point & routing
│   ├── components/          # Reusable UI components
│   │   ├── mod.rs
│   │   ├── metric_card.rs   # Dashboard metric display
│   │   ├── section_header.rs # Section titles
│   │   └── status_pill.rs   # Container status badges
│   ├── services/            # External service integrations
│   │   ├── mod.rs
│   │   └── docker.rs        # Docker API client (Bollard)
│   ├── utils/               # Shared utilities
│   │   ├── mod.rs
│   │   └── app_state.rs     # Global application state
│   └── views/               # Page-level components
│       ├── mod.rs
│       ├── dashboard.rs     # Overview page
│       ├── containers.rs    # Container management
│       ├── images.rs        # Image browser
│       ├── volumes.rs       # Volume viewer
│       ├── settings.rs      # App configuration
│       └── shell.rs         # Layout/navigation shell
├── assets/                  # Static resources
│   ├── favicon.ico
│   └── styling/
│       └── main.css
├── Cargo.toml              # Rust dependencies
├── Dioxus.toml            # Dioxus configuration
└── docs/                  # Documentation
````

## Core Concepts

### Routing

Doctainr uses Dioxus Router with enum-based routes:

````rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(AppShell)]
        #[route("/")]
        Dashboard {},
        #[route("/containers")]
        Containers {},
        // ...
}
````

The `#[layout(AppShell)]` attribute wraps all routes with the navigation shell.

### State Management

Global state is managed through Dioxus signals in `AppState`:

- **`Signal<T>`**: Reactive state that triggers re-renders when updated
- **Context API**: Provides state to all child components via `use_context_provider`
- **Clone semantics**: State is cheaply cloneable for use in async tasks

Key signals:
- `containers: Signal<Vec<ContainerInfo>>` - Container list
- `images: Signal<Vec<ImageInfo>>` - Image list
- `volumes: Signal<Vec<VolumeInfo>>` - Volume list
- `error_message: Signal<Option<String>>` - Error display
- `is_loading: Signal<bool>` - Loading state

### Async Operations

Docker operations are asynchronous and use Tokio:

````rust
spawn(async move {
    match service.list_containers().await {
        Ok(data) => containers.set(data),
        Err(e) => error_message.set(Some(format!("Error: {}", e))),
    }
});
````

The `spawn` function runs async code without blocking the UI.

## Data Flow

1. **User Action**: User clicks button in a View component
2. **State Update**: View calls method on `AppState` (e.g., `refresh_containers()`)
3. **Async Task**: `AppState` spawns async task using `DockerService`
4. **API Call**: `DockerService` makes request to Docker via Bollard
5. **Signal Update**: Response updates the appropriate `Signal`
6. **Re-render**: Components reading that signal automatically re-render

## Docker Integration

Doctainr uses [Bollard](https://docs.rs/bollard/) to communicate with Docker:

- **Connection**: Connects to local Docker socket (`unix:///var/run/docker.sock` or `DOCKER_HOST`)
- **API Coverage**: Implements container, image, and volume listing/management
- **Error Handling**: Propagates Docker errors through `anyhow::Result`

### Supported Operations

| Resource   | List | Start | Stop | Inspect | Remove |
|------------|------|-------|------|---------|--------|
| Containers | ✅   | ✅    | ✅   | ❌      | ❌     |
| Images     | ✅   | -     | -    | ❌      | ❌     |
| Volumes    | ✅   | -     | -    | ❌      | ❌     |

## Component Hierarchy

````
App (Router)
└── AppShell (Layout)
    ├── Navigation Sidebar
    └── Router Outlet
        ├── Dashboard
        │   └── MetricCard × 3
        ├── Containers
        │   ├── SectionHeader
        │   └── Container rows with StatusPill
        ├── Images
        │   ├── SectionHeader
        │   └── Image rows
        ├── Volumes
        │   ├── SectionHeader
        │   └── Volume rows
        └── Settings
````

## Performance Considerations

- **Lazy Loading**: Views only fetch data when navigated to
- **Signal Reactivity**: Only components reading changed signals re-render
- **Async Non-Blocking**: Docker API calls don't freeze the UI
- **Native Performance**: Compiled Rust with no JavaScript overhead

## Security

- **Local Only**: No network exposure, desktop-only application
- **Docker Socket**: Requires Docker socket access (same as Docker CLI)
- **No Credentials**: Uses ambient Docker authentication

## Extension Points

To add new functionality:

1. **New View**: Add route to `Route` enum and implement view component
2. **New Docker Operation**: Extend `DockerService` with Bollard methods
3. **New Component**: Create in `src/components/` for reusability
4. **New State**: Add signals to `AppState` for new data types

---

**See Also**: [API Reference](./api.md) | [Components Reference](./components.md)
