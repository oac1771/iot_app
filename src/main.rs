use dioxus::prelude::*;
use rand::Rng;

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
        div {
            id: "title",
            h1 { "Iot" }
        }
    }
}

#[component]
fn Scanner() -> Element {
    let mut trigger = use_signal(|| 0);

    let scan_results = use_resource(move || {
        let _ = trigger(); // dependency — will rerun when trigger changes
        async move { scan().await }
    });

    let scan = match scan_results() {
        Some(Ok(results)) => rsx! {
            div {
                h1 { "Scan results" }
                for item in results.iter() {
                    div { "Item {item}" }
                }
            }
        },
        Some(Err(err)) => rsx! { p { "Error Scanning: {err}" } },
        None => rsx! { p { "Loading..." } },
    };

    rsx! {
        div {
            button {
                onclick: move |_| trigger.set(trigger() + 1),
                "Scan!"
            }
            {scan}
        }
    }
}

fn scan() -> impl Future<Output = dioxus::Result<Vec<i32>, String>> {
    async move {
        let mut rng = rand::thread_rng();
        let random_length = rng.gen_range(5..=15);
        Ok((0..random_length).map(|_| rng.gen_range(0..100)).collect())
    }
}
