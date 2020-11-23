use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn init(path: &str) -> HashSet<String> {
    let file = File::open(path).unwrap();
    let mut words = HashSet::new();

    for line in BufReader::new(file).lines() {
        words.insert(line.unwrap());
    }

    words
}

pub fn count_words(text: &str, dict: &HashSet<String>) -> u32 {
    text.split(" ").fold(0, |acc, word| {
        if dict.contains(word) {
            return acc + 1;
        }
        return acc;
    })
}
