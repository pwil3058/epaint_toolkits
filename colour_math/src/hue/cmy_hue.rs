// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::{
    cmp::Ordering,
    convert::{From, Into, TryFrom},
    fmt::Debug,
    ops::{Add, Sub},
};

use serde::{Deserialize, Serialize};

use crate::fixed_denominator_rational_numbers::{self::*, debug::*};

use crate::{
    attributes::{Chroma, Family, Warmth},
    hcv::HCV,
    hue::{self::*, angle::Angle},
    rgb::RGB,
    ColourBasics, HueConstants, IntoProps, LightLevel,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord)]
pub enum CMYHue {
    Cyan = 113,
    Magenta = 3,
    Yellow = 7,
}

impl CMYHue {
    pub const HUES: [Self; 3] = [CMYHue::Magenta, CMYHue::Yellow, CMYHue::Cyan];

    pub(crate) fn prop_diff_sextant(&self, sextant_hue: &SextantHue) -> Option<Prop> {
        match self {
            CMYHue::Cyan => match sextant_hue {
                SextantHue(Sextant::GreenCyan, prop) => Some(Prop::ONE - *prop),
                SextantHue(Sextant::BlueCyan, prop) => Some(Prop::ONE - *prop),
                _ => None,
            },
            CMYHue::Magenta => match sextant_hue {
                SextantHue(Sextant::RedMagenta, prop) => Some(Prop::ONE - *prop),
                SextantHue(Sextant::BlueMagenta, prop) => Some(Prop::ONE - *prop),
                _ => None,
            },
            CMYHue::Yellow => match sextant_hue {
                SextantHue(Sextant::RedYellow, prop) => Some(Prop::ONE - *prop),
                SextantHue(Sextant::GreenYellow, prop) => Some(Prop::ONE - *prop),
                _ => None,
            },
        }
    }
}

impl From<CMYHue> for Hue {
    fn from(cmy_hue: CMYHue) -> Self {
        Hue::Secondary(cmy_hue)
    }
}

impl PropDiff for CMYHue {
    fn prop_diff(&self, other: &Self) -> Option<Prop> {
        if self == other {
            Some(Prop::ZERO)
        } else {
            None
        }
    }
}

impl ApproxEq for CMYHue {}

impl HueBasics for CMYHue {
    fn sum_for_max_chroma(&self) -> UFDRNum {
        UFDRNum::TWO
    }

    fn min_sum_for_chroma_prop(&self, c_prop: Prop) -> Option<UFDRNum> {
        match c_prop {
            Prop::ZERO => None,
            c_prop => Some(c_prop * 2),
        }
    }

    fn max_sum_for_chroma_prop(&self, c_prop: Prop) -> Option<UFDRNum> {
        match c_prop {
            Prop::ZERO => None,
            c_prop => Some(UFDRNum::THREE - c_prop),
        }
    }

    fn min_sum_for_chroma(&self, chroma: Chroma) -> Option<UFDRNum> {
        debug_assert!(chroma.is_valid());
        match chroma {
            Chroma::ZERO => None,
            Chroma::Neither(_) => Some(UFDRNum::TWO),
            Chroma::Shade(c_prop) => Some(c_prop * 2),
            Chroma::Tint(c_prop) => Some(UFDRNum::TWO + UFDRNum(3 - c_prop.0 as u128 % 3)),
        }
    }

    fn max_sum_for_chroma(&self, chroma: Chroma) -> Option<UFDRNum> {
        debug_assert!(chroma.is_valid());
        match chroma {
            Chroma::ZERO => None,
            Chroma::Neither(_) => Some(UFDRNum::TWO),
            Chroma::Shade(c_prop) => {
                Some(UFDRNum::TWO - UFDRNum(3 - (u64::MAX - c_prop.0) as u128 % 3))
            }
            Chroma::Tint(c_prop) => self.max_sum_for_chroma_prop(c_prop),
        }
    }

