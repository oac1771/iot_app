use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        Title {}
        Scanner {}
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        div { id: "title",
            h1 { "Iot" }
        }
    }
}

#[component]
fn Scanner() -> Element {
    rsx! {
        div { id: "scanner-container",
            button { id: "buttons", "scan" }
        }
    }
}
