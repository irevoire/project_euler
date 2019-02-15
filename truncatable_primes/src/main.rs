fn is_prime(n: u64) -> bool {
    if n <= 1 { return false }
    let limit = (n as f64).sqrt() as u64 + 1;

    for i in 2..limit {
        if n % i == 0 { return false }
    }
    return true
}

fn decompose(n: u64) -> Vec<u64> {
    let string = n.to_string();
    let mut res = Vec::new();
    let mut right = string.clone();
    let mut left = string.clone();

    while right.len() > 1 {
        right.pop();
        left.remove(0);

        res.push(right.parse().unwrap());
        res.push(left.parse().unwrap());
    }

    res.sort();
    res.dedup();

    return res;
}

fn all_prime(n: u64) -> bool {
    is_prime(n) && decompose(n).iter().fold(true, |acc, val| acc && is_prime(*val))
}

fn main() {
    let mut iter = 10;
    let mut sum = 0;
    loop {
        if all_prime(iter) {
            sum += iter;
            println!("OK : {}, sum is {}", iter, sum);
        }
        iter += 1;
    }
}
