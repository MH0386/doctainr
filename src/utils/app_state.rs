use dioxus::prelude::*;

use crate::services::{
    mock_containers, mock_images, mock_volumes, ContainerInfo, ContainerState, ImageInfo,
    VolumeInfo,
};

#[derive(Clone)]
pub struct AppState {
    pub docker_host: Signal<String>,
    pub containers: Signal<Vec<ContainerInfo>>,
    pub images: Signal<Vec<ImageInfo>>,
    pub volumes: Signal<Vec<VolumeInfo>>,
    pub last_action: Signal<Option<String>>,
}

impl AppState {
    pub fn new() -> Self {
        let docker_host = use_signal(|| "unix:///var/run/docker.sock".to_string());
        let containers = use_signal(mock_containers);
        let images = use_signal(mock_images);
        let volumes = use_signal(mock_volumes);
        let last_action = use_signal(|| None);

        Self {
            docker_host,
            containers,
            images,
            volumes,
            last_action,
        }
    }

    pub fn set_container_state(&self, id: &str, next_state: ContainerState) {
        let message = match next_state {
            ContainerState::Running => format!("Started container {id}"),
            ContainerState::Stopped => format!("Stopped container {id}"),
        };

        // Clone the Signal fields locally so we don't attempt to mutably borrow through `&self`.
        let mut containers_signal = self.containers.clone();
        containers_signal.with_mut(|containers| {
            if let Some(container) = containers.iter_mut().find(|container| container.id == id) {
                container.state = next_state;
                container.status = match next_state {
                    ContainerState::Running => "Up just now".to_string(),
                    ContainerState::Stopped => "Exited (0) just now".to_string(),
                };
            }
        });

        let mut last_action_signal = self.last_action.clone();
        last_action_signal.set(Some(message));
    }

    pub fn record_action(&self, message: impl Into<String>) {
        // Clone the signal and call set on the clone to avoid mutably borrowing through `&self`.
        let mut last_action_signal = self.last_action.clone();
        last_action_signal.set(Some(message.into()));
    }
}
