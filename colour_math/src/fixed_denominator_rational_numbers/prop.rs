// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::{
    cmp::Ordering,
    fmt::{self, Debug, Formatter},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use gcd::Gcd;
use serde::{Deserialize, Serialize};

use crate::{
    fixed_denominator_rational_numbers::{
        debug::{AbsDiff, ApproxEq, PropDiff},
        AsProportion, FDRNum, FromAsProp, FromU128Checked, IntoAsProp, NumberConstants, UFDRNum,
        Unsigned,
    },
    impl_into_as_prop,
};

#[derive(Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Prop(pub(crate) u64);

impl AsProportion for Prop {
    const ZERO: Self = Self(0);
    const ALMOST_ZERO: Self = Self(1);
    const ONE: Self = Self(u64::MAX);
    const ALMOST_ONE: Self = Self(u64::MAX - 1);
    const ONE_QUARTER: Self = Self(u64::ONE_QUARTER);
    const HALF: Self = Self(u64::HALF);
    const THREE_QUARTERS: Self = Self(u64::THREE_QUARTERS);
}

impl Prop {
    /// Create from a fraction
    ///
    /// Examples:
    ///
    /// ```
    /// use colour_math::fixed_denominator_rational_numbers::{self::*, debug::ApproxEq};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_eq!(Prop::from_fraction(1, 2), 0.5.into_prop());
    /// assert_approx_eq!(Prop::from_fraction(3, 5), 0.6.into_prop(), 1000);
    /// ```
    pub fn from_fraction(enumerator: u64, denominator: u64) -> Self {
        debug_assert!(denominator >= enumerator);
        let ufdrn = UFDRNum::from_rational_num(enumerator as u128, denominator as u128);
        Self(ufdrn.0 as u64)
    }

    pub fn weighted_average(props: &[(Self, u32)]) -> Self {
        let mut total = UFDRNum::default();
        let mut count = 0u32;

        for (prop, parts) in props {
            total += *prop * *parts;
            count += *parts;
        }
        (total / count).into()
    }

    pub fn comp(self) -> Self {
        Self(u64::MAX - self.0)
    }

    pub fn linear_interpolation(self, other: Self, p: Self) -> Self {
        Prop(self.mul(p.comp()).0 + other.mul(p).0)
    }

    /// Return the square root of our value.
    ///
    /// Example:
    /// ```
    /// use colour_math::fixed_denominator_rational_numbers::{Prop, debug::ApproxEq, AsProportion};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// let one_quarter: Prop = Prop::ONE / 4;
    /// assert_approx_eq!(one_quarter.sqrt(), Prop::ONE / 2);
    /// ```
    pub fn sqrt(self) -> Self {
        match self.0 {
            0 => Self(0),
            u64::MAX => Self(u64::MAX),
            _ => {
                let scaled = self.0 as u128 * u64::MAX as u128;
                Self(scaled.isqrt() as u64)
            }
        }
    }

    /// Return the complement of this proportion
    ///
    /// Examples:
    ///
    /// ```
    /// use colour_math::fixed_denominator_rational_numbers::{AsProportion, Prop, debug::ApproxEq};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_eq!(Prop::ONE_QUARTER.complement(), Prop::THREE_QUARTERS)
    /// ```
    pub fn complement(self) -> Self {
        Self::ONE - self
    }
}

// Debug stuff
impl AbsDiff for Prop {}

impl PropDiff for Prop {
    fn prop_diff(&self, other: &Self) -> Option<Prop> {
        Some(self.abs_diff(other))
    }
}

impl ApproxEq for Prop {}

impl Debug for Prop {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "Prop(0.{:016x})", self.0)
    }
}

impl fmt::UpperHex for Prop {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.0, formatter)
    }
}

impl fmt::LowerHex for Prop {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, formatter)
    }
}

impl fmt::Display for Prop {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "0.{:016x}", self.0)
    }
}

// Conversion traits
pub trait IntoProp: Sized + Copy {
    fn into_prop(self) -> Prop;
}

pub trait PropFrom<T: IntoProp + Sized + Copy> {
    fn prop_from(arg: T) -> Prop;
}
impl<T: IntoProp> PropFrom<T> for Prop {
    fn prop_from(arg: T) -> Prop {
        arg.into_prop()
    }
}

pub trait FromProp: Sized + Copy {
    fn from_prop(prop: Prop) -> Self;
}

pub trait PropInto<T: FromProp + Sized + Copy>: Sized + Copy + IntoProp {
    fn prop_into(self) -> T;
}
impl<T: FromProp> PropInto<T> for Prop {
    fn prop_into(self) -> T {
        T::from_prop(self)
    }
}

// Conversions
impl IntoProp for Prop {
    fn into_prop(self) -> Prop {
        self
    }
}

