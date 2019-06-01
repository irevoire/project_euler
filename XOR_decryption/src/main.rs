use itertools::Itertools;
use rayon::prelude::*;
use std::sync::Mutex;

mod args;
mod cipher;
mod dict;

fn main() {
    let (cipher, dict) = args::parse_args();
    println!("Will decrypt this file: {}", cipher);
    println!("Using this set of words: {}", dict);

    let dict = dict::init(&dict);
    let cipher = cipher::init(&cipher);

    let mut max = Mutex::new(0);
    let mut final_decipher = Mutex::new(String::new());
    ('!' as u8..'~' as u8)
        .combinations(3)
        .collect::<Vec<Vec<u8>>>() // does this destruct my memory?
        .into_par_iter()
        .for_each(|combination| {
            let len = combination.len();
            itertools::Permutations::from_vals(combination, len).for_each(|pass| {
                let decipher = cipher::decipher(&cipher, &pass);
                if decipher.is_none() {
                    return;
                }

                let decipher = decipher.unwrap();
                let nwords = dict::count_words(&decipher, &dict);

                let mut max = max.lock().unwrap();
                if nwords > *max {
                    *max = nwords;
                    // probably a lot of useless copy
                    *final_decipher.lock().unwrap() = decipher.to_string();
                    println!(
                        "Using the pass {} for {} english words",
                        std::str::from_utf8(&pass).unwrap(),
                        nwords
                    );
                    println!("{}\n\n", decipher);
                }
            })
        });

    let res = final_decipher
        .lock()
        .unwrap()
        .as_bytes()
        .iter()
        .fold(0 as u32, |acc, el| acc + *el as u32);
    println!("The sum of all the bytes is: {}", res);
}
