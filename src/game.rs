use rand::seq::SliceRandom;

pub struct Game {
    phrase: String,
    current_index: usize,

    words: Vec<String>,
    rng: rand::prelude::ThreadRng,
    words_per_phrase: u16,
    should_quit: bool,
}

impl Game {
    pub fn new(words: Vec<String>, words_per_phrase: u16) -> Game {
        let rng = rand::thread_rng();
        Game {
            phrase: String::new(),
            current_index: 0,
            words,
            rng,
            words_per_phrase,
            should_quit: false,
        }
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn challenge_phrase(&self) -> String {
        self.phrase.to_owned()
    }

    pub fn generate_challenge_phrase(&mut self) {
        self.phrase =
            Game::generate_random_phrase(&self.words, &mut self.rng, self.words_per_phrase);
    }

    pub fn current_char(&self) -> char {
        self.phrase.chars().nth(self.current_index).unwrap()
    }

    pub fn step(&mut self) {
        self.current_index += 1;

        if self.current_index == self.phrase.len() {
            self.quit();
        }
    }

    fn generate_random_phrase(
        words: &[String],
        rng: &mut rand::prelude::ThreadRng,
        amount: u16,
    ) -> String {
        words
            .choose_multiple(rng, amount as usize)
            .cloned()
            .collect::<Vec<String>>()
            .join(" ")
    }
}
