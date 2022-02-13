use iced::pane_grid;
use iced::{Element, Text};

pub struct Controls {}

impl Controls {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&mut self, _pane: pane_grid::Pane) -> Element<conga_core::Message> {
        Text::new("Ctrls").into()
    }
}
