use iced::{Container, Element, Length, Text};

use super::Component;

pub struct FooComponent {}

impl Component for FooComponent {
    fn view(&self) -> Element<conga_core::Message> {
        let greet = Text::new("Foo");
        Container::new(greet)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
