use crate::words::Words;
use std::path::Path;
use crate::display::Display;
use std::io::{Write, Read, Stdin};
use termion::input::TermReadEventsAndRaw;
use crate::input::Input;

pub struct Game<W:Write> {
    display: Display<W>,
    input: Input,
    words: Words
}

impl<W:Write> Game<W> {
    pub fn from_file(w: W, path: &Path) -> Result<Game<W>, failure::Error> {
        let words = Words::from_file(path)?;
        let display = Display::new(w);
        let input = Input::new();
        Ok(Game {
            display,
            words,
            input
        })
    }

    pub fn start(&mut self) -> Result<(), failure::Error> {
       self.display.welcome();
       self.input.wait_for_exit();

       Ok(())
    }
}