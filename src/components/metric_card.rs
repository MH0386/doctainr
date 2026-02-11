use dioxus::prelude::*;

#[component]
pub fn MetricCard(title: String, value: String, hint: Option<String>) -> Element {
    rsx! {
        div { class: "card",
            p { class: "card-title", "{title}" }
            p { class: "card-value", "{value}" }
            if let Some(hint) = hint {
                p { class: "card-hint", "{hint}" }
            }
        }
    }
}
