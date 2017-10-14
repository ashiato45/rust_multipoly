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
    fn zero() -> Self;
    fn one() -> Self;
    fn minus(&self) -> Self;
    // fn recipro(&self) -> Option<Self>;
    // fn subtr(&self), x: &Self, y: &Self) -> Self {
    //     self.add(x, y.minus())
    // }
    // fn mult(&self);
    // fn div(&self);
}

impl Field for BigRational{
    fn zero() -> BigRational{
        make_bigrat(0, 1)
    }
    fn one() -> BigRational{
        make_bigrat(1, 1)
    }
    fn minus(&self) -> BigRational{
        -self
    }
    // fn recipro(&self, x: &BigRational) -> Option<BigRational>{
    //     if(x == )
    // }

}

fn make_bigrat(num: u64, den: u64) -> BigRational{
    Ratio::from_integer(FromPrimitive::from_u64(num).unwrap())/Ratio::from_integer(FromPrimitive::from_u64(den).unwrap())
}