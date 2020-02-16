use std::io::stdin;
use termion::event::Key;
use termion::input::TermRead;

pub enum StartChoice {
    Exit,
    Play,
}

#[derive(Default)]
pub struct Input {}

impl Input {
    pub fn new() -> Self {
        Self {}
    }

    pub fn wait_for_start(&self) -> StartChoice {
        let stdin = stdin();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Esc => {
                    return StartChoice::Exit;
                }
                Key::Char(' ') => {
                    return StartChoice::Play;
                }
                _ => (),
            }
        }

        StartChoice::Exit
    }
}
