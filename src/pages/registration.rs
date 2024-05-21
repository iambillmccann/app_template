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
                class: "flex justify-center items-center w-full md:w-2/3 px-4 md:px-0",
                div {
                    class: "w-full max-w-md space-y-6",
                    h2 { class: "text-xl font-semibold md:text-2xl", "Sign up" }
                    p { class: "text-sm text-gray-400", "Already have an account? Sign in." }
                    RegistrationForm {}
                }
            }
        }
    }
}
