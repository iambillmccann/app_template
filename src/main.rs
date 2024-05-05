#![allow(non_snake_case)]
mod components;
mod constants;
mod pages;
mod types;

use crate::pages::HomePage;
use crate::pages::LandingPage;
use crate::pages::PageNotFound;
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
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

fn main() {
    launch(|| {
        rsx! { Router::<Route> {} }
    });
}
