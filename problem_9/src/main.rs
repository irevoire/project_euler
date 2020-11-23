/*
 * Special Pythagorean triplet
 * Problem 9 
 * 
 * A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
 * a^2 + b^2 = c^2
 * For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
 * 
 * There exists exactly one Pythagorean triplet for which a + b + c = 1000.
 * Find the product abc.
*/

fn is_pythagorian_triplet(a: u32, b: u32, c: u32) -> bool {
    a.pow(2) + b.pow(2) == c.pow(2)
}

fn main() {
    for a in 1..1000 {
        for b in a..1000 {
            let c = 1000 - a - b;
            if is_pythagorian_triplet(a, b, c) {
                println!("found {}, {}, {} : {}", a, b, c, a * b * c)
            }
        }
    }
}
