pub struct Scheduler {
    hours: String,
    minutes: String,
    seconds: String,
    milliseconds: String,
    random_enabled: bool,
    random_min: String,
    random_max: String,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            hours: "0".to_string(),
            minutes: "0".to_string(),
            seconds: "0".to_string(),
            milliseconds: "100".to_string(),
            random_enabled: false,
            random_min: "50".to_string(),
            random_max: "150".to_string(),
        }
    }

    pub fn calculate_delay_ms(&self) -> u64 {
        let hours = self.hours.parse::<u64>().unwrap_or(0);
        let minutes = self.minutes.parse::<u64>().unwrap_or(0);
        let seconds = self.seconds.parse::<u64>().unwrap_or(0);
        let milliseconds = self.milliseconds.parse::<u64>().unwrap_or(100);

        let total_ms = hours * 3600 * 1000 + minutes * 60 * 1000 + seconds * 1000 + milliseconds;

        if self.random_enabled {
            let min_ms = self.random_min.parse::<u64>().unwrap_or(50);
            let max_ms = self.random_max.parse::<u64>().unwrap_or(150);
            if min_ms < max_ms {
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                use std::time::{SystemTime, UNIX_EPOCH};

                let mut hasher = DefaultHasher::new();
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
                    .hash(&mut hasher);
                let random_value = hasher.finish() as u64;
                total_ms + (random_value % (max_ms - min_ms)) + min_ms
            } else {
                total_ms
            }
        } else {
            total_ms
        }
    }

    pub fn set_hours(&mut self, hours: String) {
        self.hours = hours;
    }

    pub fn set_minutes(&mut self, minutes: String) {
        self.minutes = minutes;
    }

    pub fn set_seconds(&mut self, seconds: String) {
        self.seconds = seconds;
    }

    pub fn set_milliseconds(&mut self, milliseconds: String) {
        self.milliseconds = milliseconds;
    }

    pub fn set_random_enabled(&mut self, enabled: bool) {
        self.random_enabled = enabled;
    }

    pub fn set_random_min(&mut self, min: String) {
        self.random_min = min;
    }

    pub fn set_random_max(&mut self, max: String) {
        self.random_max = max;
    }

    pub fn get_hours(&self) -> &str {
        &self.hours
    }

    pub fn get_minutes(&self) -> &str {
        &self.minutes
    }

    pub fn get_seconds(&self) -> &str {
        &self.seconds
    }

    pub fn get_milliseconds(&self) -> &str {
        &self.milliseconds
    }

    pub fn is_random_enabled(&self) -> bool {
        self.random_enabled
    }

    pub fn get_random_min(&self) -> &str {
        &self.random_min
    }

    pub fn get_random_max(&self) -> &str {
        &self.random_max
    }
}
