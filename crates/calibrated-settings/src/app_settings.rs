use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
    pub auto_start: bool,
    pub default_delay: u32,
}

impl AppSettings {
    pub fn new() -> Self {
        Self {
            theme: "dark".to_string(),
            auto_start: false,
            default_delay: 100,
        }
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self::new())
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
