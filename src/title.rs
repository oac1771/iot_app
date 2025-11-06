use dioxus::prelude::*;

#[component]
pub fn Title() -> Element {
    rsx! {
        div {
            id: "title",
            h1 { "Iot" }
        }
    }
}