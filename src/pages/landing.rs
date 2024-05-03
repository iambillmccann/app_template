// src/pages/landing.rs

use dioxus::prelude::*;

pub fn LandingPage() -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center h-screen bg-black text-white text-4xl",
            "Hello World"
        }
    }
}
