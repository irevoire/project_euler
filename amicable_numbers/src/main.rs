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
    let mut amicables = Vec::new();

    for n in 1..MAX {
        if amicables.contains(&n) {
            continue;
        }

        let sum = sum_of_divisors(n);
        if n != sum && sum_of_divisors(sum) == n {
            amicables.push(sum);
            amicables.push(n);
        }
    }

    println!("amicables: {:?}", amicables);
    println!("sum of amicables: {:?}", amicables.iter().sum::<u64>());
}
