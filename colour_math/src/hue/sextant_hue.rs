// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::{cmp::Ordering, convert::From, fmt::Debug};

use serde::{Deserialize, Serialize};

use crate::fixed_denominator_rational_numbers::{self::*, debug::*};

use crate::{
    attributes::{Chroma, Family, Warmth},
    hue::{self::*, angle::Angle},
    rgb::RGB,
    HueConstants, IntoProps, LightLevel,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord)]
pub enum Sextant {
    RedMagenta = 4,
    RedYellow = 6,
    GreenYellow = 8,
    GreenCyan = 10,
    BlueCyan = 0,
    BlueMagenta = 2,
}

impl Sextant {
    pub const SEXTANTS: [Self; 6] = [
        Sextant::BlueCyan,
        Sextant::BlueMagenta,
        Sextant::RedMagenta,
        Sextant::RedYellow,
        Sextant::GreenYellow,
        Sextant::GreenCyan,
    ];
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord)]
pub struct SextantHue(pub(crate) Sextant, pub(crate) Prop);

impl From<SextantHue> for Hue {
    fn from(sextant_hue: SextantHue) -> Self {
        Hue::Sextant(sextant_hue)
    }
}

impl PropDiff for SextantHue {
    fn prop_diff(&self, other: &Self) -> Option<Prop> {
        match self {
            SextantHue(Sextant::RedYellow, my_prop) => match other {
                SextantHue(Sextant::RedYellow, other_prop) => my_prop.prop_diff(other_prop),
                SextantHue(Sextant::RedMagenta, other_prop) => match *my_prop + *other_prop {
                    prop if prop <= UFDRNum::ONE => Some(prop.into_prop()),
                    _ => None,
                },
                SextantHue(Sextant::GreenYellow, other_prop) => {
                    match UFDRNum::TWO - (*my_prop + *other_prop) {
                        prop if prop <= UFDRNum::ONE => Some(prop.into_prop()),
                        _ => None,
                    }
                }
                _ => None,
            },
            SextantHue(Sextant::GreenYellow, my_prop) => match other {
                SextantHue(Sextant::GreenYellow, other_prop) => my_prop.prop_diff(other_prop),
                SextantHue(Sextant::GreenCyan, other_prop) => match *my_prop + *other_prop {
                    prop if prop <= UFDRNum::ONE => Some(prop.into_prop()),
                    _ => None,
                },
                SextantHue(Sextant::RedYellow, other_prop) => {
                    match UFDRNum::TWO - (*my_prop + *other_prop) {
                        prop if prop <= UFDRNum::ONE => Some(prop.into_prop()),
                        _ => None,
                    }
                }
                _ => None,
            },
            SextantHue(Sextant::GreenCyan, my_prop) => match other {
                SextantHue(Sextant::GreenCyan, other_prop) => my_prop.prop_diff(other_prop),
                SextantHue(Sextant::GreenYellow, other_prop) => match *my_prop + *other_prop {
                    prop if prop <= UFDRNum::ONE => Some(prop.into_prop()),
                    _ => None,
                },
                SextantHue(Sextant::BlueCyan, other_prop) => {
                    match UFDRNum::TWO - (*my_prop + *other_prop) {
                        prop if prop <= UFDRNum::ONE => Some(prop.into_prop()),
                        _ => None,
                    }
                }
                _ => None,
            },
            SextantHue(Sextant::BlueCyan, my_prop) => match other {
                SextantHue(Sextant::BlueCyan, other_prop) => my_prop.prop_diff(other_prop),
                SextantHue(Sextant::BlueMagenta, other_prop) => match *my_prop + *other_prop {
                    prop if prop <= UFDRNum::ONE => Some(prop.into_prop()),
                    _ => None,
                },
                SextantHue(Sextant::GreenCyan, other_prop) => {
                    match UFDRNum::TWO - (*my_prop + *other_prop) {
                        prop if prop <= UFDRNum::ONE => Some(prop.into_prop()),
                        _ => None,
                    }
                }
                _ => None,
            },
            SextantHue(Sextant::BlueMagenta, my_prop) => match other {
                SextantHue(Sextant::BlueMagenta, other_prop) => my_prop.prop_diff(other_prop),
                SextantHue(Sextant::BlueCyan, other_prop) => match *my_prop + *other_prop {
                    prop if prop <= UFDRNum::ONE => Some(prop.into_prop()),
                    _ => None,
                },
                SextantHue(Sextant::RedMagenta, other_prop) => {
                    match UFDRNum::TWO - (*my_prop + *other_prop) {
                        prop if prop <= UFDRNum::ONE => Some(prop.into_prop()),
                        _ => None,
                    }
                }
                _ => None,
            },
            SextantHue(Sextant::RedMagenta, my_prop) => match other {
                SextantHue(Sextant::RedMagenta, other_prop) => my_prop.prop_diff(other_prop),
                SextantHue(Sextant::RedYellow, other_prop) => match *my_prop + *other_prop {
                    prop if prop <= UFDRNum::ONE => Some(prop.into_prop()),
                    _ => None,
                },
                SextantHue(Sextant::BlueMagenta, other_prop) => {
                    match UFDRNum::TWO - (*my_prop + *other_prop) {
                        prop if prop <= UFDRNum::ONE => Some(prop.into_prop()),
                        _ => None,
                    }
                }
                _ => None,
            },
        }
    }
}

