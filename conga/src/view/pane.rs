use iced_lazy::responsive;

use super::content::Content;
use super::controls::Controls;

pub struct Pane {
    pub responsive: responsive::State,
    pub content: Content,
    pub controls: Controls,
}

impl Pane {
    pub fn new(id: usize) -> Self {
        Self {
            responsive: responsive::State::new(),
            content: Content::new(id, id == Pane::EMPTY_ID),
            controls: Controls::new(),
        }
    }
    pub const EMPTY_ID: usize = usize::MAX;
}
