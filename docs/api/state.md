# Application State API

The application state module provides reactive state management using Dioxus 0.7's signal-based architecture.

## Module: `utils::app_state`

**Location**: `src/utils/app_state.rs`

### Overview

`AppState` is the central state container for the application, managing:
- Docker resource data (containers, images, volumes)
- Connection configuration
- UI state (loading, errors, actions)
- Reactive updates through Dioxus signals

All state fields are Dioxus `Signal` types that automatically trigger re-renders when modified.

---

## Core Type

### `AppState`

Central application state with reactive signals.

````rust
#[derive(Clone)]
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

#### Fields

| Field | Type | Purpose |
|-------|------|---------|
| `docker_host` | `Signal<String>` | Docker daemon connection string |
| `containers` | `Signal<Vec<ContainerInfo>>` | List of all containers |
| `images` | `Signal<Vec<ImageInfo>>` | List of all images |
| `volumes` | `Signal<Vec<VolumeInfo>>` | List of all volumes |
| `last_action` | `Signal<Option<String>>` | Most recent user action message |
| `error_message` | `Signal<Option<String>>` | Current error message, if any |
| `is_loading` | `Signal<bool>` | Global loading indicator |
| `docker_service` | `Option<DockerService>` | Docker API client (private) |

---

## Constructor

### `new() -> Self`

Creates and initializes application state.

**Behavior**:
1. Attempts to connect to Docker daemon
2. Initializes all signals with default values
3. Reads `DOCKER_HOST` environment variable (defaults to `unix:///var/run/docker.sock`)
4. Automatically triggers initial data load via `refresh_all()`

**Returns**: `AppState`

**Example**:
````rust
use crate::utils::AppState;

#[component]
fn App() -> Element {
    let app_state = AppState::new();
    use_context_provider(|| app_state);
    
    rsx! { /* ... */ }
}
````

**Note**: Prints error to stderr if Docker connection fails, but still returns usable state object.

---

## Data Refresh Methods

### `refresh_all(&self)`

Refreshes all Docker resources concurrently.

**Behavior**: Spawns three async tasks to update containers, images, and volumes simultaneously.

**Example**:
````rust
let app_state = use_context::<AppState>();
app_state.refresh_all();
````

### `refresh_containers(&self)`

Fetches and updates container list from Docker daemon.

**Side Effects**:
- Sets `is_loading` to `true` while fetching
- Updates `containers` signal on success
- Sets `error_message` on failure
- Clears `error_message` on success

**Example**:
````rust
button {
    onclick: move |_| app_state.refresh_containers(),
    "Refresh Containers"
}
````

### `refresh_images(&self)`

Fetches and updates image list from Docker daemon.

**Side Effects**:
- Updates `images` signal on success
- Sets `error_message` on failure

### `refresh_volumes(&self)`

Fetches and updates volume list from Docker daemon.

**Side Effects**:
- Updates `volumes` signal on success
- Sets `error_message` on failure

---

## Container Operations

### `start_container(&self, id: String)`

Starts a stopped container asynchronously.

**Parameters**:
- `id` - Container ID (short or full form)

**Side Effects**:
- Sets `last_action` with success message
- Automatically refreshes container list on success
- Sets `error_message` on failure

**Example**:
````rust
let container_id = "abc123def456".to_string();
app_state.start_container(container_id);
````

**Async Behavior**: Spawns background task; UI updates occur automatically via signals.

### `stop_container(&self, id: String)`

Stops a running container asynchronously.

**Parameters**:
- `id` - Container ID (short or full form)

**Side Effects**:
- Sets `last_action` with success message
- Automatically refreshes container list on success
- Sets `error_message` on failure

**Example**:
````rust
let container_id = "abc123def456".to_string();
app_state.stop_container(container_id);
````

### `set_container_state(&self, id: &str, next_state: ContainerState)`

Toggles container state based on desired target state.

**Parameters**:
- `id` - Container ID
- `next_state` - Desired state (`ContainerState::Running` or `ContainerState::Stopped`)

**Example**:
````rust
// Start a container
app_state.set_container_state("abc123", ContainerState::Running);

// Stop a container
app_state.set_container_state("abc123", ContainerState::Stopped);
````

**Use Case**: Useful for toggle buttons where target state is known.

---

## UI State Methods

### `record_action(&self, message: impl Into<String>)`

Records a user action for display in the UI header.

**Parameters**:
- `message` - Action description

**Example**:
````rust
app_state.record_action("Container started");
app_state.record_action(format!("Deleted image {}", image_id));
````

**Note**: Does not clear automatically; typically overwritten by next action.

