use iced::{Container, Element, Length, Text};

use super::Component;

pub struct EmptyComponent {}

impl Component for EmptyComponent {
    fn view(&self) -> Element<conga_core::Message> {
        let greet = Text::new("[EMPTY]");
        Container::new(greet)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .center_x()
            .center_y()
            .into()
    }
}
