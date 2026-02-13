# Developer Guide

This guide helps developers understand how to work with the Containr codebase.

## Getting Started

### Prerequisites

- **Rust**: Install via [rustup](https://rustup.rs/)
- **Docker**: Must be running locally
- **Dioxus CLI** (optional): `cargo install dioxus-cli`
- **Code editor**: VS Code with rust-analyzer recommended

### Development Environment Setup

1. **Clone the repository:**
   ``````bash
   git clone https://github.com/MH0386/containr.git
   cd containr
   ``````

2. **Install dependencies:**
   ``````bash
   cargo fetch
   ``````

3. **Run the application:**
   ``````bash
   # Using cargo
   cargo run

   # Or using Dioxus CLI (recommended for hot reload)
   dx serve --platform desktop
   ``````

4. **Run tests:**
   ``````bash
   cargo test
   ``````

5. **Run linter:**
   ``````bash
   cargo clippy
   ``````

## Project Structure

``````
containr/
├── assets/                 # Static assets (images, styles)
│   ├── favicon.ico
│   └── styling/
│       ├── main.css       # Primary stylesheet
│       ├── navbar.css
│       ├── blog.css
│       └── echo.css
├── src/
│   ├── main.rs            # Entry point, routing
│   ├── components/        # Reusable UI components
│   │   ├── mod.rs
│   │   ├── metric_card.rs
│   │   ├── section_header.rs
│   │   └── status_pill.rs
│   ├── services/          # Business logic
│   │   ├── mod.rs
│   │   └── docker.rs      # Docker integration
│   ├── utils/             # Shared utilities
│   │   ├── mod.rs
│   │   └── app_state.rs   # Global state
│   └── views/             # Page components
│       ├── mod.rs
│       ├── dashboard.rs
│       ├── containers.rs
│       ├── images.rs
│       ├── volumes.rs
│       ├── settings.rs
│       └── shell.rs       # App layout
├── Cargo.toml             # Dependencies
├── Dioxus.toml           # Dioxus configuration
└── clippy.toml           # Linter settings
``````

## Architecture

### Component Hierarchy

``````
App (main.rs)
└── Router<Route>
    └── AppShell (views/shell.rs)
        ├── Sidebar Navigation
        └── Outlet<Route>
            ├── Dashboard
            ├── Containers
            ├── Images
            ├── Volumes
            └── Settings
``````

### State Management

Containr uses Dioxus signals for reactive state management:

1. **Global State**: `AppState` provided via Context API
2. **Local State**: Component-specific `use_signal` hooks
3. **Derived State**: `use_memo` for computed values

**Example:**

``````rust
// Providing global state (in App)
let app_state = AppState::new();
use_context_provider(|| app_state);

// Consuming state (in child component)
let app_state = use_context::<AppState>();
let containers = (app_state.containers)();

// Local component state
let mut selected = use_signal(|| None);
``````

### Routing

Routes are defined as an enum with the `Routable` derive:

``````rust
#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(AppShell)]
        #[route("/")]
        Dashboard {},
        #[route("/containers")]
        Containers {},
    // ...
}
``````

Navigation uses the `Link` component:

``````rust
Link { to: Route::Dashboard {}, "Dashboard" }
``````

## Common Development Tasks

### Adding a New Page

1. **Define the route** in `src/main.rs`:
   ``````rust
   #[derive(Debug, Clone, Routable, PartialEq)]
   enum Route {
       #[layout(AppShell)]
           // ... existing routes
           #[route("/networks")]
           Networks {},
   }
   ``````

2. **Create the view component** in `src/views/networks.rs`:
   ``````rust
   use dioxus::prelude::*;

   #[component]
   pub fn Networks() -> Element {
       rsx! {
           h1 { "Networks" }
           // Your UI here
       }
   }
   ``````

3. **Export in** `src/views/mod.rs`:
   ``````rust
   mod networks;
   pub use networks::*;
   ``````

4. **Add navigation link** in `src/views/shell.rs`:
   ``````rust
   Link { to: Route::Networks {}, class: "nav-link", "Networks" }
   ``````

### Creating a New Component

1. **Create the component file** in `src/components/your_component.rs`:
   ``````rust
   use dioxus::prelude::*;

   #[component]
   pub fn YourComponent(title: String, value: String) -> Element {
       rsx! {
           div { class: "your-component",
               h3 { "{title}" }
               p { "{value}" }
           }
       }
   }
   ``````

2. **Export in** `src/components/mod.rs`:
   ``````rust
   mod your_component;
   pub use your_component::*;
   ``````

3. **Use the component**:
   ``````rust
   use crate::components::YourComponent;

   rsx! {
       YourComponent {
           title: "Example".to_string(),
           value: "Data".to_string()
       }
   }
   ``````

### Adding State to AppState

1. **Update** `src/utils/app_state.rs`:
   ``````rust
   #[derive(Clone)]
   pub struct AppState {
       // ... existing fields
       pub new_field: Signal<YourType>,
   }

   impl AppState {
       pub fn new() -> Self {
           // ... existing initialization
           let new_field = use_signal(|| YourType::default());

           Self {
               // ... existing fields
               new_field,
           }
       }
   }
   ``````

