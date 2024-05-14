// src/pages/mod.rs

pub mod home;
pub use home::HomePage;

pub mod landing;
pub use landing::LandingPage;

pub mod page_not_found;
pub use page_not_found::PageNotFound;

pub mod app_settings;
pub use app_settings::AppSettingsPage;

pub mod registration;
pub use registration::RegistrationPage;
