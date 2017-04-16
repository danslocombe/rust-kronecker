#![feature(zero_one)]
#![allow(deprecated)]
#![deny(missing_docs)]
#![deny(warnings)]

//! Expose function 'delta' that computes the Kronecker Delta function.
//! https://en.wikipedia.org/wiki/Kronecker_delta

use std::num::{One, Zero};

/// Returns one if x == y otherwise zero.
pub fn delta<A : Eq, B : One + Zero>(x : &A, y : &A) -> B {
    if x == y {
        B::one()
    }
    else {
        B::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::delta;

    #[test]
    fn test_one() {
        assert_eq!(delta::<usize, usize>(&0, &1), 0)
    }

    #[test]
    fn test_zero() {
        assert_eq!(delta::<usize, usize>(&1, &1), 1)
    }
}
