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
    let mut count = use_signal(|| 0);
    let current = count.read().clone();

    rsx! {
        div { 
            id: "scanner-container",
            button { 
                onclick: move |_| count += 1,
                id: "buttons", 
                "Increment ({current})"
            }
        }
    }
}
