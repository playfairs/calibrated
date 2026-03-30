use enigo::{Enigo, Mouse, Settings};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct Clicker {
    active: Arc<AtomicBool>,
    thread_handle: Option<std::thread::JoinHandle<()>>,
}

impl Clicker {
    pub fn new() -> Self {
        Self {
            active: Arc::new(AtomicBool::new(false)),
            thread_handle: None,
        }
    }

    pub fn start(&mut self, delay_ms: u64) {
        if self.active.load(Ordering::Relaxed) {
            return; // Already running
        }

        self.active.store(true, Ordering::Relaxed);
        let active_clone = self.active.clone();

        let handle = thread::spawn(move || {
            let mut enigo = Enigo::new(&Settings::default()).unwrap();

            while active_clone.load(Ordering::Relaxed) {
                enigo
                    .button(enigo::Button::Left, enigo::Direction::Click)
                    .unwrap();

                thread::sleep(Duration::from_millis(delay_ms));
            }
        });

        self.thread_handle = Some(handle);
    }

    pub fn stop(&mut self) {
        self.active.store(false, Ordering::Relaxed);
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }

    pub fn is_running(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
}
