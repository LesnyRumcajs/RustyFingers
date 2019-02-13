use std::error::Error;
use std::fs::File;
use std::io::{BufRead,BufReader};

pub fn read_words_from_file(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let reader = BufReader::new(File::open(filename)?);
    Ok(reader
        .lines()
        .map(|line| String::from(line.unwrap_or_default().trim()))
        .collect())
}
