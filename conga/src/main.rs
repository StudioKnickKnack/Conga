use iced::alignment::{self, Alignment};
use iced::canvas::{self, Path};
use iced::executor;
use iced::pane_grid::{self, PaneGrid};
use iced::scrollable::{self, Scrollable};
use iced::{
    Application, Canvas, Color, Command, Container, Element, Length, Point, Row, Settings, Size,
    Text,
};
use iced_lazy::responsive::{self, Responsive};

use conga_core::Message;

pub fn main() -> iced::Result {
    CongaApp::run(Settings {
        ..Settings::default()
    })
}

struct CongaApp {
    state: State,
}

impl Application for CongaApp {
    type Executor = executor::Default;
    type Message = conga_core::Message;
    type Flags = ();
    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            CongaApp {
                state: State::new(),
            },
            Command::none(),
        )
    }
    fn title(&self) -> String {
        String::from("Conga")
    }
    fn update(&mut self, _message: Message) -> Command<Message> {
        todo!()
    }
    fn view(&mut self) -> Element<Message> {
        let total_panes = self.state.panes.len();

        let pane_grid = PaneGrid::new(&mut self.state.panes, |id, pane| {
            let Pane {
                responsive,
                content,
                controls,
                ..
            } = pane;

            if content.id == Pane::EMPTY_ID {
                // empty pane
                pane_grid::Content::new(Responsive::new(responsive, move |size| {
                    content.view(id, total_panes, false, size)
                }))
            } else {
                let title =
                    Row::with_children(vec![Text::new(format!("Title {}", content.id)).into()])
                        .spacing(5);

                let title_bar = pane_grid::TitleBar::new(title)
                    .controls(pane.controls.view(id, total_panes, false))
                    .padding(10);

                pane_grid::Content::new(Responsive::new(responsive, move |size| {
                    content.view(id, total_panes, false, size)
                }))
                .title_bar(title_bar)
            }
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10);

        Container::new(pane_grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()

        /*Canvas::new(&mut self.state)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()*/
    }
}

struct State {
    panes: pane_grid::State<Pane>,
    panes_created: usize,

    background_cache: canvas::Cache,
}

impl State {
    pub fn new() -> State {
        let config_left = pane_grid::Configuration::Pane(Pane::new(0));
        let config_main = pane_grid::Configuration::Pane(Pane::new(Pane::EMPTY_ID));
        let config_root = pane_grid::Configuration::Split {
            axis: pane_grid::Axis::Vertical,
            ratio: 0.5,
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

impl<Message> canvas::Program<Message> for State {
    fn draw(
        &self,
        bounds: iced::Rectangle,
        _: iced::canvas::Cursor,
    ) -> std::vec::Vec<iced::canvas::Geometry> {
        let background = self.background_cache.draw(bounds.size(), |frame| {
            let rect = Path::rectangle(Point::new(0.0, 0.0), frame.size());
            frame.fill(&rect, Color::BLACK);
        });

        vec![background]
    }
}

struct Pane {
    pub responsive: responsive::State,
    pub content: Content,
    pub controls: Controls,
}

impl Pane {
    fn new(id: usize) -> Self {
        Self {
            responsive: responsive::State::new(),
            content: Content::new(id),
            controls: Controls::new(),
        }
    }
    const EMPTY_ID: usize = usize::MAX;
}

struct Content {
    id: usize,
    scroll: scrollable::State,
}

impl Content {
    fn new(id: usize) -> Self {
        Self {
            id,
            scroll: scrollable::State::new(),
        }
    }

    fn view(
        &mut self,
        pane: pane_grid::Pane,
        total_panes: usize,
        is_pinned: bool,
        size: Size,
    ) -> Element<Message> {
        let Content { id, scroll, .. } = self;

        let greet = match *id {
            Pane::EMPTY_ID => Text::new("[EMPTY]"),
            _ => Text::new(format!("Content {}", id.to_string())),
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

struct Controls {}

impl Controls {
    fn new() -> Self {
        Self {}
    }

    pub fn view(
        &mut self,
        pane: pane_grid::Pane,
        total_panes: usize,
        is_pinned: bool,
    ) -> Element<Message> {
        Text::new("Ctrls").into()
    }
}
