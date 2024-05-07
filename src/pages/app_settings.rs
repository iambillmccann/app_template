// src/pages/landing.rs

use dioxus::prelude::*;

#[component]
pub fn AppSettingsPage(items: String) -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center h-screen bg-black text-white text-4xl",
            "Show this: {items}"
        }
    }
}
