use rug::Integer;

mod iter;

fn is_pandigital(n: u64) -> bool {
    let mut digits = [0; 10];
    let mut n = n;
    loop {
        let d = (n % 10) as usize;
        if d == 0 { return false }
        if digits[d] != 0 {
            return false
        }
        digits[d] = 1;
        if n < 10 {
            break
        }
        n /= 10;
    }
    return digits.iter().fold(0, |acc, el| acc + el) == 9
}

fn main() {
    for (index, t) in iter::Fibo::new().enumerate() {
        let start: String = t.to_string().chars().take(9).collect();
        let end: String = t.to_string().chars().rev().take(9).collect();
        let start: u64 = start.parse().unwrap();
        let end: u64 = end.parse().unwrap();

        if is_pandigital(start) && is_pandigital(end) {
            println!("FOUND AT IDNEX {}", index);
            println!("start : {}", start);
            println!("end : {}", end);
            break;
        }
    }
}
