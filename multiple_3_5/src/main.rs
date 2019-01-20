// Multiples of 3 and 5
// Problem 1
//
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.
fn main() {
    let v = multiple(7);
    println!("{:?}", run(999));
}

fn run(n : u32) -> u32 {
    if n <= 0 {
        return 0
    }
    return multiple(n) + run(n - 1)
}

fn multiple(n : u32) -> u32 {
    if n % 3 == 0 {
        return n
    } else if n % 5 == 0 {
        return n
    }
    0
}
