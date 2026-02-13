# API Reference

This document provides technical reference for the Containr codebase.

## Table of Contents

- [Components](#components)
- [Services](#services)
- [State Management](#state-management)
- [Views](#views)
- [Routing](#routing)

## Components

### MetricCard

Displays a metric value with title and optional hint text.

**Props:**
- `title: String` - The metric title
- `value: String` - The metric value to display
- `hint: Option<String>` - Optional hint text below the value

**Example:**
``````rust
rsx! {
    MetricCard {
        title: "Running containers".to_string(),
        value: "5".to_string(),
        hint: Some("Across all projects".to_string())
    }
}
``````

### SectionHeader

Page header component with title and optional subtitle.

**Props:**
- `title: String` - Section title
- `subtitle: Option<String>` - Optional subtitle text

**Example:**
``````rust
rsx! {
    SectionHeader {
        title: "Dashboard".to_string(),
        subtitle: Some("Overview of your Docker engine".to_string())
    }
}
``````

### StatusPill

Visual indicator for container status.

**Props:**
- `state: ContainerState` - The container state (Running or Stopped)

**Example:**
``````rust
rsx! {
    StatusPill { state: ContainerState::Running }
}
``````

## Services

### docker.rs

Provides Docker-related data structures and mock data functions.

#### Types

##### ContainerState

``````rust
pub enum ContainerState {
    Running,
    Stopped,
}
``````

**Methods:**
- `label() -> &'static str` - Returns "Running" or "Stopped"
- `css_class() -> &'static str` - Returns CSS class name for styling
- `action_label() -> &'static str` - Returns "Stop" or "Start" for buttons

##### ContainerInfo

``````rust
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub ports: String,
    pub state: ContainerState,
}
``````

Container information with ID, name, image, status, exposed ports, and current state.

##### ImageInfo

``````rust
pub struct ImageInfo {
    pub id: String,
    pub repository: String,
    pub tag: String,
    pub size: String,
}
``````

Docker image metadata including repository, tag, and size.

##### VolumeInfo

``````rust
pub struct VolumeInfo {
    pub name: String,
    pub driver: String,
    pub mountpoint: String,
    pub size: String,
}
``````

Docker volume information with driver type, mount point, and storage size.

#### Functions

##### mock_containers

``````rust
pub fn mock_containers() -> Vec<ContainerInfo>
``````

Returns mock container data for development and testing.

##### mock_images

``````rust
pub fn mock_images() -> Vec<ImageInfo>
``````

Returns mock Docker image data.

##### mock_volumes

``````rust
pub fn mock_volumes() -> Vec<VolumeInfo>
``````

Returns mock volume data.

## State Management

### AppState

Global application state managed via Dioxus Context API.

``````rust
pub struct AppState {
    pub docker_host: Signal<String>,
    pub containers: Signal<Vec<ContainerInfo>>,
    pub images: Signal<Vec<ImageInfo>>,
    pub volumes: Signal<Vec<VolumeInfo>>,
    pub last_action: Signal<Option<String>>,
}
``````

**Fields:**
- `docker_host` - Docker daemon connection URL
- `containers` - List of all containers
- `images` - List of all images
- `volumes` - List of all volumes
- `last_action` - Most recent user action message

**Methods:**

##### new

``````rust
pub fn new() -> Self
``````

Creates a new AppState with default values and mock data.

##### set_container_state

``````rust
pub fn set_container_state(&self, id: &str, next_state: ContainerState)
``````

Updates a container's state and records the action.

**Parameters:**
- `id` - Container ID to update
- `next_state` - New container state (Running or Stopped)

##### record_action

``````rust
pub fn record_action(&self, message: impl Into<String>)
``````

Records a user action message to display in the UI.

**Parameters:**
- `message` - Action description

### Using AppState

**Providing state (in App component):**
``````rust
let app_state = AppState::new();
use_context_provider(|| app_state);
``````

**Consuming state (in child components):**
``````rust
let app_state = use_context::<AppState>();
let containers = (app_state.containers)();
``````

## Views

### Dashboard

Overview page displaying Docker engine metrics.

**Route:** `/`

**State:**
- Reads `containers`, `images`, `volumes`, `docker_host` from AppState

**Features:**
- Shows count of running and stopped containers
- Displays total images and volumes
- Shows Docker engine connection info

### Containers

Container management interface.

**Route:** `/containers`

**State:**
- Reads and updates `containers` from AppState

**Features:**
- Lists all containers with details
- Start/stop controls for each container
- Status indicators

### Images

Docker image browser.

**Route:** `/images`

**State:**
- Reads `images` from AppState

**Features:**
- Displays image repository, tag, ID, and size
- Formatted table view

### Volumes

Volume management view.

**Route:** `/volumes`

**State:**
- Reads `volumes` from AppState

**Features:**
- Lists all volumes with details
- Shows driver, mount point, and size

### Settings

Configuration interface.

**Route:** `/settings`

**State:**
- Reads and updates `docker_host` from AppState

**Features:**
- Docker host URL configuration
- Connection settings

### AppShell

Main application layout with navigation.

**Features:**
- Sidebar navigation
- Page header with last action display
- Route outlet for page content

## Routing

Routes are defined in the `Route` enum:

``````rust
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
``````

All routes use the `AppShell` layout, which provides consistent navigation and header.

**Navigation example:**
``````rust
rsx! {
    Link { to: Route::Dashboard {}, "Go to Dashboard" }
}
``````

## Styling

Styles are defined in `assets/styling/main.css` with a dark theme:

**Key CSS classes:**
- `.app-shell` - Main application container
- `.sidebar` - Navigation sidebar
- `.nav-link` - Navigation links
- `.page` - Page content area
- `.card` - Content card
- `.action-bar` - Container action buttons
- `.button.primary` - Primary action button
- `.button.secondary` - Secondary action button
- `.error-message` - Error display

## Assets

Assets are loaded using the `asset!` macro:

``````rust
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
``````

Asset paths are relative to the project root.
