// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::{cmp::Ordering, ops::Index};

use crate::fixed_denominator_rational_numbers::{self::*, debug::ApproxEq};
use crate::{Chroma, IntoProp, IntoProps, LightLevel, Prop, RGB, UFDRNum};
use approximately_equal::debug_assert_approx_eq;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct OrderedTriplet([Prop; 3]);

impl PartialEq<[Prop; 3]> for OrderedTriplet {
    fn eq(&self, other: &[Prop; 3]) -> bool {
        self.0 == *other
    }
}

impl PartialEq<OrderedTriplet> for [Prop; 3] {
    fn eq(&self, other: &OrderedTriplet) -> bool {
        other == self
    }
}

impl Index<usize> for OrderedTriplet {
    type Output = Prop;

    fn index(&self, idx: usize) -> &Prop {
        &self.0[idx]
    }
}

impl<L: LightLevel> From<RGB<L>> for OrderedTriplet {
    fn from(rgb: RGB<L>) -> Self {
        let mut ot = rgb.into_proportions();
        ot.sort_by(|l, r| r.cmp(l));
        Self(ot)
    }
}

impl From<OrderedTriplet> for [Prop; 3] {
    fn from(ot: OrderedTriplet) -> Self {
        ot.0
    }
}

impl OrderedTriplet {
    /// Is this a valid OrderedTriplet?
    ///
    /// Example:
    ///
    /// ```
    /// use colour_math::{Prop, ordered_triplet::OrderedTriplet, RGB, HueConstants};
    ///
    /// assert!(OrderedTriplet::from(RGB::<u8>::YELLOW).is_valid())
    /// ```
    pub fn is_valid(&self) -> bool {
        self.0.is_sorted_by(|l, r| l >= r)
    }

    /// Is this a zero chroma triplet.
    ///
    /// ```
    /// use colour_math::{self::*, ordered_triplet::OrderedTriplet};
    ///
    /// assert!(OrderedTriplet::from(RGB::<u8>::WHITE).is_grey());
    /// assert!(!OrderedTriplet::from(RGB::<u8>::CYAN).is_grey());
    /// ```
    #[inline]
    pub fn is_grey(&self) -> bool {
        self[0] == self[2]
    }

    /// The sum of our elements.
    pub fn sum(self) -> UFDRNum {
        self.0.into_iter().fold(UFDRNum::ZERO, |sum, i| sum + i)
    }

    /// If we're not grey return the sector constant for our values?
    ///
    /// Examples:
    ///
    /// ```
    /// use colour_math::{self::*, debug::ApproxEq, ordered_triplet::OrderedTriplet, fixed_denominator_rational_numbers::*};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_eq!(OrderedTriplet::from(RGB::<u8>::RED).sector_constant(), Some(Prop::ZERO));
    /// assert_approx_eq!(OrderedTriplet::from(RGB::<Prop>::rgb_from([0.7, 0.35, 0.35])).sector_constant().unwrap(), Prop::ZERO);
    /// assert_eq!(OrderedTriplet::from(RGB::<u8>::YELLOW).sector_constant(), Some(Prop::ONE));
    /// assert_eq!(OrderedTriplet::from(RGB::<Prop>::rgb_from([1.0, 1.0, 0.2])).sector_constant(), Some(Prop::ONE));
    /// assert_eq!(OrderedTriplet::from(RGB::<Prop>::rgb_from([0.5, 0.5, 0.4])).sector_constant(), Some(Prop::ONE));
    /// assert_eq!(OrderedTriplet::from(RGB::<Prop>::RED_YELLOW).sector_constant(), Some(Prop::HALF));
    /// assert_approx_eq!(OrderedTriplet::from(RGB::<Prop>::rgb_from([0.7, 0.35, 0.0])).sector_constant().unwrap(), Prop::HALF);
    /// assert_approx_eq!(OrderedTriplet::from(RGB::<Prop>::rgb_from([0.7, 0.35, 0.2])).sector_constant().unwrap(), Prop::prop_from(0.3), 1000);
    /// assert_approx_eq!(OrderedTriplet::from(RGB::<Prop>::rgb_from([0.6, 0.2, 0.0])).sector_constant().unwrap(), Prop::from_fraction(1, 3), 1000);
    /// ```
    pub fn sector_constant(self) -> Option<Prop> {
        debug_assert!(self.is_valid(), "{self:?}");
        if self.is_grey() {
            None
        } else if self[1] == self[2] {
            // Red, Blue or Green hue
            Some(Prop::ZERO)
        } else if self[0] == self[1] {
            // Cyan, Magent or Yellow hue
            Some(Prop::ONE)
        } else {
            // one of the myriad of other colours
            let divisor = self[0] - self[2];
            let dividend = self[1] - self[2];
            let value = dividend / divisor;
            debug_assert_eq!(
                (self.sum() - (UFDRNum::ONE + value) * divisor) / 3u8,
                UFDRNum::from_prop(self[2]),
                "{self:?}, {value:?}"
            );
            Some(value.into_prop())
        }
    }

