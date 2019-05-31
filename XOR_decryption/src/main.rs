use itertools::Itertools;

mod args;
mod cipher;
mod dict;

fn main() {
    let (cipher, dict) = args::parse_args();
    println!("Will decrypt this file: {}", cipher);
    println!("Using this set of words: {}", dict);

    let dict = dict::init(&dict);
    let cipher = cipher::init(&cipher);

    ('a' as u8..='z' as u8).combinations(3).for_each(|pass| {
        let decipher = cipher::decipher(&cipher, &pass);
        if decipher.is_none() {
            return;
        }
        let decipher = decipher.unwrap();
        if decipher.contains("and") {
            println!("Using the pass {}", std::str::from_utf8(&pass).unwrap());
            println!("{}\n\n", decipher);
        }
    });
}
