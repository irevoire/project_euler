use rug::Integer;

pub struct Fibo {
    current: Integer,
    next: Integer,
}

impl Fibo {
    pub fn new() -> Fibo {
        Fibo {
            // The first positive is 1, so let's start at 1.
            current: Integer::from(1),
            next: Integer::from(0),
        }
    }
}

impl Iterator for Fibo {
    type Item = Integer;

    fn next(&mut self) -> Option<Self::Item> {
        // the next triangular next.
        let sum = &self.next + &self.current;
        let sum = Integer::from(sum);
        self.current = Integer::from(&self.next);
        self.next = sum;
        // return the current one.
        Some(Integer::from(&self.current))
    }
}
