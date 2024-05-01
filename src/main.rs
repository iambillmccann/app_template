#![allow(non_snake_case)]
mod components;
mod constants;
mod types;

use dioxus::prelude::*;

use crate::components::Preview;
use crate::components::Stories;
use crate::types::PreviewState;

// Urls are relative to your Cargo.toml file
const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    launch(App);
}

// ToDo ... Make this a page.
pub fn App() -> Element {
    use_context_provider(|| Signal::new(PreviewState::Unset));

    rsx! {
        div { class: "flex w-full",
        div { class: "w-1/2 border border-gray-300 bg-gray-50 p-4 m-4", Stories {} }
        div { class: "w-1/2 border border-gray-300 bg-gray-50 p-4 m-4", Preview {} }
    }
    }
}
