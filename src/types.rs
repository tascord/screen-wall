use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct WindowConfiguration {
    pub screen: i32,
    pub url: String,
}