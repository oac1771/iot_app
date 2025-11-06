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
        div { 
            id: "title",
            h1 { "Iot" }
        }
    }
}

#[component]
fn Scanner() -> Element {
    let scan_results = use_resource(|| scan());

    let scan_results = match scan_results() {
        Some(Ok(src)) => src,
        _ => {
            return rsx! {
                p { "Loading or error..." }
            };
        }
    };

    rsx! {
        div {
            h1 { "Scan results" }
        }
        for idx in scan_results {
            div {
                "Item {idx}"
            }
        }
    }

    // #[derive(serde::Deserialize)]
    // struct DogApi {
    //     message: String,
    // }

    // let mut img_src = use_signal(|| "image.png".to_string());

    // let fetch_new = move |_| async move {
    //     let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
    //         .await
    //         .unwrap()
    //         .json::<DogApi>()
    //         .await
    //         .unwrap();

    //     img_src.set(response.message);
    // };

    // rsx! {
    //     img { src: img_src }
    //     button { onclick: fetch_new, "Fetch a new dog!" }
    // }

    // let mut count = use_signal(|| 0);
    // let current = count.read().clone();

    // rsx! {
    //     div {
    //         id: "scanner-container",
    //         button {
    //             onclick: move |_| count += 1,
    //             id: "buttons",
    //             "Increment ({current})"
    //         }
    //     }
    // }
}

fn scan() -> impl Future<Output = dioxus::Result<Vec<String>, String>> {
    async move { Ok(vec!["foo".into(), "bar".into()]) }
}
