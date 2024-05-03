#![allow(non_snake_case)]
mod components;
mod constants;
mod pages;
mod types;

use crate::pages::HomePage;
use crate::pages::LandingPage;
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
}

fn main() {
    launch(|| {
        rsx! {
            // style { {include_str!("public/tailwind.css")} }
            Router::<Route> {}
        }
    });
}
