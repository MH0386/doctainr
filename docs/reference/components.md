# Components Reference

Complete reference for all reusable UI components in Doctainr.

## Component Index

- [MetricCard](#metriccard) - Display dashboard metrics
- [StatusPill](#statuspill) - Container state badges
- [SectionHeader](#sectionheader) - Section headings

---

## MetricCard

`src/components/metric_card.rs`

Displays a labeled metric value with optional icon, used primarily on the Dashboard.

### Props

````rust
#[component]
pub fn MetricCard(
    label: String,      // Metric label (e.g., "Total Containers")
    value: String,      // Metric value (e.g., "12")
    icon: Option<String>, // Optional icon class/emoji
) -> Element
````

### Usage

````rust
use crate::components::MetricCard;

rsx! {
    MetricCard {
        label: "Containers".to_string(),
        value: "12".to_string(),
    }
    
    MetricCard {
        label: "Images".to_string(),
        value: "45".to_string(),
        icon: Some("🐳".to_string()),
    }
}
````

### Styling

Default classes applied:
- `.metric-card` - Container
- `.metric-label` - Label text
- `.metric-value` - Value text
- `.metric-icon` - Icon (if provided)

### Example Output

````html
<div class="metric-card">
    <div class="metric-icon">🐳</div>
    <div class="metric-label">Containers</div>
    <div class="metric-value">12</div>
</div>
````

---

## StatusPill

`src/components/status_pill.rs`

Displays a colored badge indicating container state (Running/Stopped).

### Props

````rust
#[component]
pub fn StatusPill(
    state: ContainerState,  // Running or Stopped
) -> Element
````

### Usage

````rust
use crate::components::StatusPill;
use crate::services::ContainerState;

rsx! {
    StatusPill { state: ContainerState::Running }
    StatusPill { state: ContainerState::Stopped }
}
````

### States

| State     | Color | Label     | CSS Class |
|-----------|-------|-----------|-----------|
| Running   | Green | "Running" | `.running` |
| Stopped   | Gray  | "Stopped" | `.stopped` |

### Styling

Default classes applied:
- `.status-pill` - Base pill styling
- `.running` or `.stopped` - State-specific styling

### Example Output

````html
<!-- Running -->
<span class="status-pill running">Running</span>

<!-- Stopped -->
<span class="status-pill stopped">Stopped</span>
````

### Custom Styling

Override in CSS:
````css
.status-pill {
    padding: 4px 12px;
    border-radius: 12px;
    font-size: 0.875rem;
}

.status-pill.running {
    background: #10b981;
    color: white;
}

.status-pill.stopped {
    background: #6b7280;
    color: white;
}
````

---

## SectionHeader

`src/components/section_header.rs`

Renders a styled section heading with consistent formatting across views.

### Props

````rust
#[component]
pub fn SectionHeader(
    title: String,      // Section title text
) -> Element
````

### Usage

````rust
use crate::components::SectionHeader;

rsx! {
    SectionHeader { title: "Docker Containers".to_string() }
    SectionHeader { title: "Local Images".to_string() }
}
````

### Styling

Default classes applied:
- `.section-header` - Header container
- `.section-title` - Title text

### Example Output

````html
<div class="section-header">
    <h2 class="section-title">Docker Containers</h2>
</div>
````

---

## Creating Custom Components

### Component Template

````rust
use dioxus::prelude::*;

/// Brief description of what this component does.
///
/// # Props
///
/// * `prop1` - Description of prop1
/// * `prop2` - Description of prop2
#[component]
pub fn MyComponent(
    prop1: String,
    prop2: i32,
    optional_prop: Option<String>,
) -> Element {
    // Local state
    let mut local_state = use_signal(|| 0);
    
    // Event handlers
    let handle_click = move |_| {
        local_state += 1;
    };
    
    rsx! {
        div {
            class: "my-component",
            "{prop1}: {prop2}"
            button { onclick: handle_click, "Click me" }
        }
    }
}
````

### Best Practices

1. **Props are owned values**
   ````rust
   // ✅ Good
   title: String,
   items: Vec<Item>,
   
   // ❌ Bad (props can't be references)
   title: &str,
   items: &[Item],
   ````

2. **Implement PartialEq and Clone**
   Props automatically derive these, but custom types need:
   ````rust
   #[derive(Clone, PartialEq)]
   pub struct CustomProp {
       value: String,
   }
   ````

3. **Use ReadOnlySignal for reactive props**
   ````rust
   #[component]
   pub fn LiveCounter(
       count: ReadOnlySignal<i32>,  // Re-renders when count changes
   ) -> Element {
       rsx! { "Count: {count}" }
   }
   ````

4. **Document component behavior**
   ````rust
   /// Displays a container status with color-coded badge.
   ///
   /// Automatically updates when container state changes.
   /// Clicking the badge does NOT trigger any action.
   #[component]
   pub fn ContainerStatus(state: ContainerState) -> Element {
       // ...
   }
   ````

## Component Patterns

### Conditional Rendering

````rust
rsx! {
    if show_details {
        div { "Detailed view" }
    }
    
    {loading.then(|| rsx! { "Loading..." })}
}
````

### List Rendering

````rust
rsx! {
    // Prefer for loops
    for item in items {
        div { "{item.name}" }
    }
    
    // Or iterators in braces
    {items.iter().map(|item| rsx! {
        div { key: "{item.id}", "{item.name}" }
    })}
}
````

### Event Handlers

````rust
#[component]
pub fn Button(on_click: EventHandler<()>) -> Element {
    rsx! {
        button {
            onclick: move |_| on_click(()),
            "Click me"
        }
    }
}

// Usage
rsx! {
    Button {
        on_click: move |_| {
            println!("Clicked!");
        }
    }
}
````

### Children Props

````rust
#[component]
pub fn Card(children: Element) -> Element {
    rsx! {
        div {
            class: "card",
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
````

## Styling Components

### Inline Styles

````rust
rsx! {
    div {
        class: "container",
        style: "background: blue; padding: 10px;",
        "Content"
    }
}
````

### Dynamic Classes

````rust
let class_name = if active { "button active" } else { "button" };

rsx! {
    button { class: "{class_name}", "Click" }
}
````

### Conditional Styles

````rust
rsx! {
    div {
        class: "card",
        background: if error { "red" } else { "white" },
        "Content"
    }
}
````

## Testing Components

````rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn status_pill_displays_correct_label() {
        let state = ContainerState::Running;
        assert_eq!(state.label(), "Running");
    }
    
    #[test]
    fn metric_card_formats_value() {
        let value = format!("{}", 42);
        assert_eq!(value, "42");
    }
}
````

## Component Library Structure

````
src/components/
├── mod.rs              # Public exports
├── metric_card.rs      # Dashboard metrics
├── status_pill.rs      # Status badges
├── section_header.rs   # Section titles
└── [future components]
    ├── button.rs       # Reusable buttons
    ├── modal.rs        # Dialog windows
    └── table.rs        # Data tables
````

---

**See Also**: [API Reference](./api.md) | [Architecture](./architecture.md)
