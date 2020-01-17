use crate::Divisors;

pub trait Num {
    fn is_abundant(self) -> bool;
}

macro_rules! impl_num {
    ($t:ty) => {
        impl Num for $t {
            fn is_abundant(self) -> bool {
                self < self.sum_of_divisors()
            }
        }
    };
}

crate::impl_for!(impl_num: unsigned);
