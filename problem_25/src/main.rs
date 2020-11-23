/*
 * The Fibonacci sequence is defined by the recurrence relation:
 * 
 * Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
 * Hence the first 12 terms will be:
 * 
 * F1 = 1
 * F2 = 1
 * F3 = 2
 * F4 = 3
 * F5 = 5
 * F6 = 8
 * F7 = 13
 * F8 = 21
 * F9 = 34
 * F10 = 55
 * F11 = 89
 * F12 = 144
 * The 12th term, F12, is the first term to contain three digits.
 *
 * What is the index of the first term in the Fibonacci sequence to contain 1000 digits?
 */ 

use rug::{Integer, Assign};

mod iter;

fn nb_digit(n : &Integer) -> u32 {
    let mut sum = 0;
    let p = n;
    let mut n : Integer = Integer::new();
    n.assign(p);

    loop {
        sum += 1;

        if n < 10 {
            break
        }
        n /= 10;
    }
    return sum
}

fn main() {
    let mut index = Integer::from(0);
    let limit = 1000;
    for t in iter::Fibo::new() {
        index += 1;
        if nb_digit(&t) == limit {
            println!("fibo({}) = {}", index, t);
            break;
        }
    }
}
