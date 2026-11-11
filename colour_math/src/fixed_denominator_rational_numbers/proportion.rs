// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

//! Generic Fixed Denominator Proportions

use std::{
    cmp::Ordering,
    ops::{Div, Shl, Shr},
};

use num_traits::{CheckedShl, NumCast, PrimInt, Unsigned};

use gcd::Gcd;

/// A trait to define what the core of a proportion can be.
pub trait ProportionCore: std::fmt::Debug + PrimInt + Unsigned + CheckedShl + Gcd {
    fn bits() -> u32;
}

impl ProportionCore for u8 {
    fn bits() -> u32 {
        Self::BITS
    }
}

/// A generic fixed denominator representation of a proportion ie. 0.0..=1.0.
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Proportion<C: ProportionCore>(pub C);

impl<C: ProportionCore> Proportion<C> {
    /// ```
    /// use colour_math::fixed_denominator_rational_numbers::proportion::*;
    ///
    /// assert_eq!(Proportion::new(1u8, 2), Proportion::<u8>(0x80));
    /// assert_eq!(Proportion::new(1u8, 1), Proportion::<u8>(0xFF));
    /// assert_eq!(Proportion::new(0xFE, 0xFF), Proportion::<u8>(0xFE));
    /// assert_eq!(Proportion::new(0xFF, 0xFF), Proportion::<u8>(0xFF));
    /// ```
    pub fn new(enumerator: C, denominator: C) -> Self {
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
            } else {
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
            }
        }
    }
}
