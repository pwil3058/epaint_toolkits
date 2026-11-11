// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

//! Generic Fixed Denominator Proportions

use std::cmp::Ordering;

use num_traits::{CheckedShl, NumCast, PrimInt, Unsigned};

use gcd::Gcd;

use crate::debug::*;
// use useful_traits::

/// A trait to define what the core of a proportion can be.
pub trait ProportionCore:
    std::fmt::Debug + PrimInt + Unsigned + CheckedShl + Gcd + Into<u128> + ApproxEq
{
    fn bits() -> u32;
    fn isqrt(self) -> Self;
}

macro_rules! implement_proportion_core (
    ($core: ty) => {
        impl ProportionCore for $core {
            fn bits() -> u32 {
                Self::BITS
            }

           fn isqrt(self) -> Self {
                Self::isqrt(self)
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
pub struct Proportion<C: ProportionCore>(pub C);

impl<C: ProportionCore> Proportion<C> {
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
                let temp = (<usize as NumCast>::from(enumerator).unwrap() << shift)
                    / <usize as NumCast>::from(denominator).unwrap();
                use Ordering::*;
                match shift.cmp(&C::bits()) {
                    Less => {
                        let shift_amount = C::bits() - shift;
                        let final_val = temp.checked_shl(shift_amount).expect(
                            "Logic Error: Left-shift overflowed! Fraction exceeds capacity.",
                        );
                        Self(C::from(final_val).unwrap())
                    }
                    Equal => Self(C::from(temp).unwrap()),
                    Greater => Self(C::from(temp >> (shift - C::bits()) as usize).unwrap()),
                }
            } else {
                let shift = enumerator.leading_zeros() + C::bits();
                let temp = (enumerator.into() << shift) / denominator.into();
                use Ordering::*;
                match shift.cmp(&C::bits()) {
                    Less => {
                        let shift_amount = C::bits() - shift;
                        let final_val = temp.checked_shl(shift_amount).expect(
                            "Logic Error: Left-shift overflowed! Fraction exceeds capacity.",
                        );
                        Self(C::from(final_val).unwrap())
                    }
                    Equal => Self(C::from(temp).unwrap()),
                    Greater => Self(C::from(temp >> (shift - C::bits()) as usize).unwrap()),
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
            let scaled = <usize as NumCast>::from(self.0).unwrap()
                * <usize as NumCast>::from(C::max_value()).unwrap();
            Self(C::from(scaled.isqrt()).unwrap())
        } else {
            let scaled = self.0.into() * C::max_value().into();
            Self(C::from(scaled.isqrt()).unwrap())
        }
    }

    /// Am I approximately equal to other within `max_bit_diff` bits (default is 1)?
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
    fn into_proportion<C: ProportionCore>(self) -> Proportion<C>;
}

pub trait ProportionFrom<T, C: ProportionCore> {
    fn proportion_from(arg: T) -> Proportion<C>;
}

impl<T: IntoProportion, C: ProportionCore> ProportionFrom<T, C> for Proportion<C> {
    fn proportion_from(arg: T) -> Proportion<C> {
        arg.into_proportion()
    }
}

pub trait ProportionInto {
    fn proportion_into<T>(self) -> T;
}

pub trait FromProportion<C: ProportionCore> {
    fn from_proportion(prop: Proportion<C>) -> Self;
}

impl<T, C: ProportionCore> FromProportion<C> for T
where
    Proportion<C>: ProportionInto,
{
    fn from_proportion(prop: Proportion<C>) -> Self {
        prop.proportion_into::<T>()
    }
}

// OPS
// MUL
impl<C: ProportionCore> std::ops::Mul for Proportion<C> {
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
            Self(
                C::from(
                    <usize as NumCast>::from(self.0).unwrap()
                        * <usize as NumCast>::from(rhs.0).unwrap()
                        / <usize as NumCast>::from(C::max_value()).unwrap(),
                )
                .unwrap(),
            )
        } else {
            Self(C::from((self.0.into() * rhs.0.into()) / C::max_value().into()).unwrap())
        }
    }
}

impl<C: ProportionCore> std::ops::MulAssign for Proportion<C> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