    /// Return the our chroma proportion.
    ///
    /// Examples:
    ///
    /// ```
    /// use colour_math::{self::*, debug::ApproxEq, ordered_triplet::OrderedTriplet, fixed_denominator_rational_numbers::*};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_eq!(OrderedTriplet::from(RGB::<Prop>::WHITE).chroma(), None);
    /// assert_eq!(OrderedTriplet::from(RGB::<Prop>::RED).chroma().unwrap(), Chroma::Neither(Prop::ONE));
    /// assert_eq!(OrderedTriplet::from(RGB::<Prop>::CYAN).chroma().unwrap(), Chroma::Neither(Prop::ONE));
    /// assert_eq!(OrderedTriplet::from(RGB::<Prop>::BLUE_CYAN).chroma().unwrap(), Chroma::Neither(Prop::ONE));
    /// assert_approx_eq!(OrderedTriplet::from(RGB::<Prop>::RED/2).chroma().unwrap(), Chroma::Shade(Prop::HALF));
    /// assert_approx_eq!(OrderedTriplet::from(RGB::<Prop>::CYAN/2).chroma().unwrap(), Chroma::Shade(Prop::HALF));
    /// assert!(OrderedTriplet::from(RGB::<Prop>::rgb_from([0.8, 0.6, 0.4])).chroma().is_some());
    ///```
    pub fn chroma(self) -> Option<Chroma> {
        if self.is_grey() {
            None
        } else {
            let c_prop = self[0] - self[2];
            match self
                .sum()
                .cmp(&(UFDRNum::ONE + ((self[1] - self[2]) / c_prop)))
            {
                Ordering::Less => Some(Chroma::Shade(c_prop)),
                Ordering::Equal => Some(Chroma::Neither(c_prop)),
                Ordering::Greater => Some(Chroma::Tint(c_prop)),
            }
        }
    }
}

impl TryFrom<(UFDRNum, Prop, Prop)> for OrderedTriplet {
    type Error = &'static str;

    /// Try to construct an `OrderedTriplet` from a sum, chroma proportion and a sextant parameter.
    ///
    /// Examples:
    ///
    /// ```
    /// use colour_math::{self::*, debug::ApproxEq, ordered_triplet::OrderedTriplet, fixed_denominator_rational_numbers::*};
    /// use approximately_equal::assert_approx_eq;
    ///
    /// assert_eq!(
    ///     OrderedTriplet::try_from((UFDRNum::ONE, Prop::ONE, Prop::ZERO)).unwrap(),
    ///     [Prop::ONE, Prop::ZERO, Prop::ZERO]
    /// );
    /// assert_eq!(
    ///     OrderedTriplet::try_from((UFDRNum::TWO, Prop::ONE, Prop::ONE)).unwrap(),
    ///     [Prop::ONE, Prop::ONE, Prop::ZERO]
    /// );
    /// assert!(
    ///     OrderedTriplet::try_from((UFDRNum::TWO, Prop::HALF, Prop::ONE)).is_ok()
    /// );
    /// ```
    fn try_from(
        (sum, chroma_prop, sextant_parameter): (UFDRNum, Prop, Prop),
    ) -> Result<OrderedTriplet, Self::Error> {
        if chroma_prop == Prop::ZERO {
            if chroma_prop % 3 == Prop::ZERO {
                let value = chroma_prop / 3;
                Ok(Self([value, value, value]))
            } else {
                Err("Sum for zero chroma is not a multiple of 3.")
            }
        } else {
            let min_sum = (UFDRNum::ONE + sextant_parameter) * chroma_prop;
            match sum.cmp(&min_sum) {
                Ordering::Less => Err("Sum is too small for chroma and sector parameter."),
                Ordering::Equal => Ok(Self([
                    chroma_prop,
                    sextant_parameter * chroma_prop,
                    Prop::ZERO,
                ])),
                Ordering::Greater => {
                    let third = (sum - min_sum) / 3u8;
                    let first = third + chroma_prop;
                    if first.is_proportion() {
                        let second = sum - first - third;
                        debug_assert_eq!(first + second + third, sum);
                        debug_assert_eq!(first - third, UFDRNum::from_prop(chroma_prop));
                        Ok(Self([
                            first.into_prop(),
                            second.into_prop(),
                            third.into_prop(),
                        ]))
                    } else {
                        Err("Sum is too large for chroma and sector parameters")
                    }
                }
            }
        }
    }
}
