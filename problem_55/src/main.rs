mod utils;

use utils::*;

fn is_lychrel(n: u128, iter: u128) -> bool {
    let lychrel = n + reverse(n);
    if is_palindrome(lychrel) {
        return false;
    }
    if iter == 0 {
        return true;
    }
    is_lychrel(lychrel, iter - 1)
}

fn main() {
    let max = 10_000;
    let max_iter = 50;

    let mut cpt = 0;

    for n in 0..max {
        if is_lychrel(n, max_iter) {
            cpt += 1;
        }
    }
    println!(
        "There is {} lychrel numbers under {} with {} iterations",
        cpt, max, max_iter
    );
}
