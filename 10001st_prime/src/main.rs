/*
 * 10001st prime
 * Problem 7
 * By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
 *
 * What is the 10 001st prime number?
 */

mod prime;

fn main() {
    let limit : u64 = 10001;
    let mut iter : u64 = 0;

    for prime in prime::Prime::new() {
        iter += 1;
        if iter == limit {
            println!("prime is {}", prime);
            break
        }
    }
}
