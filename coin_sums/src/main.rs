fn _possibilities(amount: i32, coins: &[u32]) -> u32 {
    let mut sum = 0;
    if coins.is_empty() {
        return 1;
    }
    for (i, &c) in coins.iter().enumerate() {
        match amount - c as i32 {
            0 => sum += 1,
            n if n > 0 => sum += _possibilities(n, &coins[i..]),
            _ => (), // nothing on negative
        }
    }
    sum
}

fn possibilities(c: u32, coins: &[u32]) -> u32 {
    _possibilities(c as i32, coins)
}

fn main() {
    let mut coins = [1, 2, 5, 10, 20, 50, 100, 200];
    coins.reverse();

    println!("res: {:?}", possibilities(200, &coins));
}
