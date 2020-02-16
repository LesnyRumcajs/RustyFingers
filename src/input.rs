use std::io::stdin;
use termion::event::Key;
use termion::input::TermRead;

pub enum StartChoice {
    Exit,
    Play,
}

pub enum PlayAction {
    Exit,
    Char(char),
}

#[derive(Default)]
pub struct Input {}

impl Input {
    pub fn new() -> Self {
        Self {}
    }

    pub fn play(&self) -> PlayAction {
        let stdin = stdin();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Esc => break,
                Key::Char(c) => return PlayAction::Char(c),
                _ => {}
            }
        }

        PlayAction::Exit
    }

    pub fn wait_for_start(&self) -> StartChoice {
        let stdin = stdin();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Esc => {
                    break;
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
