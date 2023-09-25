use crate::race::Race;
use crate::{db, level};
use iced::alignment::Horizontal;
use iced::widget::{
    button, container, horizontal_rule, horizontal_space, progress_bar, text, text_input, Column,
    Row,
};
use iced::{Alignment, Element, Sandbox};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Application {
    state: GameState,
    session: Option<Race>,
    previous_session: Option<Race>,
    score: u64,
    playcount: u64,
}

#[derive(Debug, Clone)]
pub enum Message {
    GameStart,
    GameQuit,
    OnInput(String),
}

#[derive(Debug, Clone)]
pub enum GameState {
    New,
    Running,
    Complete,
}

impl Sandbox for Application {
    type Message = Message;

    fn new() -> Self {
        Self {
            state: GameState::New,
            session: None,
            previous_session: None,
            score: 0,
            playcount: 0,
        }
    }

    fn title(&self) -> String {
        String::from("Typing Game")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::GameStart => {
                self.session = Some(Race::with_line(db::random_wikipedia()));
                self.state = GameState::Running;
                self.playcount += 1;
            }
            Message::GameQuit => {
                std::process::exit(0);
            }
            Message::OnInput(input) => match self.session.as_mut() {
                None => {}
                Some(session) => {
                    let inp = input.chars().last().unwrap();

                    session.insert_char(inp);

                    if session.is_completed() {
                        self.complete();
                    }
                }
            },
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        match self.state {
            GameState::New => container(
                Column::new()
                    .push(
                        text("Main Menu")
                            .size(40)
                            .horizontal_alignment(Horizontal::Center),
                    )
                    .push(button("Start Game").on_press(Message::GameStart).width(150))
                    .push(button("Quit").on_press(Message::GameQuit).width(150))
                    .spacing(20)
                    .align_items(Alignment::Center),
            ),
            GameState::Running => {
                let sess = self.session.clone().unwrap();
                let len = sess.input.len();

                let green_text = text(&sess.line[0..len]).style(iced::Color::from([0.0, 1.0, 0.0]));
                let red_text = text(&sess.line[len..]).style(iced::Color::from([1.0, 0.0, 0.0]));

                container(
                    Column::new()
                        .push(text(format!("This extract is from: {}", sess.source)))
                        .push(text(
                            "You are to write the following text as fast as possible:",
                        ))
                        .push(Row::new().push(green_text).push(red_text))
                        .push(horizontal_rule(10))
                        .push(text("Input so far:"))
                        .push(text(sess.input.clone()).horizontal_alignment(Horizontal::Left))
                        .push(text(format!(
                            "WPM: {}, Score: {}",
                            sess.words_per_minute(),
                            sess.score()
                        )))
                        .push(horizontal_rule(10))
                        .push(
                            text_input(
                                "Type the above sentence as fast as you can!",
                                sess.input.clone().as_str(),
                            )
                            .width(400)
                            .on_input(Message::OnInput),
                        )
                        .push(button("Quit").on_press(Message::GameQuit).width(150))
                        .spacing(20)
                        .align_items(Alignment::Center),
                )
            }
            GameState::Complete => {
                let sess = match self.previous_session.clone() {
                    None => panic!("Session should exist"),
                    Some(sess) => sess,
                };

                let completed_in = sess
                    .end
                    .unwrap()
                    .duration_since(sess.start.expect("Should exist"))
                    .unwrap();

                let exp = level::current(self.score);
                let progress = level::progress(&exp);

                container(
                    Column::new()
                        .push(
                            text("Game Complete")
                                .horizontal_alignment(Horizontal::Center)
                                .size(40),
                        )
                        .push(horizontal_space(20))
                        .push(
                            text(format!("Score: {}", self.score))
                                .horizontal_alignment(Horizontal::Center),
                        )
                        .push(
                            text(format!("Games Played: {}", self.playcount))
                                .horizontal_alignment(Horizontal::Center),
                        )
                        .push(
                            text(format!(
                                "Completed in {:.2}s, that's {:.2} wpm",
                                completed_in.as_millis() / 1000,
                                sess.words_per_minute()
                            ))
                            .horizontal_alignment(Horizontal::Center),
                        )
                        .push(horizontal_space(20))
                        .push(
                            text(format!(
                                "You are level {}, the next level is at {} score",
                                exp.get_level(),
                                exp.get_next_milestone().unwrap_or(0)
                            ))
                            .horizontal_alignment(Horizontal::Center),
                        )
                        .push(
                            progress_bar(
                                (0.)..=progress.get_max() as f32,
                                progress.get_progress() as f32,
                            )
                            .width(400),
                        )
                        .push(horizontal_space(20))
                        .push(button("New Game").on_press(Message::GameStart).width(150))
                        .push(button("Quit").on_press(Message::GameQuit).width(150))
                        .spacing(5)
                        .align_items(Alignment::Center),
                )
            }
        }
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}

impl Application {
    pub fn complete(&mut self) {
        let sess = self.session.clone().unwrap();
        self.score += sess.score();
        self.previous_session = Some(sess);
        self.session = None;
        self.state = GameState::Complete;
    }
}
