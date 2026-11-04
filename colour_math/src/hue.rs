// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::{
    cmp::Ordering,
    convert::{From, TryFrom},
    fmt::Debug,
    ops::{Add, Sub},
};

use serde::{Deserialize, Serialize};

pub mod angle;
pub mod cmy_hue;
pub mod rgb_hue;
pub mod sextant_hue;
pub mod traits;

use crate::fixed_denominator_rational_numbers::{self::*, debug::*};

use crate::{
    attributes::{Chroma, Family, Warmth},
    hcv::HCV,
    hue::angle::Angle,
    rgb::RGB,
    HueConstants, IntoProps, LightLevel,
};

pub use cmy_hue::*;
pub use rgb_hue::*;
pub use sextant_hue::*;
pub use traits::*;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Hue {
    Primary(RGBHue),
    Secondary(CMYHue),
    Sextant(SextantHue),
}

impl Eq for Hue {}

impl HueConstants for Hue {
    const RED: Self = Self::Primary(RGBHue::Red);
    const GREEN: Self = Self::Primary(RGBHue::Green);
    const BLUE: Self = Self::Primary(RGBHue::Blue);

    const CYAN: Self = Self::Secondary(CMYHue::Cyan);
    const MAGENTA: Self = Self::Secondary(CMYHue::Magenta);
    const YELLOW: Self = Self::Secondary(CMYHue::Yellow);

    const BLUE_CYAN: Self = Self::Sextant(SextantHue(Sextant::BlueCyan, Prop::HALF));
    const BLUE_MAGENTA: Self = Self::Sextant(SextantHue(Sextant::BlueMagenta, Prop::HALF));
    const RED_MAGENTA: Self = Self::Sextant(SextantHue(Sextant::RedMagenta, Prop::HALF));
    const RED_YELLOW: Self = Self::Sextant(SextantHue(Sextant::RedYellow, Prop::HALF));
    const GREEN_YELLOW: Self = Self::Sextant(SextantHue(Sextant::GreenYellow, Prop::HALF));
    const GREEN_CYAN: Self = Self::Sextant(SextantHue(Sextant::GreenCyan, Prop::HALF));
}

impl Default for Hue {
    fn default() -> Self {
        Self::RED
    }
}

impl TryFrom<[Prop; 3]> for Hue {
    type Error = &'static str;

    fn try_from(arrayx: [Prop; 3]) -> Result<Self, Self::Error> {
        use Sextant::*;
        let [red, green, blue] = arrayx;
        match red.cmp(&green) {
            Ordering::Greater => match green.cmp(&blue) {
                Ordering::Greater => Ok(Hue::Sextant(SextantHue::from((
                    RedYellow,
                    [red, green, blue],
                )))),
                Ordering::Less => match red.cmp(&blue) {
                    Ordering::Greater => Ok(Hue::Sextant(SextantHue::from((
                        RedMagenta,
                        [red, blue, green],
                    )))),
                    Ordering::Less => Ok(Hue::Sextant(SextantHue::from((
                        BlueMagenta,
                        [blue, red, green],
                    )))),
                    Ordering::Equal => Ok(Hue::Secondary(CMYHue::Magenta)),
                },
                Ordering::Equal => Ok(Hue::Primary(RGBHue::Red)),
            },
            Ordering::Less => match red.cmp(&blue) {
                Ordering::Greater => Ok(Hue::Sextant(SextantHue::from((
                    GreenYellow,
                    [green, red, blue],
                )))),
                Ordering::Less => match green.cmp(&blue) {
                    Ordering::Greater => Ok(Hue::Sextant(SextantHue::from((
                        GreenCyan,
                        [green, blue, red],
                    )))),
                    Ordering::Less => Ok(Hue::Sextant(SextantHue::from((
                        BlueCyan,
                        [blue, green, red],
                    )))),
                    Ordering::Equal => Ok(Hue::Secondary(CMYHue::Cyan)),
                },
                Ordering::Equal => Ok(Hue::Primary(RGBHue::Green)),
            },
            Ordering::Equal => match red.cmp(&blue) {
                Ordering::Greater => Ok(Hue::Secondary(CMYHue::Yellow)),
                Ordering::Less => Ok(Hue::Primary(RGBHue::Blue)),
                Ordering::Equal => Err("RGB is grey and hs no hue"),
            },
        }
    }
}

impl TryFrom<&[Prop; 3]> for Hue {
    type Error = &'static str;

    fn try_from(array: &[Prop; 3]) -> Result<Self, Self::Error> {
        Self::try_from(*array)
    }
}

