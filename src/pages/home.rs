// src/pages/home.rs

use crate::components::Preview;
use crate::components::Stories;
use crate::types::PreviewState;
use dioxus::prelude::*;

pub fn HomePage() -> Element {
    use_context_provider(|| Signal::new(PreviewState::Unset));

    rsx! {
        div { class: "flex w-full",
            div { class: "w-1/2 border border-gray-300 bg-gray-50 p-4 m-4", Stories {} }
            div { class: "w-1/2 border border-gray-300 bg-gray-50 p-4 m-4", Preview {} }
        }
    }
}
