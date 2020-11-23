/*
 * Factorial digit sum
 * Problem 20
 * n! means n × (n − 1) × ... × 3 × 2 × 1
 * 
 * For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
 * and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
 * 
 * Find the sum of the digits in the number 100!
 */

use rug::{Assign, Integer}; // some bigint implementation

fn sum_digit(n : Integer) -> Integer {
    let mut sum : Integer = Integer::new();
    sum.assign(0);
    let p = n;
    let mut n : Integer = Integer::new();
    n.assign(p);

    loop {
        let d = &n % 10;
        let d = Integer::from(d);
        sum += d;

        if n < 10 {
            break
        }
        n /= 10;
    }
    return sum
}

fn fact(n : Integer) -> Integer {
    if n < 2 {
        n
    } else {
        let res = Integer::from(&n - 1);
        let res = fact(res);
        n * res
    }
}

fn main() {
    let mut n : Integer = Integer::new();
    n.assign(100);
    let res = fact(n);
    println!("fact = {}", res);
    println!("sum = {}", sum_digit(res));
}
