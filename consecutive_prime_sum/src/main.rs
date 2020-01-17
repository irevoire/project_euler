use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let limit = 1000000;
    let primes: Vec<u64> = euler::PrimeIter::<u64>::new()
        .take_while(|p| *p < limit)
        .collect();
    let hash_primes: HashSet<&u64> = HashSet::from_iter(primes.iter().clone());

    println!("Generated all the prime below {}", limit);

    for i in 2..primes.len() {
        let res: Vec<u64> = primes
            .windows(i)
            .filter_map(|arr| {
                let sum: u64 = arr.iter().sum();
                if sum > limit {
                    None
                } else if hash_primes.contains(&sum) {
                    Some(sum)
                } else {
                    None
                }
            })
            .collect();
        if !res.is_empty() {
            println!("Res: {:?}", res);
        }
    }
}