2. **Access in components**:
   ``````rust
   let app_state = use_context::<AppState>();
   let value = (app_state.new_field)();
   ``````

### Styling Components

1. **Add CSS classes** in `assets/styling/main.css`:
   ``````css
   .your-class {
       background-color: #1c2230;
       padding: 16px;
       border-radius: 8px;
   }
   ``````

2. **Use in components**:
   ``````rust
   rsx! {
       div { class: "your-class",
           "Content"
       }
   }
   ``````

### Working with Docker API

Currently, the app uses mock data. To integrate real Docker API:

1. **Add Docker client dependency** to `Cargo.toml`:
   ``````toml
   bollard = "0.17"
   ``````

2. **Update** `src/services/docker.rs`:
   ``````rust
   use bollard::Docker;

   pub async fn fetch_containers() -> Result<Vec<ContainerInfo>, Error> {
       let docker = Docker::connect_with_socket_defaults()?;
       // Implement Docker API calls
   }
   ``````

3. **Use in components with** `use_resource`:
   ``````rust
   let containers = use_resource(move || async move {
       fetch_containers().await
   });

   match containers() {
       Some(Ok(data)) => rsx! { /* render data */ },
       Some(Err(e)) => rsx! { "Error: {e}" },
       None => rsx! { "Loading..." },
   }
   ``````

## Testing

### Unit Tests

Add tests in the same file as your code:

``````rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_state_label() {
        assert_eq!(ContainerState::Running.label(), "Running");
    }
}
``````

Run tests:
``````bash
cargo test
``````

### Integration Tests

Create tests in `tests/` directory:

``````rust
// tests/integration_test.rs
use containr::services::*;

#[test]
fn test_mock_data() {
    let containers = mock_containers();
    assert!(!containers.is_empty());
}
``````

## Code Style

### Formatting

Use `rustfmt` for consistent formatting:

``````bash
cargo fmt
``````

### Linting

The project uses `clippy` with custom rules in `clippy.toml`:

``````bash
cargo clippy -- -D warnings
``````

### Conventions

- **Naming**: Use snake_case for functions/variables, PascalCase for types/components
- **Components**: Start with capital letter or underscore, annotate with `#[component]`
- **Props**: Use owned types (String, not &str; Vec<T>, not &[T])
- **State**: Prefer signals over refs for reactive values
- **Comments**: Document public APIs and complex logic

## Debugging

### Desktop App Debugging

Use standard Rust debugging tools:

``````bash
RUST_LOG=debug cargo run
``````

Or with VS Code launch configuration:

``````json
{
    "type": "lldb",
    "request": "launch",
    "name": "Debug Containr",
    "cargo": {
        "args": ["build", "--bin=containr"]
    },
    "args": [],
    "cwd": "${workspaceFolder}"
}
``````

### Dioxus Debugging

Enable Dioxus logging:

``````rust
dioxus::logger::init(Level::DEBUG).unwrap();
``````

## Performance

### Optimization Tips

1. **Minimize state reads**: Only read signals when needed
2. **Use memos**: Cache expensive computations with `use_memo`
3. **Avoid clones**: Use references where possible
4. **Batch updates**: Update multiple signals together
5. **Profile**: Use `cargo flamegraph` for profiling

### Build Optimization

Release builds are optimized by default:

``````bash
cargo build --release
``````

For smaller binaries, add to `Cargo.toml`:

``````toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
``````

## Contributing

### Workflow

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make changes and commit: `git commit -m "Add feature"`
4. Run tests and lints: `cargo test && cargo clippy`
5. Format code: `cargo fmt`
6. Push: `git push origin feature/my-feature`
7. Open a Pull Request

### PR Guidelines

- Clear title and description
- Tests for new features
- Update documentation
- Pass CI checks (tests, clippy, fmt)
- Reference related issues

## Resources

- [Dioxus Documentation](https://dioxuslabs.com/learn/0.7)
- [Dioxus Router](https://dioxuslabs.com/learn/0.7/router)
- [Dioxus Signals](https://dioxuslabs.com/learn/0.7/state)
- [Docker API](https://docs.docker.com/engine/api/)
- [Rust Book](https://doc.rust-lang.org/book/)

## Troubleshooting

### Build Errors

**Problem**: `cargo build` fails with dependency errors

**Solution**: Update dependencies and clean build
``````bash
cargo update
cargo clean
cargo build
``````

### Runtime Errors

**Problem**: Docker connection fails

**Solution**: Ensure Docker daemon is running
``````bash
docker ps
``````

**Problem**: Asset not found

**Solution**: Ensure assets are in correct location and referenced with `asset!` macro

### Development Server

**Problem**: Hot reload not working

**Solution**: Restart dev server
``````bash
dx serve --platform desktop --hot-reload
``````

## FAQ

**Q: Why mock data instead of real Docker API?**
A: The initial implementation focuses on UI/UX. Real Docker integration is planned for future releases.

**Q: Can I build for web?**
A: The current features are `["desktop"]`. Web support can be added by enabling the `web` feature.

**Q: How do I customize the theme?**
A: Edit `assets/styling/main.css` to change colors and styles.

**Q: Is there a plugin system?**
A: Not currently. This could be a future enhancement.
