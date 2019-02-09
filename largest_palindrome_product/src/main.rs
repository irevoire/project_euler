/*
 * Largest palindrome product
 * Problem 4
 * A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
 * 
 * Find the largest palindrome made from the product of two 3-digit numbers.
 */

fn reverse(n: u32) -> u32 {
    let mut rev = 0;
    let mut n = n;

    loop {
        let digit = n % 10;
        rev = (rev * 10) + digit;
        n = n / 10;

        if n == 0 {
            break
        }
    }
    return rev
}

fn is_palindrome(n: u32) -> bool {
    n == reverse(n)
}

fn main() {
    let mut max = 0;

    for a in 100..1000 {
        for b in 100..1000 {
            let product = a * b;
            if is_palindrome(product) {
                if product > max {
                    max = product;
                }
            }
        }
    }

    println!("biggest palindrome is {}", max);
}
