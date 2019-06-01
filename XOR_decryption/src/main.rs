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

    let mut max = 0;
    ('!' as u8..='~' as u8)
        .combinations(3)
        .for_each(|combination| {
            let len = combination.len();
            itertools::Permutations::from_vals(combination, len).for_each(|pass| {
                let decipher = cipher::decipher(&cipher, &pass);
                if decipher.is_none() {
                    return;
                }

                let decipher = decipher.unwrap();
                let nwords = dict::count_words(&decipher, &dict);
                let lala = std::str::from_utf8(&pass).unwrap();
                if nwords > max {
                    max = nwords;
                    println!(
                        "Using the pass {} for {} english words",
                        std::str::from_utf8(&pass).unwrap(),
                        nwords
                    );
                    println!("{}\n\n", decipher);
                }
            })
        });
}
