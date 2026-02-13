# Routing Architecture

Navigation and routing implementation using Dioxus Router.

## Overview

Doctainr uses Dioxus Router for client-side routing with a layout-based architecture. All routes share a common `AppShell` layout that provides navigation and consistent structure.

## Route Definition

Routes are defined using a Rust enum with the `Routable` derive macro.

**Location**: `src/main.rs`

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

### Attributes

| Attribute | Purpose | Example |
|-----------|---------|---------|
| `#[layout(AppShell)]` | Wraps routes with layout | All routes share sidebar |
| `#[route("/path")]` | URL pattern | `#[route("/containers")]` |
| Enum variant | Component to render | `Dashboard {}` |

---

## Route Hierarchy

````
AppShell (Layout)
├── / (Dashboard)
├── /containers (Containers)
├── /images (Images)
├── /volumes (Volumes)
└── /settings (Settings)
````

All routes are flat (no nested routes). The `#[layout(AppShell)]` attribute wraps all routes with the shared navigation and header.

---

## Router Initialization

The router is initialized in the root `App` component.

````rust
#[component]
fn App() -> Element {
    let app_state = AppState::new();
    use_context_provider(|| app_state);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        
        Router::<Route> {}
    }
}
````

**Process**:
1. App state initialized and provided via context
2. Document head elements added (favicon, CSS)
3. Router component renders based on current URL

---

## Layout Component

### AppShell Structure

The `AppShell` layout provides consistent structure across all routes:

````rust
#[component]
pub fn AppShell() -> Element {
    let app_state = use_context::<AppState>();
    let last_action = (app_state.last_action)();

    rsx! {
        div { class: "app-shell",
            // Sidebar navigation
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
            
            // Main content area
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
                    // Child route renders here
                    Outlet::<Route> {}
                }
            }
        }
    }
}
````

### Outlet Component

The `Outlet::<Route>` component is where child routes render:

- Dashboard at `/` renders inside the outlet
- Containers at `/containers` renders inside the outlet
- And so on...

**Key Point**: The sidebar and header remain constant; only the outlet content changes.

---

## Navigation

### Declarative Navigation (Links)

Use the `Link` component for declarative navigation:

````rust
Link { 
    to: Route::Containers {}, 
    class: "nav-link", 
    "Containers" 
}
````

**Behavior**:
- Renders as `<a>` tag with `href`
- Prevents page reload (client-side navigation)
- Updates browser history
- Active link styling (via CSS)

### Programmatic Navigation

Use the `Navigator` for programmatic navigation:

````rust
let nav = use_navigator();

button {
    onclick: move |_| nav.push(Route::Dashboard {}),
    "Go to Dashboard"
}
````

**Methods**:
- `nav.push(route)` - Navigate to route (adds history entry)
- `nav.replace(route)` - Navigate without history entry
- `nav.go_back()` - Browser back
- `nav.go_forward()` - Browser forward

---

## Route Parameters

### Current Implementation

Doctainr uses parameter-less routes:

````rust
#[route("/containers")]
Containers {},
````

No dynamic segments (e.g., `/containers/:id`).

### Future Enhancement

For detail views:

````rust
#[route("/containers/:id")]
ContainerDetail { id: String },
````

**Usage**:
````rust
#[component]
fn ContainerDetail(id: String) -> Element {
    rsx! { "Container ID: {id}" }
}
````

---

## State Preservation

### Across Route Changes

Application state persists across navigation:

````rust
// State provided at app root
use_context_provider(|| AppState::new());

// Accessible in all routes
let app_state = use_context::<AppState>();
````

**Result**: Navigating between pages preserves:
- Container list
- Image list
- Volume list
- Error messages
- Loading state

### Component Lifecycle

When navigating away from a route:
- Component is unmounted
- Local state (non-context) is lost
- Context state persists

**Example**:
````rust
#[component]
fn Dashboard() -> Element {
    let local_state = use_signal(|| 0); // Lost on unmount
    let app_state = use_context::<AppState>(); // Persists
    
    // ...
}
````

---

## Active Link Styling

### CSS-Based

The router adds `aria-current="page"` to active links:

````html
<!-- Inactive -->
<a href="/containers" class="nav-link">Containers</a>

