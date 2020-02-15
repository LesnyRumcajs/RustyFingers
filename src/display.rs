use std::io::Write;
use termion;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Display<W: Write> {
    out: RawTerminal<W>,
}

impl<W: Write> Display<W> {
    pub fn new(w: W) -> Self {
        Self {
            out: w.into_raw_mode().unwrap(),
        }
    }

    fn clear(&mut self) {
        write!(self.out, "{}", termion::clear::All).unwrap();
    }

    pub fn goto(&mut self, x: u16, y: u16) {
        write!(self.out, "{}", termion::cursor::Goto(x, y)).unwrap();
    }

    pub fn welcome(&mut self) {
        self.clear();
        self.goto(1, 1);
        write!(
            self.out,
            "{}Rusty Fingers!{} Press space to start, ESC to exit",
            termion::style::Bold,
            termion::style::Reset
        )
        .unwrap();
        self.out.flush().unwrap()
    }
}
