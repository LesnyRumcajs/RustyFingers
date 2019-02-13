use std::error::Error;
use std::io::{stdout, Stdout, Write};

use termion::color;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct TextScreen {
    stdout: RawTerminal<Stdout>,
}

impl TextScreen {
    pub fn new() -> TextScreen {
        let stdout = stdout().into_raw_mode().expect("Cannot obtain terminal");
        TextScreen { stdout }
    }

    pub fn clear_screen(&mut self) {
        write!(
            self.stdout,
            "{}{}M-q (exit){}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Hide
        )
        .expect("Terminal error!");
    }

    pub fn display_challenge(&mut self, phrase: &str) -> Result<(), Box<dyn Error>> {
        write!(
            self.stdout,
            "{}{}{}",
            termion::cursor::Goto(1, 2),
            phrase,
            termion::cursor::Goto(1, 2),
        )?;
        self.stdout.flush()?;

        Ok(())
    }

    pub fn print_ok(&mut self, c: char) {
        write!(
            self.stdout,
            "{}{}{}",
            color::Fg(color::Green),
            c,
            color::Fg(color::Reset)
        )
        .expect("Write to terminal error!");
        self.stdout.flush().expect("Terminal error");
    }

    pub fn print_err(&mut self, c: char) {
        write!(
            self.stdout,
            "{}{}{}",
            color::Fg(color::Red),
            c,
            color::Fg(color::Reset)
        )
        .expect("Write to terminal error!");
        self.stdout.flush().expect("Terminal error");
    }
}

impl Drop for TextScreen {
    fn drop(&mut self) {
        write!(
            self.stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Show,
            termion::cursor::Goto(1, 1)
        )
        .expect("Failed to restore the terminal!");
        self.stdout.flush().expect("Terminal error");
    }
}
