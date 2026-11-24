// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::cmp::Ordering;

use crate::fixed_denominator_rational_numbers::*;
use crate::{Chroma, hue::HueBasics};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hue {
    sextant: Sextant,
    param: Prop,
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
