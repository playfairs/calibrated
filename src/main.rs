use enigo::{Enigo, MouseControllable};
use iced::{
    Application, Command, Element, Length, Renderer, Settings, Theme,
    widget::{button, checkbox, column, container, row, text, text_input},
};
use std::ffi::CStr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
enum Message {
    StartStop,
    // UpdateCps(String),
    UpdateHours(String),
    UpdateMinutes(String),
    UpdateSeconds(String),
    UpdateMilliseconds(String),
    ToggleRandom,
    UpdateRandomMin(String),
    UpdateRandomMax(String),
}

struct CalibratedApp {
    is_running: bool,
    cps: f64,
    cps_input: String,
    hours: String,
    minutes: String,
    seconds: String,
    milliseconds: String,
    random_enabled: bool,
    random_min: String,
    random_max: String,
    clicker_active: Arc<AtomicBool>,
}

unsafe extern "C" {
    unsafe fn testing_func() -> *const libc::c_char;
}

pub fn testing() -> Option<String> {
    unsafe {
        let ptr = testing_func();
        if ptr.is_null() {
            return None;
        }

        let c_str = CStr::from_ptr(ptr);
        let string = c_str.to_string_lossy().into_owned();

        libc::free(ptr as *mut _);
        Some(string)
    }
}

impl CalibratedApp {
    fn start_clicker_thread(&self) {
        let clicker_active = self.clicker_active.clone();
        let delay_ms = self.calculate_delay_ms();

        thread::spawn(move || {
            let mut enigo = Enigo::new();

            while clicker_active.load(Ordering::Relaxed) {
                let (x, y) = Enigo::mouse_location();
                enigo.mouse_move_to(x, y);
                enigo.mouse_click(enigo::MouseButton::Left);

                thread::sleep(Duration::from_millis(delay_ms));
            }
        });
    }

    fn calculate_delay_ms(&self) -> u64 {
        let hours = self.hours.parse::<u64>().unwrap_or(0);
        let minutes = self.minutes.parse::<u64>().unwrap_or(0);
        let seconds = self.seconds.parse::<u64>().unwrap_or(0);
        let milliseconds = self.milliseconds.parse::<u64>().unwrap_or(100);

        let total_ms = hours * 3600 * 1000 + minutes * 60 * 1000 + seconds * 1000 + milliseconds;

        if self.random_enabled {
            let min_ms = self.random_min.parse::<u64>().unwrap_or(50);
            let max_ms = self.random_max.parse::<u64>().unwrap_or(150);
            total_ms + (min_ms..max_ms).next().unwrap_or(0)
        } else {
            total_ms
        }
    }
}

