fn is_palindrome(s: &str) -> bool {
    let half_len = s.len()/2;
    s.chars().take(half_len).eq(s.chars().rev().take(half_len))
}

fn main() {
    let mut sum = 0;
    for n in 0..1_000_000 {
        let num = n.to_string();
        let bin = format!("{:b}", n);

        if is_palindrome(&num) && is_palindrome(&bin) {
            sum += n;
            println!("{} is {} and sum is {}", num, bin, sum);
        }

    }

    println!("Sum is {}", sum);
}
