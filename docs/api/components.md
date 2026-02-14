# Component API Reference

Complete API reference for reusable UI components.

## Module: `components`

**Location**: `src/components/`

### Overview

Reusable UI components used across multiple views in Doctainr.

## Components

### MetricCard

Displays a metric with title, value, and optional hint.

**Location**: `src/components/metric_card.rs`

#### Props

```rust
#[component]
pub fn MetricCard(
    title: String,
    value: String,
    hint: Option<String>,
) -> Element
```

**Parameters**:
- `title`: Metric label (e.g., "Running containers")
- `value`: Metric value (e.g., "5")
- `hint`: Optional subtitle (e.g., "Across all projects")

#### Usage

```rust
use crate::components::MetricCard;

rsx! {
    MetricCard {
        title: "Running containers".to_string(),
        value: running.to_string(),
        hint: Some("Across all projects".to_string())
    }
}
```

#### Styling

Uses CSS class `.metric-card`. Apply custom styles in your stylesheet.

#### Example

```rust
#[component]
fn Dashboard() -> Element {
    let running = 5;
    let stopped = 2;
    
    rsx! {
        div { class: "cards",
            MetricCard {
                title: "Running".to_string(),
                value: running.to_string(),
                hint: None
            }
            MetricCard {
                title: "Stopped".to_string(),
                value: stopped.to_string(),
                hint: Some("Ready to start".to_string())
            }
        }
    }
}
```

---

### SectionHeader

Page header with title and optional subtitle.

**Location**: `src/components/section_header.rs`

#### Props

```rust
#[component]
pub fn SectionHeader(
    title: String,
    subtitle: Option<String>,
) -> Element
```

**Parameters**:
- `title`: Main heading text
- `subtitle`: Optional descriptive text

#### Usage

```rust
use crate::components::SectionHeader;

rsx! {
    SectionHeader {
        title: "Dashboard".to_string(),
        subtitle: Some("Overview of your Docker environment".to_string())
    }
}
```

#### Styling

Uses CSS class `.section-header`. Subtitle uses `.subtitle`.

#### Example

```rust
#[component]
fn ContainersView() -> Element {
    rsx! {
        SectionHeader {
            title: "Containers".to_string(),
            subtitle: Some("Manage your Docker containers".to_string())
        }
        
        // Rest of view content
    }
}
```

---

### StatusPill

Colored status indicator for container states.

**Location**: `src/components/status_pill.rs`

#### Props

```rust
#[component]
pub fn StatusPill(
    status: String,
    css_class: String,
) -> Element
```

**Parameters**:
- `status`: Status text to display (e.g., "Running", "Stopped")
- `css_class`: CSS class for styling (e.g., "running", "stopped")

#### Usage

```rust
use crate::components::StatusPill;
use crate::services::ContainerState;

rsx! {
    StatusPill {
        status: container.state.label().to_string(),
        css_class: container.state.css_class().to_string()
    }
}
```

#### Styling

Uses CSS class `.status-pill` with additional state-specific class:
- `.status-pill.running` - Green for running containers
- `.status-pill.stopped` - Red for stopped containers

#### Example

```rust
#[component]
fn ContainerRow(container: ContainerInfo) -> Element {
    rsx! {
        div { class: "container-row",
            span { "{container.name}" }
            StatusPill {
                status: container.state.label().to_string(),
                css_class: container.state.css_class().to_string()
            }
        }
    }
}
```

---

## Creating Custom Components

### Basic Component Template

```rust
use dioxus::prelude::*;

/// Brief component description.
///
/// Longer description of what the component does and when to use it.
#[component]
pub fn MyComponent(
    // Required props (owned types)
    title: String,
    count: i32,
    
    // Optional props
    #[props(default = false)]
    show_details: bool,
    
    // Optional with None default
    subtitle: Option<String>,
) -> Element {
    // Component logic
    let doubled = count * 2;
    
    rsx! {
        div { class: "my-component",
            h2 { "{title}" }
            
            if let Some(sub) = subtitle {
                p { class: "subtitle", "{sub}" }
            }
            
            p { "Count: {count}, Doubled: {doubled}" }
            
            if show_details {
                div { class: "details",
                    "Additional information"
                }
            }
        }
    }
}
```

