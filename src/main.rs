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
        DogView {}
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
fn DogView() -> Element {
    rsx! {
        div { id: "buttons",
            button { id: "scan", "scan" }
        }

        { (0..10).map(|idx| rsx! {
            div {
                h1 {"item {idx}"}
            }
        })}
    }
}
