#![allow(non_snake_case)]
mod components;
mod constants;
mod types;

use dioxus::prelude::*;

use crate::components::Preview;
use crate::components::Stories;
use crate::types::PreviewState;

fn main() {
    launch(App);
}

pub fn App() -> Element {
    use_context_provider(|| Signal::new(PreviewState::Unset));

    rsx! {
        div { display: "flex", flex_direction: "row", width: "100%",
            div { width: "50%", Stories {} }
            div { width: "50%", Preview {} }
        }
    }
}