impl<T: LightLevel> TryFrom<&RGB<T>> for Hue {
    type Error = &'static str;

    fn try_from(rgb: &RGB<T>) -> Result<Self, Self::Error> {
        Self::try_from(rgb.into_proportions())
    }
}

impl<T: LightLevel> TryFrom<RGB<T>> for Hue {
    type Error = &'static str;

    fn try_from(rgb: RGB<T>) -> Result<Self, Self::Error> {
        Self::try_from(rgb.into_proportions())
    }
}

impl From<Angle> for Hue {
    fn from(angle: Angle) -> Self {
        match angle {
            Angle::RED => Hue::RED,
            Angle::GREEN => Hue::GREEN,
            Angle::BLUE => Hue::BLUE,
            Angle::CYAN => Hue::CYAN,
            Angle::MAGENTA => Hue::MAGENTA,
            Angle::YELLOW => Hue::YELLOW,
            Angle::BLUE_CYAN => Hue::BLUE_CYAN,
            Angle::BLUE_MAGENTA => Hue::BLUE_MAGENTA,
            Angle::RED_MAGENTA => Hue::RED_MAGENTA,
            Angle::RED_YELLOW => Hue::RED_YELLOW,
            Angle::GREEN_YELLOW => Hue::GREEN_YELLOW,
            Angle::GREEN_CYAN => Hue::GREEN_CYAN,
            _ => {
                fn f(angle: Angle) -> Prop {
                    (angle.sin() / (Angle::GREEN - angle).sin()).into_prop()
                }
                if angle >= Angle::RED {
                    if angle < Angle::YELLOW {
                        Hue::Sextant(SextantHue(Sextant::RedYellow, f(angle)))
                    } else if angle < Angle::GREEN {
                        Hue::Sextant(SextantHue(Sextant::GreenYellow, f(Angle::GREEN - angle)))
                    } else {
                        Hue::Sextant(SextantHue(Sextant::GreenCyan, f(angle - Angle::GREEN)))
                    }
                } else if angle > Angle::MAGENTA {
                    Hue::Sextant(SextantHue(Sextant::RedMagenta, f(-angle)))
                } else if angle > Angle::BLUE {
                    Hue::Sextant(SextantHue(Sextant::BlueMagenta, f(Angle::GREEN + angle)))
                } else {
                    Hue::Sextant(SextantHue(Sextant::BlueCyan, f(-angle - Angle::GREEN)))
                }
            }
        }
    }
}

impl From<Hue> for Angle {
    fn from(hue: Hue) -> Self {
        hue.angle()
    }
}

impl HueBasics for Hue {
    fn sum_for_max_chroma(&self) -> UFDRNum {
        match self {
            Self::Primary(rgb_hue) => rgb_hue.sum_for_max_chroma(),
            Self::Secondary(cmy_hue) => cmy_hue.sum_for_max_chroma(),
            Self::Sextant(sextant_hue) => sextant_hue.sum_for_max_chroma(),
        }
    }

    fn min_sum_for_chroma_prop(&self, c_prop: Prop) -> Option<UFDRNum> {
        match self {
            Self::Primary(primary_hue) => primary_hue.min_sum_for_chroma_prop(c_prop),
            Self::Secondary(secondary_hue) => secondary_hue.min_sum_for_chroma_prop(c_prop),
            Self::Sextant(sextant_hue) => sextant_hue.min_sum_for_chroma_prop(c_prop),
        }
    }

    fn max_sum_for_chroma_prop(&self, c_prop: Prop) -> Option<UFDRNum> {
        match self {
            Self::Primary(primary_hue) => primary_hue.max_sum_for_chroma_prop(c_prop),
            Self::Secondary(secondary_hue) => secondary_hue.max_sum_for_chroma_prop(c_prop),
            Self::Sextant(sextant_hue) => sextant_hue.max_sum_for_chroma_prop(c_prop),
        }
    }

    fn sum_range_for_chroma_prop(&self, c_prop: Prop) -> Option<(UFDRNum, UFDRNum)> {
        match self {
            Self::Primary(primary_hue) => primary_hue.sum_range_for_chroma_prop(c_prop),
            Self::Secondary(secondary_hue) => secondary_hue.sum_range_for_chroma_prop(c_prop),
            Self::Sextant(sextant_hue) => sextant_hue.sum_range_for_chroma_prop(c_prop),
        }
    }

