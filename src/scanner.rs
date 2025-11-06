use dioxus::prelude::*;
use rand::Rng;

#[component]
pub fn Scanner() -> Element {
    let mut trigger = use_signal(|| 0);

    let scan_results = use_resource(move || {
        let count = trigger();
        async move {
            if count == 0 {
                return Ok(Vec::new());
            }
            scan().await
        }
    });

    let scan = match scan_results() {
        Some(Ok(results)) => rsx! {
            div {
                div {
                    id: "title",
                    h1 { "Scan Results" }
                }
                for result in results.iter() {
                    div { 
                        id: "scan_results",
                        "Item {result}" 
                    }
                }
            }
        },
        Some(Err(err)) => rsx! { p { "Error Scanning: {err}" } },
        None => rsx! { p { "Loading..." } },
    };

    rsx! {
        div {
            id: "button-container",
            button {
                id: "button",
                onclick: move |_| trigger.set(trigger() + 1),
                "Scan!"
            }
            button {
                id: "button",
                onclick: move |_| trigger.set(0),
                "Clear"
            }
        }
        {scan}
    }
}

fn scan() -> impl Future<Output = dioxus::Result<Vec<i32>, String>> {
    async move {
        let mut rng = rand::thread_rng();
        let random_length = rng.gen_range(5..=15);
        Ok((0..random_length).map(|_| rng.gen_range(0..100)).collect())
    }
}
