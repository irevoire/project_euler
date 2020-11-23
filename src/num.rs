use crate::Divisors;

pub trait Num {
    fn is_abundant(self) -> bool;

    fn split_digits(self) -> Vec<u8>;
}

macro_rules! impl_num {
    ($t:ty) => {
        impl Num for $t {
            fn is_abundant(self) -> bool {
                self < self.sum_of_divisors()
            }

            /// split the digits of any number
            /// ```rust
            /// use euler::num::Num;
            /// assert_eq!(&123456_u128.split_digits(), &[1, 2, 3, 4, 5, 6]);
            /// assert_eq!(&42_u64.split_digits(), &[4, 2]);
            /// assert_eq!(&2_usize.split_digits(), &[2]);
            /// ```
            fn split_digits(mut self) -> Vec<u8> {
                let mut v = Vec::<u8>::new();
                while self != 0 {
                    v.push((self % 10) as u8);
                    self /= 10;
                }

                v.reverse();
                v
            }
        }
    };
}

crate::impl_for!(impl_num: unsigned);
