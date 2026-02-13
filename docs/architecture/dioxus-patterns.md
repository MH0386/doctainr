# Dioxus 0.7 Patterns

Dioxus-specific patterns and best practices used in Doctainr.

## Dioxus 0.7 Overview

Dioxus 0.7 introduces significant changes from previous versions:
- **Removed**: `cx` (Scope), `use_state`, `use_ref`
- **Added**: Signals, improved reactivity, simplified hooks
- **Changed**: Component syntax, prop handling, async patterns

---

## Signal Patterns

### Basic Signal Usage

Signals are the primary state primitive in Dioxus 0.7.

````rust
// Create a signal
let mut count = use_signal(|| 0);

// Read signal value
let value = count();

// Write signal value
count.set(42);

// Mutate in place
count.with_mut(|c| *c += 1);
````

### Signal in Components

````rust
#[component]
fn Counter() -> Element {
    let mut count = use_signal(|| 0);
    
    rsx! {
        div {
            "Count: {count}"
            button {
                onclick: move |_| count.set(count() + 1),
                "Increment"
            }
        }
    }
}
````

### Signal Cloning for Closures

When using signals in closures, clone them first:

````rust
let mut value = use_signal(|| 0);
let mut value_clone = value.clone();

spawn(async move {
    // Use cloned signal in async context
    value_clone.set(42);
});
````

**Why**: Closures need owned values; signals are `Copy` so cloning is cheap.

---

## Component Patterns

### Component Definition

````rust
#[component]
pub fn MyComponent(
    required: String,
    optional: Option<String>,
    number: i32
) -> Element {
    rsx! {
        div { "{required}" }
    }
}
````

**Rules**:
- Function must return `Element`
- Props must be owned types (no `&str`, use `String`)
- Use `#[component]` macro for better error messages
- Props implement `PartialEq` and `Clone` automatically

### Props with Signals

Pass signals as props for reactivity:

````rust
#[component]
pub fn Display(value: Signal<i32>) -> Element {
    rsx! { "Value: {value}" }
}

// Usage
let count = use_signal(|| 0);
rsx! { Display { value: count } }
````

**Benefits**: Child automatically re-renders when signal changes.

### Optional Props

````rust
#[component]
pub fn Card(
    title: String,
    subtitle: Option<String>,
    #[props(default = false)] highlighted: bool
) -> Element {
    rsx! {
        div {
            h2 { "{title}" }
            if let Some(sub) = subtitle {
                p { "{sub}" }
            }
        }
    }
}
````

---

## Context API Patterns

### Providing Context

````rust
#[component]
fn App() -> Element {
    let state = AppState::new();
    use_context_provider(|| state);
    
    rsx! { Child {} }
}
````

### Consuming Context

````rust
#[component]
fn Child() -> Element {
    let state = use_context::<AppState>();
    let data = (state.some_signal)();
    
    rsx! { "Data: {data}" }
}
````

### Context with Signals

````rust
// Provide signal as context
use_context_provider(|| use_signal(|| 0));

// Consume anywhere
let count = use_context::<Signal<i32>>();
rsx! { "Count: {count}" }
````

---

## Async Patterns

### Spawning Tasks

````rust
let mut result = use_signal(|| None);

button {
    onclick: move |_| {
        let mut result_clone = result.clone();
        spawn(async move {
            let data = fetch_data().await;
            result_clone.set(Some(data));
        });
    },
    "Fetch"
}
````

### Resource Hook

For data fetching, use `use_resource`:

````rust
let data = use_resource(move || async move {
    fetch_data().await
});

match data() {
    Some(value) => rsx! { "Data: {value}" },
    None => rsx! { "Loading..." }
}
````

**Note**: Resources automatically re-run when dependencies change.

---

## RSX Patterns

### Conditional Rendering

````rust
// If/else
if condition {
    rsx! { div { "True" } }
} else {
    rsx! { div { "False" } }
}

// If let
if let Some(value) = optional {
    rsx! { div { "{value}" } }
}

// Match
match state {
    State::Loading => rsx! { "Loading..." },
    State::Ready(data) => rsx! { "{data}" },
}
````

### Iteration

````rust
// For loops (preferred)
for item in items.iter() {
    div { "{item}" }
}

// Iterator expressions (when needed)
{items.iter().map(|item| {
    rsx! { div { "{item}" } }
})}
````

**Best Practice**: Use `for` loops directly in RSX when possible; they're cleaner and more idiomatic.

### String Interpolation

````rust
let name = "World";
rsx! {
    // Direct interpolation
    div { "Hello, {name}!" }
    
    // Expression
    div { "Count: {count * 2}" }
    
    // Method call
    div { "Upper: {name.to_uppercase()}" }
}
````

---

## Event Handling

### Basic Events

````rust
button {
    onclick: move |_| {
        println!("Clicked!");
    },
    "Click Me"
}
````

### Event Data

````rust
input {
    oninput: move |event| {
        let value = event.value();
        println!("Input: {value}");
    }
}
````

### Multiple Handlers

````rust
div {
    onclick: move |_| handle_click(),
    onmouseover: move |_| handle_hover(),
    "Hover or click"
}
````

---

## Hooks Patterns

### use_signal

````rust
let mut count = use_signal(|| 0);
````

**Use when**: Local component state.

### use_context

````rust
let state = use_context::<AppState>();
````

**Use when**: Accessing global state.

### use_memo

