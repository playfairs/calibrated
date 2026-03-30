pub struct MacrosPanel {
}

impl MacrosPanel {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn view(&self) -> iced::Element<'_, crate::Message> {
        iced::widget::text("Macros - Coming Soon").into()
    }
}
