# State Management in Doctainr

This document provides detailed information about how Doctainr manages application state using Dioxus signals and the Context API.

## Overview

Doctainr uses **reactive signals** for state management. Signals are reactive primitives that automatically track dependencies and trigger re-renders when values change.

## Core Concepts

### What is a Signal?

A `Signal<T>` is a wrapper around a value that provides:
- **Automatic dependency tracking**: Components reading a signal re-render when it changes
- **Interior mutability**: Can be mutated even through shared references
- **Clone cheaply**: Signals are cheap to clone (reference-counted)

````rust
let mut count = use_signal(|| 0);

// Read the value
let current = count();

// Write the value
*count.write() = 1;

// Or use helper methods
count.set(2);
count.with_mut(|val| *val += 1);
````

### Dioxus 0.7 Changes

Doctainr uses Dioxus 0.7, which has major API changes from earlier versions:

**Old (0.6 and earlier)**:
````rust
let count = use_state(cx, || 0);
count.get();
count.set(1);
````

**New (0.7)**:
````rust
let mut count = use_signal(|| 0);
count();  // Read
count.set(1);  // Write
````

Key differences:
- No `cx` (Scope) parameter needed
- `use_signal` instead of `use_state`
- Calling the signal directly reads the value
- Signals are always `Copy` and `Clone`

## AppState Structure

The `AppState` struct holds all global application state:

````rust
pub struct AppState {
    // Docker connection endpoint
    pub docker_host: Signal<String>,
    
    // Resource lists
    pub containers: Signal<Vec<ContainerInfo>>,
    pub images: Signal<Vec<ImageInfo>>,
    pub volumes: Signal<Vec<VolumeInfo>>,
    
    // UI state
    pub last_action: Signal<Option<String>>,
    pub error_message: Signal<Option<String>>,
    pub is_loading: Signal<bool>,
    
    // Service (not a signal - owned by AppState)
    docker_service: Option<DockerService>,
}
````

## State Initialization

State is created once at app startup:

````rust
// In main.rs
#[component]
fn App() -> Element {
    // Create state
    let app_state = AppState::new();
    
    // Provide to entire component tree
    use_context_provider(|| app_state);
    
    rsx! { Router::<Route> {} }
}
````

Inside `AppState::new()`:

````rust
pub fn new() -> Self {
    // Connect to Docker
    let docker_service = DockerService::new().ok();
    
    // Create signals
    let containers = use_signal(Vec::new);
    let images = use_signal(Vec::new);
    // ... more signals
    
    let state = Self {
        docker_host,
        containers,
        images,
        volumes,
        last_action,
        error_message,
        is_loading,
        docker_service,
    };
    
    // Load initial data
    state.refresh_all();
    
    state
}
````

## Consuming State in Components

Components access state via the Context API:

````rust
#[component]
pub fn Dashboard() -> Element {
    // Get AppState from context
    let app_state = use_context::<AppState>();
    
    // Read signal values (triggers re-render on change)
    let containers = app_state.containers();
    let images = app_state.images();
    
    // Use in UI
    rsx! {
        p { "Containers: {containers.len()}" }
        p { "Images: {images.len()}" }
    }
}
````

**Important**: Calling `app_state.containers()` reads the signal. If the signal changes, this component re-renders.

## Updating State

### Pattern 1: Direct Update

For simple state updates:

````rust
let mut error_message = app_state.error_message;
error_message.set(Some("Error occurred".to_string()));
````

### Pattern 2: Async Update

For updates from async operations:

````rust
pub fn refresh_containers(&self) {
    let service = self.docker_service.clone();
    let mut containers = self.containers.clone();
    let mut error_message = self.error_message.clone();
    
    spawn(async move {
        match service.list_containers().await {
            Ok(data) => containers.set(data),
            Err(e) => error_message.set(Some(e.to_string())),
        }
    });
}
````

