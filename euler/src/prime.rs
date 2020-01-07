pub trait Prime {
    fn is_prime(self) -> bool;
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
}

pub struct PrimeIter<T> {
    current: T,
    base: Vec<T>,
}

macro_rules! impl_prime_iter {
    ($t:ty) => {
        impl PrimeIter<$t> {
            pub fn new() -> Self {
                PrimeIter {
                    current: 1,
                    base: Vec::new(),
                }
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
