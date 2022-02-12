use iced::{
    canvas::{self, Path},
    executor, Application, Canvas, Clipboard, Color, Command, Element, Length, Point, Settings,
};

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
    fn update(&mut self, _message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        todo!()
    }
    fn view(&mut self) -> Element<Message> {
        Canvas::new(&mut self.state)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

#[derive(Debug)]
struct State {
    background_cache: canvas::Cache,
}

impl State {
    pub fn new() -> State {
        State {
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
