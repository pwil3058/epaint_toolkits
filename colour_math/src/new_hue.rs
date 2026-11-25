// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::{cmp::Ordering, fmt::Debug};

use crate::fixed_denominator_rational_numbers::*;
use crate::{Chroma, LightLevel, RGB, hue::HueBasics};

#[derive(Debug, PartialEq, Eq)]
pub enum HueResult<T: Sized + Clone + Copy + PartialEq + Eq + Debug> {
    Hued(T),
    Grey(T),
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
        if rgb[0] > rgb[1] {
            if rgb[1] > rgb[2] {
                Ok(Self::RedGreen)
            } else if rgb[0] > rgb[2] {
                Ok(Self::RedBlue)
            } else {
                Ok(Self::BlueRed)
            }
        } else if rgb[1] > rgb[2] {
            if rgb[2] > rgb[0] {
                Ok(Self::GreenBlue)
            } else {
                Ok(Self::GreenRed)
            }
        } else if rgb[2] > rgb[1] {
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
    /// use colour_math::new_hue::*;
    /// use colour_math::fixed_denominator_rational_numbers::*;
    /// use Sextant::*;
    ///
    /// assert_eq!(
    ///     Hue{sextant: RedGreen, param: Prop::ZERO}.adjusted_sum(UFDRNum::TWO, Prop::ONE),
    ///     HueResult::Hued(UFDRNum::ONE)
    /// ) ;
    /// assert_eq!(
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
