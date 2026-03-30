#[derive(Debug, Clone)]
pub struct Settings {
    pub cps_input: String,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            cps_input: "10.0".to_string(),
        }
    }

    pub fn set_cps_input(&mut self, cps: String) {
        self.cps_input = cps;
    }

    pub fn get_cps_input(&self) -> &str {
        &self.cps_input
    }
}
