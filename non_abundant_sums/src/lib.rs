/*
use std::cmp::Eq;
use std::ops::{Div, Rem};
*/

pub trait Divisors {
    fn divisors(self) -> Box<dyn Iterator<Item = Self>>;
    fn sum_of_divisors(self) -> Self;
    fn is_abundant_number(self) -> bool;
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

            fn is_abundant_number(self) -> bool {
                self < self.sum_of_divisors()
            }
        }
    };
}

impl_divisors!(u8);
impl_divisors!(u32);
impl_divisors!(u64);
impl_divisors!(u128);
impl_divisors!(i8);
impl_divisors!(i32);
impl_divisors!(i64);
impl_divisors!(i128);

/*
impl<T> Divisors for T
where
    T: Rem<Output = T> + Div<Output = T> + Eq,
{
    fn divisors(self) -> Box<dyn Iterator<Item = Self>> {
        Box::new((1..self / 2 as T).filter(move |divisor| self % divisor == 0))
    }
}
*/
