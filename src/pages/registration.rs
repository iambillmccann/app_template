use crate::components::RegistrationForm;
use dioxus::prelude::*;

pub fn RegistrationPage() -> Element {
    rsx! {
        main {
            class: "flex flex-col md:flex-row-reverse md:h-screen bg-black text-white",
            section {
                class: "flex items-start w-full px-4 mx-auto md:px-0 md:items-center md:w-1/3",
                p { "Logo" }
            }
            section {
                class: "justify-center px-4 md:px-0 md:flex md:w-2/3 md:border-r",
                div {
                    h2 { class: "text-xl font-semibold md:text-2xl", "Sign up" }
                    p { class: "text-secondary", "Already have an account? Sign in." }
                    RegistrationForm {}
                }
            }
        }
    }
}
