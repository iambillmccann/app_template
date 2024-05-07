#![allow(non_snake_case)]
mod components;
mod constants;
mod pages;
mod types;

use crate::pages::AppSettingsPage;
use crate::pages::HomePage;
use crate::pages::LandingPage;
use crate::pages::PageNotFound;
use config::{Config, Environment, File};
use std::collections::HashMap;

use dioxus::prelude::*;

// Urls are relative to your Cargo.toml file
const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

// All of our routes will be a variant of this Route enum
#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[route("/")]
    LandingPage {},
    #[route("/home")]
    HomePage {},
    #[route("/settings/:items")]
    AppSettingsPage { items: String },
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

fn main() {
    // Load Firebase settings
    let mut settings = Config::default();
    match settings.merge(File::with_name("config")) {
        Ok(_) => (),
        Err(_) => eprintln!("Failed to load config file"),
    }

    match settings.merge(Environment::with_prefix("APP")) {
        Ok(_) => (),
        Err(_) => eprintln!("Failed to load environment variables"),
    }

    let _firebase_settings = match settings.get::<HashMap<String, String>>("firebase") {
        Ok(settings) => settings,
        Err(e) => {
            eprintln!("Failed to load Firebase settings: {}", e);
            HashMap::new() // Use an empty HashMap as the default settings
        }
    };

    // Now you can use firebase_settings to access your Firebase project settings

    // Launch the application
    launch(|| {
        rsx! { Router::<Route> {} }
    });
}
