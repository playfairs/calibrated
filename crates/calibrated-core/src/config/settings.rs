pub struct Settings {}

impl Settings {
    pub fn new() -> Self {
        Self {}
    }

    pub fn load() -> Self {
        Self::new()
    }

    pub fn save(&self) {}
}
