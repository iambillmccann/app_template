// src/pages/landing.rs

use config::{Config, Environment, File};
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn AppSettingsPage(items: String) -> Element {
    let mut settings = Config::default();
    match settings.merge(File::with_name("config")) {
        Ok(_) => (),
        Err(_) => eprintln!("Failed to load config file"),
    }

    match settings.merge(Environment::with_prefix("APP")) {
        Ok(_) => (),
        Err(_) => eprintln!("Failed to load environment variables"),
    }

    let firebase_settings = match settings.get::<HashMap<String, String>>("firebase") {
        Ok(settings) => settings,
        Err(e) => {
            eprintln!("Failed to load Firebase settings: {}", e);
            HashMap::new() // Use an empty HashMap as the default settings
        }
    };

    let firebase_settings_str = format!("{:?}", firebase_settings);

    rsx! {
        div {
            class: "flex items-center justify-center h-screen bg-black text-white text-4xl",
            div {
                p { "Show this: {items}" },
                p { "Firebase settings: {firebase_settings_str}" }
            }
        }
    }
}
