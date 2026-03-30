pub struct Header {
}

impl Header {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn view(&self) -> iced::Element<'_, crate::Message> {
        iced::widget::text("Header").into()
    }
}
