use dioxus::prelude::*;

#[component]
pub fn RegistrationForm() -> Element {
    rsx! {
        div { padding: "0.5rem",
            p { "This is where the registration form will go" }
        }
    }
}
