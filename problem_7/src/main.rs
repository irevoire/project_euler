/*
 * 10001st prime
 * Problem 7
 * By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
 *
 * What is the 10 001st prime number?
 */

fn main() {
    let limit = 10001;

    let prime = project_euler::PrimeIter::<u64>::new()
        .skip(limit - 1)
        .next()
        .unwrap();
    println!("prime is {}", prime);
}
