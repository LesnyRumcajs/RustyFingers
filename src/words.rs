use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct Words {
    #[allow(dead_code)]
    words: Vec<String>,
}

impl Words {
    pub fn from_file(path: &Path) -> Result<Self, failure::Error> {
        let reader = BufReader::new(File::open(path)?);
        let words = reader
            .lines()
            .map(|l| l.unwrap().trim().to_owned())
            .collect();
        Ok(Words { words })
    }

    pub fn get_shuffled(&self) -> Vec<String> {
        let mut words = self.words.clone();
        words.shuffle(&mut thread_rng());
        words
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn from_file_should_fail_with_no_file() {
        let words = Words::from_file(Path::new("please-do-not-exist"));
        assert!(words.is_err());
    }

    #[test]
    fn from_file_should_get_all() {
        let mut temp_file = NamedTempFile::new().expect("Could not create temporary file");
        let text = "the\nquick\nbrown\nfox";
        temp_file
            .write_all(text.as_bytes())
            .expect("Could not write to temporary file");

        let words = Words::from_file(temp_file.path()).expect("Could not read from file");
        assert_eq!(words.words, vec!["the", "quick", "brown", "fox"]);
    }

    #[test]
    fn get_shuffled_each_time_different() {
        let w = Words::from_file(Path::new("res/eng/top1000")).unwrap();
        assert_ne!(w.get_shuffled(), w.get_shuffled());
    }
}
