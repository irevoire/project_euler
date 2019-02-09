pub struct Prime {
    current: u64,
    base : Vec<u64>,
}

impl Prime {
    pub fn new() -> Prime {
        Prime {
            current: 1,
            base : Vec::new(),
        }
    }

    pub fn is_prime(&self, n : u64) -> bool {
        for i in &self.base {
            if n % i == 0 {
                return false
            }
        }
        return true
    }
}

impl Iterator for Prime {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.current += 1;
            if self.is_prime(self.current) {
                self.base.push(self.current);
                break;
            }
        }

        Some(self.current)
    }
}
