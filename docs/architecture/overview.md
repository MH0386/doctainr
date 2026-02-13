# System Architecture

High-level overview of Doctainr's architecture and design decisions.

## Overview

Doctainr is a Docker desktop application built with:
- **Rust** - Systems programming language for performance and safety
- **Dioxus 0.7** - Reactive UI framework with cross-platform support
- **Bollard** - Asynchronous Docker API client
- **Tokio** - Async runtime for concurrent operations

## Architecture Diagram

````
┌─────────────────────────────────────────────────────────┐
│                    User Interface                       │
│  (Dioxus Components - Views, Components, AppShell)     │
└────────────────────┬────────────────────────────────────┘
                     │ Signals (Reactive State)
                     ↓
┌─────────────────────────────────────────────────────────┐
│                   Application State                     │
│      (AppState - Containers, Images, Volumes)          │
└────────────────────┬────────────────────────────────────┘
                     │ Async Calls
                     ↓
┌─────────────────────────────────────────────────────────┐
│                  Service Layer                          │
│        (DockerService - Bollard Wrapper)               │
└────────────────────┬────────────────────────────────────┘
                     │ HTTP/Unix Socket
                     ↓
┌─────────────────────────────────────────────────────────┐
│              Docker Daemon API                          │
│    (Local Docker Engine via Socket/Named Pipe)         │
└─────────────────────────────────────────────────────────┘
````

## Architectural Layers

### 1. Presentation Layer (UI)

**Components**: `src/views/`, `src/components/`  
**Responsibility**: User interface and interaction

- **Views** - Page-level components (Dashboard, Containers, Images, etc.)
- **Components** - Reusable UI elements (MetricCard, StatusPill, etc.)
- **AppShell** - Layout and navigation wrapper

