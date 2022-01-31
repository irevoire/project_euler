use project_euler::prime::Prime;

fn main() {
    let mut max = 0;
    let mut res = 0;

    // b MUST be a prime or else 0 + 0 + b wont be a prime

    let primes: Vec<_> = project_euler::PrimeIter::<usize>::new()
        .take_while(|p| *p < 1000)
        .collect();

    for a in -1000..1000_isize {
        for b in &primes {
            let count = (1..)
                .map(|n| n * n + a * n + *b as isize)
                .take_while(|n| (n.abs() as usize).is_prime())
                .count();
            if count > max {
                max = count;
                res = a * *b as isize;
            }
        }
    }

    println!("{}", res);
}
