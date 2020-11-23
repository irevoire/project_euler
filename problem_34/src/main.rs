fn explode(mut n: u128) -> Vec<u128> {
    let mut res = Vec::new();

    loop {
        let d = n % 10;
        res.push(d);
        if n < 10 {
            break
        }
        n /= 10;
    }

    res.reverse();
    return res;
}

fn fact(n: u128) -> u128 {
    if n <= 1 { return 1 };
    return n * fact(n - 1);
}

fn compute(arr: Vec<u128>) -> u128 {
    return arr.iter().fold(0 as u128, |acc, el| acc + fact(*el));
}

fn main() {
    let mut idx = 2;
    let mut sum = 0;
    loop {
        idx += 1;
        let res = compute(explode(idx));

        if res == idx {
            sum += idx;
            println!("Found {}, current sum is {}", idx, sum);
        }
    }
}
