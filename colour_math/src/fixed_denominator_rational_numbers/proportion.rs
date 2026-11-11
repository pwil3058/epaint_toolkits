// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

//! Generic Fixed Denominator Proportions

use std::{cmp::Ordering, ops::Sub};

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
    ///     Proportion::<u8>::from_fraction(1, 2), (u64::MAX / 255) // Problem with proportional diffs
    /// );
    /// assert_approx_eq!(
    ///     Proportion::<u64>::from_fraction(1, 9).sqrt(),
    ///     Proportion::<u64>::from_fraction(1, 3), 2
    /// );
    /// ```
    pub fn sqrt(self) -> Self {
        if self.0.is_zero() || self.0 == C::max_value() {
            self
        } else {
            let scaled = self.0.into() * C::max_value().into();
            Self(C::from(scaled.isqrt()).unwrap())
        }
    }

    pub fn approx_eq(&self, other: &Self, max_diff: Option<u64>) -> bool {
        self.0.approx_eq(&other.0, max_diff)
    }
}

// Operations
// Sub
impl<C: ProportionCore> Sub for Proportion<C> {
    type Output = Self;

    /// Subtract a Prop yielding a Prop.
    ///
    /// Examples:
    ///
    /// ```
    /// use colour_math::fixed_denominator_rational_numbers::proportion::*;
    ///
    /// assert_eq!(
    ///     Proportion::from_fraction(1u16, 2) - Proportion::from_fraction(1, 4),
    ///     Proportion::from_fraction(1, 4)
    /// );
    /// assert_eq!(
    ///     Proportion::from_fraction(3u32, 5) - Proportion::from_fraction(6, 10),
    ///     Proportion(0)
    /// );
    /// ```
    fn sub(self, rhs: Self) -> Self {
        assert!(self.0 >= rhs.0);
        Self(self.0 - rhs.0)
    }
}
