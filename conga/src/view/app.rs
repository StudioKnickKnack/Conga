impl App {
    pub fn view(&mut self) -> Column<Message> {
        Column::new()
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.increment_button, Text::new("+"))
                    .on_press(Message::Increment),
            )
    }
}
