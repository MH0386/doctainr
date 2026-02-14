# Documentation Style Guide

This guide explains how to write and maintain documentation for Doctainr.

## Framework: Diátaxis

We follow the [Diátaxis framework](https://diataxis.fr/) for documentation structure:

### 1. Tutorials (Learning-Oriented)

**Purpose**: Help newcomers learn by doing

**Characteristics**:
- Step-by-step instructions
- Concrete example with guaranteed outcome
- Minimal explanation (just enough to proceed)
- Safe environment for exploration

**Example**: "Getting Started with Doctainr" tutorial

**When to write**: For onboarding new users

### 2. How-To Guides (Problem-Oriented)

**Purpose**: Show how to solve specific problems

**Characteristics**:
- Assumes user knows the basics
- Focused on a specific task
- Practical steps to achieve a goal
- May skip explanatory details

**Example**: "Managing Containers" guide

**When to write**: For common workflows and tasks

### 3. Reference (Information-Oriented)

**Purpose**: Provide technical descriptions

**Characteristics**:
- Accurate and complete
- Structured and consistent
- Dry and factual (no tutorial prose)
- Easy to scan and search

**Example**: "Architecture Overview"

**When to write**: For API docs, configuration options, data structures

### 4. Explanation (Understanding-Oriented)

**Purpose**: Deepen understanding of concepts

**Characteristics**:
- Discusses design decisions
- Provides context and background
- Explains "why" not "how"
- May include alternatives and trade-offs

**Example**: "Why Rust and Dioxus?"

**When to write**: For architectural decisions, technology choices

## Writing Style

### Voice and Tone

- **Active voice**: "Click the button" not "The button should be clicked"
- **Second person**: "You can start containers" not "Users can start containers"
- **Present tense**: "The application connects" not "The application will connect"
- **Concise**: Remove unnecessary words
- **Friendly but professional**: Avoid excessive casualness

### Language

- **Plain English**: Avoid jargon when possible
- **Define terms**: Explain technical terms on first use
- **Consistent terminology**: Use the same term for the same concept
- **Inclusive language**: Avoid gendered pronouns, use "they" or restructure

### Formatting

#### Headings

Use sentence case for headings:

````markdown
## This is correct heading style

## This Is Incorrect Heading Style
````

Heading levels:
- `#` - Document title (one per file)
- `##` - Major sections
- `###` - Subsections
- `####` - Rarely needed (consider restructuring)

#### Code Blocks

**Use 4 backticks** to prevent formatting issues:

`````markdown
````rust
fn main() {
    println!("Hello, world!");
}
````
`````

Always specify language for syntax highlighting:
- `rust` - Rust code
- `bash` - Shell commands
- `toml` - TOML configuration
- `json` - JSON data

#### Lists

**Unordered lists** for non-sequential items:

````markdown
- First item
- Second item
- Third item
````

**Ordered lists** for sequential steps:

````markdown
1. First step
2. Second step
3. Third step
````

#### Emphasis

- **Bold** for UI elements: "Click the **Start** button"
- *Italic* for emphasis: "This is *very* important"
- `Code` for code, commands, file names, and keys: "Run `cargo build`"

#### Links

Reference-style links for readability:

````markdown
See the [Architecture Guide](../reference/architecture.md) for details.
````

External links should include the full URL:

````markdown
Learn more at [Dioxus Documentation](https://dioxuslabs.com/learn/0.7/)
````

### Structure

#### Every document should have:

1. **Title** (H1)
2. **Introduction** - What is this document about?
3. **Body** - Main content
4. **Related Links** - Cross-references (optional)

#### Tutorials should have:

1. What you'll learn
2. Prerequisites
3. Step-by-step instructions
4. Troubleshooting
5. Next steps
6. Summary

#### How-To guides should have:

1. Brief description
2. Prerequisites
3. Steps to accomplish task
4. Expected results
5. Troubleshooting (if needed)

#### Reference docs should have:

1. Overview
2. Detailed information (structured consistently)
3. Examples
4. Related documentation

## Code Examples

### Rust Code

- **Correct**: Compiles and follows best practices
- **Complete**: Include necessary imports and context
- **Commented**: Explain non-obvious parts
- **Tested**: Verify examples actually work

````rust
use dioxus::prelude::*;

/// Counter component demonstrating signal usage
#[component]
fn Counter() -> Element {
    let mut count = use_signal(|| 0);
    
    rsx! {
        button { 
            onclick: move |_| *count.write() += 1,
            "Count: {count}"
        }
    }
}
````

### Shell Commands

- Show the command and expected output
- Use comments to explain what's happening
- Include error cases when relevant

````bash
# Check Docker is running
docker info

# Expected output should include:
# Server Version: 24.0.x
# ...
````

### File Paths

- Use forward slashes for cross-platform compatibility
- Show paths relative to project root
- Use placeholders for variables

````
src/components/my_component.rs
target/release/doctainr
````

## Common Patterns

### Prerequisites Section

````markdown
## Prerequisites

Before starting, ensure you have:

1. **Docker** - Version 20.x or higher
   ````bash
   docker --version
   ````

2. **Rust** - Version 1.70 or later
   ````bash
   rustc --version
   ````
````

### Troubleshooting Section

````markdown
## Troubleshooting

### Problem Title

**Problem**: Description of the issue

**Solution**:
1. First step to resolve
2. Second step
3. Verification step
````

### Tips and Notes

Use blockquotes for important information:

````markdown
> **Note**: This feature requires Docker 20.10 or later.

> **Tip**: Use `--release` for production builds.

> **Warning**: This operation cannot be undone.
````

## Maintenance

### Updating Documentation

When code changes:

1. **Review affected docs**: Check which documentation references the changed code
2. **Update examples**: Ensure code examples still compile
3. **Update screenshots**: If UI changed
4. **Update version numbers**: If API versions changed
5. **Check links**: Verify all internal links still work

### Deprecation

When features are deprecated:

1. Mark as deprecated in the docs
2. Explain migration path
3. Keep docs until feature is removed

````markdown
## Using Feature X (Deprecated)

> **Deprecated**: This feature is deprecated as of version 0.2.0 and will be removed in 1.0.0. 
> Use [Feature Y](./feature-y.md) instead.
````

### Archiving Old Documentation

Don't delete old documentation immediately:

1. Move to `docs/archive/` folder
2. Add note explaining it's archived
3. Remove from main navigation

## Accessibility

### Alt Text for Images

Always include descriptive alt text:

````markdown
![Dashboard view showing container metrics and status](images/dashboard.png)
````

### Avoid Visual-Only Instructions

Bad: "Click the red button"  
Good: "Click the **Stop** button (shown in red)"

### Descriptive Links

Bad: [Click here](./guide.md)  
Good: [Read the setup guide](./guide.md)

## Review Checklist

Before submitting documentation:

- [ ] Spell check completed
- [ ] Grammar check completed
- [ ] Code examples tested
- [ ] Links verified
- [ ] Headings use sentence case
- [ ] Code blocks use 4 backticks
- [ ] Images have alt text
- [ ] Follows Diátaxis structure
- [ ] Matches project style guide

## Tools

### Markdown Linters

Consider using:
- [markdownlint](https://github.com/DavidAnson/markdownlint)
- [Vale](https://vale.sh/) for style checking

### Spell Check

- Use your editor's spell checker
- Add technical terms to dictionary

### Link Checking

Periodically check for broken links:
````bash
# Using markdown-link-check (npm package)
npx markdown-link-check docs/**/*.md
````

## Questions?

If you're unsure about documentation style:

1. Look at existing documentation for examples
2. Open an issue to discuss
3. Err on the side of clarity over brevity

## Resources

- [Diátaxis Framework](https://diataxis.fr/)
- [Google Developer Style Guide](https://developers.google.com/style)
- [Microsoft Writing Style Guide](https://learn.microsoft.com/en-us/style-guide/welcome/)
- [Write the Docs](https://www.writethedocs.org/)
