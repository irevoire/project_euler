fn explode(mut n: u64) -> Vec<u64> {
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

fn compute(arr: Vec<u64>) -> u64 {
    return arr.iter().fold(0 as u64, |acc, el| acc + el.pow(5));
}

fn main() {
    let mut idx = 1;
    let mut sum = 0;
    loop {
        idx += 1;
        if compute(explode(idx)) == idx {
            sum += idx;
            println!("Found {}, current sum is {}", idx, sum);
        }
    }
}
