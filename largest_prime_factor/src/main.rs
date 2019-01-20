// Largest prime factor
// Problem 3
// The prime factors of 13195 are 5, 7, 13 and 29.
// 
// What is the largest prime factor of the number 600851475143 ?
fn gcd(a : u32, b : u32) -> u32 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    return a;
}

fn main() {
    let number = 13195;
    let mut iteration = 1;
    let x_fixed = 2;
    let size = 2;
    let mut x = 2;
    let factor = 1;

    while (factor == 1) {
        println!("----   iteration {}   ----", iteration);
        let mut count = 1;
        while (count <= size) && (factor <= 1) {
            x = (x * x + 1) % number;
            factor = gcd((x - x_fixed), number);
            println!("count = {}  x = {}  factor = {}", count, x, factor);
            count += 1;
        }
        size = 2 * size;
        x_fixed = x;
        iteration = iteration + 1;
    }
    println!("factor is {}", factor);
}