    fn min_sum_for_chroma(&self, chroma: Chroma) -> Option<UFDRNum> {
        match self {
            Self::Primary(primary_hue) => primary_hue.min_sum_for_chroma(chroma),
            Self::Secondary(secondary_hue) => secondary_hue.min_sum_for_chroma(chroma),
            Self::Sextant(sextant_hue) => sextant_hue.min_sum_for_chroma(chroma),
        }
    }

    fn max_sum_for_chroma(&self, chroma: Chroma) -> Option<UFDRNum> {
        match self {
            Self::Primary(primary_hue) => primary_hue.max_sum_for_chroma(chroma),
            Self::Secondary(secondary_hue) => secondary_hue.max_sum_for_chroma(chroma),
            Self::Sextant(sextant_hue) => sextant_hue.max_sum_for_chroma(chroma),
        }
    }

    fn sum_range_for_chroma(&self, chroma: Chroma) -> Option<(UFDRNum, UFDRNum)> {
        match self {
            Self::Primary(primary_hue) => primary_hue.sum_range_for_chroma(chroma),
            Self::Secondary(secondary_hue) => secondary_hue.sum_range_for_chroma(chroma),
            Self::Sextant(sextant_hue) => sextant_hue.sum_range_for_chroma(chroma),
        }
    }

    fn max_chroma_prop_for_sum(&self, sum: UFDRNum) -> Option<Prop> {
        match self {
            Self::Primary(primary_hue) => primary_hue.max_chroma_prop_for_sum(sum),
            Self::Secondary(secondary_hue) => secondary_hue.max_chroma_prop_for_sum(sum),
            Self::Sextant(sextant_hue) => sextant_hue.max_chroma_prop_for_sum(sum),
        }
    }

    fn max_chroma_for_sum(&self, sum: UFDRNum) -> Option<Chroma> {
        match self {
            Self::Primary(primary_hue) => primary_hue.max_chroma_for_sum(sum),
            Self::Secondary(secondary_hue) => secondary_hue.max_chroma_for_sum(sum),
            Self::Sextant(sextant_hue) => sextant_hue.max_chroma_for_sum(sum),
        }
    }
}

impl SumChromaCompatibility for Hue {
    fn sum_and_chroma_prop_are_compatible(&self, sum: UFDRNum, c_prop: Prop) -> bool {
        match self {
            Self::Primary(rgb_hue) => rgb_hue.sum_and_chroma_prop_are_compatible(sum, c_prop),
            Self::Secondary(cmy_hue) => cmy_hue.sum_and_chroma_prop_are_compatible(sum, c_prop),
            Self::Sextant(sextant_hue) => {
                sextant_hue.sum_and_chroma_prop_are_compatible(sum, c_prop)
            }
        }
    }
}

impl OrderedTriplets for Hue {
    fn has_valid_value_order(triplet: &[Prop; 3]) -> bool {
        match triplet[0].cmp(&triplet[1]) {
            Ordering::Equal => triplet[1] > triplet[2],
            Ordering::Less => false,
            Ordering::Greater => triplet[1] >= triplet[2],
        }
    }

    fn triplet_to_rgb_order(&self, triplet: &[Prop; 3]) -> [Prop; 3] {
        match self {
            Self::Primary(rgb_hue) => rgb_hue.triplet_to_rgb_order(triplet),
            Self::Secondary(cmy_hue) => cmy_hue.triplet_to_rgb_order(triplet),
            Self::Sextant(sextant_hue) => sextant_hue.triplet_to_rgb_order(triplet),
        }
    }

    fn rgb_ordered_triplet(&self, sum: UFDRNum, c_prop: Prop) -> Option<[Prop; 3]> {
        match self {
            Self::Primary(rgb_hue) => rgb_hue.rgb_ordered_triplet(sum, c_prop),
            Self::Secondary(cmy_hue) => cmy_hue.rgb_ordered_triplet(sum, c_prop),
            Self::Sextant(sextant_hue) => sextant_hue.rgb_ordered_triplet(sum, c_prop),
        }
    }

    fn try_rgb_ordered_triplet(
        &self,
        sum: UFDRNum,
        c_prop: Prop,
    ) -> Option<Result<[Prop; 3], [Prop; 3]>> {
        match self {
            Self::Primary(primary_hue) => primary_hue.try_rgb_ordered_triplet(sum, c_prop),
            Self::Secondary(secondary_hue) => secondary_hue.try_rgb_ordered_triplet(sum, c_prop),
            Self::Sextant(sextant_hue) => sextant_hue.try_rgb_ordered_triplet(sum, c_prop),
        }
    }

