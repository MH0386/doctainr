use dioxus::prelude::*;

use crate::components::SectionHeader;
use crate::utils::AppState;

#[component]
pub fn Settings() -> Element {
    let app_state = use_context::<AppState>();
    let mut docker_host = app_state.docker_host.clone();
    let app_state_for_test = app_state.clone();
    let app_state_for_save = app_state.clone();

    rsx! {
        SectionHeader {
            title: "Settings".to_string(),
            subtitle: Some("Connection and preferences".to_string())
        }

        div { class: "card",
            label { class: "form-label", "Docker host" }
            input {
                class: "text-input",
                value: docker_host,
                oninput: move |event| docker_host.set(event.value()),
                placeholder: "unix:///var/run/docker.sock"
            }
            div { class: "button-row",
                button {
                    class: "button",
                    onclick: move |_| app_state_for_test.record_action("Tested Docker connection"),
                    "Test connection"
                }
                button {
                    class: "button secondary",
                    onclick: move |_| app_state_for_save.record_action("Saved settings"),
                    "Save"
                }
            }
        }
    }
}
