use std::ops::Add;
extern crate num;

use num::FromPrimitive;
// use num::bigint::BigInt;
use num::rational::{Ratio, BigRational};

fn main() {
    println!("Hello, world!");
    // println!("{}", Ratio::from_integer(1)/Ratio::from_integer(2));
    println!("{}", make_bigrat(1, 2));
}

trait Field{
    fn zero(&self) -> Self;
    fn one(&self) -> Self;
    fn minus(&self, x: &Self) -> Self;
    // fn recipro(&self);
    // fn subtr(&self);
    // fn mult(&self);
    // fn div(&self);
}

impl Field for BigRational{
    fn zero(&self) -> BigRational{
        make_bigrat(0, 1)
    }
    fn one(&self) -> BigRational{
        make_bigrat(1, 1)
    }
    fn minus(&self, x: &BigRational) -> BigRational{
        -x
    }

}

fn make_bigrat(num: u64, den: u64) -> BigRational{
    Ratio::from_integer(FromPrimitive::from_u64(num).unwrap())/Ratio::from_integer(FromPrimitive::from_u64(den).unwrap())
}