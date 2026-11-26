// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::{cmp::Ordering, fmt::Debug};

use crate::fixed_denominator_rational_numbers::{self::*, debug::*};
use crate::{Angle, Chroma, HueConstants, LightLevel, RGB, hue::HueBasics, ordered_triplet::*};

#[derive(Debug, PartialEq, Eq)]
pub enum HueResult<T: Sized + Clone + Copy + PartialEq + Eq + Debug> {
    Hued(T),
    Grey(T),
}

impl<T: Sized + Clone + Copy + PartialEq + Eq + Debug + ApproxEq> ApproxEq for HueResult<T> {
    fn approx_eq(&self, other: &Self, limit: Option<u64>) -> bool {
        match self {
            Self::Hued(t) => match other {
                Self::Hued(o) => t.approx_eq(o, limit),
                _ => false,
            },
            Self::Grey(t) => match other {
                Self::Grey(o) => t.approx_eq(o, limit),
                _ => false,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// NB. These are in ascending order in agreement with Hue angle. So don't mess with them.
pub enum Sextant {
    BlueGreen,
    BlueRed,
    RedBlue,
    RedGreen,
    GreenRed,
    GreenBlue,
}

impl<L: LightLevel> TryFrom<RGB<L>> for Sextant {
    type Error = &'static str;

    fn try_from(rgb: RGB<L>) -> Result<Self, Self::Error> {
        println!("RGB: {rgb:?}");
        if rgb.red() > rgb.green() {
            // Red > Green ?Blue
            if rgb.green() > rgb.blue() {
                // Red > Green > Blue
                Ok(Self::RedGreen)
            } else if rgb.red() > rgb.blue() {
                // Red > Blue >= Green
                Ok(Self::RedBlue)
            } else {
                // Blue >= Red > Green
                Ok(Self::BlueRed)
            }
        } else if rgb.green() > rgb.blue() {
            // Green >= Red && Green > Blue
            if rgb.blue() > rgb.red() {
                // Green > Blue > Red
                Ok(Self::GreenBlue)
            } else {
                // Green >= Red >= Blue
                Ok(Self::GreenRed)
            }
        } else if rgb.blue() > rgb.green() {
            // Blue > Green >= Red
            Ok(Self::BlueGreen)
        } else {
            Err("grey")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hue {
    pub sextant: Sextant,
    pub param: Prop,
}

impl HueConstants for Hue {
    const RED: Self = Self {
        sextant: Sextant::RedBlue,
        param: Prop::ZERO,
    };
    const GREEN: Self = Self {
        sextant: Sextant::GreenRed,
        param: Prop::ZERO,
    };
    const BLUE: Self = Self {
        sextant: Sextant::BlueGreen,
        param: Prop::ZERO,
    };

    const CYAN: Self = Self {
        sextant: Sextant::GreenBlue,
        param: Prop::ZERO,
    };
    const MAGENTA: Self = Self {
        sextant: Sextant::BlueRed,
        param: Prop::ZERO,
    };
    const YELLOW: Self = Self {
        sextant: Sextant::RedGreen,
        param: Prop::ZERO,
    };

    const BLUE_CYAN: Self = Self {
        sextant: Sextant::BlueGreen,
        param: Prop::HALF,
    };
    const BLUE_MAGENTA: Self = Self {
        sextant: Sextant::RedBlue,
        param: Prop::HALF,
    };
    const RED_MAGENTA: Self = Self {
        sextant: Sextant::BlueRed,
        param: Prop::HALF,
    };
    const RED_YELLOW: Self = Self {
        sextant: Sextant::RedGreen,
        param: Prop::HALF,
    };
    const GREEN_YELLOW: Self = Self {
        sextant: Sextant::GreenRed,
        param: Prop::HALF,
    };
    const GREEN_CYAN: Self = Self {
        sextant: Sextant::GreenBlue,
        param: Prop::HALF,
    };
}

impl Ord for Hue {
    fn cmp(&self, other: &Hue) -> Ordering {
        use Sextant::*;
        match self.sextant.cmp(&other.sextant) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => match self.sextant {
                GreenBlue | RedGreen | BlueRed => self.param.cmp(&other.param),
                GreenRed | RedBlue | BlueGreen => other.param.cmp(&self.param),
            },
        }
    }
}

impl PartialOrd for Hue {
    fn partial_cmp(&self, other: &Hue) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<L: LightLevel> TryFrom<RGB<L>> for Hue {
    type Error = &'static str;

    /// Try to convert a `Hue` from an `RGB`. Fails if RGB is a grey.
    ///
    /// Examples:
    ///
    /// ```
    /// use colour_math::{new_hue::*, hue::angle::*, HueConstants, RGB};
    /// use colour_math::fixed_denominator_rational_numbers::*;
    ///
    /// assert_eq!(Hue::try_from(RGB::<Prop>::RED), Ok(Hue::RED));
    /// assert_eq!(Hue::try_from(RGB::<Prop>::GREEN), Ok(Hue::GREEN));
    /// assert_eq!(Hue::try_from(RGB::<Prop>::BLUE), Ok(Hue::BLUE));
    /// //assert_eq!(Hue::try_from(RGB::<Prop>::CYAN), Ok(Hue::CYAN));
    ///```
    fn try_from(rgb: RGB<L>) -> Result<Hue, Self::Error> {
        let sextant = Sextant::try_from(rgb)?;
        let param = OrderedTriplet::from(rgb)
            .sextant_param()
            .expect("sextant exists");
        Ok(Self { sextant, param })
    }
}

impl Hue {
    /// Convert a `Hue` into an `Angle` from Red on the circular spectrum `+`ve towards Green and`-`ve towards Blue.
    ///
    /// Examples:
    ///
    /// ```
    /// use colour_math::{new_hue::*, hue::angle::*, HueConstants,};
    ///
    /// assert_eq!(Hue::RED.angle(), Angle::RED);
    /// assert_eq!(Hue::GREEN.angle(), Angle::GREEN);
    /// assert_eq!(Hue::BLUE.angle(), Angle::BLUE);
    /// //assert_eq!(Hue::CYAN.angle(), Angle::CYAN);
    /// //assert_eq!(Hue::MAGENTA.angle(), Angle::MAGENTA);
    /// //assert_eq!(Hue::YELLOW.angle(), Angle::YELLOW);
    /// ```
    pub fn angle(&self) -> Angle {
        let k: FDRNum = self.param.prop_into();
        let dividend = FDRNum::SQRT_3 * k / 2;
        let divisor = (FDRNum::ONE - k + k * k).sqrt();
        let angle = Angle::asin(dividend / divisor);
        match self.sextant {
            Sextant::BlueGreen => Angle::BLUE - angle,
            Sextant::BlueRed => Angle::BLUE + angle,
            Sextant::RedBlue => -angle,
            Sextant::RedGreen => angle,
            Sextant::GreenRed => Angle::GREEN - angle,
            Sextant::GreenBlue => Angle::GREEN + angle,
        }
    }
}

impl HueBasics for Hue {
    fn sum_for_max_chroma(&self) -> UFDRNum {
        UFDRNum::ONE + self.param
    }

    fn min_sum_for_chroma_prop(&self, c_prop: Prop) -> Option<UFDRNum> {
        match c_prop {
            Prop::ZERO => None,
            c_prop => Some(c_prop + self.param * c_prop),
        }
    }

    fn max_sum_for_chroma_prop(&self, c_prop: Prop) -> Option<UFDRNum> {
        match (UFDRNum::TWO - self.param) * c_prop {
            UFDRNum::ZERO => None,
            sum => Some(UFDRNum::THREE - sum),
        }
    }

    fn sum_range_for_chroma_prop(&self, c_prop: Prop) -> Option<(UFDRNum, UFDRNum)> {
        let min = self.min_sum_for_chroma_prop(c_prop)?;
        let max = self.max_sum_for_chroma_prop(c_prop)?;
        Some((min, max))
    }

    fn min_sum_for_chroma(&self, chroma: Chroma) -> Option<UFDRNum> {
        debug_assert!(chroma.is_valid());
        match chroma {
            Chroma::ZERO => None,
            Chroma::Neither(_) => Some(UFDRNum::ONE + self.param),
            Chroma::Shade(c_prop) => Some(c_prop + self.param * c_prop),
            Chroma::Tint(_) => Some(UFDRNum::ONE + self.param + UFDRNum(1)),
        }
    }

    fn max_sum_for_chroma(&self, chroma: Chroma) -> Option<UFDRNum> {
        debug_assert!(chroma.is_valid());
        match chroma {
            Chroma::ZERO => None,
            Chroma::Neither(_) => Some(UFDRNum::ONE + self.param),
            Chroma::Shade(_) => Some(UFDRNum::ONE + self.param - UFDRNum(1)),
            Chroma::Tint(c_prop) => self.max_sum_for_chroma_prop(c_prop),
        }
    }

    fn sum_range_for_chroma(&self, chroma: Chroma) -> Option<(UFDRNum, UFDRNum)> {
        debug_assert!(chroma.is_valid());
        let min = self.min_sum_for_chroma(chroma)?;
        let max = self.max_sum_for_chroma(chroma)?;
        Some((min, max))
    }

    fn max_chroma_prop_for_sum(&self, sum: UFDRNum) -> Option<Prop> {
        Some(self.max_chroma_for_sum(sum)?.into_prop())
    }

    fn max_chroma_for_sum(&self, sum: UFDRNum) -> Option<Chroma> {
        debug_assert!(sum.is_valid_sum(), "sum: {sum:?}");
        if sum.is_hue_valid() {
            match sum.cmp(&(UFDRNum::ONE + self.param)) {
                Ordering::Equal => Some(Chroma::ONE),
                Ordering::Less => {
                    let mut temp = sum / (UFDRNum::ONE + self.param);
                    // NB: check for and fix rounding error
                    while sum < (UFDRNum::ONE + self.param) * temp {
                        temp -= UFDRNum(1);
                    }
                    Some(Chroma::Shade(temp.into_prop()))
                }
                Ordering::Greater => {
                    // NB: make sure it doesn't round to one
                    let c_prop = ((UFDRNum::THREE - sum) / (UFDRNum::TWO - self.param))
                        .into_prop()
                        .min(Prop::ALMOST_ONE);
                    Some(Chroma::Tint(c_prop))
                }
            }
        } else {
            None
        }
    }
}

impl Hue {
    /// Trim `sum` to the largest rounded value compatible with `c_prop` for this `Hue`.
    ///
    /// Examples:
    ///
    /// ```
    /// use approximately_equal::assert_approx_eq;
    /// use colour_math::new_hue::*;
    /// use colour_math::fixed_denominator_rational_numbers::{self::*, debug::*};
    /// use Sextant::*;
    ///
    /// assert_eq!(
    ///     Hue{sextant: RedGreen, param: Prop::ZERO}.adjusted_sum(UFDRNum::TWO, Prop::ONE),
    ///     HueResult::Hued(UFDRNum::ONE)
    /// ) ;
    /// assert_approx_eq!(
    ///     Hue{sextant: RedGreen, param: Prop::HALF}.adjusted_sum(UFDRNum::TWO, Prop::ONE),
    ///     HueResult::Hued(UFDRNum::from_rational_num(3, 2))
    /// ) ;
    /// ```
    pub fn adjusted_sum(self, sum: UFDRNum, c_prop: Prop) -> HueResult<UFDRNum> {
        debug_assert!(sum.is_valid_sum());
        if c_prop == Prop::ZERO {
            match sum % 3u8 {
                UFDRNum(1) => HueResult::Grey(sum - UFDRNum(1)),
                UFDRNum(2) => HueResult::Grey(sum + UFDRNum(1)),
                _ => HueResult::Grey(sum),
            }
        } else {
            let min_sum = self
                .min_sum_for_chroma_prop(c_prop)
                .expect("Chroma {c_prop:?} is not zero");
            match sum.cmp(&min_sum) {
                Ordering::Less => HueResult::Hued(min_sum),
                Ordering::Equal => HueResult::Hued(sum),
                Ordering::Greater => {
                    let max_sum = self
                        .max_sum_for_chroma_prop(c_prop)
                        .expect("Chroma {c_prop:?} is not zero");
                    if sum > max_sum {
                        HueResult::Hued(max_sum)
                    } else {
                        let third = (sum - min_sum) / 3u8;
                        let first = (third + c_prop).min(UFDRNum::ONE);
                        let second = (sum - first - third).min(UFDRNum::ONE);
                        debug_assert!(!(first == second && first == third));
                        HueResult::Hued(first + second + third)
                    }
                }
            }
        }
    }

    pub fn adjust_preferencing_chroma_prop(
        &self,
        c_prop: Prop,
        sum: UFDRNum,
    ) -> Option<(Prop, UFDRNum)> {
        debug_assert!(sum.is_valid_sum());
        match sum.cmp(&self.sum_for_max_chroma()) {
            Ordering::Equal => Some((c_prop, sum)),
            Ordering::Greater => {
                let max_sum = self
                    .max_sum_for_chroma_prop(c_prop)
                    .expect("chroma is non zero");
                if sum > max_sum {
                    Some((c_prop, max_sum))
                } else {
                    Some((c_prop, sum))
                }
            }
            Ordering::Less => {
                let min_sum = self
                    .min_sum_for_chroma_prop(c_prop)
                    .expect("chroma is non zero");
                if sum < min_sum {
                    Some((c_prop, min_sum))
                } else {
                    Some((c_prop, sum))
                }
            }
        }
    }
}
