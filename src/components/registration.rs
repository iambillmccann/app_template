use dioxus::prelude::*;

#[component]
pub fn RegistrationForm() -> Element {
    rsx! {
        div {
            class: "my-4",
            form {
                onsubmit: move |event| { log::info!("Submitted! {event:?}") },
                div {
                    class: "mb-3",
                    label { r#for: "email", "Email" }
                    input { name: "email" }
                }
                input { name: "password" }
                br {}
                input { name: "pwd_confirmation" }
                br {}
                input { r#type: "submit" }
            }
        }
    }
}
