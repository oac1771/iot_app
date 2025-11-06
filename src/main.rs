use dioxus::prelude::*;
use scanner::Scanner;
use title::Title;

mod scanner;
mod title;

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
