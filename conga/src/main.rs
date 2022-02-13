use iced::executor;
use iced::pane_grid::{self, PaneGrid};
use iced::{Application, Command, Container, Element, Length, Row, Settings, Text};
use iced_lazy::responsive::Responsive;

use conga_core::Message;

mod view;

pub fn main() -> iced::Result {
    CongaApp::run(Settings {
        ..Settings::default()
    })
}

struct CongaApp {
    state: view::state::State,
}

impl Application for CongaApp {
    type Executor = executor::Default;
    type Message = conga_core::Message;
    type Flags = ();
    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            CongaApp {
                state: view::state::State::new(),
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
        let pane_grid = PaneGrid::new(&mut self.state.panes, |id, pane| {
            let view::pane::Pane {
                responsive,
                content,
                ..
            } = pane;

            if content.id == view::pane::Pane::EMPTY_ID {
                // empty pane
                pane_grid::Content::new(Responsive::new(responsive, move |size| {
                    content.view(id, size)
                }))
                .style(view::style::Pane::Normal)
            } else {
                let title =
                    Row::with_children(vec![Text::new(format!("Title {}", content.id)).into()])
                        .spacing(5);

                let title_bar = pane_grid::TitleBar::new(title)
                    .controls(pane.controls.view(id))
                    .padding(10);

                pane_grid::Content::new(Responsive::new(responsive, move |size| {
                    content.view(id, size)
                }))
                .title_bar(title_bar)
                .style(view::style::Pane::Normal)
            }
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10);

        Container::new(pane_grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .style(view::style::App::Normal)
            .into()

        /*Canvas::new(&mut self.state)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()*/
    }
}
