use crate::components::RegistrationForm;
use dioxus::prelude::*;

pub fn RegistrationPage() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center h-screen bg-black text-white",
            div {
                h1 { class: "text-4xl", "Sign up" }
                p { "Already have an account? Sign in." }
                RegistrationForm {}
            }
        }
    }
}
