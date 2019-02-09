/*
Self powers
Problem 48
The series, 1^1 + 2^2 + 3^3 + ... + 10^10 = 10405071317.

Find the last ten digits of the series, 1^1 + 2^2 + 3^3 + ... + 1000^1000.
 */ 

use rug::Integer;
use rug::ops::Pow;

fn main() {
    let mut sum = Integer::from(0);
    let limit = 1000 + 1;
    for i in 1..limit {
        let cur = Integer::from(i).pow(i);
        sum += &cur;
    }
    println!("sum is : {}", sum);
}