    fn ordered_triplet(&self, sum: UFDRNum, c_prop: Prop) -> Option<[Prop; 3]> {
        match self {
            Self::Primary(primary_hue) => primary_hue.ordered_triplet(sum, c_prop),
            Self::Secondary(secondary_hue) => secondary_hue.ordered_triplet(sum, c_prop),
            Self::Sextant(sextant_hue) => sextant_hue.ordered_triplet(sum, c_prop),
        }
    }

    fn try_ordered_triplet(
        &self,
        sum: UFDRNum,
        c_prop: Prop,
    ) -> Option<Result<[Prop; 3], [Prop; 3]>> {
        match self {
            Self::Primary(primary_hue) => primary_hue.try_ordered_triplet(sum, c_prop),
            Self::Secondary(secondary_hue) => secondary_hue.try_ordered_triplet(sum, c_prop),
            Self::Sextant(sextant_hue) => sextant_hue.try_ordered_triplet(sum, c_prop),
        }
    }
}

impl ColourModificationHelpers for Hue {
    fn trim_overs(&self, sum: UFDRNum, c_prop: Prop) -> Option<(Prop, UFDRNum)> {
        match self {
            Self::Primary(primary_hue) => primary_hue.trim_overs(sum, c_prop),
            Self::Secondary(secondary_hue) => secondary_hue.trim_overs(sum, c_prop),
            Self::Sextant(sextant_hue) => sextant_hue.trim_overs(sum, c_prop),
        }
    }

    fn adjusted_favouring_chroma(&self, sum: UFDRNum, c_prop: Prop) -> Option<(Prop, UFDRNum)> {
        match self {
            Self::Primary(primary_hue) => primary_hue.adjusted_favouring_chroma(sum, c_prop),
            Self::Secondary(secondary_hue) => secondary_hue.adjusted_favouring_chroma(sum, c_prop),
            Self::Sextant(sextant_hue) => sextant_hue.adjusted_favouring_chroma(sum, c_prop),
        }
    }

    fn adjusted_favouring_sum(&self, sum: UFDRNum, c_prop: Prop) -> Option<(Prop, UFDRNum)> {
        match self {
            Self::Primary(primary_hue) => primary_hue.adjusted_favouring_sum(sum, c_prop),
            Self::Secondary(secondary_hue) => secondary_hue.adjusted_favouring_sum(sum, c_prop),
            Self::Sextant(sextant_hue) => sextant_hue.adjusted_favouring_sum(sum, c_prop),
        }
    }
}

impl HueIfce for Hue {
    fn angle(&self) -> Angle {
        match self {
            Self::Primary(rgb_hue) => rgb_hue.angle(),
            Self::Secondary(cmy_hue) => cmy_hue.angle(),
            Self::Sextant(sextant_hue) => sextant_hue.angle(),
        }
    }

    fn warmth(&self) -> Warmth {
        match self {
            Self::Primary(rgb_hue) => rgb_hue.warmth(),
            Self::Secondary(cmy_hue) => cmy_hue.warmth(),
            Self::Sextant(sextant_hue) => sextant_hue.warmth(),
        }
    }

    fn family(&self) -> Family {
        match self {
            Self::Primary(rgb_hue) => rgb_hue.family(),
            Self::Secondary(cmy_hue) => cmy_hue.family(),
            Self::Sextant(sextant_hue) => sextant_hue.family(),
        }
    }

    fn max_chroma_rgb<L: LightLevel>(&self) -> RGB<L> {
        match self {
            Self::Primary(primary_hue) => primary_hue.max_chroma_rgb::<L>(),
            Self::Secondary(secondary_hue) => secondary_hue.max_chroma_rgb::<L>(),
            Self::Sextant(sextant_hue) => sextant_hue.max_chroma_rgb::<L>(),
        }
    }

    fn max_chroma_hcv(&self) -> HCV {
        match self {
            Self::Primary(primary_hue) => primary_hue.max_chroma_hcv(),
            Self::Secondary(secondary_hue) => secondary_hue.max_chroma_hcv(),
            Self::Sextant(sextant_hue) => sextant_hue.max_chroma_hcv(),
        }
    }
}

#[cfg(test)]
impl HueTestIfce for Hue {
    #[cfg(test)]
    fn max_chroma_rgb_for_sum<T: LightLevel>(&self, sum: UFDRNum) -> Option<RGB<T>> {
        match self {
            Self::Primary(rgb_hue) => rgb_hue.max_chroma_rgb_for_sum(sum),
            Self::Secondary(cmy_hue) => cmy_hue.max_chroma_rgb_for_sum(sum),
            Self::Sextant(sextant_hue) => sextant_hue.max_chroma_rgb_for_sum(sum),
        }
    }

