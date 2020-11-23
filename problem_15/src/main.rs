/*
 * Actually the problem is pretty easy.
 *
 * To go from the (0, 0) to the (20, 20) we will do 20 move to the right and 20 move to the bottom.
 * So the answer will be the number of combination.
 */

fn fact(n: u128) -> u128 {
    if n <= 1 { return 1 }
    return n * fact(n - 1)
}

/*
 * if you want to compute 8! / 5! you can only compute 8 * 7 * 6
 * So you can call this function with the arguments semi_fact(8, 3)
 */
fn semi_fact(n: u128, i: u128) -> u128 {
    if n <= 1 || i <= 0 { return 1 }
    return n * semi_fact(n - 1, i - 1)
}

fn permutation(n: u128, k: u128) -> u128 {
    semi_fact(n, k)
}

fn combination(n: u128, k: u128) -> u128 {
    permutation(n, k) / fact(k)
}

fn main() {
    let k = 40;
    let n = 20;
    println!("{}", combination(k, n));
}
