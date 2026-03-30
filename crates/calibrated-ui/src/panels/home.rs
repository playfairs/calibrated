pub struct HomePanel {
}

impl HomePanel {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn view(&self) -> iced::Element<'_, crate::Message> {
        iced::widget::text("Home - Autoclicker Controls").into()
    }
}
