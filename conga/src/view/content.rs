use iced::alignment::Alignment;
use iced::pane_grid;
use iced::scrollable::{self, Scrollable};
use iced::{Container, Element, Length, Size, Text};

pub struct Content {
    pub id: usize,
    is_empty: bool,
    scroll: scrollable::State,
}

impl Content {
    pub fn new(id: usize, is_empty: bool) -> Self {
        Self {
            id,
            is_empty,
            scroll: scrollable::State::new(),
        }
    }

    pub fn view(&mut self, _pane: pane_grid::Pane, _size: Size) -> Element<conga_core::Message> {
        let Content { id, scroll, .. } = self;

        let greet = match self.is_empty {
            true => Text::new("[EMPTY]"),
            false => Text::new(format!("Content {}", id.to_string())),
        };

        let content = Scrollable::new(scroll)
            .width(Length::Fill)
            .spacing(10)
            .align_items(Alignment::Center)
            .push(greet);
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .center_y()
            .into()
    }
}