impl FromProp for Prop {
    fn from_prop(prop: Prop) -> Prop {
        prop
    }
}

impl IntoProp for [u64; 2] {
    fn into_prop(self) -> Prop {
        Prop::from_fraction(self[0], self[1])
    }
}

impl FromProp for [u64; 2] {
    fn from_prop(prop: Prop) -> Self {
        let gcd = u64::MAX.gcd(prop.0);
        [prop.0 / gcd, u64::MAX / gcd]
    }
}

impl From<[u64; 2]> for Prop {
    fn from(fraction: [u64; 2]) -> Self {
        Self::from_fraction(fraction[0], fraction[1])
    }
}

impl FromAsProp<Prop> for Prop {
    fn from_as_prop(prop: Prop) -> Self {
        prop
    }
}
impl_into_as_prop!(Prop, Prop);

impl IntoProp for f64 {
    fn into_prop(self) -> Prop {
        debug_assert!((0.0..=1.0).contains(&self));
        //NB: watch out for floating point not being proper reals
        Prop((self * u64::MAX as f64) as u64)
    }
}

impl FromProp for f64 {
    fn from_prop(prop: Prop) -> f64 {
        prop.0 as f64 / u64::MAX as f64
    }
}

#[macro_export]
macro_rules! impl_to_from_number {
    ($number:ty, $core:ty, $proportion:ty) => {
        impl From<$number> for $proportion {
            #[allow(unused_comparisons)]
            fn from(arg: $number) -> Self {
                debug_assert!(arg.0 >= 0 && arg.0 <= u64::MAX as $core);
                Self(arg.0 as u64)
            }
        }

        impl From<$proportion> for $number {
            fn from(arg: $proportion) -> Self {
                Self(arg.0 as $core)
            }
        }
    };
}

impl_to_from_number!(UFDRNum, u128, Prop);
impl_to_from_number!(FDRNum, i128, Prop);

impl IntoProp for UFDRNum {
    fn into_prop(self) -> Prop {
        Prop::from(self)
    }
}

impl FromProp for UFDRNum {
    fn from_prop(prop: Prop) -> Self {
        Self::from(prop)
    }
}

impl IntoProp for FDRNum {
    fn into_prop(self) -> Prop {
        self.into()
    }
}

impl FromProp for FDRNum {
    fn from_prop(prop: Prop) -> Self {
        Self::from(prop)
    }
}

impl<U: AsProportion + Into<u128> + Unsigned> IntoProp for U {
    /// Convert an unsigned type representing a proportion into a Prop (rounding to nearest)
    ///
    /// Examples:
    ///
    /// ```
    /// use colour_math::fixed_denominator_rational_numbers::*;
    ///
    /// assert_eq!(u8::ONE.into_prop(), Prop::ONE);
    /// assert_eq!(u8::ZERO.into_prop(), Prop::ZERO);
    /// assert_eq!(127_u8.into_prop(), Prop::from_fraction(127, 255));
    /// assert_eq!(Prop::prop_from(u32::ONE), Prop::ONE);
    /// assert_eq!(Prop::prop_from(u8::MAX - 1), Prop::from_fraction(254, 255));
    /// ```
    fn into_prop(self) -> Prop {
        let dividend: u128 = Into::<u128>::into(self) * u64::MAX as u128;
        let divisor: u128 = Into::<u128>::into(U::ONE);
        let val = (dividend + divisor / 2) / divisor;
        Prop(u64::from_u128_checked(val))
    }
}

impl<U: AsProportion + FromU128Checked + Unsigned + Into<u128>> FromProp for U {
    /// Converts a Prop to a U as a proportion of U::MAX (rounding to nearest)
    ///
    /// Examples:
    ///
    /// ```
    /// use colour_math::fixed_denominator_rational_numbers::*;
    ///
    /// assert_eq!(u8::from_prop(Prop::prop_from(0.5)), 128);
    /// assert_eq!(u8::from_prop(Prop::prop_from(0.999)), u8::MAX);
    /// assert_eq!(u8::from_prop(Prop::prop_from(0.998)), u8::MAX - 1);
    /// assert_eq!(u8::MAX - 1, Prop::prop_from(0.998).prop_into());
    /// assert_eq!(23_u64, Prop::prop_from(23_u64).prop_into());
    /// ```
    fn from_prop(prop: Prop) -> U {
        let dividend = prop.0 as u128 * Into::<u128>::into(U::ONE);
        let divisor = u64::MAX as u128;
        let val = (dividend + divisor / 2) / divisor;
        U::from_u128_checked(val)
    }
}

// Ops
impl Mul for Prop {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(((self.0 as u128 * rhs.0 as u128) / u64::MAX as u128) as u64)
    }
}

impl Mul<u8> for Prop {
    type Output = UFDRNum;