    #[cfg(test)]
    fn try_darkest_hcv_for_chroma(&self, chroma: Chroma) -> Option<Result<HCV, HCV>> {
        match self {
            Self::Primary(rgb_hue) => rgb_hue.try_darkest_hcv_for_chroma(chroma),
            Self::Secondary(cmy_hue) => cmy_hue.try_darkest_hcv_for_chroma(chroma),
            Self::Sextant(sextant_hue) => sextant_hue.try_darkest_hcv_for_chroma(chroma),
        }
    }

    #[cfg(test)]
    fn try_lightest_hcv_for_chroma(&self, chroma: Chroma) -> Option<Result<HCV, HCV>> {
        match self {
            Self::Primary(rgb_hue) => rgb_hue.try_lightest_hcv_for_chroma(chroma),
            Self::Secondary(cmy_hue) => cmy_hue.try_lightest_hcv_for_chroma(chroma),
            Self::Sextant(sextant_hue) => sextant_hue.try_lightest_hcv_for_chroma(chroma),
        }
    }

    #[cfg(test)]
    fn try_darkest_rgb_for_chroma<T: LightLevel>(
        &self,
        chroma: Chroma,
    ) -> Option<Result<RGB<T>, RGB<T>>> {
        match self {
            Self::Primary(rgb_hue) => rgb_hue.try_darkest_rgb_for_chroma(chroma),
            Self::Secondary(cmy_hue) => cmy_hue.try_darkest_rgb_for_chroma(chroma),
            Self::Sextant(sextant_hue) => sextant_hue.try_darkest_rgb_for_chroma(chroma),
        }
    }

    #[cfg(test)]
    fn try_lightest_rgb_for_chroma<T: LightLevel>(
        &self,
        chroma: Chroma,
    ) -> Option<Result<RGB<T>, RGB<T>>> {
        match self {
            Self::Primary(rgb_hue) => rgb_hue.try_lightest_rgb_for_chroma(chroma),
            Self::Secondary(cmy_hue) => cmy_hue.try_lightest_rgb_for_chroma(chroma),
            Self::Sextant(sextant_hue) => sextant_hue.try_lightest_rgb_for_chroma(chroma),
        }
    }

    #[cfg(test)]
    fn try_rgb_for_sum_and_chroma<T: LightLevel>(
        &self,
        sum: UFDRNum,
        chroma: Chroma,
    ) -> Option<Result<RGB<T>, RGB<T>>> {
        match self {
            Self::Primary(rgb_hue) => rgb_hue.try_rgb_for_sum_and_chroma(sum, chroma),
            Self::Secondary(cmy_hue) => cmy_hue.try_rgb_for_sum_and_chroma(sum, chroma),
            Self::Sextant(sextant_hue) => sextant_hue.try_rgb_for_sum_and_chroma(sum, chroma),
        }
    }
}

impl PropDiff for Hue {
    fn prop_diff(&self, other: &Self) -> Option<Prop> {
        match self {
            Self::Primary(self_hue) => match other {
                Self::Primary(other_hue) => self_hue.prop_diff(other_hue),
                Self::Sextant(sextant_hue) => self_hue.prop_diff_sextant(sextant_hue),
                _ => None,
            },
            Self::Secondary(self_hue) => match other {
                Self::Secondary(other_hue) => self_hue.prop_diff(other_hue),
                Self::Sextant(sextant_hue) => self_hue.prop_diff_sextant(sextant_hue),
                _ => None,
            },
            Self::Sextant(self_hue) => match other {
                Self::Sextant(other_hue) => self_hue.prop_diff(other_hue),
                Self::Primary(primary_hue) => primary_hue.prop_diff_sextant(self_hue),
                Self::Secondary(secondary_hue) => secondary_hue.prop_diff_sextant(self_hue),
            },
        }
    }
}

impl ApproxEq for Hue {}

impl Hue {
    pub fn ord_index(&self) -> u8 {
        0
    }
}

impl Add<Angle> for Hue {
    type Output = Self;

    fn add(self, angle: Angle) -> Self {
        Hue::from(self.angle().add(angle))
    }
}

impl Sub<Angle> for Hue {
    type Output = Self;

    fn sub(self, angle: Angle) -> Self {
        Hue::from(self.angle().sub(angle))
    }
}

impl Sub for Hue {
    type Output = Angle;

    fn sub(self, other: Self) -> Angle {
        self.angle().sub(other.angle())
    }
}

impl PartialOrd for Hue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ord_index().cmp(&other.ord_index())
    }
}

#[cfg(test)]
mod hue_tests;
