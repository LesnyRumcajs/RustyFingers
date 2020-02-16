use crate::display::Display;
use crate::words::Words;
use std::io::Write;
use std::path::Path;

use crate::input::{Input, StartChoice};

pub struct Game<W: Write> {
    display: Display<W>,
    input: Input,
    #[allow(dead_code)]
    words: Words,
}

impl<W: Write> Game<W> {
    pub fn from_file(w: W, path: &Path) -> Result<Game<W>, failure::Error> {
        let words = Words::from_file(path)?;
        let display = Display::new(w);
        let input = Input::new();
        Ok(Game {
            display,
            words,
            input,
        })
    }

    pub fn play(&mut self) -> Result<(), failure::Error> {
        self.display.welcome();
        match self.input.wait_for_start() {
            StartChoice::Play => {
                let words = self.words.get_shuffled();
                self.display.show_words(&words);
            }
            StartChoice::Exit => (),
        }
        self.display.farewell();

        Ok(())
    }
}