    fn max_chroma_for_sum(&self, sum: UFDRNum) -> Option<Chroma> {
        debug_assert!(sum.is_valid_sum(), "sum: {sum:?}");
        if sum.is_hue_valid() {
            match sum.cmp(&UFDRNum::TWO) {
                Ordering::Equal => Some(Chroma::ONE),
                // TODO: fix max chroma for sum (more complicated for uneven sums)
                Ordering::Less => Some(Chroma::Shade((sum / 2u8).into_prop().max(Prop(1)))),
                Ordering::Greater => Some(Chroma::Tint((UFDRNum::THREE - sum).into_prop())),
            }
        } else {
            None
        }
    }
}

impl OrderedTriplets for CMYHue {
    fn has_valid_value_order(triplet: &[Prop; 3]) -> bool {
        triplet[0] == triplet[1] && triplet[1] > triplet[2]
    }

    fn triplet_to_rgb_order(&self, triplet: &[Prop; 3]) -> [Prop; 3] {
        //debug_assert!(self.has_valid_value_order(&triplet));
        use CMYHue::*;
        match self {
            Cyan => [triplet[2], triplet[0], triplet[1]],
            Magenta => [triplet[0], triplet[2], triplet[1]],
            Yellow => *triplet,
        }
    }

    fn try_ordered_triplet(
        &self,
        sum: UFDRNum,
        c_prop: Prop,
    ) -> Option<Result<[Prop; 3], [Prop; 3]>> {
        debug_assert!(sum.is_valid_sum());
        match c_prop * 2 {
            UFDRNum::ZERO => None,
            min_sum if sum >= min_sum => {
                let third = (sum - min_sum) / 3u8;
                match third + c_prop {
                    first if first > UFDRNum::ONE => None,
                    first => match sum - first - third {
                        second if second == first => {
                            debug_assert!(second > third);
                            Some(Ok([
                                first.into_prop(),
                                second.into_prop(),
                                third.into_prop(),
                            ]))
                        }
                        second => {
                            debug_assert!(second > first && first > third);
                            let diff = second - first;
                            debug_assert!(diff < UFDRNum(3));
                            if diff == UFDRNum(1) {
                                if second.is_proportion() {
                                    let first = first - UFDRNum(1);
                                    let third = third + UFDRNum(1);
                                    debug_assert!(second > third && first > third);
                                    debug_assert_eq!(second - third, UFDRNum::from_prop(c_prop));
                                    debug_assert_eq!(first + second + third, sum);
                                    Some(Err([
                                        second.into_prop(),
                                        first.into_prop(),
                                        third.into_prop(),
                                    ]))
                                } else {
                                    None
                                }
                            } else {
                                let first = first + UFDRNum(1);
                                if first.is_proportion() {
                                    let third = third + UFDRNum(1);
                                    let second = second - UFDRNum(2);
                                    debug_assert!(second > third);
                                    debug_assert_eq!(first - third, UFDRNum::from_prop(c_prop));
                                    debug_assert_eq!(first + second + third, sum);
                                    Some(Err([
                                        first.into_prop(),
                                        second.into_prop(),
                                        third.into_prop(),
                                    ]))
                                } else {
                                    None
                                }
                            }
                        }
                    },
                }
            }
            _ => None,
        }
    }
}

impl ColourModificationHelpers for CMYHue {}

impl SumChromaCompatibility for CMYHue {}

impl HueIfce for CMYHue {
    fn angle(&self) -> Angle {
        match self {
            CMYHue::Cyan => Angle::CYAN,
            CMYHue::Magenta => Angle::MAGENTA,
            CMYHue::Yellow => Angle::YELLOW,
        }
    }

    fn warmth(&self) -> Warmth {
        match self {
            CMYHue::Cyan => Warmth::CYAN,
            CMYHue::Magenta => Warmth::MAGENTA,
            CMYHue::Yellow => Warmth::YELLOW,
        }
    }

    fn family(&self) -> Family {
        match self {
            CMYHue::Cyan => Family::Cyans,
            CMYHue::Magenta => Family::Magentas,
            CMYHue::Yellow => Family::Yellows,
        }
    }
}
