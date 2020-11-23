pub trait Prime {
    fn is_prime(self) -> bool;
    fn all_prime_divisors(self) -> Box<dyn Iterator<Item = Self>>;
}

macro_rules! impl_prime {
    ($t:ty) => {
        impl Prime for $t {
            fn is_prime(self) -> bool {
                if self < 2 {
                    return false;
                }
                if self == 2 {
                    return true;
                }
                if self % 2 == 0 {
                    return false;
                }
                !(3..self / 2).step_by(2).any(|el| self % el == 0)
            }

            fn all_prime_divisors(self) -> Box<dyn Iterator<Item = Self>> {
                PrimeFactor::<Self>::new(self)
            }
        }
    };
}

crate::impl_for!(impl_prime: unsigned);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_prime() {
        assert_eq!(0_u32.is_prime(), false);
        assert_eq!(1_u8.is_prime(), false);
        assert_eq!(2_u16.is_prime(), true);
        assert_eq!(3_u64.is_prime(), true);
        assert_eq!(4_u128.is_prime(), false);
        assert_eq!(5_u32.is_prime(), true);
        assert_eq!(69191_u32.is_prime(), true);
        assert_eq!(69193_u32.is_prime(), true);
        assert_eq!(69195_u32.is_prime(), false);
        assert_eq!(23456789_u32.is_prime(), true);
        assert_eq!(123456789_u32.is_prime(), false);
    }

    #[test]
    fn prime_divisors() {
        let empty: Vec<u64> = Vec::new();
        assert_eq!(empty, 0_u64.all_prime_divisors().collect::<Vec<u64>>());
        assert_eq!(empty, 1_u64.all_prime_divisors().collect::<Vec<u64>>());
        assert_eq!(vec![2], 2_u64.all_prime_divisors().collect::<Vec<u64>>());
        assert_eq!(vec![2, 2], 4_u64.all_prime_divisors().collect::<Vec<u64>>());
        assert_eq!(
            vec![2, 2, 2],
            8_u64.all_prime_divisors().collect::<Vec<u64>>()
        );
        assert_eq!(vec![2, 3], 6_u64.all_prime_divisors().collect::<Vec<u64>>());
        assert_eq!(
            vec![3, 7],
            21_u64.all_prime_divisors().collect::<Vec<u64>>()
        );
    }
}

pub struct PrimeIter<T> {
    current: T,
    base: Vec<T>,
}

macro_rules! impl_prime_iter {
    ($t:ty) => {
        impl PrimeIter<$t> {
            pub fn new() -> Box<dyn Iterator<Item = $t>> {
                Box::new(PrimeIter::<$t> {
                    current: 1,
                    base: Vec::new(),
                })
            }

            pub fn is_prime(&self, n: $t) -> bool {
                for i in &self.base {
                    if n % i == 0 {
                        return false;
                    }
                }
                return true;
            }
        }

        impl Iterator for PrimeIter<$t> {
            type Item = $t;

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
    };
}

crate::impl_for!(impl_prime_iter: unsigned);

pub struct PrimeFactor<T> {
    current: T,
    primes: Box<dyn Iterator<Item = T>>,
    prime: T,
}

macro_rules! impl_prime_factor {
    ($t:ty) => {
        impl PrimeFactor<$t> {
            pub fn new(n: $t) -> Box<dyn Iterator<Item = $t>> {
                let mut primes = PrimeIter::<$t>::new();
                Box::new(PrimeFactor::<$t> {
                    current: n,
                    prime: primes.next().unwrap(),
                    primes,
                })
            }
        }

        impl Iterator for PrimeFactor<$t> {
            type Item = $t;

            fn next(&mut self) -> Option<Self::Item> {
                if self.current <= 0 {
                    return None;
                }
                while self.current % self.prime != 0 {
                    if self.current < self.prime {
                        return None;
                    }
                    self.prime = self.primes.next().unwrap();
                }
                self.current /= self.prime;
                Some(self.prime)
            }
        }
    };
}

crate::impl_for!(impl_prime_factor: unsigned);
