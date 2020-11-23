mod permutation;

pub fn is_prime(n: u64) -> bool {
    let limit = (n as f64).sqrt() as u64 + 1;

    for i in 2..limit {
        if n % i == 0 { return false }
    }
    return true

}

fn main() {
    let mut base = String::from("1234567890");
    let mut max = 0;

    while base.len() > 1 {
        base.pop();

        let permutation = permutation::generate_all_permutations(base.clone());
        for n in permutation {
            let n: u64 = n.parse().unwrap();
            if is_prime(n) && max < n { max = n }
        }
    }
    println!("Max is {}", max);
}
