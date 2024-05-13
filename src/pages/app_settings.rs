use crate::configuration::get_config;
use dioxus::prelude::*;

#[component]
pub fn AppSettingsPage(items: String) -> Element {
    let config = get_config();
    log::info!("Firebase Project ID: {}", config.FIREBASE_PROJECT_ID);

    rsx! {
        div {
            class: "flex items-center justify-center h-screen bg-black text-white text-4xl",
            div {
                p { "Show this: {items}" },
                p { "Project id: {config.FIREBASE_PROJECT_ID}" },
            }
        }
    }
}
