# UI Components API

Reusable UI building blocks for consistent interface elements.

## Module: `components`

**Location**: `src/components/`

### Overview

The components module provides small, focused UI elements that are reused across multiple views:
- **MetricCard** - Numeric metrics with titles and hints
- **SectionHeader** - Page titles with optional subtitles
- **StatusPill** - Colored status indicators

All components follow Dioxus 0.7 patterns with the `#[component]` macro.

---

## Components

### `MetricCard`

Displays a metric value with a title and optional hint text.

**Location**: `src/components/metric_card.rs`

#### Props

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `title` | `String` | Yes | Metric label (e.g., "Total Containers") |
| `value` | `String` | Yes | Metric value (e.g., "42") |
| `hint` | `Option<String>` | No | Additional context text |

#### Example

````rust
use crate::components::MetricCard;

rsx! {
    MetricCard {
        title: "Running Containers".to_string(),
        value: "8".to_string(),
        hint: Some("2 stopped".to_string())
    }
}
````

#### Rendered HTML Structure

````html
<div class="card">
    <p class="card-title">Running Containers</p>
    <p class="card-value">8</p>
    <p class="card-hint">2 stopped</p>
</div>
````

#### CSS Classes

- `.card` - Container element
- `.card-title` - Metric label
- `.card-value` - Primary metric value
- `.card-hint` - Optional supplementary text

#### Use Cases

- Dashboard metrics (container count, image count, volume count)
- Summary statistics
- Key performance indicators
- Status overviews

---

### `SectionHeader`

Page-level header with title and optional subtitle.

**Location**: `src/components/section_header.rs`

#### Props

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `title` | `String` | Yes | Main heading text |
| `subtitle` | `Option<String>` | No | Descriptive subheading |

#### Example

````rust
use crate::components::SectionHeader;

rsx! {
    SectionHeader {
        title: "Containers".to_string(),
        subtitle: Some("Manage running services".to_string())
    }
}
````

#### Rendered HTML Structure

````html
<div class="section-header">
    <div>
        <h2>Containers</h2>
        <p class="section-subtitle">Manage running services</p>
    </div>
</div>
````

#### CSS Classes

- `.section-header` - Outer container
- `h2` - Page title (unstyled by component)
- `.section-subtitle` - Optional subtitle text

#### Use Cases

- Page headers for each view
- Section introductions
- Feature area titles

---

### `StatusPill`

Colored badge for status indication.

**Location**: `src/components/status_pill.rs`

#### Props

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `label` | `String` | Yes | Status text (e.g., "Running") |
| `class_name` | `String` | Yes | CSS modifier class (e.g., "running") |

#### Example

````rust
use crate::components::StatusPill;
use crate::services::ContainerState;

let state = ContainerState::Running;

rsx! {
    StatusPill {
        label: state.label().to_string(),
        class_name: state.css_class().to_string()
    }
}
````

#### Rendered HTML Structure

````html
<span class="pill running">Running</span>
````

#### CSS Classes

- `.pill` - Base pill styling
- `.running` - Green styling for running state
- `.stopped` - Gray styling for stopped state
- Custom classes can be added for additional states

#### Use Cases

- Container state indicators
- Status badges
- State visualization
- Colorized labels

#### Integration with ContainerState

The `StatusPill` component is designed to work seamlessly with `ContainerState`:

````rust
let container: ContainerInfo = /* ... */;

rsx! {
    StatusPill {
        label: container.state.label().to_string(),
        class_name: container.state.css_class().to_string()
    }
}
````

This pattern ensures consistent styling and labeling across the application.

---

## Component Patterns

### Prop Types

All components use owned types for props:
- `String` instead of `&str`
- `Option<String>` for optional text
- No lifetime parameters

**Rationale**: Dioxus 0.7 requires owned props for the component system to work correctly across async boundaries.

### Optional Props

Use `Option<T>` for optional content:

