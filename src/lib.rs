use std::error::Error;
use std::io::stdin;

use screen::TextScreen;
use termion::event::Key;
use termion::input::TermRead;

use std::io::Read;
use std::io::Write;

mod game;
mod screen;
mod words;

fn pause() {
    let mut stdin = stdin();
    let mut stdout = std::io::stdout();

    write!(stdout, "\n\rPress any key to continue...").unwrap();
    stdout.flush().unwrap();

    stdin.read(&mut [0u8]).unwrap();
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut screen = TextScreen::new();
    let mut game = game::Game::new(words::read_words_from_file("res/rust/top500")?, 10);

    screen.clear_screen();

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
