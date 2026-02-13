# Development Guide

Guide for developers working on Doctainr.

## Table of Contents

- [Development Environment](#development-environment)
- [Project Structure](#project-structure)
- [Building and Running](#building-and-running)
- [Dioxus 0.7 Guide](#dioxus-07-guide)
- [Testing](#testing)
- [Debugging](#debugging)
- [Common Tasks](#common-tasks)

## Development Environment

### Recommended Setup

#### Using devenv (Recommended)

The project includes a `devenv.nix` configuration for reproducible development environments:

````bash
# Install devenv
curl -fsSL https://get.jetpack.io/devenv | sh

# Enter development shell
devenv shell

# Everything is automatically installed and configured!
````

#### Manual Setup

If you prefer manual setup:

1. **Install Rust:**
   ````bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ````

2. **Install Dioxus CLI:**
   ````bash
   curl -sSL http://dioxus.dev/install.sh | sh
   ````

3. **Install Docker:**
   - Follow instructions at https://docs.docker.com/get-docker/

4. **Clone Repository:**
   ````bash
   git clone https://github.com/MH0386/containr.git
   cd containr
   ````

### IDE Setup

#### VS Code

Recommended extensions:
- `rust-lang.rust-analyzer` - Rust language support
- `tamasfe.even-better-toml` - TOML syntax highlighting
- `vadimcn.vscode-lldb` - Debugger

Settings (`.vscode/settings.json`):
````json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "editor.formatOnSave": true
}
````

#### IntelliJ IDEA / CLion

Install the Rust plugin from JetBrains marketplace.

## Project Structure

````
containr/
├── src/
│   ├── main.rs              # Entry point and routing
│   ├── components/          # Reusable UI components
│   │   ├── mod.rs
│   │   ├── metric_card.rs
│   │   ├── section_header.rs
│   │   └── status_pill.rs
│   ├── services/            # Business logic
│   │   ├── mod.rs
│   │   └── docker.rs        # Docker API wrapper
│   ├── utils/               # Utilities and state
│   │   ├── mod.rs
│   │   └── app_state.rs     # Global app state
│   └── views/               # Page components
│       ├── mod.rs
│       ├── dashboard.rs
│       ├── containers.rs
│       ├── images.rs
│       ├── volumes.rs
│       └── settings.rs
├── assets/                  # Static assets
│   ├── favicon.ico
│   ├── header.svg
│   └── styling/
│       ├── main.css
│       ├── navbar.css
│       ├── echo.css
│       └── blog.css
├── docs/                    # Documentation
├── Cargo.toml              # Dependencies
├── Dioxus.toml             # Dioxus configuration
└── devenv.nix              # Development environment
````

## Building and Running

### Development Mode

With hot-reloading using Dioxus CLI:

````bash
dx serve --platform desktop
````

This enables:
- Hot reload on file changes
- Fast iteration
- Better error messages

### Standard Build

````bash
# Debug build
cargo run

# Release build (optimized)
cargo run --release
````

### Build Targets

````bash
# Desktop (default)
cargo build --features desktop

# Web (future)
cargo build --features web
````

## Dioxus 0.7 Guide

### Key Differences from 0.6

Dioxus 0.7 introduces breaking changes:

❌ **Removed (0.6):**
- `cx` / `Scope` parameter
- `use_state` hook
- `to_owned![]` macro

✅ **New (0.7):**
- Direct `Signal<T>` API
- Automatic dependency tracking
- Simplified component syntax

### Component Basics

````rust
#[component]
fn MyComponent(name: String, count: i32) -> Element {
    rsx! {
        div { "Hello {name}, count: {count}" }
    }
}
````

**Rules:**
- Use `#[component]` macro
- Props must be owned types (`String`, not `&str`)
- Props must implement `Clone + PartialEq`
- Return type is always `Element`

### State Management

#### Local State

````rust
#[component]
fn Counter() -> Element {
    let mut count = use_signal(|| 0);
    
    rsx! {
        div {
            "Count: {count}"
            button {
                onclick: move |_| *count.write() += 1,
                "Increment"
            }
        }
    }
}
````

**Reading Signals:**
- `count()` - Clone the value
- `count.read()` - Get immutable reference
- `count.with(|val| ...)` - Read with closure

**Writing Signals:**
- `*count.write() = value` - Set value
- `count.with_mut(|val| ...)` - Mutate with closure

#### Memoized Values

````rust
let doubled = use_memo(move || count() * 2);
````

Memos automatically recompute when dependencies change.

#### Context (Global State)

````rust
// Provider (in parent)
fn App() -> Element {
    let state = AppState::new();
    use_context_provider(|| state);
    rsx! { /* children */ }
}

// Consumer (in child)
fn Child() -> Element {
    let state = use_context::<AppState>();
    rsx! { /* use state */ }
}
````

### Async Operations

````rust
let resource = use_resource(move || async move {
    fetch_data().await
});

match resource() {
    Some(data) => rsx! { "Data: {data}" },
    None => rsx! { "Loading..." },
}
````

Resources automatically re-run when dependencies change.

### Event Handling

````rust
button {
    onclick: move |event| {
        println!("Clicked at: {:?}", event.coordinates());
    },
    "Click me"
}

input {
    value: "{text}",
    oninput: move |e| *text.write() = e.value(),
}
````

### Conditional Rendering

````rust
rsx! {
    if show_message {
        div { "Hello!" }
    }
    
    // With else
    if is_loading {
        div { "Loading..." }
    } else {
        div { "Loaded!" }
    }
}
````

### Lists

````rust
rsx! {
    // Preferred: for loop
    for item in items() {
        div { key: "{item.id}", "{item.name}" }
    }
    
    // Alternative: iterator
    {items().iter().map(|item| rsx! {
        div { key: "{item.id}", "{item.name}" }
    })}
}
````

**Important:** Always provide unique `key` attributes for list items.

### Assets

````rust
const IMAGE: Asset = asset!("/assets/image.png");

rsx! {
    img { src: IMAGE, alt: "Description" }
}
````

Assets are:
- Relative to project root
- Automatically optimized
- Bundled in production builds

## Testing

### Unit Tests

````rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_state() {
        let state = ContainerState::Running;
        assert_eq!(state.label(), "Running");
    }
}
````

Run with:
````bash
cargo test
````

### Integration Tests

````rust
// tests/docker_integration.rs
#[tokio::test]
async fn test_list_containers() {
    let docker = DockerService::new().unwrap();
    let containers = docker.list_containers().await;
    assert!(containers.is_ok());
}
````

### Mocking Docker

For tests without Docker:

````rust
#[cfg(test)]
mod tests {
    use mockall::mock;
    
    mock! {
        DockerService {
            async fn list_containers(&self) -> Result<Vec<ContainerInfo>>;
        }
    }
}
````

## Debugging

### Console Logging

````rust
// Debug output
println!("Debug: {:?}", value);
eprintln!("Error: {}", error);

// Dioxus logging
use tracing::{info, warn, error};
info!("Container started: {}", id);
error!("Failed to connect: {}", err);
````

### LLDB Debugger

````bash
# Run with debugger
rust-lldb target/debug/doctainr

# Set breakpoints
(lldb) breakpoint set -f main.rs -l 50
(lldb) run
````

### VS Code Debugging

Add to `.vscode/launch.json`:

````json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug Doctainr",
      "cargo": {
        "args": ["build", "--bin=doctainr"]
      },
      "args": [],
      "cwd": "${workspaceFolder}"
    }
  ]
}
````

### Docker Debugging

````bash
# Test Docker connection
docker version
docker info

# Monitor Docker events
docker events

# Check Docker logs
journalctl -u docker.service
````

## Common Tasks

### Adding a New Route

1. **Define route in `main.rs`:**
   ````rust
   #[derive(Debug, Clone, Routable, PartialEq)]
   enum Route {
       #[layout(AppShell)]
           // ... existing routes
           #[route("/new-page")]
           NewPage {},
   }
   ````

2. **Create view component:**
   ````rust
   // src/views/new_page.rs
   use dioxus::prelude::*;
   
   #[component]
   pub fn NewPage() -> Element {
       rsx! {
           div { "New Page Content" }
       }
   }
   ````

3. **Export from `views/mod.rs`:**
   ````rust
   pub use new_page::NewPage;
   ````

4. **Add navigation link:**
   ````rust
   Link { to: Route::NewPage {}, "New Page" }
   ````

### Adding a New Component

1. **Create component file:**
   ````rust
   // src/components/my_component.rs
   use dioxus::prelude::*;
   
   #[component]
   pub fn MyComponent(title: String) -> Element {
       rsx! {
           div { class: "my-component",
               h2 { "{title}" }
           }
       }
   }
   ````

2. **Export from `components/mod.rs`:**
   ````rust
   mod my_component;
   pub use my_component::MyComponent;
   ````

3. **Use in other components:**
   ````rust
   use crate::components::MyComponent;
   
   rsx! {
       MyComponent { title: "Hello" }
   }
   ````

### Adding CSS Styles

1. **Create CSS file:**
   ````css
   /* assets/styling/my-styles.css */
   .my-component {
       padding: 1rem;
       background: #f0f0f0;
   }
   ````

2. **Link in `main.rs`:**
   ````rust
   const MY_CSS: Asset = asset!("/assets/styling/my-styles.css");
   
   rsx! {
       document::Link { rel: "stylesheet", href: MY_CSS }
   }
   ````

### Adding a Docker Operation

1. **Add method to `DockerService`:**
   ````rust
   // src/services/docker.rs
   impl DockerService {
       pub async fn inspect_container(&self, id: &str) -> Result<ContainerDetails> {
           let container = self.client
               .inspect_container(id, None)
               .await?;
           
           Ok(ContainerDetails::from(container))
       }
   }
   ````

2. **Add to `AppState`:**
   ````rust
   // src/utils/app_state.rs
   impl AppState {
       pub async fn inspect_container(&self, id: &str) -> Result<ContainerDetails> {
           let service = self.docker_service.as_ref()
               .ok_or_else(|| anyhow!("Docker not connected"))?;
           
           service.inspect_container(id).await
       }
   }
   ````

3. **Use in component:**
   ````rust
   let details = use_resource(move || async move {
       app_state.inspect_container(&container_id).await
   });
   ````

## Performance Tips

### Avoid Unnecessary Re-renders

````rust
// Bad: Component re-renders when any part of AppState changes
let app_state = use_context::<AppState>();

// Good: Only re-render when containers change
let containers = use_context::<AppState>().containers;
````

### Use Memos for Expensive Computations

````rust
let filtered = use_memo(move || {
    containers()
        .into_iter()
        .filter(|c| c.state == ContainerState::Running)
        .collect::<Vec<_>>()
});
````

### Lazy Loading

````rust
let resource = use_resource(move || async move {
    // Only load when component mounts
    load_data().await
});
````

## Troubleshooting

### "Cannot find function `use_state`"

Dioxus 0.7 removed `use_state`. Use `use_signal` instead:

````rust
// Old (0.6)
let count = use_state(cx, || 0);

// New (0.7)
let mut count = use_signal(|| 0);
````

### "Borrowed value does not live long enough"

Use owned types in props:

````rust
// Bad
#[component]
fn MyComponent(name: &str) -> Element { }

// Good
#[component]
fn MyComponent(name: String) -> Element { }
````

### Hot Reload Not Working

1. Check Dioxus CLI version: `dx --version`
2. Restart dev server: `dx serve --platform desktop`
3. Clear cache: `rm -rf target/`

## Additional Resources

- [Dioxus 0.7 Documentation](https://dioxuslabs.com/learn/0.7)
- [Bollard Documentation](https://docs.rs/bollard)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
