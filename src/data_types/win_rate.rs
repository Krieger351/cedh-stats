use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct WinRate(f64);

impl<'a> Sum<&'a WinRate> for WinRate {
    fn sum<I: Iterator<Item=&'a WinRate>>(iter: I) -> Self {
        let total = iter.map(|win_rate| win_rate.0).sum();
        WinRate(total)
    }
}

impl WinRate {
    pub fn from_f64(f: f64) -> WinRate {
        WinRate(f)
    }
    pub fn powi(&self, power: i32) -> WinRate {
        WinRate(self.0.powi(power))
    }
    pub fn abs(&self) -> WinRate {
        WinRate(self.0.abs())
    }
}

impl Display for WinRate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Sum for WinRate {
    fn sum<I: Iterator<Item=Self>>(_iter: I) -> Self {
        todo!()
    }
}

impl PartialEq<f64> for WinRate {
    fn eq(&self, other: &f64) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<f64> for WinRate {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialEq for WinRate {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl PartialOrd for WinRate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl Sub for &WinRate {
    type Output = WinRate;

    fn sub(self, rhs: Self) -> Self::Output {
        WinRate(self.0 - rhs.0)
    }
}

impl Mul for &WinRate {
    type Output = WinRate;

    fn mul(self, rhs: Self) -> Self::Output {
        WinRate(self.0 * rhs.0)
    }
}

impl Div<usize> for WinRate {
    type Output = WinRate;

    fn div(self, rhs: usize) -> Self::Output {
        WinRate(self.0 / rhs as f64)
    }
}

impl Div<f64> for WinRate {
    type Output = WinRate;

    fn div(self, rhs: f64) -> Self::Output {
        WinRate(self.0 / rhs)
    }
}

impl Div for WinRate {
    type Output = WinRate;

    fn div(self, rhs: Self) -> Self::Output {
        WinRate(self.0 / rhs.0)
    }
}

impl Add for &WinRate {
    type Output = WinRate;

    fn add(self, rhs: Self) -> Self::Output {
        WinRate(self.0 + rhs.0)
    }
}