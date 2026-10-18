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
    impl_to_from_float, FDRNum, Prop, Unsigned, UnsignedInteger,
};

#[derive(Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct UFDRNum(pub(crate) u128);

impl Unsigned for UFDRNum {
    const MAX: Self = UFDRNum(u128::MAX);
    const ZERO: Self = UFDRNum(0);
}

impl UFDRNum {
    pub const DENOMINATOR: u128 = u64::MAX as u128;
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(Self::DENOMINATOR);
    pub const TWO: Self = Self(u64::MAX as u128 * 2);
    pub const THREE: Self = Self(u64::MAX as u128 * 3);

    pub const SQRT_2: Self = Self(FDRNum::SQRT_2.0 as u128);
    pub const SQRT_3: Self = Self(FDRNum::SQRT_3.0 as u128);

    pub fn is_valid_sum(self) -> bool {
        self <= Self::THREE
    }

    pub fn is_hue_valid(self) -> bool {
        self > Self::ZERO && self < Self::THREE
    }

    pub fn is_proportion(self) -> bool {
        self <= Self::ONE
    }

    pub fn to_prop(&self) -> Prop {
        debug_assert!(self.is_proportion());
        Prop(self.0 as u64)
    }

    /// Generate an UFDRNum from a rational number
    ///
    /// Example:
    /// ```
    /// use colour_math::{assert_approx_eq, UFDRNum, Prop};
    /// use colour_math::debug::ApproxEq;
    ///
    /// assert_eq!(UFDRNum::from_rational_num(4, 2), UFDRNum::ONE * 2_u8);
    /// assert_approx_eq!(UFDRNum::from_rational_num(5, 2), UFDRNum::from(2.5), 2);
    /// assert_eq!(UFDRNum::from_rational_num(17, 17), UFDRNum::ONE);
    /// assert_approx_eq!(UFDRNum::from_rational_num(5, 10), UFDRNum::ONE / 2, 1);
    /// assert_eq!(UFDRNum::from_rational_num(u64::MAX as u128 * 2, u64::MAX as u128), UFDRNum::ONE * 2u8);
    /// ```
    pub fn from_rational_num(enumerator: u128, denominator: u128) -> Self {
        if enumerator == 0 {
            return Self::ZERO;
        }
        use Ordering::*;
        match denominator.cmp(&enumerator) {
            Equal => Self::ONE,
            Greater => {
                let shift = enumerator.leading_zeros();
                let result = (enumerator << shift) / denominator;
                Self(result >> (shift - 64))
            }
            Less => {
                let int = Self(enumerator / denominator * Self::DENOMINATOR);
                int + Self::from_rational_num(enumerator % denominator, denominator)
            }
        }
    }

    /// Return the square root of our value.
    ///
    /// Example:
    /// ```
    /// use colour_math::{UFDRNum, debug::*};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// let four: UFDRNum = UFDRNum::ONE * 4u32;
    /// let two: UFDRNum = UFDRNum::ONE * 2u32;
    /// assert_approx_eq!(four.sqrt(), two);
    /// let twenty_five: UFDRNum = UFDRNum::ONE * 25u32;
    /// let five: UFDRNum = UFDRNum::ONE * 5u32;
    /// assert_approx_eq!(twenty_five.sqrt(), five);
    /// ```
    pub fn sqrt(self) -> Self {
        let half_shift = self.0.leading_zeros() / 2;
        let temp = (self.0 << (half_shift as usize * 2)).isqrt();
        Self(temp << (u64::BITS / 2 - half_shift))
    }
}

impl Debug for UFDRNum {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let int = self.0 / u64::MAX as u128;
        let frac = self.0 % u64::MAX as u128;
        formatter.write_fmt(format_args!("UFDRNumber({int:X}.{frac:016X})"))
    }
}

impl fmt::UpperHex for UFDRNum {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.0, formatter)
    }
}

impl AbsDiff for UFDRNum {}

