pub trait Divisors {
    fn divisors(self) -> Box<dyn Iterator<Item = Self>>;
    fn sum_of_divisors(self) -> Self;
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
        }
    };
}

crate::impl_for!(impl_divisors: unsigned);
