mod cheapdeck_application;

use cheapdeck_application::CheapdeckApplication;
use iced::{Application, Settings};

fn main() -> iced::Result {
  CheapdeckApplication::run(Settings::default())
}