impl PropDiff for UFDRNum {
    fn prop_diff(&self, other: &Self) -> Option<Prop> {
        if self.is_proportion() && other.is_proportion() {
            Some(self.abs_diff(other).into())
        } else {
            let denom = self.max(other);
            if *denom == UFDRNum::ZERO {
                Some(Prop::ZERO)
            } else {
                Some(Prop::from(self.abs_diff(other) / *denom))
            }
        }
    }
}

impl ApproxEq for UFDRNum {}

impl From<FDRNum> for UFDRNum {
    fn from(signed: FDRNum) -> Self {
        debug_assert!(signed.0 >= 0);
        Self(signed.0 as u128)
    }
}

impl_to_from_float!(f64, u128, UFDRNum);
impl_to_from_float!(f32, u128, UFDRNum);

impl From<i32> for UFDRNum {
    fn from(signed: i32) -> Self {
        debug_assert!(signed > 0);
        Self(signed as u128 * u64::MAX as u128)
    }
}

macro_rules! impl_to_from_unsigned {
    (u128) => {
        impl From<u128> for UFDRNum {
            fn from(unsigned: u128) -> Self {
                Self(unsigned)
            }
        }

        impl From<UFDRNum> for u128 {
            fn from(arg: UFDRNum) -> Self {
                arg.0
            }
        }
    };
    ($unsigned:ty) => {
        impl From<$unsigned> for UFDRNum {
            fn from(unsigned: $unsigned) -> Self {
                Self(unsigned as u128 * u64::MAX as *const () as u128)
            }
        }

        impl From<UFDRNum> for $unsigned {
            fn from(arg: UFDRNum) -> Self {
                (arg.0 / u64::MAX as u128) as $unsigned
            }
        }
    };
}

impl_to_from_unsigned!(u8);
impl_to_from_unsigned!(u16);
impl_to_from_unsigned!(u32);
impl_to_from_unsigned!(u64);
impl_to_from_unsigned!(u128);

impl Add for UFDRNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for UFDRNum {
    fn add_assign(&mut self, rhs: Self) {
        *self = (*self + rhs).into()
    }
}

impl Sub for UFDRNum {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        debug_assert!(self.0 >= rhs.0);
        Self(self.0 - rhs.0)
    }
}

impl Mul for UFDRNum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let one = u64::MAX as u128;
        let l_int = self.0 / one;
        let l_rem = self.0 % one;
        let r_int = rhs.0 / one;
        let r_rem = rhs.0 % one;
        Self(l_int * r_int * one + l_int * r_rem + r_int * l_rem + r_rem * l_rem / one)
    }
}

// impl Mul<u32> for UFDRNum {
//     type Output = Self;
//
//     fn mul(self, rhs: u32) -> Self {
//         Self(self.0.mul(rhs as u128))
//     }
// }

impl<U: UnsignedInteger> Mul<U> for UFDRNum {
    type Output = Self;

    /// Multiply an UFDRnum by an unsigned integer
    ///
    /// Example:
    ///
    /// ```
    /// use colour_math::{UFDRNum};
    ///
    /// assert_eq!(UFDRNum::ONE * 5_u32, UFDRNum::from(5_u32));
    /// ```
    fn mul(self, rhs: U) -> Self::Output {
        // N.B. Do this separately to avoid ambiguity with U::into::<UFDRNum>().
        let rhs: u128 = rhs.into();
        Self(self.0 * rhs)
    }
}

impl Div for UFDRNum {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        match rhs.0 % u64::MAX as u128 {
            // NB: faster AND more accurate
            0 => Self(self.0 / (rhs.0 / u64::MAX as u128)),
            _ => match self.0.cmp(&rhs.0) {
                Ordering::Equal => Self::ONE,
                Ordering::Less | Ordering::Greater => self.mul(Self(u128::MAX / rhs.0)),
            },
        }
    }
}

impl Div<u32> for UFDRNum {
    type Output = Self;

    fn div(self, rhs: u32) -> Self {
        Self(self.0.div(rhs as u128))
    }
}

impl Rem for UFDRNum {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Self(self.0 % rhs.0)
    }
}

impl Rem<u32> for UFDRNum {
    type Output = Self;

    fn rem(self, rhs: u32) -> Self {
        Self(self.0 % rhs as u128)
    }
}
