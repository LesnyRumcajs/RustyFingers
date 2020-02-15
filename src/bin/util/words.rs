use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::File;


#[allow(dead_code)]
struct Words {
    words: Vec<String>
}

impl Words {
    #[allow(dead_code)]
    fn from_file(path: &Path) -> Result<Self, failure::Error> {
        let reader = BufReader::new(File::open(path)?);
        let words = reader
            .lines()
            .map(|l| l.unwrap().trim().to_owned())
            .collect();
        Ok(Words { words })
    }
}
