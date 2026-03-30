use iced::{
    Element, Length, Renderer, Theme,
    widget::{button, checkbox, column, container, row, text, text_input},
};

pub struct Content {
    is_running: bool,
    cps: f64,
}

impl Content {
    pub fn new() -> Self {
        Self {
            is_running: false,
            cps: 0.0,
        }
    }

    pub fn set_running(&mut self, running: bool) {
        self.is_running = running;
    }

    pub fn set_cps(&mut self, cps: f64) {
        self.cps = cps;
    }

    pub fn view<'a>(
        &self,
        hours: &str,
        minutes: &str,
        seconds: &str,
        milliseconds: &str,
        random_enabled: bool,
        random_min: &str,
        random_max: &str,
    ) -> Element<'a, crate::Message> {
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

        column![
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
                            text_input::<crate::Message, Theme, Renderer>("0", hours)
                                .on_input(crate::Message::UpdateHours)
                                .size(10)
                                .padding(6)
                        )
                        .width(Length::Fixed(55.0)),
                        text("h")
                            .size(14)
                            .style(iced::Color::from_rgb(0.7, 0.7, 0.7)),
                        container(
                            text_input::<crate::Message, Theme, Renderer>("0", minutes)
                                .on_input(crate::Message::UpdateMinutes)
                                .size(10)
                                .padding(6)
                        )
                        .width(Length::Fixed(55.0)),
                        text("m")
                            .size(14)
                            .style(iced::Color::from_rgb(0.7, 0.7, 0.7)),
                        container(
                            text_input::<crate::Message, Theme, Renderer>("0", seconds)
                                .on_input(crate::Message::UpdateSeconds)
                                .size(10)
                                .padding(6)
                        )
                        .width(Length::Fixed(55.0)),
                        text("s")
                            .size(14)
                            .style(iced::Color::from_rgb(0.7, 0.7, 0.7)),
                        container(
                            text_input::<crate::Message, Theme, Renderer>("100", milliseconds)
                                .on_input(crate::Message::UpdateMilliseconds)
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
                        checkbox("Enable random interval", random_enabled)
                            .on_toggle(|_| crate::Message::ToggleRandom)
                            .style(iced::theme::Checkbox::Primary),
                        text("Randomize delay between clicks")
                            .size(12)
                            .style(iced::Color::from_rgb(0.6, 0.6, 0.6))
                    ]
                    .spacing(10),
                    row![
                        container(
                            text_input::<crate::Message, Theme, Renderer>("50", random_min)
                                .on_input(crate::Message::UpdateRandomMin)
                                .size(10)
                                .padding(6)
                        )
                        .width(Length::Fixed(75.0)),
                        text("−")
                            .size(18)
                            .style(iced::Color::from_rgb(0.8, 0.8, 0.8)),
                        container(
                            text_input::<crate::Message, Theme, Renderer>("150", random_max)
                                .on_input(crate::Message::UpdateRandomMax)
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
                .on_press(crate::Message::StartStop)
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
        .align_items(iced::Alignment::Center)
        .into()
    }
}
