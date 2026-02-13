# Architecture

This document describes the architecture and design patterns used in Doctainr.

## Overview

Doctainr follows a component-based architecture using Dioxus 0.7's reactive paradigms. The application is structured into distinct layers for separation of concerns and maintainability.

## Application Structure

````
src/
├── main.rs              # Application entry point and routing
├── components/          # Reusable UI components
│   ├── metric_card.rs   # Dashboard metric display
│   ├── section_header.rs # Section headers
│   └── status_pill.rs   # Status indicators
├── services/            # Business logic and external integrations
│   └── docker.rs        # Docker API client wrapper
├── utils/              # Shared utilities and state
│   └── app_state.rs     # Global application state
└── views/              # Page components
    ├── dashboard.rs     # Dashboard view
    ├── containers.rs    # Container management
    ├── images.rs        # Image management
    ├── volumes.rs       # Volume management
    ├── settings.rs      # Settings page
    └── mod.rs          # Layout component (AppShell)
````

## Core Concepts

### 1. Routing

Doctainr uses Dioxus's built-in router with the `Routable` derive macro:

````rust
#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(AppShell)]
        #[route("/")]
        Dashboard {},
        #[route("/containers")]
        Containers {},
        // ...
}
````

**Key Points:**
- All routes share the `AppShell` layout for consistent navigation
- Routes are type-safe and compile-time checked
- URL changes trigger reactive component updates

### 2. State Management

The application uses Dioxus 0.7's `Signal` API for reactive state management:

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

**Signal Characteristics:**
- Automatically track dependencies
- Trigger re-renders only for components that read them
- Can be read with `signal()` or `signal.read()`
- Can be written with `*signal.write()` or `signal.with_mut(|val| ...)`

**Context Provider Pattern:**
The global `AppState` is provided at the app root and consumed by child components:

````rust
// In App component
fn App() -> Element {
    let app_state = AppState::new();
    use_context_provider(|| app_state);
    // ...
}

// In child component
fn Dashboard() -> Element {
    let mut app_state = use_context::<AppState>();
    // Access state...
}
````

### 3. Docker Service Layer

The `DockerService` struct wraps the Bollard Docker client and provides high-level operations:

````rust
pub struct DockerService {
    client: Docker,
}

impl DockerService {
    pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>>
    pub async fn start_container(&self, id: &str) -> Result<()>
    pub async fn stop_container(&self, id: &str) -> Result<()>
    // ...
}
````

**Design Decisions:**
- Async operations using Tokio runtime
- Error handling with `anyhow::Result`
- Transformation from Bollard types to domain models
- Encapsulation of Docker API complexity

### 4. Component Design

Components follow Dioxus 0.7 conventions:

````rust
#[component]
fn MetricCard(title: String, value: String, icon: String) -> Element {
    rsx! {
        div { class: "metric-card",
            // Component content
        }
    }
}
````

**Component Guidelines:**
- Use `#[component]` macro for all components
- Props must be owned types (not references)
- Props must implement `PartialEq` and `Clone`
- Use `ReadOnlySignal<T>` for reactive props

### 5. Async Operations

The application uses Dioxus's `use_resource` hook for async data fetching:

````rust
let containers_resource = use_resource(move || async move {
    app_state.refresh_containers().await
});

match containers_resource() {
    Some(_) => rsx! { /* Display containers */ },
    None => rsx! { "Loading..." },
}
````

**Resource Pattern:**
- Automatically manages loading states
- Re-runs when dependencies change
- Handles async errors gracefully

## Data Flow

1. **User Action** → Component event handler
2. **Event Handler** → Update Signal or call service method
3. **Signal Update** → Triggers reactive re-render
4. **Service Call** → Async Docker operation
5. **Result** → Update Signal with new data
6. **Re-render** → UI reflects new state

## Error Handling

Errors are managed at multiple levels:

1. **Service Layer**: Returns `Result<T, anyhow::Error>`
2. **App State**: Stores errors in `error_message` Signal
3. **UI Layer**: Displays error messages to users

## Performance Considerations

- **Fine-grained Reactivity**: Only components reading changed Signals re-render
- **Async Runtime**: Non-blocking Docker operations
- **Efficient Updates**: Signal writes batch updates automatically
- **Memoization**: Use `use_memo` for expensive computations

## Future Architecture Improvements

1. **State Persistence**: Save user preferences to disk
2. **WebSocket Integration**: Real-time Docker events
3. **Plugin System**: Extensible architecture for custom features
4. **Testing**: Comprehensive unit and integration tests
5. **Error Recovery**: Automatic reconnection to Docker daemon

## References

- [Dioxus 0.7 Documentation](https://dioxuslabs.com/learn/0.7)
- [Bollard Documentation](https://docs.rs/bollard)
- [Tokio Documentation](https://tokio.rs)
