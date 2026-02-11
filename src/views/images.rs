use dioxus::prelude::*;

use crate::components::SectionHeader;
use crate::utils::AppState;

#[component]
pub fn Images() -> Element {
    let app_state = use_context::<AppState>();
    let images = (app_state.images)();

    rsx! {
        SectionHeader {
            title: "Images".to_string(),
            subtitle: Some("Local image cache".to_string())
        }

        div { class: "table",
            div { class: "row header",
                span { "Repository" }
                span { "Tag" }
                span { "Image ID" }
                span { "Size" }
            }
            for image in images {
                div { class: "row item images-row",
                    span { "{image.repository}" }
                    span { "{image.tag}" }
                    span { "{image.id}" }
                    span { "{image.size}" }
                }
            }
        }
    }
}
