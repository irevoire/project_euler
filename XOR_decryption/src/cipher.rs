use std::fs::File;
use std::io::Read;
use std::iter;
use std::str;

pub fn init(path: &str) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).unwrap();

    buffer
        .split(|c| *c == ',' as u8)
        .map(|n| str::from_utf8(n).unwrap().parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
}

pub fn decipher(cipher: &[u8], pass: &[u8]) -> Option<String> {
    let res = iter::repeat(pass)
        .flatten()
        .take(cipher.len())
        .zip(cipher)
        .map(|(p, c)| *p ^ *c)
        .collect::<Vec<u8>>();

    str::from_utf8(&res).ok().map(|s| s.to_string())
}