impl Application for CalibratedApp {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let clicker_active = Arc::new(AtomicBool::new(false));
        (
            Self {
                is_running: false,
                cps: 0.0,
                cps_input: "10.0".to_string(),
                hours: "0".to_string(),
                minutes: "0".to_string(),
                seconds: "0".to_string(),
                milliseconds: "100".to_string(),
                random_enabled: false,
                random_min: "50".to_string(),
                random_max: "150".to_string(),
                clicker_active: clicker_active.clone(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::new()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::StartStop => {
                self.is_running = !self.is_running;
                if self.is_running {
                    if let Ok(cps) = self.cps_input.parse::<f64>() {
                        self.cps = cps;
                    }
                    self.clicker_active.store(true, Ordering::Relaxed);
                    self.start_clicker_thread();
                } else {
                    self.cps = 0.0;
                    self.clicker_active.store(false, Ordering::Relaxed);
                }
            }
            // Message::UpdateCps(value) => self.cps_input = value,
            Message::UpdateHours(value) => self.hours = value,
            Message::UpdateMinutes(value) => self.minutes = value,
            Message::UpdateSeconds(value) => self.seconds = value,
            Message::UpdateMilliseconds(value) => self.milliseconds = value,
            Message::ToggleRandom => self.random_enabled = !self.random_enabled,
            Message::UpdateRandomMin(value) => self.random_min = value,
            Message::UpdateRandomMax(value) => self.random_max = value,
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let button_text = if self.is_running { "STOP" } else { "START" };
        let status = if self.is_running {
            "● RUNNING"
        } else {
            "■ STOPPED"
        };
        let status_color = if self.is_running {
            iced::Color::from_rgb(0.2, 0.8, 0.2)
        } else {
            iced::Color::from_rgb(0.8, 0.2, 0.2)
        };

        let content = column![
            container(row![
                container(
                    text("CALIBRATED")
                        .size(24)
                        .style(iced::Color::from_rgb(0.1, 0.4, 0.8))
                )
                .width(Length::Fill),
                container(text(status).size(14).style(status_color))
                    .padding([8, 16])
                    .style(iced::theme::Container::Box)
            ])
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .padding([15, 25]),
            text("AUTO CLICKER")
                .size(14)
                .style(iced::Color::from_rgb(0.6, 0.6, 0.6)),
            container(
                container(text("").size(1))
                    .width(Length::Fill)
                    .height(Length::Fixed(1.0))
                    .style(iced::theme::Container::Box)
            )
            .width(Length::Fill)
            .padding([0, 35]),
            text(format!("CPS: {:.1}", self.cps))
                .size(32)
                .style(iced::Color::from_rgb(0.9, 0.9, 0.9)),
            container(
                container(text("").size(1))
                    .width(Length::Fill)
                    .height(Length::Fixed(1.0))
                    .style(iced::theme::Container::Box)
            )
            .width(Length::Fill)
            .padding([0, 35]),
            container(
                column![
                    text("CLICK INTERVAL")
                        .size(12)
                        .style(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                    row![
                        container(
                            text_input::<Message, Theme, Renderer>("0", &self.hours)
                                .on_input(Message::UpdateHours)
                                .size(10)
                                .padding(6)
                        )
                        .width(Length::Fixed(55.0)),
                        text("h")
                            .size(14)
                            .style(iced::Color::from_rgb(0.7, 0.7, 0.7)),
                        container(
                            text_input::<Message, Theme, Renderer>("0", &self.minutes)
                                .on_input(Message::UpdateMinutes)
                                .size(10)
                                .padding(6)
                        )
                        .width(Length::Fixed(55.0)),
                        text("m")
                            .size(14)
                            .style(iced::Color::from_rgb(0.7, 0.7, 0.7)),
                        container(
                            text_input::<Message, Theme, Renderer>("0", &self.seconds)
                                .on_input(Message::UpdateSeconds)
                                .size(10)
                                .padding(6)
                        )
                        .width(Length::Fixed(55.0)),
                        text("s")
                            .size(14)
                            .style(iced::Color::from_rgb(0.7, 0.7, 0.7)),
                        container(
                            text_input::<Message, Theme, Renderer>("100", &self.milliseconds)
                                .on_input(Message::UpdateMilliseconds)
                                .size(10)
                                .padding(6)
                        )
                        .width(Length::Fixed(75.0)),
                        text("ms")
                            .size(14)
                            .style(iced::Color::from_rgb(0.7, 0.7, 0.7))
                    ]
                    .spacing(10)
                ]
                .spacing(10)
            )
            .padding(18)
            .width(Length::Fill)
            .style(iced::theme::Container::Box),
            container(
                column![
                    row![
                        checkbox("Enable random interval", self.random_enabled)
                            .on_toggle(|_| Message::ToggleRandom)
                            .style(iced::theme::Checkbox::Primary),
                        text("Randomize delay between clicks")
                            .size(12)
                            .style(iced::Color::from_rgb(0.6, 0.6, 0.6))
                    ]
                    .spacing(10),
                    row![
                        container(
                            text_input::<Message, Theme, Renderer>("50", &self.random_min)
                                .on_input(Message::UpdateRandomMin)
                                .size(10)
                                .padding(6)
                        )
                        .width(Length::Fixed(75.0)),
                        text("−")
                            .size(18)
                            .style(iced::Color::from_rgb(0.8, 0.8, 0.8)),
                        container(
                            text_input::<Message, Theme, Renderer>("150", &self.random_max)
                                .on_input(Message::UpdateRandomMax)
                                .size(10)
                                .padding(6)
                        )
                        .width(Length::Fixed(75.0)),
                        text("ms")
                            .size(14)
                            .style(iced::Color::from_rgb(0.7, 0.7, 0.7))
                    ]
                    .spacing(10)
                ]
                .spacing(10)
            )
            .padding(18)
            .width(Length::Fill)
            .style(iced::theme::Container::Box),
            container(
                button(
                    container(text(button_text).size(16).style(iced::Color::WHITE))
                        .padding([12, 0])
                        .width(Length::Fill)
                        .align_x(iced::alignment::Horizontal::Center)
                )
                .on_press(Message::StartStop)
                .padding(0)
                .style(if self.is_running {
                    iced::theme::Button::Destructive
                } else {
                    iced::theme::Button::Primary
                })
            )
            .width(Length::Fill)
            .padding([0, 50])
        ]
        .spacing(18)
        .align_items(iced::Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(25)
            .style(iced::theme::Container::Transparent)
            .into()
    }
}

fn main() -> iced::Result {
    println!("{:#?}", testing());
    CalibratedApp::run(Settings {
        window: iced::window::Settings {
            transparent: true,
            size: iced::Size::new(480.0, 600.0),
            min_size: Some(iced::Size::new(480.0, 600.0)),
            max_size: Some(iced::Size::new(480.0, 600.0)),
            resizable: false,
            ..Default::default()
        },
        default_text_size: iced::Pixels(14.0),
        ..Default::default()
    })
}