````rust
#[component]
pub fn MyComponent(required: String, optional: Option<String>) -> Element {
    rsx! {
        h1 { "{required}" }
        if let Some(text) = optional {
            p { "{text}" }
        }
    }
}
````

### Conditional Rendering

Components use `if let` for optional content:

````rust
if let Some(hint) = hint {
    p { class: "card-hint", "{hint}" }
}
````

### Component Composition

Components can be nested:

````rust
rsx! {
    SectionHeader {
        title: "Dashboard".to_string(),
        subtitle: Some("System overview".to_string())
    }
    
    div { class: "metrics-grid",
        MetricCard {
            title: "Containers".to_string(),
            value: "5".to_string()
        }
        MetricCard {
            title: "Images".to_string(),
            value: "12".to_string()
        }
    }
}
````

---

## Styling Integration

### CSS Architecture

Components use semantic class names that are styled in `assets/styling/main.css`:

````css
/* MetricCard styles */
.card {
    background: white;
    border-radius: 8px;
    padding: 1rem;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.card-title {
    font-size: 0.875rem;
    color: #666;
    margin-bottom: 0.5rem;
}

.card-value {
    font-size: 2rem;
    font-weight: 600;
    color: #111;
}

/* StatusPill styles */
.pill {
    display: inline-block;
    padding: 0.25rem 0.75rem;
    border-radius: 12px;
    font-size: 0.875rem;
    font-weight: 500;
}

.pill.running {
    background: #d1fae5;
    color: #065f46;
}

.pill.stopped {
    background: #e5e7eb;
    color: #374151;
}
````

### Theming

To customize component appearance, override CSS classes in your stylesheet:

````css
/* Custom theme */
.card {
    background: #1a1a1a;
    border: 1px solid #333;
}

.card-value {
    color: #3b82f6; /* Blue accent */
}
````

---

## Accessibility

### Semantic HTML

All components use semantic HTML elements:
- `<h2>` for section headers
- `<span>` for inline status indicators
- `<div>` for card containers

### Best Practices

````rust
// ✅ Good: Descriptive text
StatusPill {
    label: "Container is running".to_string(),
    class_name: "running".to_string()
}

// ❌ Avoid: Non-descriptive text
StatusPill {
    label: "OK".to_string(), // What does "OK" mean?
    class_name: "green".to_string()
}
````

### ARIA Considerations

For enhanced accessibility, wrap components with ARIA attributes when needed:

````rust
rsx! {
    div { 
        role: "status",
        aria_live: "polite",
        StatusPill {
            label: state.label().to_string(),
            class_name: state.css_class().to_string()
        }
    }
}
````

---

## Testing

### Unit Testing

Components can be tested in isolation:

````rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn metric_card_renders_with_hint() {
        // Test implementation would verify rendering
    }
}
````

### Integration Testing

Test components in context:

````rust
#[test]
fn dashboard_shows_metrics() {
    // Verify MetricCard displays correct values from AppState
}
````

---

## Creating New Components

### Template

````rust
use dioxus::prelude::*;

#[component]
pub fn NewComponent(
    required_prop: String,
    optional_prop: Option<String>
) -> Element {
    rsx! {
        div { class: "new-component",
            h3 { "{required_prop}" }
            if let Some(text) = optional_prop {
                p { "{text}" }
            }
        }
    }
}
````

### Registration

1. Create file in `src/components/`
2. Add `mod` declaration in `src/components/mod.rs`
3. Add `pub use` statement to export
4. Add CSS styles to `assets/styling/main.css`

**Example** (`src/components/mod.rs`):
````rust
mod metric_card;
pub use metric_card::MetricCard;

mod new_component;
pub use new_component::NewComponent;
````

---

## See Also

- [Views Documentation](views.md) - Page-level components that use these building blocks
- [Styling Guide](../guides/styling-guide.md) - CSS architecture and theming
- [Dioxus Components Guide](https://dioxuslabs.com/learn/0.7/reference/components)
