use dioxus::prelude::*;

#[component]
pub fn SectionHeader(title: String, subtitle: Option<String>) -> Element {
    rsx! {
        div { class: "section-header",
            div {
                h2 { "{title}" }
                if let Some(subtitle) = subtitle {
                    p { class: "section-subtitle", "{subtitle}" }
                }
            }
        }
    }
}