````rust
let expensive = use_memo(move || {
    // Expensive calculation
    compute_value(count())
});
````

**Use when**: Caching derived values.

### use_effect

````rust
use_effect(move || {
    // Side effect
    println!("Count changed: {}", count());
});
````

**Use when**: Running side effects on state changes.

---

## Ownership and Lifetimes

### Moving vs Cloning

````rust
// Clone for multiple closures
let count = use_signal(|| 0);
let count_clone = count.clone();

button {
    onclick: move |_| count.set(count() + 1),
    "+"
}
button {
    onclick: move |_| count_clone.set(count_clone() - 1),
    "-"
}
````

### 'static Closure Requirements

Event handlers must be `'static`:

````rust
let items = vec!["a".to_string(), "b".to_string()];

// ✅ Clone items for closure
{items.iter().map(|item| {
    let item_clone = item.clone();
    rsx! {
        button {
            onclick: move |_| println!("{item_clone}"),
            "{item}"
        }
    }
})}
````

---

## Component Composition

### Parent-Child Communication

**Downward (Props)**:
````rust
#[component]
fn Parent() -> Element {
    let value = use_signal(|| 0);
    rsx! { Child { value } }
}

#[component]
fn Child(value: Signal<i32>) -> Element {
    rsx! { "Value: {value}" }
}
````

**Upward (Callbacks)**:
````rust
#[component]
fn Parent() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        Child {
            on_increment: move |_| count.set(count() + 1)
        }
    }
}

#[component]
fn Child(on_increment: EventHandler) -> Element {
    rsx! {
        button {
            onclick: move |_| on_increment.call(()),
            "Increment"
        }
    }
}
````

---

## Performance Patterns

### Avoid Unnecessary Re-renders

````rust
// ✅ Read signal once
let value = count();
let doubled = value * 2;
let tripled = value * 3;

// ❌ Read signal multiple times
let doubled = count() * 2;
let tripled = count() * 3;
````

### Memoization

````rust
let expensive = use_memo(move || {
    (0..1000000).sum::<i32>()
});

// Only recalculates when dependencies change
````

### Lazy Evaluation

````rust
// Only load data when tab is active
if active_tab() == Tab::Details {
    let data = use_resource(|| async { fetch_details().await });
    // ...
}
````

---

## Error Handling

### Result Handling

````rust
let mut data = use_signal(|| None);
let mut error = use_signal(|| None);

spawn(async move {
    match fetch_data().await {
        Ok(result) => data.set(Some(result)),
        Err(e) => error.set(Some(e.to_string())),
    }
});

rsx! {
    if let Some(err) = error() {
        div { class: "error", "{err}" }
    }
    if let Some(value) = data() {
        div { "{value}" }
    }
}
````

---

## Doctainr-Specific Patterns

### AppState Pattern

````rust
// Single global state object
#[derive(Clone)]
pub struct AppState {
    pub field: Signal<T>,
}

// Provide at root
use_context_provider(|| AppState::new());

// Consume anywhere
let state = use_context::<AppState>();
````

### Async Operation Pattern

````rust
pub fn do_operation(&self) {
    let service = self.service.clone();
    let mut result = self.result.clone();
    
    spawn(async move {
        match service.operation().await {
            Ok(data) => result.set(data),
            Err(e) => /* handle error */,
        }
    });
}
````

### Table Iteration Pattern

````rust
{items.iter().map(|item| {
    // Clone everything needed for closure
    let id = item.id.clone();
    let name = item.name.clone();
    let state_clone = app_state.clone();
    
    rsx! {
        div { class: "row",
            span { "{name}" }
            button {
                onclick: move |_| state_clone.action(&id),
                "Action"
            }
        }
    }
})}
````

---

## Common Pitfalls

### ❌ Using &str in Props

````rust
// Wrong
#[component]
fn MyComponent(text: &str) -> Element { /* ... */ }

// Correct
#[component]
fn MyComponent(text: String) -> Element { /* ... */ }
````

### ❌ Forgetting to Clone for Closures

````rust
// Wrong (won't compile)
onclick: move |_| println!("{}", external_value)

// Correct
let value_clone = external_value.clone();
onclick: move |_| println!("{}", value_clone)
````

### ❌ Reading Signal Multiple Times

````rust
// Wrong (inefficient)
if count() > 5 && count() < 10 { /* ... */ }

// Correct
let c = count();
if c > 5 && c < 10 { /* ... */ }
````

---

## Migration from Dioxus 0.6

### Key Changes

| 0.6 | 0.7 |
|-----|-----|
| `use_state` | `use_signal` |
| `cx` parameter | Removed |
| `&str` props | `String` props |
| `UseState::set` | `Signal::set` |
| `use_shared_state` | `use_context` |

### Migration Example

**Before (0.6)**:
````rust
fn Component(cx: Scope) -> Element {
    let count = use_state(cx, || 0);
    cx.render(rsx! {
        button {
            onclick: move |_| count.set(count + 1),
            "{count}"
        }
    })
}
````

**After (0.7)**:
````rust
#[component]
fn Component() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        button {
            onclick: move |_| count.set(count() + 1),
            "{count}"
        }
    }
}
````

---

## See Also

- [Official Dioxus 0.7 Guide](https://dioxuslabs.com/learn/0.7/)
- [Application State](../api/state.md) - AppState implementation
- [System Overview](overview.md) - Architecture context
- [AGENTS.md](../../AGENTS.md) - Quick reference guide
