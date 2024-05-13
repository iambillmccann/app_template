// src/pages/landing.rs
extern crate dotenv;

use dioxus::{html::p, prelude::*};
use std::env;

#[component]
pub fn AppSettingsPage(items: String) -> Element {
    let firebase_project_id = env::var("FIREBASE_PROJECT_ID").unwrap_or_default();
    log::info!("Firebase Project ID: {}", firebase_project_id);

    rsx! {
        div {
            class: "flex items-center justify-center h-screen bg-black text-white text-4xl",
            div {
                p { "Show this: {items}" },
                p { "Project id: {firebase_project_id}" },
            }
        }
    }
}