impl ApproxEq for SextantHue {}

impl Eq for SextantHue {}

impl HueBasics for SextantHue {
    fn sum_for_max_chroma(&self) -> UFDRNum {
        UFDRNum::ONE + self.1
    }

    fn min_sum_for_chroma_prop(&self, c_prop: Prop) -> Option<UFDRNum> {
        match c_prop {
            Prop::ZERO => None,
            c_prop => Some(c_prop + self.1 * c_prop),
        }
    }

    fn max_sum_for_chroma_prop(&self, c_prop: Prop) -> Option<UFDRNum> {
        match (UFDRNum::TWO - self.1) * c_prop {
            UFDRNum::ZERO => None,
            sum => Some(UFDRNum::THREE - sum),
        }
    }

    fn min_sum_for_chroma(&self, chroma: Chroma) -> Option<UFDRNum> {
        debug_assert!(chroma.is_valid());
        match chroma {
            Chroma::ZERO => None,
            Chroma::Neither(_) => Some(UFDRNum::ONE + self.1),
            Chroma::Shade(c_prop) => Some(c_prop + self.1 * c_prop),
            Chroma::Tint(_) => Some(UFDRNum::ONE + self.1 + UFDRNum(1)),
        }
    }

    fn max_sum_for_chroma(&self, chroma: Chroma) -> Option<UFDRNum> {
        debug_assert!(chroma.is_valid());
        match chroma {
            Chroma::ZERO => None,
            Chroma::Neither(_) => Some(UFDRNum::ONE + self.1),
            Chroma::Shade(_) => Some(UFDRNum::ONE + self.1 - UFDRNum(1)),
            Chroma::Tint(c_prop) => self.max_sum_for_chroma_prop(c_prop),
        }
    }

