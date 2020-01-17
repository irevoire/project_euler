#![feature(test)]

mod macros;

pub mod divisors;
pub mod num;
pub mod prime;

pub use divisors::*;
pub use num::*;
pub use prime::*;

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[bench]
    fn small_bench_prime(b: &mut Bencher) {
        b.iter(|| {
            let mut res = 0;
            for i in 0_u64..1000 {
                res ^= i.all_prime_divisors().count();
            }
            res
        });
    }

    #[bench]
    fn small_bench_divisors(b: &mut Bencher) {
        b.iter(|| {
            let mut res = 0;
            for i in 0_u64..1000 {
                res ^= i.prime_divisors().count();
            }
            res
        });
    }

    #[bench]
    fn large_bench_prime(b: &mut Bencher) {
        b.iter(|| {
            let mut res = 0;
            for i in 1_000_000_u64..1_000_100 {
                res ^= i.all_prime_divisors().count();
            }
            res
        });
    }

    #[bench]
    fn large_bench_divisors(b: &mut Bencher) {
        b.iter(|| {
            let mut res = 0;
            for i in 1_000_000_u64..1_000_100 {
                res ^= i.prime_divisors().count();
            }
            res
        });
    }
}
