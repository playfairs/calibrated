#[derive(Debug, Clone)]
pub struct Settings {
    pub theme: String,
    pub auto_start: bool,
    pub default_delay: u32,
    pub cps_input: String,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            theme: "dark".to_string(),
            auto_start: false,
            default_delay: 100,
            cps_input: "10.0".to_string(),
        }
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // TODO: Implement actual file loading
        Ok(Self::new())
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implement actual file saving
        Ok(())
    }

    pub fn set_cps_input(&mut self, cps: String) {
        self.cps_input = cps;
    }

    pub fn get_cps_input(&self) -> &str {
        &self.cps_input
    }

    pub fn get_cps(&self) -> f64 {
        self.cps_input.parse::<f64>().unwrap_or(0.0)
    }
}
