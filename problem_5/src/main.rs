/*
 * Smallest multiple
 * Problem 5
 * 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
 *
 * What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
 */
fn main() {
    let divisors = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    let mut n = divisors.last().unwrap().clone();
    loop {
        if let Some(res) = divisible(n, &divisors) {
            println!("Found a result !!! {}", res);
            return;
        };
        n += divisors.last().unwrap();
    }
}

fn divisible(n : u32, divisors : &[u32]) -> Option<u32> {
    for el in divisors {
        if n % el != 0 {
            return None
        };
    };
    return Some(n)
}
