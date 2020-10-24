use iced::{executor, Application, Command, Element, Column, Row, Button, button, Text, Length};

pub struct CheapdeckApplication {
  button_states: Vec<button::State>,
}

impl Application for CheapdeckApplication {
  type Executor = executor::Default;
  type Message = ();
  type Flags = ();

  fn new(_flags: ()) -> (CheapdeckApplication, Command<Self::Message>) {
      (
        CheapdeckApplication {
          button_states: vec![button::State::new(); 15],
        },
        Command::none()
      )
  }

  fn title(&self) -> String {
      String::from("Cheapdeck")
  }

  fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
      Command::none()
  }

  fn view(&mut self) -> Element<Self::Message> {
    let mut iter = self.button_states.iter_mut();

      Column::with_children(vec![
        Row::with_children(vec![
          Button::new(iter.next().unwrap(), Text::new("Ouech")).width(Length::Fill).height(Length::Fill).into(),
          Button::new(iter.next().unwrap(), Text::new("Ouech")).width(Length::Fill).height(Length::Fill).into(),
          Button::new(iter.next().unwrap(), Text::new("Ouech")).width(Length::Fill).height(Length::Fill).into(),
          Button::new(iter.next().unwrap(), Text::new("Ouech")).width(Length::Fill).height(Length::Fill).into(),
          Button::new(iter.next().unwrap(), Text::new("Ouech")).width(Length::Fill).height(Length::Fill).into()
        ]).width(Length::Fill).height(Length::Fill).spacing(8).into(),
        Row::with_children(vec![
          Button::new(iter.next().unwrap(), Text::new("Ouech")).width(Length::Fill).height(Length::Fill).into(),
          Button::new(iter.next().unwrap(), Text::new("Ouech")).width(Length::Fill).height(Length::Fill).into(),
          Button::new(iter.next().unwrap(), Text::new("Ouech")).width(Length::Fill).height(Length::Fill).into(),
          Button::new(iter.next().unwrap(), Text::new("Ouech")).width(Length::Fill).height(Length::Fill).into(),
          Button::new(iter.next().unwrap(), Text::new("Ouech")).width(Length::Fill).height(Length::Fill).into()
        ]).width(Length::Fill).height(Length::Fill).spacing(8).into(),
        Row::with_children(vec![
          Button::new(iter.next().unwrap(), Text::new("Ouech")).width(Length::Fill).height(Length::Fill).into(),
          Button::new(iter.next().unwrap(), Text::new("Ouech")).width(Length::Fill).height(Length::Fill).into(),
          Button::new(iter.next().unwrap(), Text::new("Ouech")).width(Length::Fill).height(Length::Fill).into(),
          Button::new(iter.next().unwrap(), Text::new("Ouech")).width(Length::Fill).height(Length::Fill).into(),
          Button::new(iter.next().unwrap(), Text::new("Ouech")).width(Length::Fill).height(Length::Fill).into()
        ]).width(Length::Fill).height(Length::Fill).spacing(8).into()
      ]).width(Length::Fill).height(Length::Fill).spacing(8).padding(8).into()
  }
}
