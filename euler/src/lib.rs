#![feature(test)]

mod macros;

pub mod divisors;
pub mod num;
pub mod prime;

pub use divisors::*;
pub use num::*;
pub use prime::*;
