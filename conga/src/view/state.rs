use iced::canvas;
use iced::pane_grid::{self};

use super::pane::Pane;

pub struct State {
    pub panes: pane_grid::State<Pane>,
    pub panes_created: usize,

    pub background_cache: canvas::Cache,
}

impl State {
    pub fn new() -> State {
        let config_left = pane_grid::Configuration::Pane(Pane::new(0));
        let config_main = pane_grid::Configuration::Pane(Pane::new(Pane::EMPTY_ID));
        let config_root = pane_grid::Configuration::Split {
            axis: pane_grid::Axis::Vertical,
            ratio: 0.3,
            a: Box::new(config_left),
            b: Box::new(config_main),
        };
        let panes = pane_grid::State::with_configuration(config_root);
        let panes_created = panes.len();
        State {
            panes,
            panes_created,

            background_cache: Default::default(),
        }
    }
}
