use rayon::prelude::*;
use std::sync::Mutex;

const MAX: u64 = 10000;

fn sum_of_divisors(n: u64) -> u64 {
    let mut sum = 0;

    for i in 1..n {
        if n % i == 0 {
            sum += i;
        }
    }

    sum
}
fn main() {
    let amicables = Mutex::new(Vec::new());

    (1..MAX).into_par_iter().for_each(|n| {
        if amicables.lock().unwrap().contains(&n) {
            return;
        }

        let sum = sum_of_divisors(n);
        if n != sum && sum_of_divisors(sum) == n {
            {
                let mut ami = amicables.lock().unwrap();
                ami.push(sum);
                ami.push(n);
            }
        }
    });

    println!("amicables: {:?}", amicables);
    println!(
        "sum of amicables: {}",
        amicables.lock().unwrap().iter().sum::<u64>()
    );
}
