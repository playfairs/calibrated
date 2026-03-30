pub struct Sidebar {
}

impl Sidebar {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn view(&self) -> iced::Element<'_, crate::Message> {
        iced::widget::text("Sidebar").into()
    }
}
