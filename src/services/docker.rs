#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContainerState {
    Running,
    Stopped,
}

impl ContainerState {
    pub fn label(&self) -> &'static str {
        match self {
            ContainerState::Running => "Running",
            ContainerState::Stopped => "Stopped",
        }
    }

    pub fn css_class(&self) -> &'static str {
        match self {
            ContainerState::Running => "running",
            ContainerState::Stopped => "stopped",
        }
    }

    pub fn action_label(&self) -> &'static str {
        match self {
            ContainerState::Running => "Stop",
            ContainerState::Stopped => "Start",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub ports: String,
    pub state: ContainerState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImageInfo {
    pub id: String,
    pub repository: String,
    pub tag: String,
    pub size: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VolumeInfo {
    pub name: String,
    pub driver: String,
    pub mountpoint: String,
    pub size: String,
}

pub fn mock_containers() -> Vec<ContainerInfo> {
    vec![
        ContainerInfo {
            id: "1a2b3c4d".to_string(),
            name: "api-gateway".to_string(),
            image: "nginx:1.25".to_string(),
            status: "Up 12 minutes".to_string(),
            ports: "80:8080".to_string(),
            state: ContainerState::Running,
        },
        ContainerInfo {
            id: "5e6f7g8h".to_string(),
            name: "payments".to_string(),
            image: "rust-payments:local".to_string(),
            status: "Exited (0) 2 hours ago".to_string(),
            ports: "--".to_string(),
            state: ContainerState::Stopped,
        },
        ContainerInfo {
            id: "9i0j1k2l".to_string(),
            name: "redis".to_string(),
            image: "redis:7".to_string(),
            status: "Up 4 days".to_string(),
            ports: "6379:6379".to_string(),
            state: ContainerState::Running,
        },
    ]
}

pub fn mock_images() -> Vec<ImageInfo> {
    vec![
        ImageInfo {
            id: "sha256:aa11".to_string(),
            repository: "nginx".to_string(),
            tag: "1.25".to_string(),
            size: "146MB".to_string(),
        },
        ImageInfo {
            id: "sha256:bb22".to_string(),
            repository: "redis".to_string(),
            tag: "7".to_string(),
            size: "117MB".to_string(),
        },
        ImageInfo {
            id: "sha256:cc33".to_string(),
            repository: "rust-payments".to_string(),
            tag: "local".to_string(),
            size: "512MB".to_string(),
        },
    ]
}

pub fn mock_volumes() -> Vec<VolumeInfo> {
    vec![
        VolumeInfo {
            name: "containr-cache".to_string(),
            driver: "local".to_string(),
            mountpoint: "/var/lib/docker/volumes/containr-cache".to_string(),
            size: "2.4GB".to_string(),
        },
        VolumeInfo {
            name: "postgres-data".to_string(),
            driver: "local".to_string(),
            mountpoint: "/var/lib/docker/volumes/postgres-data".to_string(),
            size: "8.1GB".to_string(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_data_contains_running_container() {
        let containers = mock_containers();
        assert!(containers
            .iter()
            .any(|container| container.state == ContainerState::Running));
    }

    #[test]
    fn container_state_labels_match() {
        assert_eq!(ContainerState::Running.label(), "Running");
        assert_eq!(ContainerState::Stopped.label(), "Stopped");
    }
}
