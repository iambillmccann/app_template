use dioxus::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn use_state<T: 'static + Clone>(initial_value: T) -> (Rc<RefCell<T>>, Box<dyn Fn(T)>) {
    let state = use_hook(|| Rc::new(RefCell::new(initial_value.clone())));
    let set_state = {
        let state = state.clone();
        Box::new(move |new_value: T| {
            *state.borrow_mut() = new_value;
            force_update();
        })
    };
    (state.clone(), set_state)
}

fn force_update() {
    use_effect(|| {});
}

#[component]
pub fn RegistrationForm() -> Element {
    let (password, set_pwd) = use_state(|| "".to_string());
    let (password_confirmation, set_pwd_confirm) = use_state(|| "".to_string());
    let (error, set_error) = use_state(|| None);

    rsx! {
        form {

            onsubmit: move |event| {
                if *password.borrow() != *password_confirmation.borrow() {
                    set_error(Some("Passwords do not match".to_string()));
                    return;
                }
                set_error(None);
                log::info!("Submitted! {event:?}")
            },

            class: "space-y-4",
            div {
                class: "space-y-4",
                // Email Field
                div {
                    label {
                        class: "block text-sm font-medium text-gray-300",
                        "Email"
                    }
                    input {
                        class: "block w-full mt-1 p-2.5 bg-gray-800 border border-gray-600 text-gray-300 rounded focus:ring-blue-500 focus:border-blue-500",
                        "type": "email",
                        name: "email",
                        placeholder: "Email",
                        required: true
                    }
                }
                // Password Field
                div {
                    label {
                        class: "block text-sm font-medium text-gray-300",
                        "Password"
                    }
                    input {
                        class: "block w-full mt-1 p-2.5 bg-gray-800 border border-gray-600 text-gray-300 rounded focus:ring-blue-500 focus:border-blue-500",
                        "type": "password",
                        name: "password",
                        placeholder: "Password",
                        required: true
                    }
                }
                // Password Confirmation Field
                div {
                    label {
                        class: "block text-sm font-medium text-gray-300",
                        "Password confirmation"
                    }
                    input {
                        class: "block w-full mt-1 p-2.5 bg-gray-800 border border-gray-600 text-gray-300 rounded focus:ring-blue-500 focus:border-blue-500",
                        "type": "password",
                        name: "password_confirmation",
                        placeholder: "Password confirmation",
                        required: true
                    }
                }
                // Terms and Conditions
                div {
                    class: "flex items-start",
                    input {
                        class: "h-4 w-4 mt-1 text-blue-600 bg-gray-700 border-gray-600 rounded focus:ring-blue-500",
                        "type": "checkbox",
                        name: "terms"
                    }
                    label {
                        class: "ml-2 text-sm text-gray-400",
                        "I've read and agree to the Terms of Service"
                    }
                }
                // Sign Up Button
                div {
                    button {
                        class: "w-full py-2.5 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded focus:ring-4 focus:ring-blue-500",
                        "type": "submit",
                        "Sign up"
                    }
                }
                // Horizontal Rule with OR Divider
                div {
                    class: "relative flex items-center my-6",
                    div {
                        class: "flex-grow border-t border-gray-600"
                    }
                    span {
                        class: "mx-4 text-sm text-gray-500",
                        "OR"
                    }
                    div {
                        class: "flex-grow border-t border-gray-600"
                    }
                }
                // GitHub Button
                div {
                    button {
                        class: "w-full py-2.5 bg-gray-800 hover:bg-gray-900 text-white font-semibold rounded flex items-center justify-center focus:ring-4 focus:ring-gray-600",
                        "type": "button",
                        // GitHub Icon (use appropriate icon here)
                        span { class: "mr-2", "🔗" } // Placeholder for Google icon
                        "Continue with Google"
                    }
                }
            }
        }
    }
}
