/*
 * Summation of primes
 * Problem 10
 * The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
 *
 * Find the sum of all the primes below two million.
 */

fn main() {
    let limit: u64 = 2000000;
    let mut sum: u64 = 0;

    for prime in project_euler::PrimeIter::<u64>::new() {
        if prime > limit {
            break;
        }
        sum += prime;
    }
    println!("sum is {}", sum);
}
