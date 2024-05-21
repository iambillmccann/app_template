use crate::components::RegistrationForm;
use dioxus::prelude::*;

pub fn RegistrationPage() -> Element {
    rsx! {
        main {
            class: "flex flex-col md:flex-row-reverse md:h-screen bg-black text-white",
            section {
                class: "relative flex items-center justify-center w-full md:w-1/3 px-4 md:px-0 border-l border-gray-600", // Use border-left for the vertical line
                div {
                    class: "absolute inset-0 flex items-center justify-center",
                    span {
                        class: "px-4 text-lg font-semibold bg-black text-white", // Adjust padding and background to match design
                        "Logo"
                    }
                }
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
