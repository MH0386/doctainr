# Views API

Page-level components and application layout.

## Module: `views`

**Location**: `src/views/`

### Overview

The views module contains:
- **AppShell** - Main application layout with navigation
- **Page Components** - Individual feature pages (Dashboard, Containers, Images, Volumes, Settings)

All views are routable components that consume `AppState` via context.

---

## Layout Component

### `AppShell`

Main application shell providing navigation and page layout.

**Location**: `src/views/shell.rs`

#### Structure

````
┌─────────────────────────────────────┐
│         Doctainr Desktop            │ Header
├──────────┬──────────────────────────┤
│ Doctainr │ Dashboard                │
│          │                          │
│ Nav      │ Page Content             │
│ Links    │ (Outlet renders here)    │
│          │                          │
└──────────┴──────────────────────────┘
  Sidebar      Main Section
````

#### Features

- **Fixed Sidebar** - Persistent navigation
- **Header Bar** - App title and last action indicator
- **Content Area** - `Outlet<Route>` renders current page
- **Responsive Layout** - Adapts to window size

#### Template

````rust
#[component]
pub fn AppShell() -> Element {
    let app_state = use_context::<AppState>();
    let last_action = (app_state.last_action)();

    rsx! {
        div { class: "app-shell",
            aside { class: "sidebar",
                div { class: "brand", "Doctainr" }
                nav { class: "nav-list",
                    Link { to: Route::Dashboard {}, class: "nav-link", "Dashboard" }
                    Link { to: Route::Containers {}, class: "nav-link", "Containers" }
                    Link { to: Route::Images {}, class: "nav-link", "Images" }
                    Link { to: Route::Volumes {}, class: "nav-link", "Volumes" }
                    Link { to: Route::Settings {}, class: "nav-link", "Settings" }
                }
            }
            section { class: "main",
                header { class: "main-header",
                    div {
                        h1 { class: "app-title", "Doctainr Desktop" }
                        p { class: "app-subtitle", "Local engine workspace" }
                    }
                    if let Some(action) = last_action {
                        div { class: "header-action", "Last action: {action}" }
                    }
                }
                div { class: "content",
                    Outlet::<Route> {}
                }
            }
        }
    }
}
````

#### CSS Classes

| Class | Element | Purpose |
|-------|---------|---------|
| `.app-shell` | Root container | Flexbox layout |
| `.sidebar` | Left panel | Navigation sidebar |
| `.brand` | Logo area | App branding |
| `.nav-list` | Navigation | Link container |
| `.nav-link` | Link items | Styled navigation links |
| `.main` | Right panel | Content area |
| `.main-header` | Top bar | Page header |
| `.content` | Content area | Outlet wrapper |

---

## Page Components

### `Dashboard`

System overview page with metrics and engine status.

**Location**: `src/views/dashboard.rs`  
**Route**: `/`

#### Features

- Container count breakdown (running/stopped)
- Image and volume counts
- Docker engine information
- Global refresh button
- Error message display

#### Data Sources

Reads from `AppState`:
- `containers` - For running/stopped counts
- `images` - For total image count
- `volumes` - For total volume count
- `docker_host` - For engine connection info
- `error_message` - For error display

#### Layout

````rust
rsx! {
    SectionHeader { /* ... */ }
    
    // Error banner (conditional)
    if let Some(error) = error_message {
        div { class: "error-message", "⚠️ {error}" }
    }
    
    // Action bar
    div { class: "action-bar",
        button { /* Refresh All */ }
    }
    
    // Metrics grid
    div { class: "cards",
        MetricCard { title: "Running containers", /* ... */ }
        MetricCard { title: "Stopped containers", /* ... */ }
        MetricCard { title: "Images", /* ... */ }
        MetricCard { title: "Volumes", /* ... */ }
    }
    
    // Engine info card
    div { class: "card",
        h3 { "Engine" }
        p { "Host: {docker_host}" }
        p { "Context: local" }
        p { "Compose: ready" }
    }
}
````

#### Key Calculations

````rust
let running = containers
    .iter()
    .filter(|c| c.state == ContainerState::Running)
    .count();
let stopped = containers.len().saturating_sub(running);
````

---

### `Containers`

Container management interface.

