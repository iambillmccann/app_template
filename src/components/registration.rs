use dioxus::prelude::*;
use dioxus_elements::sub;
use reqwest::Error;
use serde::Deserialize;
use web_sys::window; // This is temporary code

#[derive(Deserialize, Debug)]
struct Config {
    firebase_api_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self, envy::Error> {
        envy::from_env()
    }
}

#[derive(Debug, Deserialize)]
struct FirebaseResponse {
    idToken: String,
    email: String,
    refreshToken: String,
    expiresIn: String,
    localId: String,
}

async fn register_user(email: &str, password: &str) -> Result<FirebaseResponse, Error> {
    let config = Config::from_env().expect("Failed to load config");
    let client = reqwest::Client::new();
    let params = [
        ("email", email),
        ("password", password),
        ("returnSecureToken", "true"),
    ];
    let res = client
        .post(
            "https://identitytoolkit.googleapis.com/v1/accounts:signUp?key=".to_owned()
                + &config.firebase_api_key,
        )
        .json(&params)
        .send()
        .await?
        .json::<FirebaseResponse>()
        .await?;
    Ok(res)
}

#[component]
pub fn RegistrationForm() -> Element {
    let _window = window().expect("no global `window` exists"); // This is temporary code
    let mut password = use_signal(|| "Thisisatest".to_string());
    let mut password_confirmation = use_signal(|| "Thisisalsoatest".to_string());
    let mut not_matched = use_signal(|| false);
    let mut complexity_error = use_signal(|| false);

    let validate = move |event: FormEvent| {
        event.stop_propagation(); // Prevent the form from being submitted
        let values = event.values();
        password.set(values.get("password").unwrap().as_value().to_string());
        password_confirmation.set(
            values
                .get("password_confirmation")
                .unwrap()
                .as_value()
                .to_string(),
        );

        if password.to_string() != password_confirmation.to_string() {
            not_matched.set(true);
            return;
        } else {
            not_matched.set(false);
        }

        // Password complexity check
        let binding = password.to_string();
        let pass = binding.as_str();

        let length_ok = pass.len() >= 12;
        let has_uppercase = pass.chars().any(|c| c.is_uppercase());
        let has_lowercase = pass.chars().any(|c| c.is_lowercase());
        let has_digit = pass.chars().any(|c| c.is_digit(10));
        let special_chars = "~!@#$%^&*-_()[]{}\\|,<.>/?";
        let has_special = pass.chars().any(|c| special_chars.contains(c));

        let mut complexity_count = 0;
        if has_uppercase {
            complexity_count += 1;
        }
        if has_lowercase {
            complexity_count += 1;
        }
        if has_digit {
            complexity_count += 1;
        }
        if has_special {
            complexity_count += 1;
        }

        if length_ok && complexity_count >= 2 {
            complexity_error.set(false);
        } else {
            complexity_error.set(true);
            return;
        }

        // Form submission logic
        let email_value = values.get("email").unwrap().as_value().to_string();
        let password_value = pass.to_string();

        wasm_bindgen_futures::spawn_local(async move {
            match register_user(&email_value, &password_value).await {
                Ok(response) => {
                    // Handle successful registration
                }
                Err(e) => {
                    // Handle error
                }
            }
        });
    };

    rsx! {
        form {
            onsubmit: validate,
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
                        value: password,
                        required: true
                    }
                }
                // Error Message
                if complexity_error() {
                    div {
                        class: "error",
                        "Your password must be more complex."
                    }
                }
                // Password Confirmation Field
                div {
                    label {
                        class: "block text-sm font-medium text-gray-300",
                        "Password Confirmation"
                    }
                    input {
                        class: "block w-full mt-1 p-2.5 bg-gray-800 border border-gray-600 text-gray-300 rounded focus:ring-blue-500 focus:border-blue-500",
                        "type": "password",
                        name: "password_confirmation",
                        placeholder: "Password confirmation",
                        value: password_confirmation,
                        required: true
                    }
                }
                // Error Message
                if not_matched() {
                    div {
                        class: "error",
                        "Passwords do not match."
                    }
                }
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