### Exporting Component

In `src/components/mod.rs`:

```rust
mod my_component;
pub use my_component::MyComponent;
```

### Using Component

```rust
use crate::components::MyComponent;

rsx! {
    MyComponent {
        title: "Test".to_string(),
        count: 42,
        show_details: true,
        subtitle: Some("Subtitle text".to_string())
    }
}
```

## Component Patterns

### Component with State

```rust
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
```

### Component with Context

```rust
#[component]
fn StatusDisplay() -> Element {
    let app_state = use_context::<AppState>();
    let is_loading = (app_state.is_loading)();
    
    rsx! {
        if is_loading {
            "Loading..."
        } else {
            "Ready"
        }
    }
}
```

### Component with Children

```rust
#[component]
fn Card(children: Element) -> Element {
    rsx! {
        div { class: "card",
            {children}
        }
    }
}

// Usage
rsx! {
    Card {
        h2 { "Title" }
        p { "Content" }
    }
}
```

### Component with Event Handlers

```rust
#[component]
fn ClickableCard(
    title: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div {
            class: "card clickable",
            onclick: move |e| onclick.call(e),
            h2 { "{title}" }
        }
    }
}

// Usage
rsx! {
    ClickableCard {
        title: "Click me".to_string(),
        onclick: move |_| {
            println!("Card clicked!");
        }
    }
}
```

## Styling Components

### CSS Classes

```rust
rsx! {
    div { class: "component-name",
        // Use consistent naming
    }
}
```

### Conditional Classes

```rust
let class_name = if is_active { "active" } else { "inactive" };

rsx! {
    div { class: "button {class_name}",
        "Button"
    }
}
```

### Dynamic Styles

```rust
rsx! {
    div {
        class: "card",
        style: "background-color: {color}; padding: {padding}px",
        "Content"
    }
}
```

## Component Best Practices

### 1. Keep Components Focused

Each component should have a single, clear responsibility.

```rust
// ✅ Good: Single purpose
#[component]
fn UserAvatar(username: String, size: i32) -> Element { }

// ❌ Bad: Multiple purposes
#[component]
fn UserProfileAndSettings(/* many props */) -> Element { }
```

### 2. Use Descriptive Prop Names

```rust
// ✅ Good
#[component]
fn Button(
    label: String,
    is_disabled: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element { }

// ❌ Bad
#[component]
fn Button(
    text: String,
    disabled: bool,
    handler: EventHandler<MouseEvent>,
) -> Element { }
```

### 3. Document Public Components

```rust
/// Displays a container status badge.
///
/// Shows the current state (Running/Stopped) with appropriate
/// color coding and styling.
///
/// # Props
/// - `status`: Display text
/// - `css_class`: Style class
#[component]
pub fn StatusPill(status: String, css_class: String) -> Element { }
```

### 4. Provide Sensible Defaults

```rust
#[component]
fn Card(
    title: String,
    #[props(default = false)]
    elevated: bool,
    #[props(default = String::from("medium"))]
    size: String,
) -> Element { }
```

### 5. Handle Edge Cases

```rust
#[component]
fn ImageDisplay(url: Option<String>) -> Element {
    rsx! {
        match url {
            Some(u) => rsx! {
                img { src: "{u}", alt: "Image" }
            },
            None => rsx! {
                div { class: "placeholder", "No image" }
            }
        }
    }
}
```

## Testing Components

### Logic Tests

Test component logic separately from rendering:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_logic() {
        let result = format_status("Up 2 hours");
        assert_eq!(result, "Running");
    }
}
```

### Integration Tests

Test components in context:

```rust
#[test]
fn test_component_with_state() {
    // Test component behavior with state
}
```

## Related Documentation

- [Component Architecture](../architecture/components.md)
- [Creating Custom Components Tutorial](../examples/custom-component.md)
- [Code Style Guide](../guides/code-style.md)
- [State Management](../architecture/state-management.md)
