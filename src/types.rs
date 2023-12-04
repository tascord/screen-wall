use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct WindowConfiguration {
    pub screen: i32,
    pub url: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub windows: Vec<WindowConfiguration>,
    pub chrome_path: String,
}