**Key Characteristics**:
- Declarative UI using RSX macros
- Pure functions (components don't manage state directly)
- Reactive re-rendering via signal subscriptions
- Zero business logic (delegates to AppState)

### 2. State Management Layer

**Components**: `src/utils/app_state.rs`  
**Responsibility**: Application state and reactive data flow

- Manages all application data (containers, images, volumes)
- Provides methods for data refresh and operations
- Handles error state and loading indicators
- Bridges UI and service layers

**Key Characteristics**:
- Uses Dioxus signals for reactivity
- Spawns async tasks for non-blocking operations
- Provides context API for global state access
- Implements observer pattern (signal subscriptions)

### 3. Service Layer

**Components**: `src/services/docker.rs`  
**Responsibility**: Docker API integration

- Wraps Bollard Docker client
- Provides simplified API for Docker operations
- Transforms Docker API responses into app-specific types
- Handles connection management

**Key Characteristics**:
- Async/await interface
- Type-safe error handling (Result types)
- Domain-specific data structures
- Abstraction over Docker complexity

### 4. External Integration

**Components**: Bollard, Docker Daemon  
**Responsibility**: Docker engine communication

- Bollard library handles HTTP/Socket communication
- Docker daemon exposes REST-like API
- Platform-specific socket paths (Unix vs Windows)

---

## Design Patterns

### Pattern 1: Signal-Based Reactivity

**Problem**: Keep UI synchronized with asynchronous data changes.

**Solution**: Use Dioxus signals for automatic dependency tracking.

````rust
// State layer
pub struct AppState {
    pub containers: Signal<Vec<ContainerInfo>>,
}

// UI layer (automatically re-renders when signal changes)
let containers = (app_state.containers)();
rsx! { "Count: {containers.len()}" }
````

**Benefits**:
- Eliminates manual re-render logic
- Prevents stale data display
- Simplifies component code

### Pattern 2: Context Provider

**Problem**: Avoid prop drilling for globally shared state.

**Solution**: Use Dioxus context API.

````rust
// App root
let app_state = AppState::new();
use_context_provider(|| app_state);

// Any child component
let app_state = use_context::<AppState>();
````

**Benefits**:
- Clean component interfaces
- Easy state access anywhere
- Type-safe (compiler-enforced)

### Pattern 3: Async Spawn Pattern

**Problem**: Keep UI responsive during slow operations.

**Solution**: Spawn background tasks, update signals on completion.

````rust
pub fn refresh_containers(&self) {
    let service = self.docker_service.clone();
    let mut containers = self.containers.clone();
    
    spawn(async move {
        match service.list_containers().await {
            Ok(data) => containers.set(data),
            Err(e) => /* handle error */,
        }
    });
}
````

**Benefits**:
- Non-blocking UI
- Automatic re-render on completion
- Error isolation

### Pattern 4: Repository Pattern

**Problem**: Separate data access logic from business logic.

**Solution**: Service layer encapsulates all Docker API calls.

````rust
// Service (repository)
impl DockerService {
    pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>> {
        // Implementation details hidden
    }
}

// State (consumer)
let containers = service.list_containers().await?;
````

**Benefits**:
- Testability (can mock service)
- Single source of truth for Docker operations
- Easy to swap implementations

---

## Data Flow

### Read Flow (Display Data)

````
1. Component reads signal:
   let containers = (app_state.containers)();

2. Signal returns current value:
   Vec<ContainerInfo>

3. Component renders with data:
   rsx! { "Total: {containers.len()}" }

4. Component subscribes to signal (automatic)
````

### Write Flow (Update Data)

````
1. User clicks "Refresh" button:
   button { onclick: move |_| app_state.refresh_containers() }

2. AppState spawns async task:
   spawn(async move { /* ... */ })

3. DockerService calls Docker API:
   service.list_containers().await

4. Signal updated with new data:
   containers.set(new_data)

5. Subscribed components automatically re-render
````

### Error Flow

````
1. Service operation fails:
   Err(e) in async task

2. AppState sets error signal:
   error_message.set(Some(error_string))

3. Dashboard shows error banner:
   if let Some(err) = error { /* display */ }

4. Next successful operation clears error:
   error_message.set(None)
````

---

## Technology Choices

### Why Rust?

**Rationale**:
- Memory safety without garbage collection
- Zero-cost abstractions
- Excellent async support (Tokio)
- Cross-platform compilation
- Strong type system prevents bugs

**Trade-offs**:
- Steeper learning curve
- Longer compilation times
- Smaller ecosystem than Node.js

### Why Dioxus 0.7?

**Rationale**:
- Native Rust (no JS bridge)
- Familiar React-like API (RSX)
- Cross-platform (desktop, web, mobile)
- Signal-based reactivity (simple mental model)
- Growing ecosystem

**Trade-offs**:
- Younger framework (evolving APIs)
- Smaller community than Electron/Tauri
- Limited third-party component libraries

### Why Bollard?

**Rationale**:
- Pure Rust Docker client
- Async/await support
- Actively maintained
- Feature-complete API coverage

**Trade-offs**:
- Verbose API (requires wrapper layer)
- Error handling complexity
- Not as mature as Docker CLI

---

## Deployment Architecture

### Desktop Application

````
┌──────────────────────────────────┐
│   Doctainr Binary                │
│   ├── Embedded Assets            │
│   ├── Dioxus Runtime             │
│   └── WebView Engine             │
└──────────────────┬───────────────┘
                   │ Socket/Pipe
┌──────────────────▼───────────────┐
│   Docker Daemon                  │
│   (Local System Service)         │
└──────────────────────────────────┘
````

**Distribution**:
- Single executable (no installer required)
- Statically linked (no runtime dependencies)
- Platform-specific builds (Linux, macOS, Windows)

**Requirements**:
- Docker daemon running locally
- Socket/pipe accessible to user
- WebView system library (platform-provided)

---

## Security Considerations

### Docker Socket Access

**Risk**: Docker socket provides root-equivalent access.

**Mitigation**:
- Application runs with user privileges (not root)
- Socket permissions enforced by OS
- Read operations preferred over write operations
- No credential storage in application

### Input Validation

**Risk**: Malicious container IDs or names.

**Mitigation**:
- Docker API handles validation
- Application uses IDs from trusted source (Docker daemon)
- No user-provided IDs directly executed

### Error Information Disclosure

**Risk**: Error messages reveal system information.

**Mitigation**:
- Generic error messages in production builds
- Detailed errors logged (not displayed)
- No stack traces exposed to UI

---

## Performance Characteristics

### Startup Performance

- **Cold Start**: ~100-200ms (Rust binary launch)
- **Initial Data Load**: ~50-500ms (depends on Docker resource count)
- **UI Ready**: Immediate (progressive enhancement)

### Runtime Performance

- **Memory Usage**: ~10-50 MB (typical desktop app)
- **CPU Usage**: <1% idle, 2-5% during refresh
- **Network**: Local socket (negligible latency)

### Optimization Strategies

1. **Lazy Loading**: Load data only when viewed
2. **Caching**: Store frequently accessed data
3. **Debouncing**: Rate-limit refresh operations
4. **Pagination**: Future enhancement for large datasets

---

## Scalability

### Current Limitations

- Designed for single local Docker engine
- No support for remote Docker hosts (via SSH/TLS)
- UI may slow with 1000+ containers (not paginated)

### Future Enhancements

- Multi-host support
- Virtual scrolling for large lists
- Background sync with change notifications
- Docker Compose integration

---

## Testing Strategy

### Unit Tests

- Service layer data transformations
- State management logic
- Helper functions

### Integration Tests

- Docker API integration
- End-to-end data flow
- Error handling scenarios

### Manual Testing

- UI interactions
- Cross-platform compatibility
- Performance profiling

---

## Maintenance Considerations

### Dependency Management

- Regular updates via Cargo.toml
- Security audits via `cargo audit`
- Breaking change reviews (especially Dioxus)

### Code Quality

- Linting via `cargo clippy`
- Formatting via `cargo fmt`
- Type safety via Rust compiler

### Documentation

- Inline code documentation (rustdoc)
- Architecture documentation (this file)
- API reference documentation

---

## See Also

- [Dioxus Patterns](dioxus-patterns.md) - Framework-specific patterns
- [Docker Integration](docker-integration.md) - Bollard usage details
- [Routing Architecture](routing.md) - Navigation implementation
- [API Reference](../api/README.md) - Detailed API documentation
