use std::io::Write;
use termion;
use termion::cursor::DetectCursorPos;
use termion::raw::{IntoRawMode, RawTerminal};
use failure::_core::time::Duration;

pub struct Display<W: Write> {
    out: RawTerminal<W>,
}

impl<W: Write> Display<W> {
    pub fn new(w: W) -> Self {
        Self {
            out: w.into_raw_mode().unwrap(),
        }
    }

    fn clear_all(&mut self) {
        write!(self.out, "{}", termion::clear::All).unwrap();
    }

    pub fn clear(&mut self) {
        self.goto(0,2);
        write!(self.out, "{}", termion::clear::AfterCursor);
    }

    pub fn goto(&mut self, x: u16, y: u16) {
        write!(self.out, "{}", termion::cursor::Goto(x, y)).unwrap();
    }

    pub fn welcome(&mut self) {
        self.clear_all();
        self.goto(1, 1);
        write!(
            self.out,
            "{}Rusty Fingers! 🦀\r\n{}Press space to start, ESC to exit",
            termion::style::Bold,
            termion::style::Reset
        )
        .unwrap();
        self.out.flush().unwrap()
    }

    fn newline(&mut self) {
        let (_, y) = self.out.cursor_pos().unwrap();
        self.goto(0, y + 1);
    }

    pub fn farewell(&mut self) {
        self.newline();
        write!(self.out, "Thanks for playing! ❤️").unwrap()
    }

    pub fn show_words(&mut self, words: &[String]) {
        self.newline();
        let (x, y) = self.out.cursor_pos().unwrap();
        write!(
            self.out,
            "{}{}{}",
            termion::color::Fg(termion::color::LightBlack),
            words.join(" "),
            termion::color::Fg(termion::color::Reset)
        )
        .unwrap();
        self.goto(x, y);
        self.out.flush().unwrap();
    }

    pub fn good(&mut self, c: char) {
        write!(
            self.out,
            "{}{}{}",
            termion::color::Fg(termion::color::Green),
            c,
            termion::color::Fg(termion::color::Reset)
        )
        .unwrap();
        self.out.flush().unwrap();
    }

    pub fn bad(&mut self, c: char) {
        write!(
            self.out,
            "{}{}{}",
            termion::color::Fg(termion::color::Red),
            c,
            termion::color::Fg(termion::color::Reset)
        )
        .unwrap();
        self.out.flush().unwrap();
    }

    pub fn show_score(&mut self, good: u32, bad: u32, time: Duration) {
        write!(self.out,
            "Good: {}{}{}\n\rBad: {}{}{}\n\rTime: {:.2}s\r\n",
            termion::color::Fg(termion::color::Green),
            good,
            termion::color::Fg(termion::color::Reset),
            termion::color::Fg(termion::color::Red),
            bad,
            termion::color::Fg(termion::color::Reset),
            time.as_secs_f32()
        );
        write!(
            self.out,
            "Your score is {}{:.2}{} characters per second!",
            termion::color::Fg(termion::color::Yellow),
            good as f32 / time.as_secs_f32(),
            termion::color::Fg(termion::color::Reset)
        )
            .unwrap();
        self.out.flush().unwrap();
    }
}