---

## Accessing State in Components

### Pattern 1: Context Consumer

````rust
#[component]
fn Dashboard() -> Element {
    let app_state = use_context::<AppState>();
    let containers = (app_state.containers)();
    let images = (app_state.images)();
    
    rsx! {
        h1 { "Total Containers: {containers.len()}" }
        h1 { "Total Images: {images.len()}" }
    }
}
````

**Key Points**:
- Call the signal like a function `()` to read current value
- Component automatically re-renders when signal changes
- Clone operations on `AppState` are cheap (signals use `Copy`)

### Pattern 2: Signal Cloning for Event Handlers

````rust
#[component]
fn RefreshButton() -> Element {
    let app_state = use_context::<AppState>();
    
    rsx! {
        button {
            onclick: move |_| app_state.refresh_all(),
            "Refresh All Data"
        }
    }
}
````

### Pattern 3: Conditional Rendering

````rust
#[component]
fn ErrorDisplay() -> Element {
    let app_state = use_context::<AppState>();
    let error_message = (app_state.error_message)();
    
    rsx! {
        if let Some(error) = error_message {
            div { class: "error-banner", "{error}" }
        }
    }
}
````

---

## Signal Usage Patterns

### Reading Signal Values

````rust
// Clone the current value
let containers = (app_state.containers)();

// Access without cloning (borrow)
let count = (app_state.containers)().len();
````

### Writing Signal Values

````rust
// Direct set (replaces entire value)
app_state.error_message.set(Some("Error occurred".to_string()));

// Clear optional value
app_state.error_message.set(None);

// Update boolean
app_state.is_loading.set(true);
````

### Signal in Async Context

````rust
spawn(async move {
    let mut loading = app_state.is_loading.clone();
    loading.set(true);
    
    // Perform async work...
    
    loading.set(false);
});
````

---

## Reactive Data Flow

### Data Flow Diagram

````
User Action (Button Click)
    ↓
AppState Method Call (e.g., refresh_containers)
    ↓
Spawn Async Task
    ↓
Docker API Call (via DockerService)
    ↓
Update Signal (containers.set(...))
    ↓
Automatic Component Re-render
````

### Automatic Reactivity

Components that read signals automatically subscribe to changes:

````rust
#[component]
fn ContainerCount() -> Element {
    let app_state = use_context::<AppState>();
    // Reading this signal registers a dependency
    let count = (app_state.containers)().len();
    
    // This component will re-render whenever containers signal changes
    rsx! { "Total: {count}" }
}
````

---

## Error Handling

### Connection Errors

If Docker service fails to initialize:
- `docker_service` is set to `None`
- Refresh methods set `error_message` to `"Docker service not available"`
- UI can display graceful fallback

**Example**:
````rust
if let Some(error) = (app_state.error_message)() {
    rsx! { div { class: "error", "{error}" } }
}
````

### Operation Errors

All async operations capture errors and set `error_message`:
- Docker daemon unreachable
- Invalid container ID
- Permission denied
- Operation timeout

---

## Thread Safety

- `AppState` is `Clone` and safe to share across async tasks
- Signals use internal `Copy` semantics for efficient sharing
- All async operations use `.clone()` to capture signals for background tasks

---

## Performance Considerations

### Signal Efficiency
- Reading a signal is O(1)
- Writing a signal triggers minimal re-renders (only dependent components)
- Signals are optimized for cheap cloning

### Async Task Management
- Each operation spawns independent async task
- Tasks do not block UI thread
- Concurrent refreshes are safe (last-write-wins)

### Best Practices
````rust
// ✅ Good: Read signal once if using multiple times
let containers = (app_state.containers)();
let running = containers.iter().filter(|c| c.state == ContainerState::Running).count();
let stopped = containers.len() - running;

// ❌ Avoid: Reading signal multiple times unnecessarily
let running = (app_state.containers)().iter().filter(...).count();
let stopped = (app_state.containers)().len() - running; // Redundant read
````

---

## Initialization Sequence

1. `AppState::new()` called in `App` component
2. Docker connection attempted
3. Signals initialized with empty/default values
4. `refresh_all()` triggered automatically
5. Three async tasks spawn to load initial data
6. UI renders with loading state
7. Signals update as data arrives
8. Components automatically re-render

---

## See Also

- [Docker Services](services.md) - Docker API integration
- [Views Documentation](views.md) - How components consume state
- [Dioxus Patterns](../architecture/dioxus-patterns.md) - Signal patterns and best practices
- [Dioxus 0.7 Signals Guide](https://dioxuslabs.com/learn/0.7/reference/signals)
