// src/pages/landing.rs
extern crate dotenv;

use dioxus::{html::p, prelude::*};
use dotenv::dotenv;
use std::env;
// use config::{Config, Environment};

#[component]
pub fn AppSettingsPage(items: String) -> Element {
    // let mut settings = Config::default();
    let mut message: String = String::from("This is Ok."); // Change the type of `message` to `String`

    dotenv().ok();
    match env::var("firebase_project_id") {
        Ok(firebase_project_id) => {
            message = format!("Read .env successfully {}", firebase_project_id)
        }
        Err(e) => message = format!("Read .env failed, {}", e),
    }

    println!("{}", message);
    // match settings.merge(Environment::with_prefix("APP")) {
    //     Ok(_) => (),
    //     Err(_) => eprintln!("Failed to load environment variables"),
    // }

    // let items_clone = items.clone();
    // let message_clone = message.clone();

    rsx! {
        div {
            class: "flex items-center justify-center h-screen bg-black text-white text-4xl",
            div {
                p { "Show this: {items}" },
                p { "{message}" },
            }
        }
    }
}
