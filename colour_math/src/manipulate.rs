// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::cmp::Ordering;

use crate::fixed_denominator_rational_numbers::*;
use crate::{
    hue::{ColourModificationHelpers, HueBasics},
    Angle, ColourAttributes, FromProps, Hue, IntoHCV, IntoProps, IntoRGB, LightLevel, PropsFrom,
    Value, HCV, RGB,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SetScalarPolicy {
    Clamp,
    #[default]
    Accommodate,
    Reject,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SetHuePolicy {
    #[default]
    FavourChroma,
    FavourValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Outcome<T: Sized + Copy + PartialEq> {
    Ok(T),
    Clamped(T),
    Accommodated(T),
    NoChange,
    Rejected,
}

impl<T: Sized + Copy + PartialEq> Outcome<T> {
    pub fn expect(self, msg: &str) -> T {
        use Outcome::*;
        match self {
            Ok(t) | Clamped(t) | Accommodated(t) => t,
            _ => panic!("{msg}"),
        }
    }

    pub fn unwrap(self) -> T {
        use Outcome::*;
        match self {
            Ok(t) | Clamped(t) | Accommodated(t) => t,
            _ => panic!("attempt to extract content from rejected Outcome"),
        }
    }
}

pub trait ManipulatedColour: Sized + Copy + ColourAttributes + PartialEq {
    // TODO: modify Manipulated colour to make it more widely applicable
    fn lightened(self, prop: Prop) -> Self;
    fn darkened(self, prop: Prop) -> Self;
    fn saturated(self, prop: Prop) -> Self;
    fn greyed(self, prop: Prop) -> Self;
    fn rotated(self, angle: Angle) -> Self;

    fn try_incr_value(self, delta: Prop, policy: SetScalarPolicy) -> Outcome<Self>;
    fn try_decr_value(self, delta: Prop, policy: SetScalarPolicy) -> Outcome<Self>;
    fn try_incr_chroma_prop(
        self,
        delta: Prop,
        policy: SetScalarPolicy,
        new_hue: Hue,
    ) -> Outcome<Self>;
    fn try_decr_chroma_prop(self, delta: Prop, policy: SetScalarPolicy) -> Outcome<Self>;
    fn try_rotated(self, angle: Angle, policy: SetHuePolicy) -> Option<Self>;
}

impl ManipulatedColour for HCV {
    fn lightened(self, prop: Prop) -> Self {
        let compl = Prop::ONE - prop;
        let mut array = <[Prop; 3]>::proportions_from(self);
        for item in &mut array {
            *item = (*item * compl + prop).into_prop();
        }
        HCV::from_proportions(array)
    }

    fn darkened(self, prop: Prop) -> Self {
        let compl = Prop::ONE - prop;
        let mut array = <[Prop; 3]>::proportions_from(self);
        for item in &mut array {
            *item *= compl;
        }
        HCV::from_proportions(array)
    }

    fn saturated(self, prop: Prop) -> Self {
        if let Some(hue) = self.hue {
            let new_c_prop = (self.c_prop - self.c_prop * prop + prop).into_prop();
            let new_sum =
                if let Some((min_sum, max_sum)) = hue.sum_range_for_chroma_prop(new_c_prop) {
                    if self.sum < min_sum {
                        min_sum
                    } else if self.sum > max_sum {
                        max_sum
                    } else {
                        self.sum
                    }
                } else {
                    self.sum
                };
            if let Some((c_prop, sum)) = hue.adjusted_favouring_chroma(new_sum, new_c_prop) {
                // near enough is good enough
                match HCV::try_new(Some((hue, c_prop)), sum) {
                    Ok(hcv) => hcv,
                    Err(hcv) => hcv,
                }
            } else {
                HCV::new_grey((new_sum / 3u8).into())
            }
        } else {
            self
        }
    }

    fn greyed(self, prop: Prop) -> Self {
        if let Some(hue) = self.hue {
            let new_c_prop = self.c_prop - self.c_prop * prop;
            let new_sum =
                if let Some((min_sum, max_sum)) = hue.sum_range_for_chroma_prop(new_c_prop) {
                    if self.sum < min_sum {
                        min_sum
                    } else if self.sum > max_sum {
                        max_sum
                    } else {
                        self.sum
                    }
                } else {
                    self.sum
                };
            if let Some((c_prop, sum)) = hue.adjusted_favouring_chroma(new_sum, new_c_prop) {
                // near enough is good enough
                match HCV::try_new(Some((hue, c_prop)), sum) {
                    Ok(hcv) => hcv,
                    Err(hcv) => hcv,
                }
            } else {
                HCV::new_grey((new_sum / 3u8).into())
            }
        } else {
            self
        }
    }

    fn rotated(self, angle: Angle) -> Self {
        self + angle
    }

    /// Try to create an HCV with the same chroma and hue but with the value increased by the
    /// specified amount.
    /// If this is not possible and `policy` is `Clamp` return the nearest
    /// achievable result favouring `Chroma` else if `policy` is `Accommodate` return the
    /// nearest achievable result favouring `Value` else `Reject`. N.B. Clamping/accomodating
    /// may also involve slight `Hue` drift in addition to the `Chroma/Value` compromises.
    fn try_incr_value(self, delta: Prop, policy: SetScalarPolicy) -> Outcome<Self> {
        let new_sum = (self.sum + delta * 3u64).min(UFDRNum::THREE);
        if new_sum == self.sum {
            Outcome::NoChange
        } else if let Some(hue) = self.hue() {
            let max_sum = hue
                .max_sum_for_chroma(self.chroma())
                .expect("chroma is non zero");
            match max_sum.cmp(&new_sum) {
                Ordering::Equal | Ordering::Greater => Outcome::Ok(HCV {
                    hue: Some(hue),
                    c_prop: self.c_prop,
                    sum: new_sum,
                }),
                Ordering::Less => match policy {
                    SetScalarPolicy::Reject => Outcome::Rejected,
                    SetScalarPolicy::Clamp => {
                        if self.sum < max_sum {
                            let hcv = HCV {
                                hue: Some(hue),
                                c_prop: self.c_prop,
                                sum: max_sum,
                            };
                            Outcome::Clamped(hcv)
                        } else {
                            Outcome::NoChange
                        }
                    }
                    SetScalarPolicy::Accommodate => {
                        if self.sum == new_sum {
                            Outcome::NoChange
                        } else {
                            if let Some((c_prop, sum)) =
                                hue.adjusted_favouring_sum(new_sum, self.c_prop)
                            {
                                let hcv = HCV {
                                    hue: Some(hue),
                                    c_prop,
                                    sum,
                                };
                                debug_assert!(
                                    hcv.is_valid(),
                                    "{hcv:?} max {max_sum:?} new {new_sum:?}"
                                );
                                Outcome::Accommodated(hcv)
                            } else {
                                Outcome::Ok(HCV::new_grey(Value::from(new_sum / 3u64)))
                            }
                        }
                    }
                },
            }
        } else {
            Outcome::Ok(HCV::new_grey(Value::from(new_sum / 3u64)))
        }
    }

    /// Try to create an HCV with the same chroma and hue but with the value decreased by the
    /// specified amount.
    /// If this is not possible and `policy` is `Clamp` return the nearest
    /// achievable result favouring `Chroma` else if `policy` is `Accommodate` return the
    /// nearest achievable result favouring `Value` else `Reject`. N.B. Clamping/accomodating
    /// may also involve slight `Hue` drift in addition to the `Chroma/Value` compromises.
    fn try_decr_value(self, delta: Prop, policy: SetScalarPolicy) -> Outcome<Self> {
        if self.sum == UFDRNum::ZERO {
            return Outcome::NoChange;
        }
        let delta_sum = delta * 3u64;
        let new_sum = if delta_sum >= self.sum {
            UFDRNum::ZERO
        } else {
            self.sum - delta_sum
        };
        if new_sum == self.sum {
            Outcome::NoChange
        } else if let Some(hue) = self.hue() {
            let min_sum = hue
                .min_sum_for_chroma(self.chroma())
                .expect(&format!("chroma {:?} is non zero", self.chroma()));
            match min_sum.cmp(&new_sum) {
                Ordering::Equal | Ordering::Less => {
                    if new_sum == self.sum {
                        Outcome::NoChange
                    } else {
                        Outcome::Ok(HCV {
                            hue: Some(hue),
                            c_prop: self.c_prop,
                            sum: new_sum,
                        })
                    }
                }
                Ordering::Greater => match policy {
                    SetScalarPolicy::Reject => Outcome::Rejected,
                    SetScalarPolicy::Clamp => {
                        if min_sum == self.sum {
                            Outcome::NoChange
                        } else {
                            let hcv = HCV {
                                hue: Some(hue),
                                c_prop: self.c_prop,
                                sum: min_sum,
                            };
                            Outcome::Clamped(hcv)
                        }
                    }
                    SetScalarPolicy::Accommodate => {
                        let hcv = HCV {
                            hue: Some(hue),
                            c_prop: self.c_prop,
                            sum: new_sum,
                        };
                        Outcome::Accommodated(hcv)
                    }
                },
            }
        } else {
            let hcv = HCV::new_grey(Value::from(new_sum / 3u64));
            if hcv.sum == self.sum {
                Outcome::NoChange
            } else {
                Outcome::Ok(hcv)
            }
        }
    }

    /// Try to create an HCV with the same hue and value but with the chroma value increased by the
    /// specified amount.
    /// If this is not possible and `policy` is `Clamp` return the nearest
    /// achievable result favouring `Value` else if `policy` is `Accommodate` return the
    /// nearest achievable result favouring `Chroma` else `Reject`. N.B. Clamping/accomodating
    /// may also involve slight `Hue` drift in addition to the `Chroma/Value` compromises.
    fn try_incr_chroma_prop(
        self,
        delta: Prop,
        policy: SetScalarPolicy,
        new_hue: Hue,
    ) -> Outcome<Self> {
        if self.c_prop == Prop::ONE {
            return Outcome::NoChange;
        }
        let new_c_prop = if delta <= Prop::ONE - self.c_prop {
            Prop(self.c_prop.0 + delta.0)
        } else {
            Prop::ONE
        };
        if let Some(hue) = self.hue {
            let max_c_prop = hue
                .max_chroma_prop_for_sum(self.sum)
                .expect("sum is hue valid");
            if new_c_prop <= max_c_prop {
                let (c_prop, sum) = hue.adjusted_favouring_sum(self.sum, new_c_prop).unwrap();
                if c_prop == self.c_prop {
                    Outcome::NoChange
                } else {
                    let hcv = HCV {
                        hue: Some(hue),
                        c_prop,
                        sum,
                    };
                    debug_assert!(
                        hcv.is_valid(),
                        "{hcv:?} max {max_c_prop:?} new {new_c_prop:?}"
                    );
                    Outcome::Ok(hcv)
                }
            } else {
                match policy {
                    SetScalarPolicy::Clamp => {
                        if max_c_prop == self.c_prop {
                            Outcome::NoChange
                        } else {
                            let hcv = HCV {
                                hue: Some(hue),
                                c_prop: max_c_prop,
                                sum: self.sum,
                            };
                            debug_assert!(hcv.is_valid(), "{hcv:?}");
                            Outcome::Clamped(hcv)
                        }
                    }
                    SetScalarPolicy::Accommodate => {
                        let (min_sum, max_sum) = hue
                            .sum_range_for_chroma_prop(new_c_prop)
                            .expect("chroma {new_c_prop:? is non zero}");
                        let hcv = HCV {
                            hue: Some(hue),
                            c_prop: new_c_prop,
                            sum: self.sum.max(min_sum).min(max_sum),
                        };
                        debug_assert!(hcv.is_valid(), "{hcv:?}");
                        Outcome::Accommodated(hcv)
                    }
                    _ => Outcome::Rejected,
                }
            }
        } else {
            match policy {
                SetScalarPolicy::Accommodate => {
                    let (c_prop, sum) = new_hue
                        .adjusted_favouring_chroma(self.sum, new_c_prop)
                        .expect("chroma {new_c_prop:?} is non zero");
                    if c_prop == self.c_prop {
                        Outcome::NoChange
                    } else {
                        let hcv = HCV {
                            hue: Some(new_hue),
                            c_prop: new_c_prop,
                            sum: sum,
                        };
                        debug_assert!(hcv.is_valid(), "{hcv:?}");
                        Outcome::Accommodated(hcv)
                    }
                }
                SetScalarPolicy::Clamp => {
                    let sum = self.sum.adjusted_to_hue_valid_sum();
                    let c_prop = new_hue
                        .max_chroma_prop_for_sum(sum)
                        .expect("{sum:?} is a valid hue sum");
                    if c_prop == self.c_prop {
                        Outcome::NoChange
                    } else {
                        let hcv = HCV {
                            hue: Some(new_hue),
                            c_prop,
                            sum,
                        };
                        debug_assert!(hcv.is_valid(), "{hcv:?} sum = {sum:?}");
                        Outcome::Clamped(hcv)
                    }
                }
                SetScalarPolicy::Reject => Outcome::Rejected,
            }
        }
    }

    fn try_decr_chroma_prop(self, delta: Prop, policy: SetScalarPolicy) -> Outcome<Self> {
        let c_prop = if delta <= self.c_prop {
            self.c_prop - delta
        } else {
            Prop::ZERO
        };
        if c_prop == self.c_prop || self.c_prop == Prop::ZERO {
            return Outcome::NoChange;
        }
        if let Some(hue) = self.hue {
            if c_prop == Prop::ZERO {
                match policy {
                    SetScalarPolicy::Reject => Outcome::Rejected,
                    SetScalarPolicy::Clamp => {
                        if let Some((c_prop, sum)) =
                            hue.adjusted_favouring_sum(self.sum, Prop::ALMOST_ZERO)
                        {
                            if c_prop == self.c_prop {
                                Outcome::NoChange
                            } else {
                                let hcv = HCV {
                                    hue: Some(hue),
                                    c_prop,
                                    sum,
                                };
                                debug_assert!(hcv.is_valid());
                                Outcome::Clamped(hcv)
                            }
                        } else {
                            Outcome::Rejected
                        }
                    }
                    SetScalarPolicy::Accommodate => {
                        let hcv = HCV::new_grey(self.value());
                        debug_assert!(hcv.is_valid());
                        Outcome::Accommodated(hcv)
                    }
                }
            } else {
                let hcv = HCV {
                    hue: Some(hue),
                    c_prop,
                    sum: self.sum,
                };
                if hcv.is_valid() {
                    Outcome::Ok(hcv)
                } else {
                    match policy {
                        SetScalarPolicy::Reject => Outcome::Rejected,
                        SetScalarPolicy::Clamp => {
                            if let Some((c_prop, sum)) =
                                hue.adjusted_favouring_sum(self.sum, c_prop)
                            {
                                let hcv = HCV {
                                    hue: Some(hue),
                                    c_prop,
                                    sum,
                                };
                                debug_assert!(hcv.is_valid());
                                Outcome::Clamped(hcv)
                            } else {
                                Outcome::Rejected
                            }
                        }
                        SetScalarPolicy::Accommodate => {
                            if let Some((c_prop, sum)) =
                                hue.adjusted_favouring_chroma(self.sum, c_prop)
                            {
                                let hcv = HCV {
                                    hue: Some(hue),
                                    c_prop,
                                    sum,
                                };
                                debug_assert!(hcv.is_valid());
                                Outcome::Clamped(hcv)
                            } else {
                                Outcome::Rejected
                            }
                        }
                    }
                }
            }
        } else {
            Outcome::Rejected
        }
    }

    fn try_rotated(self, angle: Angle, policy: SetHuePolicy) -> Option<Self> {
        match self.hue_angle() {
            None => None,
            Some(cur_angle) => {
                let new_angle = cur_angle + angle;
                if new_angle == cur_angle {
                    None
                } else {
                    let hue = Hue::from(new_angle);
                    match policy {
                        SetHuePolicy::FavourChroma => {
                            if let Some((c_prop, sum)) =
                                hue.adjusted_favouring_chroma(self.sum, self.c_prop)
                            {
                                let hcv = HCV {
                                    hue: Some(hue),
                                    c_prop,
                                    sum,
                                };
                                debug_assert!(hcv.is_valid(), "Invalid HCV: {hcv:?}");
                                Some(hcv)
                            } else {
                                None
                            }
                        }
                        SetHuePolicy::FavourValue => {
                            if let Some((c_prop, sum)) =
                                hue.adjusted_favouring_sum(self.sum, self.c_prop)
                            {
                                let hcv = HCV {
                                    hue: Some(hue),
                                    c_prop,
                                    sum,
                                };
                                // debug_assert!(hcv.is_valid(), "Invalid HCV: {hcv:?}");
                                Some(hcv)
                            } else {
                                None
                            }
                        }
                    }
                }
            }
        }
    }
}

impl<L: LightLevel> ManipulatedColour for RGB<L> {
    fn lightened(self, prop: Prop) -> Self {
        let compl = Prop::ONE - prop;
        let mut array = self.into_proportions();
        for item in &mut array {
            *item = (*item * compl + prop).into_prop();
        }
        RGB::<L>::from_proportions(array)
    }

    fn darkened(self, prop: Prop) -> Self {
        let compl = Prop::ONE - prop;
        let mut array = self.into_proportions();
        for item in &mut array {
            *item *= compl;
        }
        RGB::<L>::from_proportions(array)
    }

    fn saturated(self, prop: Prop) -> Self {
        HCV::from(self).saturated(prop).into_rgb()
    }

    fn greyed(self, prop: Prop) -> Self {
        HCV::from(self).greyed(prop).into_rgb()
    }

    fn rotated(self, angle: Angle) -> Self {
        HCV::from(self).rotated(angle).into_rgb()
    }

    fn try_incr_value(self, delta: Prop, policy: SetScalarPolicy) -> Outcome<Self> {
        use Outcome::*;
        match self.into_hcv().try_incr_value(delta, policy) {
            Ok(hcv) => Ok(hcv.into_rgb::<L>()),
            Clamped(hcv) => Clamped(hcv.into_rgb::<L>()),
            Accommodated(hcv) => Accommodated(hcv.into_rgb::<L>()),
            NoChange => NoChange,
            Rejected => Rejected,
        }
    }

    fn try_decr_value(self, delta: Prop, policy: SetScalarPolicy) -> Outcome<Self> {
        use Outcome::*;
        match self.into_hcv().try_decr_value(delta, policy) {
            Ok(hcv) => Ok(hcv.into_rgb::<L>()),
            Clamped(hcv) => Clamped(hcv.into_rgb::<L>()),
            Accommodated(hcv) => Accommodated(hcv.into_rgb::<L>()),
            NoChange => NoChange,
            Rejected => Rejected,
        }
    }

    fn try_incr_chroma_prop(
        self,
        delta: Prop,
        policy: SetScalarPolicy,
        new_hue: Hue,
    ) -> Outcome<Self> {
        use Outcome::*;
        match self.into_hcv().try_incr_chroma_prop(delta, policy, new_hue) {
            Ok(hcv) => Ok(hcv.into_rgb::<L>()),
            Clamped(hcv) => Clamped(hcv.into_rgb::<L>()),
            Accommodated(hcv) => Accommodated(hcv.into_rgb::<L>()),
            NoChange => NoChange,
            Rejected => Rejected,
        }
    }

    fn try_decr_chroma_prop(self, delta: Prop, policy: SetScalarPolicy) -> Outcome<Self> {
        use Outcome::*;
        match self.into_hcv().try_decr_chroma_prop(delta, policy) {
            Ok(hcv) => Ok(hcv.into_rgb::<L>()),
            Clamped(hcv) => Clamped(hcv.into_rgb::<L>()),
            Accommodated(hcv) => Accommodated(hcv.into_rgb::<L>()),
            NoChange => NoChange,
            Rejected => Rejected,
        }
    }

    fn try_rotated(self, angle: Angle, policy: SetHuePolicy) -> Option<Self> {
        match self.into_hcv().try_rotated(angle, policy) {
            None => None,
            Some(hcv) => Some(hcv.into_rgb::<L>()),
        }
    }
}
