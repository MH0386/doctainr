# State Management in Doctainr

Understanding how Doctainr manages application state using Dioxus 0.7 signals.

## Overview

Doctainr uses **reactive state management** through Dioxus signals. When state changes, only components reading that specific state re-render automatically.

## Core Concepts

### Signals

A `Signal<T>` is a reactive wrapper around a value:

````rust
let mut count = use_signal(|| 0);

// Read: clone the value
let value = count();

// Read: get reference
let value = count.read();

// Write: get mutable reference
*count.write() = 5;

// Modify in place
count.with_mut(|c| *c += 1);
````

### Context API

Dioxus Context API enables sharing state across the component tree without prop drilling.

**Provider** (in `main.rs`):
````rust
#[component]
fn App() -> Element {
    let app_state = AppState::new();
    use_context_provider(|| app_state);
    
    rsx! { Router::<Route> {} }
}
````

**Consumer** (in any child component):
````rust
#[component]
fn Dashboard() -> Element {
    let app_state = use_context::<AppState>();
    let containers = app_state.containers();
    
    rsx! { "Total: {containers.len()}" }
}
````

## AppState Structure

`src/utils/app_state.rs`

The single source of truth for application state:

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

### State Categories

1. **Docker Resources**: `containers`, `images`, `volumes`
   - Fetched from Docker API
   - Updated via refresh operations

2. **UI State**: `is_loading`, `error_message`, `last_action`
   - Tracks user feedback and loading states
   - Updated during async operations

3. **Configuration**: `docker_host`
   - User settings
   - Persisted across sessions (future)

4. **Services**: `docker_service`
   - Not a signal (doesn't trigger re-renders)
   - Handles Docker API communication

## State Flow

### 1. Initialization

When app launches (`AppState::new()`):

````rust
pub fn new() -> Self {
    let docker_service = DockerService::new().ok();
    
    let state = Self {
        containers: use_signal(Vec::new),
        images: use_signal(Vec::new),
        volumes: use_signal(Vec::new),
        // ... other signals
        docker_service,
    };
    
    // Load initial data
    state.refresh_all();
    
    state
}
````

### 2. Data Refresh

User clicks "Refresh" button:

````rust
pub fn refresh_containers(&self) {
    let service = self.docker_service.clone();
    let mut containers = self.containers.clone();
    let mut error_message = self.error_message.clone();
    
    spawn(async move {
        match service.list_containers().await {
            Ok(data) => {
                containers.set(data);        // ✅ Updates signal
                error_message.set(None);     // ✅ Clears errors
            }
            Err(e) => {
                error_message.set(Some(format!("Error: {}", e)));
            }
        }
    });
}
````

### 3. Reactive Updates

When `containers` signal updates, components automatically re-render:

````rust
#[component]
fn Containers() -> Element {
    let app_state = use_context::<AppState>();
    
    // This component re-renders when containers signal changes
    let containers = app_state.containers();
    
    rsx! {
        for container in containers {
            div { "{container.name}" }
        }
    }
}
````

## Async State Updates

### Pattern: Spawn + Signal Update

````rust
pub fn start_container(&self, id: String) {
    let service = self.docker_service.clone();
    let mut last_action = self.last_action.clone();
    let app_state = self.clone();
    
    spawn(async move {
        match service.start_container(&id).await {
            Ok(_) => {
                last_action.set(Some(format!("Started {}", id)));
                app_state.refresh_containers(); // Refresh to show new state
            }
            Err(e) => {
                // Error handling
            }
        }
    });
}
````

**Key Points**:
1. Clone signals before moving into `async move` block
2. Use `spawn` to run async code without blocking UI
3. Update signals with results
4. Refresh related data after mutations

## Local vs Global State

### Global State (AppState)

Use for data shared across views:
- Docker resources (containers, images, volumes)
- Error messages
- Configuration

### Local State (use_signal)

Use for component-specific state:
````rust
#[component]
fn SearchBar() -> Element {
    let mut query = use_signal(String::new);
    
    rsx! {
        input {
            value: "{query}",
            oninput: move |e| query.set(e.value()),
        }
    }
}
````

## Memoization

Use `use_memo` for derived/computed values:

````rust
#[component]
fn Dashboard() -> Element {
    let app_state = use_context::<AppState>();
    
    // Only recalculates when containers signal changes
    let running_count = use_memo(move || {
        app_state.containers()
            .iter()
            .filter(|c| c.state == ContainerState::Running)
            .count()
    });
    
    rsx! {
        div { "Running: {running_count}" }
    }
}
````

## Error Handling

Error state is managed centrally:

````rust
// Set error
app_state.error_message.set(Some("Failed to connect".to_string()));

// Clear error
app_state.error_message.set(None);

// Display error
if let Some(err) = app_state.error_message() {
    rsx! { div { class: "error", "{err}" } }
}
````

## Best Practices

### ✅ Do

1. **Clone signals before async blocks**
   ````rust
   let mut signal = self.signal.clone();
   spawn(async move {
       signal.set(new_value);
   });
   ````

2. **Use read() for borrowing**
   ````rust
   let value_ref = signal.read();
   process(&value_ref);
   ````

3. **Batch updates**
   ````rust
   pub fn refresh_all(&self) {
       self.refresh_containers();
       self.refresh_images();
       self.refresh_volumes();
   }
   ````

4. **Clear errors on success**
   ````rust
   error_message.set(None);
   ````

### ❌ Don't

1. **Don't move signals directly into async**
   ````rust
   // ❌ Wrong
   spawn(async move {
       self.signal.set(value); // `self` moved
   });
   ````

2. **Don't hold read/write locks across await**
   ````rust
   // ❌ Wrong
   let mut data = signal.write();
   some_async_function().await;
   data.push(item);
   ````

3. **Don't create signals inside loops**
   ````rust
   // ❌ Wrong
   for item in items {
       let signal = use_signal(|| item); // Creates new signal each iteration
   }
   ````

## Debugging State

### Temporary State Logging

Add to components:
````rust
let containers = app_state.containers();
println!("Container count: {}", containers.len());
````

### Effect Hook for Side Effects

````rust
use_effect(move || {
    println!("Containers changed: {:?}", app_state.containers());
});
````

## Future Enhancements

Potential state management improvements:

1. **State Persistence**: Save/restore state to disk
2. **Undo/Redo**: Track state history
3. **Optimistic Updates**: Update UI before API confirmation
4. **State Middleware**: Logging, debugging tools
5. **Filtered Views**: Memoized subsets of data

## Related Resources

- [Dioxus State Management](https://dioxuslabs.com/learn/0.7/state)
- [API Reference](../reference/api.md)
- [Architecture Overview](../reference/architecture.md)

---

**Next**: [Contributing Guide](./contributing.md)
