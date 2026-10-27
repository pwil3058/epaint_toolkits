// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::{
    cmp::Ordering,
    convert::{From, Into, TryFrom},
    fmt::Debug,
    ops::{Add, Sub},
};

use serde::{Deserialize, Serialize};

pub mod angle;

use crate::fixed_denominator_rational_numbers::{
    debug::{AbsDiff, ApproxEq, PropDiff},
    AsProportion, FDRNum, IntoProp, NumberConstants, Prop, UFDRNum,
};

use crate::{
    attributes::{Chroma, Family, Warmth},
    hcv::HCV,
    hue::angle::Angle,
    rgb::RGB,
    ColourBasics, HueConstants, LightLevel,
};

pub(crate) trait HueBasics: Copy + Debug + Sized + Into<Hue> {
    fn sum_for_max_chroma(&self) -> UFDRNum;

    fn min_sum_for_chroma_prop(&self, c_prop: Prop) -> Option<UFDRNum>;

    fn max_sum_for_chroma_prop(&self, c_prop: Prop) -> Option<UFDRNum>;

    fn sum_range_for_chroma_prop(&self, c_prop: Prop) -> Option<(UFDRNum, UFDRNum)> {
        let min = self.min_sum_for_chroma_prop(c_prop)?;
        let max = self.max_sum_for_chroma_prop(c_prop)?;
        Some((min, max))
    }

    fn min_sum_for_chroma(&self, chroma: Chroma) -> Option<UFDRNum>;

    fn max_sum_for_chroma(&self, chroma: Chroma) -> Option<UFDRNum>;

    fn sum_range_for_chroma(&self, chroma: Chroma) -> Option<(UFDRNum, UFDRNum)> {
        debug_assert!(chroma.is_valid());
        let min = self.min_sum_for_chroma(chroma)?;
        let max = self.max_sum_for_chroma(chroma)?;
        Some((min, max))
    }

    fn max_chroma_prop_for_sum(&self, sum: UFDRNum) -> Option<Prop> {
        Some(self.max_chroma_for_sum(sum)?.into_prop())
    }

    fn max_chroma_for_sum(&self, sum: UFDRNum) -> Option<Chroma>;
}

pub(crate) trait SumChromaCompatibility: HueBasics {
    fn sum_in_chroma_prop_range(&self, sum: UFDRNum, c_prop: Prop) -> bool {
        if let Some((min_sum, max_sum)) = self.sum_range_for_chroma_prop(c_prop) {
            sum >= min_sum && sum <= max_sum
        } else {
            false
        }
    }

    fn sum_and_chroma_prop_are_compatible(&self, sum: UFDRNum, c_prop: Prop) -> bool {
        if let Some((min_sum, max_sum)) = self.sum_range_for_chroma_prop(c_prop) {
            sum >= min_sum
                && sum <= max_sum
                && (sum - self.sum_for_max_chroma() * c_prop) % 3u8 == UFDRNum::ZERO
        } else {
            false
        }
    }
}

pub(crate) trait OrderedTriplets: HueBasics + SumChromaCompatibility {
    fn has_valid_value_order(triplet: &[Prop; 3]) -> bool;

    fn triplet_to_rgb_order(&self, triplet: &[Prop; 3]) -> [Prop; 3];

    fn rgb_ordered_triplet(&self, sum: UFDRNum, c_prop: Prop) -> Option<[Prop; 3]> {
        let triplet = self.ordered_triplet(sum, c_prop)?;
        debug_assert!(Self::has_valid_value_order(&triplet));
        Some(self.triplet_to_rgb_order(&triplet))
    }

    fn try_rgb_ordered_triplet(
        &self,
        sum: UFDRNum,
        c_prop: Prop,
    ) -> Option<Result<[Prop; 3], [Prop; 3]>> {
        match self.try_ordered_triplet(sum, c_prop)? {
            Ok(triplet) => {
                debug_assert!(Self::has_valid_value_order(&triplet));
                Some(Ok(self.triplet_to_rgb_order(&triplet)))
            }
            Err(triplet) => Some(Err(self.triplet_to_rgb_order(&triplet))),
        }
    }

