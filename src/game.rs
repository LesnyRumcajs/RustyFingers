use crate::display::Display;
use crate::words::Words;
use std::io::Write;
use std::path::Path;

use crate::input::{Input, PlayAction, StartChoice};
use std::time::Instant;

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
                let words = self.words.get_shuffled()[0..50].to_vec();
                self.display.show_words(&words);

                let mut now = Instant::now();
                let (mut good_count, mut bad_count) = (0,0);
                for c in words.join(" ").chars() {

                    let input = self.input.play();

                    // count time only after the first hit
                    if good_count == 0 && bad_count == 0 {
                        now = Instant::now();
                    }
                    match input {
                        PlayAction::Exit => break,
                        PlayAction::Char(ch) => {
                            if ch == c {
                                self.display.good(c);
                                good_count += 1;
                            } else {
                                self.display.bad(c);
                                bad_count += 1;
                            }
                        }
                    }
                }
                self.display.clear();
                self.display.show_score(good_count, bad_count, now.elapsed());
            }
            StartChoice::Exit => (),
        }
        self.display.farewell();

        Ok(())
    }
}
