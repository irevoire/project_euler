fn apply_fact(coins: &[u32], factor: &[u32]) -> u32 {
    coins
        .iter()
        .zip(factor.iter())
        .fold(0, |acc, (c, f)| acc + c * f)
}

fn increment(coins: &[u32], factor: &mut [u32]) {
    for i in 0..factor.len() {
        if factor[i] * coins[i] >= 200 {
            factor[i] = 0;
        } else {
            factor[i] += 1;
            break;
        }
    }
}

fn main() {
    let mut coins = [1, 2, 5, 10, 20, 50, 100, 200];
    let mut factor = [0; 8];

    println!("factor: {:?}", factor);
    println!("coins: {:?}", coins);

    let mut cpt = 0;
    loop {
        if apply_fact(&coins, &factor) == 200 {
            cpt += 1;
        }
        if *factor.last().unwrap() == 1 {
            break;
        }
        increment(&coins, &mut factor);
    }
    println!("res = {}", cpt);
}
