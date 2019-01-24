extern crate rand;
extern crate termion;

use std::error::Error;
use std::fs::File;
use std::io::stdin;
use std::io::BufRead;
use std::io::BufReader;

use screen::TextScreen;
use termion::event::Key;
use termion::input::TermRead;

use crate::game::Game;
use std::io::Read;
use std::io::Write;

mod game;
mod screen;

fn read_words_from_file(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let reader = BufReader::new(File::open(filename)?);
    Ok(reader
        .lines()
        .map(|line| String::from(line.unwrap_or_default().trim()))
        .collect())
}

fn pause() {
    let mut stdin = stdin();
    let mut stdout = std::io::stdout();

    write!(stdout, "\n\rPress any key to continue...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut screen = TextScreen::new();
    let mut game = Game::new(read_words_from_file("res/rust/top500")?, 5);

    screen.prepare_screen();

    game.generate_challenge_phrase();
    screen.display_challenge(&game.challenge_phrase())?;

    let stdin = stdin();

    for pressed_key in stdin.keys() {
        let current_char = game.current_char();
        match pressed_key? {
            Key::Char(c) => match c {
                _ if c == current_char => {
                    screen.print_ok(current_char);
                    game.step();
                }
                _ => {
                    screen.print_err(current_char);
                    game.step();
                }
            },
            Key::Alt(c) => {
                if let 'q' = c {
                    game.quit();
                }
            }
            _ => (),
        }

        if game.should_quit() {
            pause();
            break;
        }
    }

    Ok(())
}
