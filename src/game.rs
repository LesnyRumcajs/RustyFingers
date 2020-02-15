use crate::words::Words;
use std::path::Path;
use crate::display::Display;
use std::io::{Write, Read};

pub struct Game<W:Write,R:Read> {
    display: Display<W,R>,
    words: Words
}

impl<W:Write, R:Read> Game<W,R> {
    pub fn from_file(w: W, r: R, path: &Path) -> Result<Game<W,R>, failure::Error> {
        let words = Words::from_file(path)?;
        let display = Display::new(w,r);
        Ok(Game {
            display,
            words
        })
    }

    pub fn start(&mut self) -> Result<(), failure::Error> {
       self.display.welcome();

       Ok(())
    }
}