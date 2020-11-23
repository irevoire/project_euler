struct Triangle {
    n: u128,
}

fn triangle(n: u128) -> u128 {
    n * (n + 1) / 2
}

impl Iterator for Triangle {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let res = triangle(self.n);
        self.n += 1;

        Some(res)
    }
}

struct Pentagonal {
    n: u128,
}

fn pentagonal(n: u128) -> u128 {
    n * (3 * n - 1) / 2
}

impl Iterator for Pentagonal {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let res = pentagonal(self.n);
        self.n += 1;

        Some(res)
    }
}

struct Hexagonal {
    n: u128,
}

fn hexagonal(n: u128) -> u128 {
    n * (2 * n - 1)
}

impl Iterator for Hexagonal {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let res = hexagonal(self.n);
        self.n += 1;

        Some(res)
    }
}

fn main() {
    let mut t = Triangle { n: 1 };
    let mut h = Hexagonal { n: 1 };
    let mut p = Pentagonal { n: 1 };

    let mut ct = t.next().unwrap();
    let mut ch = t.next().unwrap();
    let mut cp = t.next().unwrap();

    loop {
        match (ct, ch, cp) {
            (cct, cch, ccp) if cct == cch && cct == ccp => {
                println!(
                    "Found the match {} for: T({}), H({}) and P({})",
                    cct, t.n, h.n, p.n
                );
                ct = t.next().unwrap();
            }
            (cct, cch, _) if cct < cch => ct = t.next().unwrap(),
            (cct, cch, _) if cct > cch => ch = h.next().unwrap(),

            (cct, _, ccp) if cct < ccp => ct = t.next().unwrap(),
            (cct, _, ccp) if cct > ccp => cp = p.next().unwrap(),

            (_, cch, ccp) if cch < ccp => ch = h.next().unwrap(),
            (_, cch, ccp) if cch > ccp => cp = p.next().unwrap(),

            (_, _, _) => break,
        }
    }
}
