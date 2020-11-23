use project_euler::num::Num;
use std::collections::HashSet;

fn main() {
    let mut base = [0; 9];
    let mut res = 0;

    // we stop at 99999999 because we don't want to get any direct pandigital number
    for _ in 0..99999999 {
        base.iter_mut().enumerate().for_each(|(i, e)| *e += i + 1);

        let mut set = HashSet::new();
        for digits in base.iter().map(|e| e.split_digits()) {
            let still_pandigital = digits.iter().all(|d| set.insert(*d));
            if digits.contains(&0) {
                break;
            } else if still_pandigital && set.len() == 9 {
                res = base[0]; // current number
                println!("current biggest number {}", res);
                break;
            } else if !still_pandigital {
                break;
            }
        }
    }

    // ugly hardcoded solution because *I know* it'll be only made of the two first multiplication
    println!("The biggest pandigital number is {}{}", res, res * 2);
}