**Location**: `src/views/containers.rs`  
**Route**: `/containers`

#### Features

- Tabular container listing
- Start/stop controls per container
- Status indicators
- Port mapping display
- Refresh button

#### Data Sources

- `AppState::containers` - Container list

#### Layout

````rust
rsx! {
    SectionHeader { /* ... */ }
    
    div { class: "action-bar",
        button { /* Refresh */ }
    }
    
    div { class: "table",
        div { class: "row header",
            span { "Name" }
            span { "Image" }
            span { "Ports" }
            span { "State" }
            span { "Action" }
        }
        
        {containers.iter().map(|container| {
            rsx! {
                div { class: "row item",
                    // Name column with subtitle
                    div {
                        p { class: "row-title", "{name}" }
                        p { class: "row-subtitle", "{status}" }
                    }
                    span { "{image}" }
                    span { "{ports}" }
                    StatusPill { /* ... */ }
                    button { /* Start/Stop */ }
                }
            }
        })}
    }
}
````

#### Interactive Elements

**Toggle Button Logic**:
````rust
let next_state = if container.state == ContainerState::Running {
    ContainerState::Stopped
} else {
    ContainerState::Running
};

button {
    onclick: move |_| app_state.set_container_state(&id, next_state),
    "{button_label}"
}
````

---

### `Images`

Image browser interface.

**Location**: `src/views/images.rs`  
**Route**: `/images`

#### Features

- Tabular image listing
- Repository and tag display
- Image ID and size
- Refresh button

#### Data Sources

- `AppState::images` - Image list

#### Layout

````rust
rsx! {
    SectionHeader {
        title: "Images".to_string(),
        subtitle: Some("Local image cache".to_string())
    }
    
    div { class: "action-bar",
        button { /* Refresh */ }
    }
    
    div { class: "table",
        div { class: "row header",
            span { "Repository" }
            span { "Tag" }
            span { "Image ID" }
            span { "Size" }
        }
        
        {images.iter().map(|image| {
            rsx! {
                div { class: "row item",
                    span { "{repository}" }
                    span { "{tag}" }
                    span { "{id}" }
                    span { "{size}" }
                }
            }
        })}
    }
}
````

---

### `Volumes`

Volume management interface.

**Location**: `src/views/volumes.rs`  
**Route**: `/volumes`

#### Features

- Tabular volume listing
- Driver and mountpoint display
- Size information
- Refresh button

#### Data Sources

- `AppState::volumes` - Volume list

#### Layout

````rust
rsx! {
    SectionHeader {
        title: "Volumes".to_string(),
        subtitle: Some("Persistent storage".to_string())
    }
    
    div { class: "action-bar",
        button { /* Refresh */ }
    }
    
    div { class: "table",
        div { class: "row header",
            span { "Name" }
            span { "Driver" }
            span { "Mountpoint" }
            span { "Size" }
        }
        
        {volumes.iter().map(|volume| {
            rsx! {
                div { class: "row item",
                    span { "{name}" }
                    span { "{driver}" }
                    span { "{mountpoint}" }
                    span { "{size}" }
                }
            }
        })}
    }
}
````

---

### `Settings`

Configuration interface.

**Location**: `src/views/settings.rs`  
**Route**: `/settings`

#### Features

- Docker host configuration
- Connection testing
- Settings persistence
- Input validation

#### Data Sources

- `AppState::docker_host` - Connection string

#### Layout

````rust
rsx! {
    SectionHeader {
        title: "Settings".to_string(),
        subtitle: Some("Connection and preferences".to_string())
    }
    
    div { class: "card",
        label { class: "form-label", "Docker host" }
        input {
            class: "text-input",
            value: docker_host,
            oninput: move |event| docker_host.set(event.value()),
            placeholder: "unix:///var/run/docker.sock"
        }
        div { class: "button-row",
            button { /* Test Connection */ }
            button { /* Save Settings */ }
        }
    }
}
````

#### Interactive Elements

**Input Binding**:
````rust
let mut docker_host = app_state.docker_host.clone();

input {
    value: docker_host,
    oninput: move |event| docker_host.set(event.value())
}
````

---

## Common Patterns

### Context Access

All views access state via context:

