use dioxus::prelude::*;

#[component]
pub fn RegistrationForm() -> Element {
    rsx! {
        div {
            class: "my-4",
            form {
                onsubmit: move |event| { log::info!("Submitted! {event:?}") },
                input { name: "email" }
                br {}
                input { name: "password" }
                br {}
                input { name: "pwd_confirmation" }
                br {}
                input { r#type: "submit" }
            }
        }
    }
}
