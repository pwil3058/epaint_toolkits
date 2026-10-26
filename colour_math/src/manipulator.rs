// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use fixed_denominator_rational_numbers::*;

use crate::{
    hcv::HCV,
    hue::Hue,
    hue::{angle::Angle, ColourModificationHelpers, HueBasics, HueIfce, SumChromaCompatibility},
    rgb::RGB,
    ColourBasics, HueConstants, IsValidSum, LightLevel, Value,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetScalarPolicy {
    Clamp,
    Accommodate,
    Reject,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SetHuePolicy {
    #[default]
    FavourChroma,
    FavourValue,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DeltaSize {
    Small,
    Normal,
    Large,
}

impl DeltaSize {
    pub fn for_value(self) -> Prop {
        match self {
            DeltaSize::Small => 0.0025_f64.into_prop(),
            DeltaSize::Normal => 0.005.into_prop(),
            DeltaSize::Large => 0.01.into_prop(),
        }
    }

    pub fn for_chroma(self) -> Prop {
        match self {
            DeltaSize::Small => 0.0025.into_prop(),
            DeltaSize::Normal => 0.005.into_prop(),
            DeltaSize::Large => 0.01.into_prop(),
        }
    }

    pub fn for_hue_anticlockwise(self) -> Angle {
        match self {
            DeltaSize::Small => 0.5.into(),
            DeltaSize::Normal => 1.0.into(),
            DeltaSize::Large => 5.0.into(),
        }
    }

    pub fn for_hue_clockwise(self) -> Angle {
        -self.for_hue_anticlockwise()
    }
}

#[derive(Debug)]
pub enum Outcome {
    Ok,
    Clamped,
    Accommodated,
    NoChange,
    Rejected,
}

#[derive(Debug, Default)]
pub struct ColourManipulator {
    hcv: HCV,
    clamped: bool,
    rotation_policy: SetHuePolicy,
    saved_hue: Hue,
}

impl ColourManipulator {
    pub fn builder() -> ColourManipulatorBuilder {
        ColourManipulatorBuilder::default()
    }

    pub fn rgb<L: LightLevel>(&self) -> RGB<L> {
        self.hcv.rgb()
    }

    pub fn hcv(&self) -> HCV {
        self.hcv
    }

    pub fn set_colour(&mut self, colour: &impl ColourBasics) {
        self.hcv = colour.hcv()
    }

    pub fn clamped(&self) -> bool {
        self.clamped
    }

    pub fn set_clamped(&mut self, clamped: bool) {
        self.clamped = clamped
    }

    pub fn rotation_policy(&self) -> SetHuePolicy {
        self.rotation_policy
    }

    pub fn set_rotation_policy(&mut self, rotation_policy: SetHuePolicy) {
        self.rotation_policy = rotation_policy
    }

    pub fn set_chroma(&mut self, new_c_prop: Prop, policy: SetScalarPolicy) -> Outcome {
        debug_assert!(self.hcv.is_valid());
        // let cur_c_prop = self.hcv.c_prop;
        if let Some(hue) = self.hcv.hue {
            self.saved_hue = hue; // Just in case we end up grey (which is possible)
            if hue.sum_and_chroma_prop_are_compatible(self.hcv.sum, new_c_prop) {
                let new_hcv = match HCV::try_new(Some((hue, new_c_prop)), self.hcv.sum) {
                    Ok(hcv) => hcv,
                    Err(hcv) => hcv,
                };
                if new_hcv.c_prop == self.hcv.c_prop {
                    Outcome::NoChange
                } else {
                    self.hcv = new_hcv;
                    Outcome::Ok
                }
            } else {
                match policy {
                    SetScalarPolicy::Clamp => {
                        if let Some(max_c_prop) = hue.max_chroma_prop_for_sum(self.hcv.sum) {
                            let clamped_new_c_prop = if new_c_prop < max_c_prop {
                                new_c_prop
                            } else if self.hcv.c_prop < max_c_prop {
                                max_c_prop
                            } else {
                                return Outcome::NoChange;
                            };
                            let new_hcv = if let Some((c_prop, sum)) =
                                hue.adjusted_favouring_sum(self.hcv.sum, clamped_new_c_prop)
                            {
                                if c_prop == Prop::ZERO {
                                    HCV::new_grey(Value::from_as_prop(sum / 3u8))
                                } else {
                                    match HCV::try_new(Some((hue, c_prop)), sum) {
                                        Ok(hcv) => hcv,
                                        Err(hcv) => hcv,
                                    }
                                }
                            } else {
                                HCV::new_grey(Value::from_as_prop(self.hcv.sum / 3u8))
                            };
                            if new_hcv.c_prop == self.hcv.c_prop {
                                Outcome::NoChange
                            } else {
                                self.hcv = new_hcv;
                                Outcome::Clamped
                            }
                        } else {
                            Outcome::NoChange
                        }
                    }
                    SetScalarPolicy::Accommodate => {
                        let new_hcv = if let Some((c_prop, sum)) =
                            hue.adjusted_favouring_chroma(self.hcv.sum, new_c_prop)
                        {
                            if c_prop == Prop::ZERO {
                                HCV::new_grey(Value::from_as_prop(self.hcv.sum / 3u8))
                            } else {
                                match HCV::try_new(Some((hue, c_prop)), sum) {
                                    Ok(hcv) => hcv,
                                    Err(hcv) => hcv,
                                }
                            }
                        } else {
                            HCV::new_grey(Value::from_as_prop(self.hcv.sum / 3u8))
                        };
                        if new_hcv.c_prop == self.hcv.c_prop {
                            Outcome::NoChange
                        } else {
                            self.hcv = new_hcv;
                            Outcome::Accommodated
                        }
                    }
                    SetScalarPolicy::Reject => Outcome::Rejected,
                }
            }
        } else if new_c_prop == Prop::ZERO {
            Outcome::NoChange
        } else if self
            .saved_hue
            .sum_and_chroma_prop_are_compatible(self.hcv.sum, new_c_prop)
        {
            let new_hcv = match HCV::try_new(Some((self.saved_hue, new_c_prop)), self.hcv.sum) {
                Ok(hcv) => hcv,
                Err(hcv) => hcv,
            };
            if new_hcv.c_prop == self.hcv.c_prop {
                Outcome::NoChange
            } else {
                self.hcv = new_hcv;
                Outcome::Ok
            }
        } else {
            match policy {
                SetScalarPolicy::Clamp => {
                    if let Some(max_c_prop) = self.saved_hue.max_chroma_prop_for_sum(self.hcv.sum) {
                        let clamped_new_c_prop = if new_c_prop < max_c_prop {
                            new_c_prop
                        } else if self.hcv.c_prop < max_c_prop {
                            max_c_prop
                        } else {
                            return Outcome::NoChange;
                        };
                        if let Some((c_prop, sum)) = self
                            .saved_hue
                            .adjusted_favouring_sum(self.hcv.sum, clamped_new_c_prop)
                        {
                            let new_hcv = match HCV::try_new(Some((self.saved_hue, c_prop)), sum) {
                                Ok(hcv) => hcv,
                                Err(hcv) => hcv,
                            };
                            if new_hcv.c_prop == self.hcv.c_prop {
                                Outcome::NoChange
                            } else {
                                self.hcv = new_hcv;
                                Outcome::Clamped
                            }
                        } else {
                            Outcome::NoChange
                        }
                    } else {
                        Outcome::NoChange
                    }
                }
                SetScalarPolicy::Accommodate => {
                    let new_hcv = if let Some((c_prop, sum)) = self
                        .saved_hue
                        .adjusted_favouring_chroma(self.hcv.sum, new_c_prop)
                    {
                        match HCV::try_new(Some((self.saved_hue, c_prop)), sum) {
                            Ok(hcv) => hcv,
                            Err(hcv) => hcv,
                        }
                    } else {
                        HCV::new_grey(Value::from_as_prop(self.hcv.sum / 3u8))
                    };
                    if new_hcv.c_prop == self.hcv.c_prop {
                        Outcome::NoChange
                    } else {
                        self.hcv = new_hcv;
                        Outcome::Accommodated
                    }
                }
                SetScalarPolicy::Reject => Outcome::Rejected,
            }
        }
    }

    pub fn decr_chroma(&mut self, delta: Prop) -> bool {
        debug_assert!(self.hcv.is_valid());
        match self.hcv.c_prop {
            Prop::ZERO => false,
            c_prop => {
                let new_c_prop = if c_prop > delta {
                    c_prop - delta
                } else {
                    Prop::ZERO
                };
                let policy = if self.clamped {
                    SetScalarPolicy::Clamp
                } else {
                    SetScalarPolicy::Accommodate
                };
                match self.set_chroma(new_c_prop, policy) {
                    Outcome::Ok | Outcome::Clamped | Outcome::Accommodated => {
                        debug_assert!(self.hcv.is_valid());
                        true
                    }
                    _ => false,
                }
            }
        }
    }

    pub fn incr_chroma(&mut self, delta: Prop) -> bool {
        debug_assert!(self.hcv.is_valid());
        let policy = if self.clamped {
            SetScalarPolicy::Clamp
        } else {
            SetScalarPolicy::Accommodate
        };
        match self.hcv.c_prop {
            Prop::ONE => false,
            Prop::ZERO => match self.set_chroma(delta, policy) {
                Outcome::Ok | Outcome::Clamped | Outcome::Accommodated => {
                    debug_assert!(self.hcv.is_valid());
                    true
                }
                _ => false,
            },
            c_prop => {
                let new_c_prop = if c_prop < Prop::ONE - delta {
                    (c_prop + delta).into_prop()
                } else {
                    Prop::ONE
                };
                match self.set_chroma(new_c_prop, policy) {
                    Outcome::Ok | Outcome::Clamped | Outcome::Accommodated => {
                        debug_assert!(self.hcv.is_valid());
                        true
                    }
                    _ => false,
                }
            }
        }
    }

    pub fn set_sum(&mut self, new_sum: UFDRNum, policy: SetScalarPolicy) -> Outcome {
        debug_assert!(self.hcv.is_valid());
        debug_assert!(new_sum.is_valid_sum());
        if let Some(hue) = self.hcv.hue {
            self.saved_hue = hue;
            if hue.sum_and_chroma_prop_are_compatible(new_sum, self.hcv.c_prop) {
                let new_hcv = match HCV::try_new(Some((hue, self.hcv.c_prop)), new_sum) {
                    Ok(hcv) => hcv,
                    Err(hcv) => hcv,
                };
                if new_hcv.sum == self.hcv.sum {
                    Outcome::NoChange
                } else {
                    self.hcv = new_hcv;
                    Outcome::Ok
                }
            } else {
                match policy {
                    SetScalarPolicy::Clamp => {
                        let new_hcv = if let Some((c_prop, sum)) =
                            hue.adjusted_favouring_chroma(new_sum, self.hcv.c_prop)
                        {
                            match HCV::try_new(Some((hue, c_prop)), sum) {
                                Ok(hcv) => hcv,
                                Err(hcv) => hcv,
                            }
                        } else {
                            HCV::new_grey(Value::from_as_prop(new_sum / 3u8))
                        };
                        if new_hcv.sum == self.hcv.sum {
                            Outcome::NoChange
                        } else {
                            self.hcv = new_hcv;
                            Outcome::Clamped
                        }
                    }
                    SetScalarPolicy::Accommodate => {
                        let new_hcv = if let Some((c_prop, sum)) =
                            hue.adjusted_favouring_sum(new_sum, self.hcv.c_prop)
                        {
                            match HCV::try_new(Some((hue, c_prop)), sum) {
                                Ok(hcv) => hcv,
                                Err(hcv) => hcv,
                            }
                        } else {
                            HCV::new_grey(Value::from_as_prop(new_sum / 3u8))
                        };
                        if new_hcv.sum == self.hcv.sum {
                            Outcome::NoChange
                        } else {
                            self.hcv = new_hcv;
                            Outcome::Accommodated
                        }
                    }
                    SetScalarPolicy::Reject => Outcome::Rejected,
                }
            }
        } else {
            let new_hcv = HCV::new_grey(Value::from_as_prop(new_sum / 3u8));
            if new_hcv.sum == self.hcv.sum {
                Outcome::NoChange
            } else {
                self.hcv = new_hcv;
                Outcome::Ok
            }
        }
    }

    pub fn decr_value(&mut self, delta: Prop) -> bool {
        debug_assert!(self.hcv.is_valid());
        if self.hcv.sum == UFDRNum::ZERO {
            false
        } else {
            let new_sum = if delta * 3u64 < self.hcv.sum {
                self.hcv.sum - delta * 3u64
            } else {
                UFDRNum::ZERO
            };
            let policy = if self.clamped {
                SetScalarPolicy::Clamp
            } else {
                SetScalarPolicy::Accommodate
            };
            match self.set_sum(new_sum, policy) {
                Outcome::Ok | Outcome::Clamped | Outcome::Accommodated => {
                    debug_assert!(self.hcv.is_valid());
                    true
                }
                _ => false,
            }
        }
    }

    pub fn incr_value(&mut self, delta: Prop) -> bool {
        debug_assert!(self.hcv.is_valid());
        if self.hcv.sum == UFDRNum::THREE {
            false
        } else {
            let new_sum = if delta * 3u64 < UFDRNum::THREE - self.hcv.sum {
                self.hcv.sum + delta * 3u64
            } else {
                UFDRNum::THREE
            };
            let policy = if self.clamped {
                SetScalarPolicy::Clamp
            } else {
                SetScalarPolicy::Accommodate
            };
            match self.set_sum(new_sum, policy) {
                Outcome::Ok | Outcome::Clamped | Outcome::Accommodated => {
                    debug_assert!(self.hcv.is_valid());
                    true
                }
                _ => false,
            }
        }
    }

    pub fn set_hue(&mut self, new_hue: Hue, policy: SetHuePolicy) -> bool {
        // TODO: change argument to Option<Hue>
        debug_assert!(self.hcv.is_valid());
        let result = match self.hcv.c_prop {
            Prop::ZERO => {
                self.saved_hue = new_hue;
                false
            }
            Prop::ONE => match policy {
                SetHuePolicy::FavourChroma => {
                    let new_hcv = new_hue.max_chroma_hcv();
                    if new_hcv.hue() == self.hcv.hue() {
                        false
                    } else {
                        self.hcv = new_hcv;
                        true
                    }
                }
                SetHuePolicy::FavourValue => {
                    let c_prop = new_hue.max_chroma_prop_for_sum(self.hcv.sum).unwrap();
                    let (c_prop, sum) = new_hue
                        .adjusted_favouring_sum(self.hcv.sum, c_prop)
                        .unwrap();
                    let new_hcv = match HCV::try_new(Some((new_hue, c_prop)), sum) {
                        Ok(hcv) => hcv,
                        Err(hcv) => hcv,
                    };
                    if new_hcv.hue() == self.hcv.hue() {
                        false
                    } else {
                        self.hcv = new_hcv;
                        true
                    }
                }
            },
            c_prop => {
                if new_hue.sum_and_chroma_prop_are_compatible(self.hcv.sum, c_prop) {
                    let new_hcv = match HCV::try_new(Some((new_hue, c_prop)), self.hcv.sum) {
                        Ok(hcv) => hcv,
                        Err(hcv) => hcv,
                    };
                    if new_hcv.hue() == self.hcv.hue() {
                        false
                    } else {
                        self.hcv = new_hcv;
                        true
                    }
                } else {
                    let new_hcv = match policy {
                        SetHuePolicy::FavourChroma => {
                            if let Some((c_prop, sum)) = if let Some((min_sum, max_sum)) =
                                new_hue.sum_range_for_chroma_prop(c_prop)
                            {
                                if self.hcv.sum < min_sum {
                                    new_hue.trim_overs(min_sum + UFDRNum(2), c_prop)
                                } else if self.hcv.sum > max_sum {
                                    new_hue.trim_overs(max_sum, c_prop)
                                } else {
                                    new_hue.trim_overs(self.hcv.sum, c_prop)
                                }
                            } else {
                                new_hue.trim_overs(self.hcv.sum, c_prop)
                            } {
                                match HCV::try_new(Some((new_hue, c_prop)), sum) {
                                    Ok(hcv) => hcv,
                                    Err(hcv) => hcv,
                                }
                            } else {
                                self.saved_hue = new_hue;
                                HCV::new_grey(Value::from_as_prop(self.hcv.sum / 3u8))
                            }
                        }
                        SetHuePolicy::FavourValue => {
                            let max_c_prop = new_hue
                                .max_chroma_prop_for_sum(self.hcv.sum)
                                .expect("0.0 < sum < 3.0");
                            let c_prop = if self.hcv.c_prop < max_c_prop {
                                self.hcv.c_prop
                            } else {
                                max_c_prop
                            };
                            let (c_prop, sum) = new_hue
                                .adjusted_favouring_sum(self.hcv.sum, c_prop)
                                .unwrap();
                            match HCV::try_new(Some((new_hue, c_prop)), sum) {
                                Ok(hcv) => hcv,
                                Err(hcv) => hcv,
                            }
                        }
                    };
                    if new_hcv.hue() == self.hcv.hue() {
                        false
                    } else {
                        self.hcv = new_hcv;
                        true
                    }
                }
            }
        };
        debug_assert!(self.hcv.is_valid());
        result
    }

    pub fn rotate(&mut self, angle: Angle) -> bool {
        match self.hcv.hue_angle() {
            None => false,
            Some(cur_angle) => {
                let new_angle = cur_angle + angle;
                if new_angle == cur_angle {
                    false
                } else {
                    if !self.set_hue(Hue::from(new_angle), self.rotation_policy) {
                        false
                    } else {
                        // NB: set_hue() doesn't guarantee hue won't be None
                        if let Some(new_hue) = self.hcv.hue() {
                            self.saved_hue = new_hue;
                        };
                        true
                    }
                }
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct ColourManipulatorBuilder {
    init_hcv: Option<HCV>,
    clamped: bool,
    rotation_policy: SetHuePolicy,
}

impl ColourManipulatorBuilder {
    pub fn new() -> Self {
        Self {
            init_hcv: None,
            clamped: false,
            rotation_policy: SetHuePolicy::FavourChroma,
        }
    }

    pub fn init_rgb<L: LightLevel>(&mut self, rgb: &RGB<L>) -> &mut Self {
        self.init_hcv = Some(rgb.into());
        self
    }

    pub fn init_hcv(&mut self, hcv: &HCV) -> &mut Self {
        self.init_hcv = Some(*hcv);
        self
    }

    pub fn clamped(&mut self, clamped: bool) -> &mut Self {
        self.clamped = clamped;
        self
    }

    pub fn rotation_policy(&mut self, rotation_policy: SetHuePolicy) -> &mut Self {
        self.rotation_policy = rotation_policy;
        self
    }

    pub fn build(&self) -> ColourManipulator {
        let hcv = self.init_hcv.unwrap_or_default();
        let saved_hue = if let Some(hue) = hcv.hue() {
            hue
        } else {
            Hue::RED
        };
        ColourManipulator {
            hcv,
            saved_hue,
            clamped: self.clamped,
            rotation_policy: self.rotation_policy,
        }
    }
}

#[cfg(test)]
mod manipulator_tests;
