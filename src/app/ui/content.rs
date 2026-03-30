pub struct Content {
}

impl Content {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn view(&self) -> iced::Element<'_, crate::Message> {
        iced::widget::text("Main Content").into()
    }
}
