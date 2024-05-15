use crate::components::RegistrationForm;
use dioxus::prelude::*;

pub fn RegistrationPage() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center h-screen bg-black text-white",
            div {
                h2 { class: "text-xl font-semibold md:text-2xl", "Sign up" }
                p { class: "text-secondary", "Already have an account? Sign in." }
                RegistrationForm {}
            }
        }
    }
}
