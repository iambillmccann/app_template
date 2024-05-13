// src/pages/landing.rs
extern crate dotenv;

use dioxus::{html::p, prelude::*};
// use dotenv::dotenv;
use std::env;
// use config::{Config, Environment};

#[component]
pub fn AppSettingsPage(items: String) -> Element {
    // let mut settings = Config::default();
    // let mut firebase_project_id: String = String::from("This is Ok.");
    let firebase_project_id = env::var("firebase_project_id").unwrap_or_default();
    log::info!("Firebase Project ID: {}", firebase_project_id);
    // const message: String = format!("Read .env successfully {}", firebase_project_id);

    // dotenv().ok();
    // match env::var("firebase_project_id") {
    //     Ok(firebase_project_id) => {
    //         message = format!("Read .env successfully {}", firebase_project_id)
    //     }
    //     Err(e) => message = format!("Read .env failed, {}", e),
    // }

    // println!("{}", message);
    // match settings.merge(Environment::with_prefix("APP")) {
    //     Ok(_) => (),
    //     Err(_) => eprintln!("Failed to load environment variables"),
    // }

    // let items_clone = items.clone();
    // let message_clone = message.clone();

    println!("Firebase Project ID: {}", firebase_project_id);

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
