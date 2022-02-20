use iced::pane_grid;
use iced::scrollable::{self};
use iced::{Element, Size};

mod component;
pub use component::Component;

mod empty;
pub use empty::EmptyComponent;

mod foo;
pub use foo::FooComponent;

pub struct Content {
    pub id: usize,
    root: Box<dyn Component>,
    scroll: scrollable::State,
}

impl Content {
    pub fn new(id: usize, root: Box<dyn Component>) -> Self {
        Self {
            id,
            root,
            scroll: scrollable::State::new(),
        }
    }

    pub fn view(&mut self, pane: pane_grid::Pane, size: Size) -> Element<conga_core::Message> {
        return self.root.view();
    }
}
