use std::ops::Add;
extern crate num;

use num::FromPrimitive;
// use num::bigint::BigInt;
use num::rational::{Ratio, BigRational, Rational};

fn main() {
    println!("Hello, world!");
    // println!("{}", Ratio::from_integer(1)/Ratio::from_integer(2));
    println!("{}", make_rat(1, 2));
}

trait Field: Sized{
    fn zero() -> Self;
    fn one() -> Self;
    fn minus(&self) -> Self;
    fn recipro(&self) -> Option<Self>;
    // fn subtr(&self), x: &Self, y: &Self) -> Self {
    //     self.add(x, y.minus())
    // }
    // fn mult(&self);
    // fn div(&self);
}

impl Field for Rational{
    fn zero() -> Rational{
        make_rat(0, 1)
    }
    fn one() -> Rational{
        make_rat(1, 1)
    }
    fn minus(&self) -> Rational{
        -self
    }
    fn recipro(&self) -> Option<Rational>{
        if(*self == Self::zero()){
            None
        }else{
            Some(Self::one()/(*self))
        }
    }

}

fn make_rat(num: u64, den: u64) -> Rational{
    Ratio::from_integer(FromPrimitive::from_u64(num).unwrap())/Ratio::from_integer(FromPrimitive::from_u64(den).unwrap())
}