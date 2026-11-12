// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

//! Generic Fixed Denominator Proportions

use std::cmp::Ordering;

use num_traits::{AsPrimitive, CheckedShl, NumCast, PrimInt, Unsigned};

use gcd::Gcd;

use crate::debug::*;
// use useful_traits::

/// A trait to define what the core of a proportion can be.
pub trait CoreNumber:
    std::fmt::Debug
    + PrimInt
    + Unsigned
    + CheckedShl
    + Gcd
    + ApproxEq
    + AsPrimitive<f32>
    + AsPrimitive<f64>
{
    fn bits() -> u32;
    fn isqrt(self) -> Self;
    fn as_usize(self) -> usize;
    fn as_u128(self) -> u128;
    fn fm_usize(arg: usize) -> Self;
    fn fm_u128(arg: u128) -> Self;
}

macro_rules! implement_proportion_core (
    ($core: ty) => {
        impl CoreNumber for $core {
            fn bits() -> u32 {
                Self::BITS
            }

            fn isqrt(self) -> Self {
                Self::isqrt(self)
            }

            fn as_usize(self) -> usize {
                if usize::BITS >= Self::bits() {
                    self as usize
                } else {
                    <usize as NumCast>::from(self).expect("value out of usize range")
                }
            }

            fn as_u128(self) -> u128 {
                self as u128
            }

            fn fm_usize(arg: usize) -> Self {
                <Self as NumCast>::from(arg).unwrap()
            }

            fn fm_u128(arg: u128) -> Self {
                <Self as NumCast>::from(arg).unwrap()
            }
        }
    }
);

implement_proportion_core!(u8);
implement_proportion_core!(u16);
implement_proportion_core!(u32);
implement_proportion_core!(u64);

/// A generic fixed denominator representation of a proportion ie. 0.0..=1.0.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Proportion<C: CoreNumber>(pub C);

impl<C: CoreNumber> Proportion<C> {
    /// ```
    /// use colour_math::fixed_denominator_rational_numbers::proportion::*;
    ///
    /// assert_eq!(Proportion::from_fraction(1u8, 2), Proportion::<u8>(0x80));
    /// assert_eq!(Proportion::from_fraction(1u8, 1), Proportion::<u8>(0xFF));
    /// assert_eq!(Proportion::from_fraction(0xFE, 0xFF), Proportion::<u8>(0xFE));
    /// assert_eq!(Proportion::from_fraction(0xFF, 0xFF), Proportion::<u8>(0xFF));
    /// ```
    pub fn from_fraction(enumerator: C, denominator: C) -> Self {
        assert!(denominator >= enumerator && !denominator.is_zero());
        if enumerator.is_zero() {
            Self(C::zero())
        } else {
            let gcd = enumerator.gcd(denominator);
            let enumerator = enumerator / gcd;
            let denominator = denominator / gcd;
            if enumerator == denominator {
                Self(C::max_value())
            } else if denominator.is_one() {
                Self(enumerator)
            } else if usize::BITS > C::bits() {
                let shift = enumerator.leading_zeros() + C::bits();
                let temp = (enumerator.as_usize() << shift) / denominator.as_usize();
                use Ordering::*;
                match shift.cmp(&C::bits()) {
                    Less => {
                        let shift_amount = C::bits() - shift;
                        let final_val = temp.checked_shl(shift_amount).expect(
                            "Logic Error: Left-shift overflowed! Fraction exceeds capacity.",
                        );
                        Self(C::fm_usize(final_val))
                    }
                    Equal => Self(C::fm_usize(temp)),
                    Greater => Self(C::fm_usize(temp >> (shift - C::bits()) as usize)),
                }
            } else {
                let shift = enumerator.leading_zeros() + C::bits();
                let temp = (enumerator.as_u128() << shift) / denominator.as_u128();
                use Ordering::*;
                match shift.cmp(&C::bits()) {
                    Less => {
                        let shift_amount = C::bits() - shift;
                        let final_val = temp.checked_shl(shift_amount).expect(
                            "Logic Error: Left-shift overflowed! Fraction exceeds capacity.",
                        );
                        Self(C::fm_u128(final_val))
                    }
                    Equal => Self(C::fm_u128(temp)),
                    Greater => Self(C::fm_u128(temp >> (shift - C::bits()) as usize)),
                }
            }
        }
    }

    /// Return the square root of our value.
    ///
    /// Example:
    /// ```
    /// use colour_math::fixed_denominator_rational_numbers::{proportion::*, debug::ApproxEq, AsProportion};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_approx_eq!(Proportion::<u8>(255), Proportion::<u8>(255));
    /// assert_approx_eq!(Proportion::<u8>(0), Proportion::<u8>(0));
    /// assert_approx_eq!(
    ///     Proportion::<u8>::from_fraction(1, 4).sqrt(),
    ///     Proportion::<u8>::from_fraction(1, 2),
    /// );
    /// assert_approx_eq!(
    ///     Proportion::<u64>::from_fraction(1, 9).sqrt(),
    ///     Proportion::<u64>::from_fraction(1, 3), 2
    /// );
    /// ```
    pub fn sqrt(self) -> Self {
        if self.0.is_zero() || self.0 == C::max_value() {
            self
        } else if usize::BITS > C::bits() {
            let scaled = self.0.as_usize() * C::max_value().as_usize();
            Self(C::fm_usize(scaled.isqrt()))
        } else {
            let scaled = self.0.as_u128() * C::max_value().as_u128();
            Self(C::fm_u128(scaled.isqrt()))
        }
    }

