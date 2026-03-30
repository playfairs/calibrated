pub struct SettingsPanel {}

impl SettingsPanel {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> iced::Element<'_, crate::Message> {
        iced::widget::text("Settings - App Configuration").into()
    }
}
