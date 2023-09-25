use iced::Sandbox;

mod db;
mod level;
mod race;
mod ui;

fn main() -> iced::Result {
    ui::Application::run(iced::Settings::default())
}
