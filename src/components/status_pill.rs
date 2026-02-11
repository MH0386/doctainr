use dioxus::prelude::*;

#[component]
pub fn StatusPill(label: String, class_name: String) -> Element {
    let class_value = format!("pill {class_name}");

    rsx! {
        span { class: "{class_value}", "{label}" }
    }
}
