// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::{cmp::Ordering, ops::Index};

use crate::fixed_denominator_rational_numbers::*;
use crate::{Chroma, IntoProp, IntoProps, LightLevel, Prop, RGB, UFDRNum};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct OrderedTriplet([Prop; 3]);

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
        // } else if self[1] == self[2] {
        //    // Red, Blue or Green hue
        // Prop::ZERO
        // } else if self[0] == self[1] {
        //    // Cyan, Magent or Yellow hue
        // Prop::ONE
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

    fn chroma_divisor(sc: Prop) -> Prop {
        (UFDRNum::ONE - sc + sc * sc).sqrt().into_prop()
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
        let sc = self.sector_constant()?;
        let divisor = Self::chroma_divisor(sc);
        let b = self[1] - self[2];
        let a = self[0];
        let dividend = (a * a + b * b - a * b).sqrt();
        let value = (dividend / divisor).into_prop();
        // println!("value: {value:?} vs {:?}", self[0] - self[2]));
        match self.sum().cmp(&(UFDRNum::ONE + sc)) {
            Ordering::Less => Some(Chroma::Shade(value)),
            Ordering::Equal => Some(Chroma::Neither(value)),
            Ordering::Greater => Some(Chroma::Tint(value)),
        }
    }
}