    fn max_chroma_for_sum(&self, sum: UFDRNum) -> Option<Chroma> {
        debug_assert!(sum.is_valid_sum(), "sum: {sum:?}");
        if sum.is_hue_valid() {
            match sum.cmp(&(UFDRNum::ONE + self.1)) {
                Ordering::Equal => Some(Chroma::ONE),
                Ordering::Less => {
                    let mut temp = sum / (UFDRNum::ONE + self.1);
                    // NB: check for and fix rounding error
                    while sum < (UFDRNum::ONE + self.1) * temp {
                        temp -= UFDRNum(1);
                    }
                    Some(Chroma::Shade(temp.into_prop()))
                }
                Ordering::Greater => {
                    // NB: make sure it doesn't round to one
                    let c_prop = ((UFDRNum::THREE - sum) / (UFDRNum::TWO - self.1))
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

impl OrderedTriplets for SextantHue {
    fn has_valid_value_order(triplet: &[Prop; 3]) -> bool {
        triplet[0] > triplet[1] && triplet[1] > triplet[2]
    }

    fn triplet_to_rgb_order(&self, triplet: &[Prop; 3]) -> [Prop; 3] {
        debug_assert!(Self::has_valid_value_order(triplet));
        use Sextant::*;
        match self.0 {
            RedMagenta => [triplet[0], triplet[2], triplet[1]],
            RedYellow => *triplet,
            GreenYellow => [triplet[1], triplet[0], triplet[2]],
            GreenCyan => [triplet[2], triplet[0], triplet[1]],
            BlueCyan => [triplet[2], triplet[1], triplet[0]],
            BlueMagenta => [triplet[1], triplet[2], triplet[0]],
        }
    }

    fn try_ordered_triplet(
        &self,
        sum: UFDRNum,
        c_prop: Prop,
    ) -> Option<Result<[Prop; 3], [Prop; 3]>> {
        debug_assert!(sum.is_valid_sum());
        match self.sum_for_max_chroma() * c_prop {
            UFDRNum::ZERO => None,
            min_sum if sum >= min_sum => {
                let third = (sum - min_sum) / 3u8;
                match third + c_prop {
                    first if !first.is_proportion() => None,
                    first => {
                        let second = sum - first - third;
                        match second.cmp(&first) {
                            Ordering::Less => {
                                debug_assert_eq!(first + second + third, sum);
                                debug_assert_eq!(first - third, UFDRNum::from_prop(c_prop));
                                let triplet =
                                    [first.into_prop(), second.into_prop(), third.into_prop()];
                                debug_assert!(first > second && second > third);
                                if Self::calculate_hue_parameter(triplet) == self.1 {
                                    Some(Ok(triplet))
                                } else {
                                    Some(Err(triplet))
                                }
                            }
                            Ordering::Equal => {
                                debug_assert_eq!(first + second + third, sum);
                                debug_assert_eq!(first - third, UFDRNum::from_prop(c_prop));
                                debug_assert!(second > third);
                                Some(Err([
                                    first.into_prop(),
                                    second.into_prop(),
                                    third.into_prop(),
                                ]))
                            }
                            Ordering::Greater => {
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
                            }
                        }
                    }
                }
            }
            _ => None,
        }
    }
}

impl ColourModificationHelpers for SextantHue {}

impl SumChromaCompatibility for SextantHue {
    fn sum_and_chroma_prop_are_compatible(&self, sum: UFDRNum, c_prop: Prop) -> bool {
        debug_assert!(sum.is_valid_sum());
        match c_prop + self.1 * c_prop {
            min_sum if sum < min_sum || min_sum == UFDRNum::ZERO => false,
            min_sum => {
                let third = (sum - min_sum) / 3u8;
                match third + c_prop {
                    first if first.is_proportion() => {
                        let second = sum - first - third;
                        if first > second && second > third {
                            Self::calculate_hue_parameter([
                                first.into_prop(),
                                second.into_prop(),
                                third.into_prop(),
                            ]) == self.1
                        } else {
                            false
                        }
                    }
                    _ => false,
                }
            }
        }
    }
}

impl SextantHue {
    pub fn abs_diff(&self, other: &Self) -> Prop {
        if self.0 == other.0 {
            self.1.abs_diff(&other.1)
        } else {
            Prop::ONE
        }
    }

    pub fn sextant(&self) -> Sextant {
        self.0
    }

    pub fn prop(&self) -> Prop {
        self.1
    }

    fn calculate_hue_parameter(triplet: [Prop; 3]) -> Prop {
        let [first, second, third] = triplet;
        debug_assert!(first > second && second > third);
        let mut sum_for_max_chroma = UFDRNum::ONE + ((second - third) / (first - third));
        // Handle possible (fatal) rounding error
        while sum_for_max_chroma > UFDRNum::ONE + UFDRNum(1)
            && sum_for_max_chroma * (first - third) > first + second + third
        {
            sum_for_max_chroma -= UFDRNum(1);
        }
        while sum_for_max_chroma > UFDRNum::ONE + UFDRNum(1)
            && (first + second + third - sum_for_max_chroma * (first - third)) / 3u8
                < UFDRNum::from_prop(third)
        {
            sum_for_max_chroma -= UFDRNum(1);
        }
        debug_assert!(sum_for_max_chroma > UFDRNum::ONE && sum_for_max_chroma < UFDRNum::TWO);
        debug_assert_eq!(
            (first + second + third - sum_for_max_chroma * (first - third)) / 3u8,
            UFDRNum::from_prop(third)
        );
        (sum_for_max_chroma - UFDRNum::ONE).into_prop()
    }
}

#[cfg(test)]
impl SextantHue {
    pub fn approx_eq(&self, other: &Self, acceptable_rounding_error: Option<u64>) -> bool {
        if self.0 == other.0 {
            self.1.approx_eq(&other.1, acceptable_rounding_error)
        } else {
            false
        }
    }
}

impl<T: LightLevel> From<(Sextant, &RGB<T>)> for SextantHue {
    fn from(arg: (Sextant, &RGB<T>)) -> Self {
        // Self::from((arg.0, <[Prop; 3]>::from(*arg.1)))
        Self::from((arg.0, arg.1.into_proportions()))
    }
}

impl From<(Sextant, [Prop; 3])> for SextantHue {
    fn from(arg: (Sextant, [Prop; 3])) -> Self {
        let hue_param = Self::calculate_hue_parameter(arg.1);
        Self(arg.0, hue_param)
    }
}

impl From<(Sextant, &[Prop; 3])> for SextantHue {
    fn from(arg: (Sextant, &[Prop; 3])) -> Self {
        Self::from((arg.0, *arg.1))
    }
}

impl HueIfce for SextantHue {
    fn angle(&self) -> Angle {
        use crate::UsefulFloatConstants;
        match self {
            SextantHue(Sextant::BlueCyan, Prop::HALF) => Angle::BLUE_CYAN,
            SextantHue(Sextant::BlueMagenta, Prop::HALF) => Angle::BLUE_MAGENTA,
            SextantHue(Sextant::RedMagenta, Prop::HALF) => Angle::RED_MAGENTA,
            SextantHue(Sextant::RedYellow, Prop::HALF) => Angle::RED_YELLOW,
            SextantHue(Sextant::GreenYellow, Prop::HALF) => Angle::GREEN_YELLOW,
            SextantHue(Sextant::GreenCyan, Prop::HALF) => Angle::GREEN_CYAN,
            _ => {
                let second: f64 = self.1.prop_into();
                let sin = f64::SQRT_3 * second / 2.0 / (1.0 - second + second.powi(2)).sqrt();
                let angle = Angle::asin(FDRNum::from(sin));
                match self.0 {
                    Sextant::RedMagenta => -angle,
                    Sextant::RedYellow => angle,
                    Sextant::GreenYellow => Angle::GREEN - angle,
                    Sextant::GreenCyan => Angle::GREEN + angle,
                    Sextant::BlueCyan => Angle::BLUE - angle,
                    Sextant::BlueMagenta => Angle::BLUE + angle,
                }
            }
        }
    }

    fn warmth(&self) -> Warmth {
        match self {
            SextantHue(Sextant::BlueCyan, Prop::HALF) => Warmth::BLUE_CYAN,
            SextantHue(Sextant::BlueMagenta, Prop::HALF) => Warmth::BLUE_MAGENTA,
            SextantHue(Sextant::RedMagenta, Prop::HALF) => Warmth::RED_MAGENTA,
            SextantHue(Sextant::RedYellow, Prop::HALF) => Warmth::RED_YELLOW,
            SextantHue(Sextant::GreenYellow, Prop::HALF) => Warmth::GREEN_YELLOW,
            SextantHue(Sextant::GreenCyan, Prop::HALF) => Warmth::GREEN_CYAN,
            _ => {
                let delta_warmth = Warmth::ONE_THIRD * self.1;
                match self.0 {
                    Sextant::RedMagenta => Warmth::RED - delta_warmth,
                    Sextant::RedYellow => Warmth::RED - delta_warmth,
                    Sextant::GreenYellow => Warmth::GREEN + delta_warmth,
                    Sextant::GreenCyan => Warmth::GREEN - delta_warmth,
                    Sextant::BlueCyan => Warmth::BLUE - delta_warmth,
                    Sextant::BlueMagenta => Warmth::BLUE + delta_warmth,
                }
            }
        }
    }

    fn family(&self) -> Family {
        match self {
            SextantHue(Sextant::BlueCyan, prop) => {
                if prop < &Prop::HALF {
                    Family::Blues
                } else {
                    Family::Cyans
                }
            }
            SextantHue(Sextant::BlueMagenta, prop) => {
                if prop < &Prop::HALF {
                    Family::Blues
                } else {
                    Family::Magentas
                }
            }
            SextantHue(Sextant::RedMagenta, prop) => {
                if prop < &Prop::HALF {
                    Family::Reds
                } else {
                    Family::Magentas
                }
            }
            SextantHue(Sextant::RedYellow, prop) => {
                if prop < &Prop::HALF {
                    Family::Reds
                } else {
                    Family::Yellows
                }
            }
            SextantHue(Sextant::GreenYellow, prop) => {
                if prop < &Prop::HALF {
                    Family::Greens
                } else {
                    Family::Yellows
                }
            }
            SextantHue(Sextant::GreenCyan, prop) => {
                if prop < &Prop::HALF {
                    Family::Greens
                } else {
                    Family::Cyans
                }
            }
        }
    }
}

#[cfg(test)]
impl HueTestIfce for SextantHue {}
