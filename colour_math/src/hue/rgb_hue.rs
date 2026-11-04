// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::{cmp::Ordering, convert::From, fmt::Debug};

use serde::{Deserialize, Serialize};

use crate::fixed_denominator_rational_numbers::{self::*, debug::*};

use crate::{
    attributes::{Chroma, Family, Warmth},
    hue::HueBasics,
    hue::{self::*, angle::Angle},
    HueConstants, Sextant, SextantHue,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord)]
pub enum RGBHue {
    Red = 5,
    Green = 9,
    Blue = 1,
}

impl RGBHue {
    pub const HUES: [Self; 3] = [RGBHue::Blue, RGBHue::Red, RGBHue::Green];

    pub(crate) fn prop_diff_sextant(&self, sextant_hue: &SextantHue) -> Option<Prop> {
        match self {
            RGBHue::Red => match sextant_hue {
                SextantHue(Sextant::RedYellow, prop) => Some(*prop),
                SextantHue(Sextant::RedMagenta, prop) => Some(*prop),
                _ => None,
            },
            RGBHue::Green => match sextant_hue {
                SextantHue(Sextant::GreenYellow, prop) => Some(*prop),
                SextantHue(Sextant::GreenCyan, prop) => Some(*prop),
                _ => None,
            },
            RGBHue::Blue => match sextant_hue {
                SextantHue(Sextant::BlueCyan, prop) => Some(*prop),
                SextantHue(Sextant::BlueMagenta, prop) => Some(*prop),
                _ => None,
            },
        }
    }
}

impl From<RGBHue> for Hue {
    fn from(rgb_hue: RGBHue) -> Self {
        Hue::Primary(rgb_hue)
    }
}

impl PropDiff for RGBHue {
    fn prop_diff(&self, other: &Self) -> Option<Prop> {
        if self == other {
            Some(Prop::ZERO)
        } else {
            None
        }
    }
}

impl ApproxEq for RGBHue {}

impl HueBasics for RGBHue {
    fn sum_for_max_chroma(&self) -> UFDRNum {
        UFDRNum::ONE
    }

    fn min_sum_for_chroma_prop(&self, c_prop: Prop) -> Option<UFDRNum> {
        match c_prop {
            Prop::ZERO => None,
            c_prop => Some(UFDRNum::from_prop(c_prop)),
        }
    }

    fn max_sum_for_chroma_prop(&self, c_prop: Prop) -> Option<UFDRNum> {
        match c_prop {
            Prop::ZERO => None,
            c_prop => Some(UFDRNum::THREE - c_prop * 2),
        }
    }

    fn min_sum_for_chroma(&self, chroma: Chroma) -> Option<UFDRNum> {
        debug_assert!(chroma.is_valid());
        match chroma {
            Chroma::ZERO => None,
            Chroma::Neither(_) => Some(UFDRNum::ONE),
            Chroma::Shade(c_prop) => Some(UFDRNum::from_prop(c_prop)),
            Chroma::Tint(c_prop) => {
                Some(UFDRNum::ONE + UFDRNum(3 - (u64::MAX - c_prop.0) as u128 % 3))
            }
        }
    }

    fn max_sum_for_chroma(&self, chroma: Chroma) -> Option<UFDRNum> {
        debug_assert!(chroma.is_valid());
        match chroma {
            Chroma::ZERO => None,
            Chroma::Neither(_) => Some(UFDRNum::ONE),
            Chroma::Shade(_) => Some(UFDRNum::ONE - UFDRNum(1)),
            Chroma::Tint(c_prop) => self.max_sum_for_chroma_prop(c_prop),
        }
    }

    fn max_chroma_for_sum(&self, sum: UFDRNum) -> Option<Chroma> {
        debug_assert!(sum.is_valid_sum(), "sum: {sum:?}");
        if sum.is_hue_valid() {
            match sum.cmp(&self.sum_for_max_chroma()) {
                Ordering::Equal => Some(Chroma::ONE),
                Ordering::Less => Some(Chroma::Shade(sum.into_prop())),
                Ordering::Greater => {
                    // NB: make sure it doesn't round to one or zero
                    let c_prop = ((UFDRNum::THREE - sum) / 2u8)
                        .into_prop()
                        .min(Prop::ALMOST_ONE)
                        .max(Prop(1));
                    Some(Chroma::Tint(c_prop))
                }
            }
        } else {
            None
        }
    }
}

impl OrderedTriplets for RGBHue {
    fn has_valid_value_order(triplet: &[Prop; 3]) -> bool {
        triplet[0] > triplet[1] && triplet[1] == triplet[2]
    }

    fn triplet_to_rgb_order(&self, triplet: &[Prop; 3]) -> [Prop; 3] {
        use RGBHue::*;
        match self {
            Red => *triplet,
            Green => [triplet[1], triplet[0], triplet[2]],
            Blue => [triplet[2], triplet[1], triplet[0]],
        }
    }

    fn try_ordered_triplet(
        &self,
        sum: UFDRNum,
        c_prop: Prop,
    ) -> Option<Result<[Prop; 3], [Prop; 3]>> {
        debug_assert!(sum.is_valid_sum());
        match c_prop {
            Prop::ZERO => None,
            min_sum if sum >= UFDRNum::from_prop(min_sum) => {
                let third = (sum - min_sum) / 3u8;
                match third + c_prop {
                    first if first > UFDRNum::ONE => None,
                    first => {
                        let second = sum - first - third;
                        debug_assert_eq!(first - third, UFDRNum::from_prop(c_prop));
                        debug_assert_eq!(first + second + third, sum);
                        if second == third {
                            debug_assert!(first > second);
                            Some(Ok([
                                first.into_prop(),
                                second.into_prop(),
                                third.into_prop(),
                            ]))
                        } else {
                            debug_assert!(first > second && second > third);
                            Some(Err([
                                first.into_prop(),
                                second.into_prop(),
                                third.into_prop(),
                            ]))
                        }
                    }
                }
            }
            _ => None,
        }
    }
}

impl ColourModificationHelpers for RGBHue {}

impl SumChromaCompatibility for RGBHue {}

impl HueIfce for RGBHue {
    fn angle(&self) -> Angle {
        match self {
            RGBHue::Red => Angle::RED,
            RGBHue::Green => Angle::GREEN,
            RGBHue::Blue => Angle::BLUE,
        }
    }

    fn warmth(&self) -> Warmth {
        match self {
            RGBHue::Red => Warmth::RED,
            RGBHue::Green => Warmth::GREEN,
            RGBHue::Blue => Warmth::BLUE,
        }
    }

    fn family(&self) -> Family {
        match self {
            RGBHue::Red => Family::Reds,
            RGBHue::Green => Family::Greens,
            RGBHue::Blue => Family::Blues,
        }
    }
}

#[cfg(test)]
impl HueTestIfce for RGBHue {}
