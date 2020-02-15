use std::io::stdin;
use termion::input::TermRead;
use termion::event::Key;

pub struct Input {
}

impl Input {
    pub fn new() -> Self {
        Self {}
    }

    pub fn wait_for_exit(&self) {
        let stdin = stdin();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Esc => break,
                _ => {}
            }
        }

    }
}