**Why clone signals?**: 
- Signals are cheap to clone (they're just reference-counted pointers)
- The `async` block needs to own the signals to move them across threads

### Pattern 3: Mutation with Closure

For complex updates:

````rust
count.with_mut(|val| {
    *val += 1;
    println!("New value: {}", *val);
});
````

## State Flow Examples

### Example 1: Loading Containers

1. User opens Containers view
2. Component mounts, reads `app_state.containers()`
3. Component calls `app_state.refresh_containers()`
4. `refresh_containers()` spawns async task:
   - Clones `containers` and `error_message` signals
   - Calls Docker API
   - Updates signals with results
5. Signal update triggers component re-render
6. UI shows updated container list

### Example 2: Starting a Container

1. User clicks "Start" button
2. Button's `onclick` handler calls `app_state.start_container(id)`
3. `start_container()` spawns async task:
   - Clones signals for capture
   - Calls `docker_service.start_container(&id)`
   - On success:
     - Updates `last_action` signal
     - Calls `refresh_containers()` to update list
   - On error:
     - Updates `error_message` signal
4. Signal updates trigger re-renders:
   - Dashboard shows updated metrics
   - Containers view shows updated status
   - Error banner appears if there was an error

## Memoization

For expensive computations, use `use_memo`:

````rust
let containers = app_state.containers();

// This only recomputes when containers changes
let running_count = use_memo(move || {
    containers
        .iter()
        .filter(|c| c.state == ContainerState::Running)
        .count()
});

rsx! {
    p { "Running: {running_count}" }
}
````

**Benefits**:
- Computation only runs when dependencies change
- Multiple components can read the memo without recomputing

## Error Handling Pattern

All state-changing operations follow this pattern:

````rust
pub fn some_operation(&self) {
    if let Some(service) = &self.docker_service {
        let service = service.clone();
        let mut error_message = self.error_message.clone();
        
        spawn(async move {
            match service.do_something().await {
                Ok(_) => {
                    error_message.set(None); // Clear errors
                    // Update other signals as needed
                }
                Err(e) => {
                    error_message.set(Some(format!("Failed: {}", e)));
                }
            }
        });
    } else {
        self.error_message.set(Some("Docker not available".into()));
    }
}
````

This ensures:
- Errors are always captured and displayed
- Previous errors are cleared on success
- Missing Docker service is handled gracefully

## Best Practices

### 1. Clone Signals for Async

Always clone signals before moving into async blocks:

````rust
let mut signal = self.some_signal.clone();
spawn(async move {
    signal.set(new_value);
});
````

### 2. Read Signals Sparingly

Only read signals in components that need to re-render:

````rust
// ❌ Bad: Re-renders on any container change
let containers = app_state.containers();
let count = containers.len();

// ✅ Better: Use memo to isolate
let count = use_memo(move || app_state.containers().len());
````

### 3. Batch Updates

Update multiple signals together to avoid multiple re-renders:

````rust
// Do this in a single async block
spawn(async move {
    containers.set(new_containers);
    images.set(new_images);
    volumes.set(new_volumes);
    is_loading.set(false);
});
````

### 4. Clear Errors on Success

Always clear error messages when operations succeed:

````rust
match result {
    Ok(data) => {
        signal.set(data);
        error_message.set(None); // Important!
    }
    Err(e) => {
        error_message.set(Some(e.to_string()));
    }
}
````

## State Persistence

Currently, Doctainr does **not** persist state:
- All state is in-memory
- Closing the app loses all state
- No window position/size saving

**Future enhancement**: Persist preferences using platform-specific storage.

## Thread Safety

Signals are thread-safe:
- Can be sent across threads (`Send`)
- Can be shared between threads (`Sync`)
- Updates are atomic
- No data races possible (enforced by Rust)

## Related Documentation

- [Architecture Overview](architecture.md) - How state fits into overall architecture
- [Design Principles](../explanation/design-principles.md) - Why these state patterns
- [Dioxus Documentation](https://dioxuslabs.com/learn/0.7/reference/hooks) - Official Dioxus hooks reference