    fn mul(self, rhs: u8) -> UFDRNum {
        UFDRNum(self.0 as u128 * rhs as u128)
    }
}

impl Mul<u32> for Prop {
    type Output = UFDRNum;

    fn mul(self, rhs: u32) -> UFDRNum {
        UFDRNum(self.0 as u128 * rhs as u128)
    }
}

impl Mul<i32> for Prop {
    type Output = UFDRNum;

    fn mul(self, rhs: i32) -> UFDRNum {
        UFDRNum(self.0 as u128 * TryInto::<u32>::try_into(rhs).unwrap() as u128)
    }
}

impl Div for Prop {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        if rhs == Self::ONE {
            self
        } else {
            Self((((u128::MAX / rhs.0 as u128) * self.0 as u128) / u64::MAX as u128) as u64)
        }
    }
}

impl Div<u8> for Prop {
    type Output = Self;

    fn div(self, rhs: u8) -> Self {
        Prop(self.0 / rhs as u64)
    }
}

impl Div<u32> for Prop {
    type Output = Self;

    fn div(self, rhs: u32) -> Self {
        Prop(self.0 / rhs as u64)
    }
}

impl Div<i32> for Prop {
    type Output = Self;

    fn div(self, rhs: i32) -> Self {
        Prop(self.0 / TryInto::<u32>::try_into(rhs).unwrap() as u64)
    }
}

impl Add for Prop {
    type Output = UFDRNum;

    fn add(self, rhs: Self) -> UFDRNum {
        UFDRNum(self.0 as u128 + rhs.0 as u128)
    }
}

impl Sub for Prop {
    type Output = Self;

    /// Subtract a Prop yielding a Prop.
    ///
    /// Examples:
    ///
    /// ```
    /// use colour_math::fixed_denominator_rational_numbers::*;
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_eq!(Prop::from_fraction(1, 2) - Prop::from_fraction(1, 4), Prop::from_fraction(1, 4));
    /// assert_eq!(Prop::from_fraction(3, 5) - Prop::from_fraction(6, 10), Prop::ZERO);
    /// ```
    fn sub(self, rhs: Self) -> Self {
        assert!(self.0 >= rhs.0);
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Prop {
    fn sub_assign(&mut self, rhs: Self) {
        debug_assert!(self.0 >= rhs.0);
        self.0 -= rhs.0;
    }
}

// Ops with UFDRNum
impl Add<Prop> for UFDRNum {
    type Output = Self;

    fn add(self, rhs: Prop) -> UFDRNum {
        UFDRNum(self.0 + rhs.0 as u128)
    }
}

impl AddAssign<Prop> for UFDRNum {
    fn add_assign(&mut self, rhs: Prop) {
        self.0 += rhs.0 as u128;
    }
}

impl Sub<Prop> for UFDRNum {
    type Output = Self;

    fn sub(self, rhs: Prop) -> Self {
        debug_assert!(self.0 >= rhs.0 as u128);
        Self(self.0 - rhs.0 as u128)
    }
}

impl SubAssign<Prop> for UFDRNum {
    fn sub_assign(&mut self, rhs: Prop) {
        debug_assert!(self.0 >= rhs.0 as u128);
        self.0 -= rhs.0 as u128;
    }
}

impl Div<Prop> for UFDRNum {
    type Output = Self;

    fn div(self, rhs: Prop) -> Self {
        assert_ne!(rhs.0, 0);
        if rhs == Prop::ONE {
            self
        } else {
            match self.0.cmp(&(rhs.0 as u128)) {
                Ordering::Equal => Self::ONE,
                Ordering::Less | Ordering::Greater => self.mul(Self(u128::MAX / rhs.0 as u128)),
            }
        }
    }
}

impl DivAssign<Prop> for UFDRNum {
    fn div_assign(&mut self, rhs: Prop) {
        *self = self.div(rhs);
    }
}

impl Mul<Prop> for UFDRNum {
    type Output = Self;

    /// Multiply by a Prop
    ///
    /// Examples:
    ///
    /// ```
    /// use colour_math::fixed_denominator_rational_numbers::*;
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_eq!(UFDRNum::from(2.5) * Prop::ONE, UFDRNum::from(2.5));
    /// assert_eq!(UFDRNum::ONE * Prop::ZERO, UFDRNum::ZERO);
    /// assert_eq!(UFDRNum::ONE * 0.5.into_prop(), UFDRNum::from(0.5));
    /// ```
    fn mul(self, rhs: Prop) -> Self {
        match rhs {
            Prop::ZERO => Self::ZERO,
            Prop::ONE => self,
            _ => self.mul(Self(rhs.0 as u128)),
        }
    }
}

impl MulAssign<Prop> for UFDRNum {
    fn mul_assign(&mut self, rhs: Prop) {
        *self = self.mul(rhs);
    }
}
