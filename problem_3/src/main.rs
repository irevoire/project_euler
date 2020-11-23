// Largest prime factor
// Problem 3
// The prime factors of 13195 are 5, 7, 13 and 29.
//
// What is the largest prime factor of the number 600851475143 ?

fn main() {
    let number = 600851475143;
    let mut largest_prime = 1;

    for prime in project_euler::PrimeIter::<u64>::new() {
        if prime > number / 2 {
            break;
        }
        if number % prime == 0 {
            println!("----   factor {}   ----", prime);
            largest_prime = prime;
        }
    }
    println!("factor is {}", largest_prime);
}
