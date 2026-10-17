// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::ops::{AddAssign, Neg};
use std::{
    cmp::Ordering,
    fmt::{self, Debug, Formatter},
    ops::{Add, Div, Mul, Rem, Sub},
};

use crate::{
    debug::{AbsDiff, ApproxEq, PropDiff},
    impl_to_from_float, FDRNumber, Prop,
};

#[derive(Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct UFDRNumber(pub(crate) u128);

impl UFDRNumber {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(u64::MAX as u128);
    pub const TWO: Self = Self(u64::MAX as u128 * 2);
    pub const THREE: Self = Self(u64::MAX as u128 * 3);

    pub const SQRT_2: Self = Self(FDRNumber::SQRT_2.0 as u128);
    pub const SQRT_3: Self = Self(FDRNumber::SQRT_3.0 as u128);

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

    /// Return the square root of our value.
    ///
    /// Example:
    /// ```
    /// use colour_math::{UFDRNumber, debug::*};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// let four: UFDRNumber = UFDRNumber::ONE * 4;
    /// let two: UFDRNumber = UFDRNumber::ONE * 2;
    /// assert_approx_eq!(four.sqrt(), two);
    /// let twenty_five: UFDRNumber = UFDRNumber::ONE * 25;
    /// let five: UFDRNumber = UFDRNumber::ONE * 5;
    /// assert_approx_eq!(twenty_five.sqrt(), five);
    /// ```
    pub fn sqrt(self) -> Self {
        let half_shift = self.0.leading_zeros() / 2;
        let temp = (self.0 << (half_shift as usize * 2)).isqrt();
        Self(temp << (u64::BITS / 2 - half_shift))
    }
}

impl Debug for UFDRNumber {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let int = self.0 / u64::MAX as u128;
        let frac = self.0 % u64::MAX as u128;
        formatter.write_fmt(format_args!("UFDRNumber({int:X}.{frac:016X})"))
    }
}

impl fmt::UpperHex for UFDRNumber {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.0, formatter)
    }
}

impl AbsDiff for UFDRNumber {}

impl PropDiff for UFDRNumber {
    fn prop_diff(&self, other: &Self) -> Option<Prop> {
        if self.is_proportion() && other.is_proportion() {
            Some(self.abs_diff(other).into())
        } else {
            let denom = self.max(other);
            if *denom == UFDRNumber::ZERO {
                Some(Prop::ZERO)
            } else {
                Some(Prop::from(self.abs_diff(other) / *denom))
            }
        }
    }
}

impl ApproxEq for UFDRNumber {}

impl From<FDRNumber> for UFDRNumber {
    fn from(signed: FDRNumber) -> Self {
        debug_assert!(signed.0 >= 0);
        Self(signed.0 as u128)
    }
}

impl_to_from_float!(f64, u128, UFDRNumber);
impl_to_from_float!(f32, u128, UFDRNumber);

impl From<i32> for UFDRNumber {
    fn from(signed: i32) -> Self {
        debug_assert!(signed > 0);
        Self(signed as u128 * u64::MAX as u128)
    }
}

macro_rules! impl_to_from_unsigned {
    (u128) => {
        impl From<u128> for UFDRNumber {
            fn from(unsigned: u128) -> Self {
                Self(unsigned)
            }
        }

        impl From<UFDRNumber> for u128 {
            fn from(arg: UFDRNumber) -> Self {
                arg.0
            }
        }
    };
    ($unsigned:ty) => {
        impl From<$unsigned> for UFDRNumber {
            fn from(unsigned: $unsigned) -> Self {
                Self(unsigned as u128 * u64::MAX as *const () as u128)
            }
        }

        impl From<UFDRNumber> for $unsigned {
            fn from(arg: UFDRNumber) -> Self {
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

impl Add for UFDRNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for UFDRNumber {
    fn add_assign(&mut self, rhs: Self) {
        *self = (*self + rhs).into()
    }
}

impl Sub for UFDRNumber {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        debug_assert!(self.0 >= rhs.0);
        Self(self.0 - rhs.0)
    }
}

impl Mul for UFDRNumber {
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

impl Mul<i32> for UFDRNumber {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        debug_assert!(rhs >= 0);
        Self(self.0.mul(rhs as u128))
    }
}

impl Div for UFDRNumber {
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

impl Div<i32> for UFDRNumber {
    type Output = Self;

    fn div(self, rhs: i32) -> Self {
        debug_assert!(rhs >= 0);
        Self(self.0.div(rhs as u128))
    }
}

impl Rem for UFDRNumber {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Self(self.0 % rhs.0)
    }
}

impl Rem<i32> for UFDRNumber {
    type Output = Self;

    fn rem(self, rhs: i32) -> Self {
        debug_assert!(rhs >= 0);
        Self(self.0 % rhs as u128)
    }
}