<!-- Active -->
<a href="/containers" class="nav-link" aria-current="page">Containers</a>
````

**CSS**:
````css
.nav-link {
    color: #666;
}

.nav-link[aria-current="page"] {
    color: #000;
    font-weight: 600;
    background: #f0f0f0;
}
````

---

## URL Structure

### Desktop Application

Dioxus desktop uses internal routing (no actual URL bar):

- Routes work as expected
- Browser history API functions
- No server-side routing needed

### Web Deployment

For web target, routes map to browser URLs:

| Route | URL |
|-------|-----|
| `Dashboard` | `https://example.com/` |
| `Containers` | `https://example.com/containers` |
| `Images` | `https://example.com/images` |

**Deployment Consideration**: Requires server configuration for SPA routing (e.g., rewrite all routes to `/index.html`).

---

## Navigation Flow

### User Clicks Link

````
1. User clicks "Containers" link
   ↓
2. Dioxus Router intercepts click
   ↓
3. Updates internal route state
   ↓
4. Unmounts Dashboard component
   ↓
5. Mounts Containers component inside Outlet
   ↓
6. AppShell sidebar remains mounted (persistent)
   ↓
7. Browser history updated
````

### Programmatic Navigation

````
1. Code calls nav.push(Route::Settings {})
   ↓
2. Router updates route state
   ↓
3. Outlet re-renders with Settings component
   ↓
4. Browser history updated
````

---

## Error Handling

### 404 - Not Found

**Current**: No catch-all route defined.

**Future Enhancement**:
````rust
enum Route {
    // ... existing routes
    
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
````

### Invalid Routes

If a route is manually typed (desktop doesn't have URL bar, but web does):
- Router fails to match
- Renders nothing (blank screen)
- Console error logged

**Best Practice**: Always define a catch-all for production.

---

## Route Guards

### Authentication (Future)

For authenticated routes:

````rust
#[component]
fn ProtectedRoute() -> Element {
    let auth = use_context::<AuthState>();
    
    if !auth.is_authenticated() {
        return rsx! { Redirect { to: Route::Login {} } };
    }
    
    rsx! { /* protected content */ }
}
````

**Note**: Doctainr currently has no authentication (local app).

---

## Performance Considerations

### Lazy Loading

Routes are not code-split in Rust (all code bundled):

- Desktop: No lazy loading needed (single binary)
- Web: Consider WASM optimization for smaller initial load

### Route Transitions

Navigation is instant (no loading states):

- No network requests for routes
- All components compiled into binary
- State persists across navigation

---

## Debugging

### Route State

Log current route:

````rust
let route = use_route::<Route>();
println!("Current route: {:?}", route);
````

### Navigation Events

Track navigation:

````rust
use_effect(move || {
    let route = use_route::<Route>();
    println!("Navigated to: {:?}", route);
});
````

---

## Best Practices

### ✅ Do

- Use `Link` for navigation (declarative)
- Define all routes in single enum
- Use layouts for shared UI
- Provide state at app root
- Use semantic route names

### ❌ Avoid

- Manual URL manipulation
- Storing route state in local component state
- Creating separate route enums
- Duplicating navigation in multiple places

---

## Testing

### Route Matching

````rust
#[test]
fn test_route_parsing() {
    let route = Route::from_str("/containers");
    assert_eq!(route, Route::Containers {});
}
````

### Navigation

````rust
#[test]
fn test_navigation() {
    // Integration test with router
    // Verify component mounts/unmounts
}
````

---

## Future Enhancements

### Nested Routes

For complex features:

````rust
#[route("/containers")]
#[layout(ContainerLayout)]
    #[route("/")]
    ContainerList {},
    #[route("/:id")]
    ContainerDetail { id: String },
````

### Route Transitions

Animated transitions between routes:

````rust
rsx! {
    Router::<Route> {
        config: RouterConfig {
            transition: Transition::Fade,
        }
    }
}
````

### Query Parameters

````rust
#[route("/search?q=:query")]
Search { query: String },
````

---

## See Also

- [Views Documentation](../api/views.md) - Page components
- [Application State](../api/state.md) - State management
- [Dioxus Router Guide](https://dioxuslabs.com/learn/0.7/reference/router)
- [System Overview](overview.md) - Architecture context
