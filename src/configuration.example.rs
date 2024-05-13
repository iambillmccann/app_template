// This file is used to define the environment variables that are used in the application.
//
// Copy this file to env.rs and replace the dummy values with your actual Firebase project settings.
// Be sure to add env.rs to your .gitignore file to prevent it from being committed to your repository.

pub struct Config {
    pub FIREBASE_API_KEY: String,
    pub FIREBASE_AUTH_DOMAIN: String,
    pub FIREBASE_DATABASE_URL: String,
    pub FIREBASE_PROJECT_ID: String,
    pub FIREBASE_STORAGE_BUCKET: String,
    pub FIREBASE_MESSAGING_SENDER_ID: String,
    pub FIREBASE_APP_ID: String,
    pub FIREBASE_MEASUREMENT_ID: String,
}

pub fn get_config() -> Config {
    Config {
        FIREBASE_PROJECT_ID: "dummy_project_id".to_string(),
        FIREBASE_API_KEY: "dummy_api".to_string(),
        FIREBASE_AUTH_DOMAIN: "dummy_auth".to_string(),
        FIREBASE_DATABASE_URL: "dummy_db".to_string(),
        FIREBASE_STORAGE_BUCKET: "dummy_bucket".to_string(),
        FIREBASE_MESSAGING_SENDER_ID: "dummy_sender".to_string(),
        FIREBASE_APP_ID: "dummy_app".to_string(),
        FIREBASE_MEASUREMENT_ID: "dummy_measurement".to_string(),
    }
}