    fn ordered_triplet(&self, sum: UFDRNum, c_prop: Prop) -> Option<[Prop; 3]> {
        match self.sum_for_max_chroma() * c_prop {
            min_sum if sum >= min_sum => {
                debug_assert!(self.sum_and_chroma_prop_are_compatible(sum, c_prop));
                let third = (sum - min_sum) / 3u8;
                let first = third + c_prop;
                if first.is_proportion() {
                    let second = sum - first - third;
                    debug_assert_eq!(first + second + third, sum);
                    debug_assert_eq!(first - third, c_prop.into());
                    let triplet = [first.into_prop(), second.into_prop(), third.into_prop()];
                    debug_assert!(Self::has_valid_value_order(&triplet));
                    debug_assert_eq!(
                        Hue::try_from(self.triplet_to_rgb_order(&triplet)).unwrap(),
                        (*self).into()
                    );
                    Some(triplet)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn try_ordered_triplet(
        &self,
        sum: UFDRNum,
        c_prop: Prop,
    ) -> Option<Result<[Prop; 3], [Prop; 3]>>;
}

pub(crate) trait HueIfce:
    HueBasics + OrderedTriplets + ColourModificationHelpers + SumChromaCompatibility
{
    fn angle(&self) -> Angle;

    fn warmth(&self) -> Warmth;

    fn family(&self) -> Family;

    fn max_chroma_rgb<T: LightLevel>(&self) -> RGB<T> {
        self.max_chroma_hcv().rgb::<T>()
    }

    fn max_chroma_hcv(&self) -> HCV {
        HCV {
            hue: Some((*self).into()),
            c_prop: Prop::ONE,
            sum: self.sum_for_max_chroma(),
        }
    }

    #[cfg(test)]
    fn try_max_chroma_rgb_for_sum<L: LightLevel>(
        &self,
        sum: UFDRNum,
    ) -> Option<Result<RGB<L>, RGB<L>>> {
        debug_assert!(sum.is_valid_sum(), "sum: {sum:?}");
        match sum {
            UFDRNum::ZERO | UFDRNum::THREE => None,
            sum => {
                let max_c_prop = self.max_chroma_prop_for_sum(sum)?;
                match self.try_rgb_ordered_triplet(sum, max_c_prop)? {
                    Ok(triplet) => Some(Ok(RGB::<L>::from_proportions(triplet))),
                    Err(triplet) => Some(Err(RGB::<L>::from_proportions(triplet))),
                }
            }
        }
    }

    #[cfg(test)]
    fn max_chroma_rgb_for_sum<L: LightLevel>(&self, sum: UFDRNum) -> Option<RGB<L>> {
        match self.try_max_chroma_rgb_for_sum::<L>(sum)? {
            Ok(rgb) => Some(rgb),
            Err(rgb) => Some(rgb),
        }
    }

    #[cfg(test)]
    fn try_darkest_hcv_for_chroma(&self, chroma: Chroma) -> Option<Result<HCV, HCV>> {
        debug_assert!(chroma.is_valid());
        let sum = self.min_sum_for_chroma(chroma)?;
        self.try_hcv_for_sum_and_chroma_prop(sum, chroma.into_prop())
    }

    #[cfg(test)]
    fn try_lightest_hcv_for_chroma(&self, chroma: Chroma) -> Option<Result<HCV, HCV>> {
        debug_assert!(chroma.is_valid());
        let sum = self.max_sum_for_chroma(chroma)?;
        self.try_hcv_for_sum_and_chroma_prop(sum, chroma.into_prop())
    }

    #[cfg(test)]
    fn try_darkest_rgb_for_chroma<T: LightLevel>(
        &self,
        chroma: Chroma,
    ) -> Option<Result<RGB<T>, RGB<T>>> {
        debug_assert!(chroma.is_valid());
        let sum = self.min_sum_for_chroma(chroma)?;
        self.try_rgb_for_sum_and_chroma_prop(sum, chroma.into_prop())
    }

    #[cfg(test)]
    fn try_lightest_rgb_for_chroma<T: LightLevel>(
        &self,
        chroma: Chroma,
    ) -> Option<Result<RGB<T>, RGB<T>>> {
        debug_assert!(chroma.is_valid());
        let sum = self.max_sum_for_chroma(chroma)?;
        self.try_rgb_for_sum_and_chroma_prop(sum, chroma.into_prop())
    }

    #[cfg(test)]
    fn try_hcv_for_sum_and_chroma_prop(
        &self,
        sum: UFDRNum,
        c_prop: Prop,
    ) -> Option<Result<HCV, HCV>> {
        debug_assert!(sum.is_valid_sum());
        match self.try_rgb_ordered_triplet(sum, c_prop)? {
            Ok(triplet) => Some(Ok(HCV::from(triplet))),
            Err(triplet) => Some(Err(HCV::from(triplet))),
        }
    }

    #[cfg(test)]
    fn try_rgb_for_sum_and_chroma_prop<T: LightLevel>(
        &self,
        sum: UFDRNum,
        c_prop: Prop,
    ) -> Option<Result<RGB<T>, RGB<T>>> {
        debug_assert!(sum.is_valid_sum());
        match self.try_rgb_ordered_triplet(sum, c_prop)? {
            Ok(triplet) => Some(Ok(RGB::<T>::from_proportions(triplet))),
            Err(triplet) => Some(Err(RGB::<T>::from_proportions(triplet))),
        }
    }

    #[cfg(test)]
    fn try_rgb_for_sum_and_chroma<T: LightLevel>(
        &self,
        sum: UFDRNum,
        chroma: Chroma,
    ) -> Option<Result<RGB<T>, RGB<T>>> {
        debug_assert!(chroma.is_valid() && sum.is_valid_sum());
        let (min_sum, max_sum) = self.sum_range_for_chroma(chroma)?;
        match sum {
            sum if sum < min_sum || sum > max_sum => None,
            sum => match chroma.into_prop() {
                Prop::ZERO => None,
                c_prop => self.try_rgb_for_sum_and_chroma_prop(sum, c_prop),
            },
        }
    }
}

pub(crate) trait ColourModificationHelpers: HueBasics + Debug + Sized {
    fn trim_overs(&self, sum: UFDRNum, c_prop: Prop) -> Option<(Prop, UFDRNum)> {
        debug_assert!(sum.is_valid_sum());
        match self.sum_for_max_chroma() * c_prop {
            UFDRNum::ZERO => None,
            min_sum if sum < min_sum => Some((c_prop, min_sum)),
            min_sum => match sum - (sum - min_sum) % 3u8 {
                UFDRNum::ZERO => None,
                sum => Some((c_prop, sum)),
            },
        }
    }

    fn adjusted_favouring_chroma(&self, sum: UFDRNum, c_prop: Prop) -> Option<(Prop, UFDRNum)> {
        debug_assert!(sum.is_valid_sum());
        match c_prop {
            Prop::ZERO => None,
            Prop::ONE => Some((Prop::ONE, self.sum_for_max_chroma())),
            c_prop => {
                let (c_prop, sum) = self.trim_overs(sum, c_prop)?;
                match sum.cmp(&self.sum_for_max_chroma()) {
                    Ordering::Equal | Ordering::Less => Some((c_prop, sum)),
                    Ordering::Greater => {
                        let max_sum = self.max_sum_for_chroma_prop(c_prop).expect("chroma > 0");
                        if sum > max_sum {
                            Some(self.trim_overs(max_sum, c_prop)?)
                        } else {
                            Some((c_prop, sum))
                        }
                    }
                }
            }
        }
    }

    fn adjusted_favouring_sum(&self, sum: UFDRNum, c_prop: Prop) -> Option<(Prop, UFDRNum)> {
        debug_assert!(sum.is_valid_sum());
        match c_prop {
            Prop::ZERO => None,
            c_prop => match sum.cmp(&self.sum_for_max_chroma()) {
                Ordering::Equal => Some(self.trim_overs(sum, c_prop)?),
                Ordering::Less => {
                    let min_sum = self.min_sum_for_chroma_prop(c_prop).expect("c_prop > 0");
                    if sum < min_sum {
                        let max_chroma = self.max_chroma_prop_for_sum(sum)?;
                        Some(self.trim_overs(sum, max_chroma)?)
                    } else {
                        Some(self.trim_overs(sum, c_prop)?)
                    }
                }
                Ordering::Greater => {
                    let max_chroma = self.max_chroma_prop_for_sum(sum)?;
                    if c_prop > max_chroma {
                        Some(self.trim_overs(sum, max_chroma)?)
                    } else {
                        Some(self.trim_overs(sum, c_prop)?)
                    }
                }
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord)]
pub enum RGBHue {
    Red = 5,
    Green = 9,
    Blue = 1,
}

impl RGBHue {
    pub const HUES: [Self; 3] = [RGBHue::Blue, RGBHue::Red, RGBHue::Green];

    fn prop_diff_sextant(&self, sextant_hue: &SextantHue) -> Option<Prop> {
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
            c_prop => Some(c_prop.into()),
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
            Chroma::Shade(c_prop) => Some(c_prop.into()),
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
            min_sum if sum >= min_sum.into() => {
                let third = (sum - min_sum) / 3u8;
                match third + c_prop {
                    first if first > UFDRNum::ONE => None,
                    first => {
                        let second = sum - first - third;
                        debug_assert_eq!(first - third, c_prop.into());
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord)]
pub enum CMYHue {
    Cyan = 113,
    Magenta = 3,
    Yellow = 7,
}

impl CMYHue {
    pub const HUES: [Self; 3] = [CMYHue::Magenta, CMYHue::Yellow, CMYHue::Cyan];

    fn prop_diff_sextant(&self, sextant_hue: &SextantHue) -> Option<Prop> {
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
                                    debug_assert_eq!(second - third, c_prop.into());
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
                                    debug_assert_eq!(first - third, c_prop.into());
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
pub struct SextantHue(Sextant, Prop);

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
                                debug_assert_eq!(first - third, c_prop.into());
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
                                debug_assert_eq!(first - third, c_prop.into());
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
                                    debug_assert_eq!(second - third, c_prop.into());
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
            && (first + second + third - sum_for_max_chroma * (first - third)) / 3u8 < third.into()
        {
            sum_for_max_chroma -= UFDRNum(1);
        }
        debug_assert!(sum_for_max_chroma > UFDRNum::ONE && sum_for_max_chroma < UFDRNum::TWO);
        debug_assert_eq!(
            (first + second + third - sum_for_max_chroma * (first - third)) / 3u8,
            third.into()
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
                let second: f64 = self.1.into();
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
        // Self::try_from(<[Prop; 3]>::from(*rgb))
        Self::try_from(rgb.into_proportions())
    }
}

impl<T: LightLevel> TryFrom<RGB<T>> for Hue {
    type Error = &'static str;

    fn try_from(rgb: RGB<T>) -> Result<Self, Self::Error> {
        // Self::try_from(<[Prop; 3]>::from(rgb))
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
                    (angle.sin() / (Angle::GREEN - angle).sin()).into()
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
