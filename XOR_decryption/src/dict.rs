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
