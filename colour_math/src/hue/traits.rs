// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::{
    cmp::Ordering,
    convert::{Into, TryFrom},
    fmt::Debug,
};

use crate::fixed_denominator_rational_numbers::*;

use crate::{
    attributes::{Chroma, Family, Warmth},
    hcv::HCV,
    hue::{self::*, angle::Angle},
    rgb::RGB,
    ColourBasics, LightLevel,
};

pub trait HueBasics: Copy + Debug + Sized + Into<Hue> {
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
                    debug_assert_eq!(first - third, UFDRNum::from_prop(c_prop));
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

#[cfg(test)]
use crate::FromProps;

pub trait HueIfce: HueBasics {
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
}

#[cfg(test)]
pub(crate) trait HueTestIfce:
    HueBasics + OrderedTriplets + ColourModificationHelpers + SumChromaCompatibility
{
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
            Ok(triplet) => Some(Ok(HCV::from_proportions(triplet))),
            Err(triplet) => Some(Err(HCV::from_proportions(triplet))),
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
