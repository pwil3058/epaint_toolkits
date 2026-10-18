// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::ops::{AddAssign, Neg};
use std::{
    cmp::Ordering,
    fmt::{self, Debug, Formatter},
    ops::{Add, Div, Mul, Rem, Sub},
};

use serde::{Deserialize, Serialize};

use crate::{
    debug::{AbsDiff, ApproxEq, PropDiff},
    UFDRNum,
};

#[macro_export]
macro_rules! impl_to_from_float {
    ($float:ty, $core:ty, $number:ty) => {
        impl From<$float> for $number {
            fn from(arg: $float) -> Self {
                Self((arg * u64::MAX as $float) as $core)
            }
        }

        impl From<$number> for $float {
            fn from(arg: $number) -> Self {
                arg.0 as $float / u64::MAX as $float
            }
        }
    };
}

#[derive(Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct FDRNum(pub(crate) i128);

// NB: ONE is the same value as for UFDRNumber
impl FDRNum {
    pub const ZERO: Self = Self(0);
    // u64::MAX: 18446744073709551615
    pub const ONE: Self = Self(u64::MAX as i128);
    // SQRT_2: 1.41421356237309504880168872420969808
    pub const SQRT_2: Self =
        Self(u64::MAX as i128 + 4142135623730950488 * u64::MAX as i128 / 10000000000000000000);
    pub const SQRT_3: Self =
        Self(u64::MAX as i128 + 17320508075688772 * u64::MAX as i128 / 10000000000000000);

    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }

    pub fn is_valid_sum(self) -> bool {
        self >= Self::ZERO && self <= Self::ONE * 3
    }

    pub fn is_hue_valid(self) -> bool {
        self > Self::ZERO && self < Self::ONE * 3
    }

    pub fn is_proportion(self) -> bool {
        self >= Self::ZERO && self <= Self::ONE
    }

    pub fn abs_diff(&self, other: &Self) -> FDRNum {
        match self.cmp(other) {
            Ordering::Greater => FDRNum(self.0 - other.0),
            Ordering::Less => FDRNum(other.0 - self.0),
            Ordering::Equal => FDRNum(0),
        }
    }
}

impl Debug for FDRNum {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let int = self.0 / u64::MAX as i128;
        let frac = self.0 % u64::MAX as i128;
        formatter.write_fmt(format_args!("FDRNumber({:X}.{:016X})", int, frac.abs()))
        //formatter.write_fmt(format_args!("UFDRNumber({:?})", f64::from(*self)))
    }
}

impl fmt::UpperHex for FDRNum {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.0, formatter)
    }
}

#[cfg(test)]
impl FDRNum {
    pub fn approx_eq(&self, other: &Self, acceptable_rounding_error: Option<u64>) -> bool {
        let abs_diff = self.abs_diff(other);
        let scaled_diff = if self.0.abs() >= other.0.abs() {
            if self.0.abs() > 0 {
                (u128::MAX / self.0.abs() as u128) * abs_diff.0 as u128 / u64::MAX as u128
            } else {
                abs_diff.0 as u128
            }
        } else {
            (u128::MAX / other.0.abs() as u128) * abs_diff.0 as u128 / u64::MAX as u128
        };
        if let Some(acceptable_rounding_error) = acceptable_rounding_error {
            scaled_diff < acceptable_rounding_error as u128
        } else {
            scaled_diff < u64::MAX as u128 / 1_000_000_000_000_000
        }
    }
}

impl Div for FDRNum {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        if rhs == Self::ONE {
            self
        } else {
            let val: Self = UFDRNum::from(self.abs())
                .div(UFDRNum::from(rhs.abs()))
                .into();
            if self.0.is_negative() {
                if rhs.0.is_negative() {
                    val
                } else {
                    val.neg()
                }
            } else if rhs.0.is_negative() {
                val.neg()
            } else {
                val
            }
        }
    }
}

impl Div<i32> for FDRNum {
    type Output = Self;

    fn div(self, rhs: i32) -> Self {
        Self(self.0.div(rhs as i128))
    }
}

impl Add for FDRNum {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0.add(other.0))
    }
}

impl Sub for FDRNum {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0.sub(other.0))
    }
}

impl Neg for FDRNum {
    type Output = Self;

    fn neg(self) -> Self {
        Self(self.0.neg())
    }
}

impl Mul for FDRNum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let val: Self = UFDRNum::from(self.abs())
            .mul(UFDRNum::from(rhs.abs()))
            .into();
        if self.0.is_negative() {
            if rhs.0.is_negative() {
                val
            } else {
                val.neg()
            }
        } else if rhs.0.is_negative() {
            val.neg()
        } else {
            val
        }
    }
}

impl Mul<i32> for FDRNum {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Self(self.0.mul(rhs as i128))
    }
}

impl From<UFDRNum> for FDRNum {
    fn from(unsigned: UFDRNum) -> Self {
        Self(unsigned.0 as i128)
    }
}

impl_to_from_float!(f64, i128, FDRNum);
impl_to_from_float!(f32, i128, FDRNum);

macro_rules! impl_to_from_signed {
    (i128) => {
        impl From<i128> for FDRNum {
            fn from(signed: i128) -> Self {
                Self(signed)
            }
        }

        impl From<FDRNum> for i128 {
            fn from(arg: FDRNum) -> Self {
                arg.0
            }
        }
    };
    ($signed:ty) => {
        impl From<$signed> for FDRNum {
            fn from(signed: $signed) -> Self {
                Self(signed as i128 * u64::MAX as *const () as i128)
            }
        }

        impl From<FDRNum> for $signed {
            fn from(arg: FDRNum) -> Self {
                (arg.0 / u64::MAX as i128) as $signed
            }
        }
    };
}

impl_to_from_signed!(i8);
impl_to_from_signed!(i16);
impl_to_from_signed!(i32);
impl_to_from_signed!(i64);
impl_to_from_signed!(i128);
