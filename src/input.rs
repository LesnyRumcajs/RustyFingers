use std::io::stdin;
use termion::event::Key;
use termion::input::TermRead;

#[derive(Default)]
pub struct Input {}

impl Input {
    pub fn new() -> Self {
        Self {}
    }

    pub fn wait_for_exit(&self) {
        let stdin = stdin();

        for c in stdin.keys() {
            if let Key::Esc = c.unwrap() {
                break;
            }
        }
    }
}