    /// Am I approximately equal to other within `max_bit_diff` bits (default is 1)?
    /// This is needed  because f32, f64 and the fixed denominator rational numbers used here are
    /// only approximate representations of real numbers.
    pub fn approx_eq(&self, other: &Self, max_bit_diff: Option<u32>) -> bool {
        let abs_diff = if self.0 > other.0 {
            self.0 - other.0
        } else {
            other.0 - self.0
        };
        if let Some(max_bit_diff) = max_bit_diff {
            abs_diff <= C::one() << max_bit_diff
        } else {
            abs_diff <= C::one()
        }
    }
}

// Conversion traits
pub trait IntoProportion {
    fn into_proportion<C: CoreNumber>(self) -> Proportion<C>;
}

pub trait ProportionFrom<T, C: CoreNumber> {
    fn proportion_from(arg: T) -> Proportion<C>;
}

impl<T: IntoProportion, C: CoreNumber> ProportionFrom<T, C> for Proportion<C> {
    fn proportion_from(arg: T) -> Proportion<C> {
        arg.into_proportion()
    }
}

pub trait ProportionInto<T> {
    fn proportion_into(self) -> T;
}

pub trait FromProportion<C: CoreNumber> {
    fn from_proportion(prop: Proportion<C>) -> Self;
}

impl<T, C: CoreNumber> FromProportion<C> for T
where
    Proportion<C>: ProportionInto<T>,
{
    fn from_proportion(prop: Proportion<C>) -> Self {
        prop.proportion_into()
    }
}

// Conversions
impl IntoProportion for f64 {
    /// Convert an f64 into a Proportion
    ///
    /// Examples:
    ///
    /// ```
    /// use colour_math::fixed_denominator_rational_numbers::proportion::*;
    /// use approximately_equal::*;
    ///
    /// assert_approx_eq!(0.5_f64.into_proportion::<u8>(), Proportion::from_fraction(1u8, 2));
    /// assert_approx_eq!(Proportion::<u8>::proportion_from(0.5_f64), Proportion::from_fraction(1u8, 2));
    /// ```
    fn into_proportion<C: CoreNumber>(self) -> Proportion<C> {
        debug_assert!((0.0..=1.0).contains(&self), "invalid input: {self}");
        Proportion(C::from(self * AsPrimitive::<f64>::as_(C::max_value())).unwrap())
    }
}

impl<C: CoreNumber> ProportionInto<f64> for Proportion<C> {
    /// Convert this `Proportion` into an f64
    ///
    /// Examples:
    ///
    /// ```
    /// use colour_math::fixed_denominator_rational_numbers::proportion::*;
    /// use approximately_equal::*;
    ///
    /// assert_approx_eq!(0.2_f64, Proportion::from_fraction(1u8,5).proportion_into());
    /// assert_approx_eq!(f64::from_proportion(Proportion::from_fraction(1u8,5)), 0.2);
    /// ```
    fn proportion_into(self) -> f64 {
        <C as AsPrimitive<f64>>::as_(self.0) / <C as AsPrimitive<f64>>::as_(C::max_value())
    }
}

impl IntoProportion for f32 {
    fn into_proportion<C: CoreNumber>(self) -> Proportion<C> {
        debug_assert!((0.0..=1.0).contains(&self), "invalid input: {self}");
        Proportion(C::from(self * AsPrimitive::<f32>::as_(C::max_value())).unwrap())
    }
}

impl<C: CoreNumber> ProportionInto<f32> for Proportion<C> {
    fn proportion_into(self) -> f32 {
        <C as AsPrimitive<f32>>::as_(self.0) / <C as AsPrimitive<f32>>::as_(C::max_value())
    }
}

// OPS
// MUL
impl<C: CoreNumber> std::ops::Mul for Proportion<C> {
    type Output = Self;

    /// Multiply by another Prop.
    ///
    /// Examples:
    ///
    /// ```
    /// use colour_math::fixed_denominator_rational_numbers::proportion::*;
    ///
    /// assert_eq!(
    ///     Proportion::from_fraction(1u8, 2) * Proportion::from_fraction(1, 2),
    ///     Proportion::from_fraction(1, 4)
    /// );
    /// assert_eq!(
    ///     Proportion::from_fraction(1u64, 2) * Proportion::from_fraction(1, 3),
    ///     Proportion::from_fraction(1, 6)
    /// );
    /// ```
    fn mul(self, rhs: Self) -> Self {
        if usize::BITS > C::bits() {
            Self(C::fm_usize(
                self.0.as_usize() * rhs.0.as_usize() / C::max_value().as_usize(),
            ))
        } else {
            Self(C::fm_u128(
                (self.0.as_u128() * rhs.0.as_u128()) / C::max_value().as_u128(),
            ))
        }
    }
}

impl<C: CoreNumber> std::ops::MulAssign for Proportion<C> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
