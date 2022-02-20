use iced::Element;

pub trait Component {
    fn view(&self) -> Element<conga_core::Message>;
}
