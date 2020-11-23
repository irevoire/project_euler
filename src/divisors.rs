pub trait Divisors {
    fn divisors(self) -> Box<dyn Iterator<Item = Self>>;
    fn sum_of_divisors(self) -> Self;
    fn divide(self, denom: Self) -> Box<dyn Iterator<Item = Self>>;
    fn cycle(self, denom: Self) -> Option<Vec<Self>>
    where
        Self: Sized;
}

macro_rules! impl_divisors {
    ($t:ty) => {
        impl Divisors for $t {
            fn divisors(self) -> Box<dyn Iterator<Item = Self>> {
                Box::new((1..self / 2 + 1).filter(move |divisor| self % divisor == 0))
            }

            fn sum_of_divisors(self) -> Self {
                self.divisors().sum()
            }

            fn divide(self, denom: Self) -> Box<dyn Iterator<Item = Self>> {
                Box::new(Divide::<$t>::new(self, denom))
            }

            fn cycle(self, denom: Self) -> Option<Vec<Self>> {
                Divide::<$t>::new(self, denom).get_cycle()
            }
        }
    };
}

crate::impl_for!(impl_divisors: unsigned);

use std::collections::HashMap;

struct Divide<T> {
    num: T,
    denom: T,
}

macro_rules! impl_divide {
    ($t:ty) => {
        impl Divide<$t> {
            pub fn new(num: $t, denom: $t) -> Self {
                Self { num, denom }
            }

            pub fn get_cycle(&mut self) -> Option<Vec<$t>> {
                let mut digits = Vec::new();
                // used to store the position of each couple num / denom
                let mut hash: HashMap<($t, $t), usize> = HashMap::new();
                while !hash.contains_key(&(self.num, self.denom)) {
                    hash.insert((self.num, self.denom), digits.len());
                    if let Some(next) = self.next() {
                        digits.push(next);
                    } else {
                        return None;
                    }
                }
                let start = hash.get(&(self.num, self.denom)).unwrap();
                Some(digits[*start..].to_vec())
            }
        }

        impl Iterator for Divide<$t> {
            type Item = $t;
            fn next(&mut self) -> Option<Self::Item> {
                if self.num == 0 {
                    return None;
                }
                while self.num < self.denom {
                    self.num *= 10;
                }
                let res = self.num / self.denom;
                self.num %= self.denom;

                Some(res)
            }
        }
    };
}

crate::impl_for!(impl_divide: unsigned);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_test() {
        assert_eq!(vec![5], 1.divide(2).collect::<Vec<u32>>());
        assert_eq!(vec![3, 3, 3], 1.divide(3).take(3).collect::<Vec<u64>>());
        assert_eq!(vec![2, 5], 1.divide(4).collect::<Vec<u128>>());
        assert_eq!(vec![2], 1.divide(5).collect::<Vec<u16>>());
    }

    #[test]
    fn cycle_test() {
        assert_eq!(None, 1_u32.cycle(2));
        assert_eq!(Some(vec![3]), 1_u32.cycle(3));
        assert_eq!(Some(vec![1, 4, 2, 8, 5, 7]), 1_u32.cycle(7));
    }
}