````rust
#[component]
pub fn MyView() -> Element {
    let app_state = use_context::<AppState>();
    let data = (app_state.some_signal)();
    
    rsx! { /* ... */ }
}
````

### Signal Reading

Read signals once and reuse:

````rust
// ✅ Efficient
let containers = (app_state.containers)();
let count = containers.len();
let running = containers.iter().filter(|c| c.state == Running).count();

// ❌ Redundant reads
let count = (app_state.containers)().len();
let running = (app_state.containers)().iter()...;
````

### Iteration

Use iterator expressions for dynamic lists:

````rust
{items.iter().map(|item| {
    let id = item.id.clone(); // Clone for closure
    rsx! {
        div {
            onclick: move |_| handle_click(&id),
            "{item.name}"
        }
    }
})}
````

### Conditional Rendering

Use `if let` for optional content:

````rust
if let Some(error) = error_message {
    div { class: "error", "{error}" }
}
````

---

## Routing

### Route Definition

Routes are defined in `src/main.rs`:

````rust
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
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

### Navigation

Use `Link` component for navigation:

````rust
Link { to: Route::Containers {}, "View Containers" }
````

### Programmatic Navigation

Access navigator via context:

````rust
let nav = use_navigator();
nav.push(Route::Dashboard {});
````

---

## Accessibility

### Semantic Structure

- Use `<header>`, `<nav>`, `<main>`, `<section>` for structure
- Table views use `div` with role attributes (consider `<table>` for better semantics)
- Links use `<a>` elements via Dioxus `Link`

### Keyboard Navigation

- All interactive elements are keyboard-accessible
- Tab order follows visual layout
- Links and buttons have focus states

### Screen Readers

````rust
// Add ARIA labels for context
button {
    aria_label: "Refresh container list",
    "Refresh"
}
````

---

## Performance

### Render Optimization

- Views only re-render when consumed signals change
- Iteration uses stable keys (container IDs, image IDs)
- Expensive computations can be memoized:

````rust
let running_count = use_memo(move || {
    (app_state.containers)()
        .iter()
        .filter(|c| c.state == Running)
        .count()
});
````

### Data Fetching

- Initial data loaded on `AppState` creation
- Manual refresh triggered by user action
- No automatic polling (preserves resources)

---

## Error Handling

### Error Display

Dashboard shows errors prominently:

````rust
if let Some(error) = error_message {
    div { class: "error-message", "⚠️ {error}" }
}
````

### Fallback Content

When Docker is unavailable:
- Views still render
- Empty state messaging
- Connection instructions

---

## Styling

### Layout Classes

| Class | Purpose |
|-------|---------|
| `.action-bar` | Button container above content |
| `.cards` | Grid layout for metric cards |
| `.table` | Table-like list layout |
| `.row.header` | Table header row |
| `.row.item` | Table data row |
| `.error-message` | Error banner |

### Responsive Design

Views adapt to window size:
- Desktop: Full sidebar + content
- Tablet: Collapsible sidebar
- Mobile: Bottom navigation (future enhancement)

---

## Testing

### Unit Tests

Test view logic in isolation:

````rust
#[test]
fn dashboard_calculates_running_count() {
    // Verify running container calculation
}
````

### Integration Tests

Test with live data:

````rust
#[test]
fn containers_view_shows_all_containers() {
    // Verify container list displays correctly
}
````

---

## Adding New Views

### Checklist

1. Create file in `src/views/`
2. Define `#[component]` function
3. Add to `src/views/mod.rs`
4. Add route to `Route` enum
5. Add navigation link to `AppShell`
6. Add CSS styles
7. Update documentation

### Template

````rust
use dioxus::prelude::*;
use crate::components::SectionHeader;
use crate::utils::AppState;

#[component]
pub fn NewView() -> Element {
    let app_state = use_context::<AppState>();
    
    rsx! {
        SectionHeader {
            title: "New Feature".to_string(),
            subtitle: Some("Description".to_string())
        }
        
        div { class: "content",
            // Your content here
        }
    }
}
````

---

## See Also

- [Application State](state.md) - State management patterns
- [Components](components.md) - Reusable UI elements
- [Routing Documentation](../architecture/routing.md) - Routing architecture
- [Dioxus Router Guide](https://dioxuslabs.com/learn/0.7/reference/router)
