extern crate rand;
extern crate termion;

use std::error::Error;

use rand::seq::SliceRandom;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Stdout;
use std::io::{stdin, stdout, Write};
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

struct Stats {
    correct: u16,
    mistakes: u16,
}

impl Stats {
    fn new() -> Stats {
        Stats {
            correct: 0,
            mistakes: 0,
        }
    }
}

fn read_words_from_file(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let reader = BufReader::new(File::open(filename)?);
    Ok(reader
        .lines()
        .map(|line| String::from(line.unwrap_or_default().trim()))
        .collect())
}

fn generate_random_line(
    rng: &mut rand::prelude::ThreadRng,
    words: &[String],
    amount: u16,
) -> String {
    words
        .choose_multiple(rng, amount as usize)
        .cloned()
        .collect::<Vec<String>>()
        .join(" ")
}

fn prepare_terminal(stdout: &mut RawTerminal<Stdout>, instruction_line: u16) {
    write!(
        stdout,
        "{}{}M-q (exit), M-r (restart){}",
        termion::clear::All,
        termion::cursor::Goto(1, instruction_line),
        termion::cursor::Hide
    )
    .expect("Terminal error!");
}

pub fn run() -> Result<(), Box<dyn Error>> {
    const INSTRUCTION_LINE: u16 = 1;
    const TEXT_LINE: u16 = 2;
    const WORDS_PER_LINE: u16 = 10;

    let words = read_words_from_file("res/rust/top500")?;
    let mut rng = rand::thread_rng();

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode()?;

    let mut challenge_phrase = generate_random_line(&mut rng, &words, WORDS_PER_LINE);

    prepare_terminal(&mut stdout, INSTRUCTION_LINE);
    write!(
        stdout,
        "{}{}{}",
        termion::cursor::Goto(1, TEXT_LINE),
        challenge_phrase,
        termion::cursor::Goto(1, TEXT_LINE),
    )?;
    stdout.flush()?;

    let mut current_index = 0;
    let mut should_quit = false;
    let mut should_restart = false;

    let mut stats = Stats::new();

    let mut phrase_keys = challenge_phrase.chars();
    for pressed_key in stdin.keys() {
        let current_key = match phrase_keys.next() {
            Some(c) => c,
            None => {
                challenge_phrase = generate_random_line(&mut rng, &words, WORDS_PER_LINE);
                write!(
                    stdout,
                    "{}{}{}{}",
                    termion::cursor::Goto(1, TEXT_LINE),
                    termion::clear::CurrentLine,
                    challenge_phrase,
                    termion::cursor::Goto(1, TEXT_LINE),
                )?;
                stdout.flush()?;

                current_index = 0;
                phrase_keys = challenge_phrase.chars();
                phrase_keys.next().unwrap()
            }
        };
        write!(
            stdout,
            "{}",
            termion::cursor::Goto(1 + current_index, TEXT_LINE),
        )?;

        match pressed_key? {
            Key::Char(c) => match c {
                _ if c == current_key => {
                    write!(
                        stdout,
                        "{}{}{}",
                        color::Fg(color::Green),
                        current_key,
                        color::Fg(color::Reset)
                    )?;
                    current_index += 1;
                }
                _ => {
                    write!(
                        stdout,
                        "{}{}{}",
                        color::Fg(color::Red),
                        current_key,
                        color::Fg(color::Reset)
                    )?;
                    current_index += 1;
                    stats.mistakes += 1;
                }
            },
            Key::Alt(c) => match c {
                'q' => {
                    should_quit = true;
                    break;
                }
                'r' => {
                    should_restart = true;
                    break;
                }
                _ => (),
            },
            _ => (),
        }

        if should_quit {
            break;
        }

        if should_restart {
            break;
        }

        // Flush again.
        stdout.flush()?;
    }

    // Show the cursor again before we exit.
    write!(stdout, "{}", termion::cursor::Show)?;

    Ok(())
}
