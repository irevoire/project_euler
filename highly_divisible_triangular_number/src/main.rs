struct Triangular {
    current: u32,
    number: u32,
}

impl Triangular {
    fn new() -> Triangular {
        Triangular {
            // The first positive is 1, so let's start at 1.
            current: 1,
            number: 1,
        }
    }
}

impl Iterator for Triangular {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // the next triangular number.
        self.number += 1;
        self.current += self.number;
        // return the current one.
        Some(self.current - self.number)
    }
}

fn nb_divisors(n: u32) -> u32 {
    let mut cpt = 1;
    for div in 1..n {
        if n % div == 0 {
            cpt += 1;
        }
    }
    return cpt;
}

fn main() {
    for t in Triangular::new() {
        let divisors = nb_divisors(t);
        if divisors >= 500 {
            println!("number {} has {} divisors", t, divisors);
            break
        }
    }
}
