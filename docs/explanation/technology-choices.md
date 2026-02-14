# Why Rust and Dioxus?

This document explains the technology choices behind Doctainr and the rationale for using Rust and Dioxus.

## The Problem Space

Docker Desktop is a powerful tool, but it can be resource-intensive. Many developers want a lightweight alternative that:

- Uses minimal system resources
- Starts quickly
- Provides essential Docker management features
- Runs natively without browser overhead

## Technology Choices

### Why Rust?

**1. Performance**

Rust compiles to native machine code with zero-cost abstractions. A Rust application:
- Has no garbage collector pauses
- Uses minimal memory
- Starts in milliseconds, not seconds
- Runs close to C/C++ performance levels

**Comparison**: 
- Electron apps (JavaScript) typically use 100-500MB of memory
- Rust GUI apps typically use 10-50MB of memory

**2. Memory Safety**

Rust's ownership system prevents entire classes of bugs at compile time:
- No null pointer dereferences
- No use-after-free errors
- No data races in concurrent code
- No buffer overflows

This makes Doctainr more reliable and secure.

**3. Excellent Library Ecosystem**

For Docker management, Rust provides:
- **Bollard**: Fully async Docker Engine API client
- **Tokio**: High-performance async runtime
- **Serde**: Efficient serialization/deserialization

**4. Developer Experience**

Rust tooling is excellent:
- `cargo`: Build system and package manager
- `rustfmt`: Automatic code formatting
- `clippy`: Powerful linter
- `rust-analyzer`: IDE integration

**5. Cross-Platform**

Rust compiles to native binaries for:
- Linux (x86_64, ARM)
- macOS (Intel, Apple Silicon)
- Windows (x86_64)

Same codebase, different targets.

### Why Dioxus?

**1. React-Like Developer Experience**

Dioxus brings React's component model to Rust:

````rust
#[component]
fn Counter() -> Element {
    let mut count = use_signal(|| 0);
    
    rsx! {
        button { 
            onclick: move |_| count += 1,
            "Count: {count}"
        }
    }
}
````

If you know React, you know Dioxus.

**2. Native Rendering**

Unlike Electron, Dioxus renders using native platform APIs:
- **Linux**: GTK3 WebView
- **macOS**: System WebView
- **Windows**: WebView2

This means:
- Smaller binary size (no bundled Chromium)
- Lower memory usage
- Better OS integration

**3. Type Safety**

Dioxus leverages Rust's type system:
- Props are type-checked at compile time
- RSX macro validates HTML structure
- No runtime errors from typos in component names

**4. Multiple Render Targets**

One codebase can target:
- **Desktop**: Native applications
- **Web**: WebAssembly (Wasm)
- **Server**: Server-side rendering
- **Mobile**: iOS and Android (experimental)

**5. Reactive Signals**

Dioxus 0.7 introduces signals for fine-grained reactivity:

````rust
let mut count = use_signal(|| 0);
let doubled = use_memo(move || count() * 2);

// Only components reading `doubled` re-render when count changes
````

This is more efficient than React's virtual DOM diffing.

**6. Active Development**

Dioxus is actively maintained with:
- Regular releases
- Growing community
- Excellent documentation
- Responsive maintainers

## Alternatives Considered

### Electron + React/Vue

**Pros**:
- Large ecosystem
- Many developers familiar
- Rapid prototyping

**Cons**:
- High memory usage (100-500MB baseline)
- Large bundle size (100+ MB)
- Slow startup time
- Resource intensive

**Verdict**: Defeats the purpose of a "lightweight" Docker Desktop alternative.

### Qt/C++

**Pros**:
- Excellent performance
- Mature framework
- Native look and feel

**Cons**:
- Memory unsafe (requires manual memory management)
- Verbose boilerplate
- Complex build system
- Larger learning curve

**Verdict**: Rust provides similar performance with better safety.

### Python (PyQt/Tkinter)

**Pros**:
- Easy to learn
- Rapid development
- Good Docker library (docker-py)

**Cons**:
- Requires Python runtime
- Slower than compiled languages
- Distribution complexity (packaging Python apps)
- No compile-time type checking

**Verdict**: Not suitable for high-performance desktop apps.

### Go (Fyne/Wails)

**Pros**:
- Fast compilation
- Simple language
- Good Docker library

**Cons**:
- Garbage collector pauses
- Higher memory usage than Rust
- Less mature GUI frameworks
- No memory safety guarantees

**Verdict**: Close second choice, but Rust edges ahead on memory safety and performance.

### Native Swift (macOS) / C# (Windows)

**Pros**:
- Platform-native APIs
- Excellent performance
- Great tooling

**Cons**:
- Platform-specific code (need separate apps for each OS)
- No cross-platform benefits
- Smaller Docker library ecosystems

**Verdict**: Cross-platform requirement rules this out.

## Dioxus vs Other Rust GUI Frameworks

### Tauri

**Comparison**:
- Similar to Dioxus Desktop (uses WebView)
- Frontend is HTML/CSS/JS, backend is Rust
- Requires learning both JavaScript and Rust

**Why Dioxus?**: Single-language development (only Rust).

### Iced

**Comparison**:
- Pure Rust GUI (no WebView)
- Custom rendering engine
- More control, more complexity

**Why Dioxus?**: Easier to style with CSS, React-like patterns.

### egui

**Comparison**:
- Immediate-mode GUI (different paradigm)
- Very lightweight
- Game-focused

**Why Dioxus?**: Retained-mode GUI fits traditional app development better.

## Trade-offs and Limitations

### Learning Curve

**Challenge**: Rust has a steep learning curve
- Ownership and borrowing concepts
- Lifetimes and trait bounds
- Async/await model

**Mitigation**: 
- Excellent error messages
- Comprehensive documentation
- Supportive community

### Ecosystem Maturity

**Challenge**: Dioxus is younger than React or Electron
- Fewer third-party components
- Some features still experimental
- Smaller community

**Mitigation**:
- Core features are stable
- Active development
- Easy to create custom components

### Web Rendering Dependency

**Challenge**: Desktop target uses system WebView
- Requires WebView2 on Windows
- Rendering depends on system browser engine

**Mitigation**:
- WebView2 is auto-installed on Windows 11
- Most Linux distros include WebKitGTK
- macOS includes WebKit by default

## Conclusion

Rust + Dioxus provides:

✅ **Native performance** - Compiled Rust is as fast as C/C++  
✅ **Memory safety** - No segfaults, no data races  
✅ **Small binaries** - <10MB executables  
✅ **Low memory usage** - <50MB RAM typically  
✅ **Modern DX** - React-like component model  
✅ **Cross-platform** - Linux, macOS, Windows from one codebase  
✅ **Type safety** - Catch errors at compile time  
✅ **Future-proof** - Active development, growing community  

For a lightweight Docker Desktop alternative, this combination is ideal.

## Related Documentation

- [Design Principles](design-principles.md) - Application design philosophy
- [Docker Integration](docker-integration.md) - How we connect to Docker
- [Architecture](../reference/architecture.md) - System architecture details
