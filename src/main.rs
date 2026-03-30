mod app;

use app::App;
use iced::{Application, Command, Element, Length, Settings, widget::container};

#[derive(Debug, Clone)]
pub enum Message {
    StartStop,
    UpdateCps(String),
    UpdateHours(String),
    UpdateMinutes(String),
    UpdateSeconds(String),
    UpdateMilliseconds(String),
    ToggleRandom,
    UpdateRandomMin(String),
    UpdateRandomMax(String),
}

struct CalibratedApp {
    app: App,
}

impl Application for CalibratedApp {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self { app: App::new() }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Calibrated")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::StartStop => {
                if self.app.clicker().is_running() {
                    self.app.clicker_mut().stop();
                    self.app.content_mut().set_running(false);
                    self.app.content_mut().set_cps(0.0);
                } else {
                    let delay_ms = self.app.scheduler().calculate_delay_ms();
                    self.app.clicker_mut().start(delay_ms);
                    self.app.content_mut().set_running(true);
                    if let Ok(cps) = self.app.settings().get_cps_input().parse::<f64>() {
                        self.app.content_mut().set_cps(cps);
                    }
                }
            }
            Message::UpdateCps(value) => {
                self.app.settings_mut().set_cps_input(value);
            }
            Message::UpdateHours(value) => {
                self.app.scheduler_mut().set_hours(value);
            }
            Message::UpdateMinutes(value) => {
                self.app.scheduler_mut().set_minutes(value);
            }
            Message::UpdateSeconds(value) => {
                self.app.scheduler_mut().set_seconds(value);
            }
            Message::UpdateMilliseconds(value) => {
                self.app.scheduler_mut().set_milliseconds(value);
            }
            Message::ToggleRandom => {
                let current = self.app.scheduler().is_random_enabled();
                self.app.scheduler_mut().set_random_enabled(!current);
            }
            Message::UpdateRandomMin(value) => {
                self.app.scheduler_mut().set_random_min(value);
            }
            Message::UpdateRandomMax(value) => {
                self.app.scheduler_mut().set_random_max(value);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let scheduler = self.app.scheduler();
        let content_view = self.app.content().view(
            scheduler.get_hours(),
            scheduler.get_minutes(),
            scheduler.get_seconds(),
            scheduler.get_milliseconds(),
            scheduler.is_random_enabled(),
            scheduler.get_random_min(),
            scheduler.get_random_max(),
        );

        container(content_view)
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
    CalibratedApp::run(Settings {
        window: iced::window::Settings {
            transparent: true,
            size: iced::Size::new(480.0, 600.0),
            min_size: Some(iced::Size::new(480.0, 600.0)),
            max_size: Some(iced::Size::new(480.0, 600.0)),
            resizable: true,
            ..Default::default()
        },
        default_text_size: iced::Pixels(14.0),
        ..Default::default()
    })
}
