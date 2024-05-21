use crate::components::RegistrationForm;
use dioxus::prelude::*;

pub fn RegistrationPage() -> Element {
    rsx! {
        main {
            class: "flex items-center justify-center h-screen bg-black text-white",
            div {
                class: "w-full max-w-md space-y-6 p-4",
                h2 { class: "text-2xl font-semibold text-center", "Sign up" }
                p { class: "text-sm text-gray-400 text-center", "Already have an account? Sign in." }
                RegistrationForm {}
            }
        }
    }
}
