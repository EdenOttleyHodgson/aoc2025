use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum LeftOrRight {
    Left,
    Right,
}

pub fn div_rem<T>(x: T, y: T) -> (T, T)
where
    T: std::ops::Div<Output = T> + std::ops::Rem<Output = T> + Copy + Debug,
{
    let quot = x / y;
    let rem = x % y;

    println!("{x:?} / {y:?} = {quot:?} r {rem:?}");

    (quot, rem)
}
