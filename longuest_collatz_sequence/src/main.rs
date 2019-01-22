fn main() {
    let mut max = 1;
    let mut max_size = 1;

    for n in 1..1000000 {
        let current = collatz_size(n);
        if current > max_size {
            max_size = current;
            max = n;
            println!("collatz({}) = {}", n, current);
        }
    }

    println!("maximum is {} for value {}", max_size, max);
}

fn collatz_size(n: u64) -> u64 {
    if n == 1 {
        return 1
    }
    return 1 + collatz_size(collatz_step(n))

}

fn collatz_step(n: u64) -> u64 {
    if n % 2 == 0 {
        return n / 2
    }
    return 3 * n + 1
}
