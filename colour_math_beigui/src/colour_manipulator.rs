// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use colour_math::fixed_denominator_rational_numbers::*;

use colour_math::{
    Colour, ColourAttributes, HueConstants, LightLevel,
    hcv::HCV,
    hue::Hue,
    hue::angle::Angle,
    manipulate::{ManipulatedColour, Outcome, SetHuePolicy, SetScalarPolicy},
    rgb::{IntoRGB, RGB},
};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum DeltaSize {
    Small,
    #[default]
    Normal,
    Large,
}

impl DeltaSize {
    pub fn for_value(self) -> Prop {
        match self {
            DeltaSize::Small => 0.0025.into_prop(),
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

    pub fn for_hue(self) -> Angle {
        match self {
            DeltaSize::Small => 0.5.into(),
            DeltaSize::Normal => 1.0.into(),
            DeltaSize::Large => 5.0.into(),
        }
    }
}

#[derive(Debug, Default)]
pub struct ColourManipulator {
    hcv: HCV,
    set_scalar_policy: SetScalarPolicy,
    rotation_policy: SetHuePolicy,
    delta_size: DeltaSize,
    saved_hue: Hue,
}

impl ColourManipulator {
    pub fn builder() -> ColourManipulatorBuilder {
        ColourManipulatorBuilder::default()
    }

    pub fn rgb(&self) -> RGB<u8> {
        self.hcv.into_rgb()
    }

    pub fn hcv(&self) -> HCV {
        self.hcv
    }

    pub fn set_colour(&mut self, colour: &impl Colour) {
        self.hcv = colour.into_hcv()
    }

    pub fn clamped(&self) -> bool {
        self.set_scalar_policy == SetScalarPolicy::Clamp
    }

    pub fn delta_size(&self) -> DeltaSize {
        self.delta_size
    }

    pub fn set_delta_size(&mut self, delta_size: DeltaSize) {
        self.delta_size = delta_size;
    }

    pub fn set_clamped(&mut self, clamped: bool) {
        if clamped {
            self.set_scalar_policy = SetScalarPolicy::Clamp;
        } else {
            self.set_scalar_policy = SetScalarPolicy::Accommodate;
        }
    }

    pub fn rotation_policy(&self) -> SetHuePolicy {
        self.rotation_policy
    }

    pub fn set_rotation_policy(&mut self, rotation_policy: SetHuePolicy) {
        self.rotation_policy = rotation_policy
    }

    pub fn decr_chroma(&mut self) -> bool {
        debug_assert!(self.hcv.is_valid());
        if let Some(hue) = self.hcv.hue() {
            self.saved_hue = hue
        };
        let delta = self.delta_size.for_chroma();
        match self.hcv.try_decr_chroma_prop(delta, self.set_scalar_policy) {
            Outcome::Ok(hcv) | Outcome::Clamped(hcv) | Outcome::Accommodated(hcv) => {
                self.hcv = hcv;
                true
            }
            _ => false,
        }
    }

    pub fn incr_chroma(&mut self) -> bool {
        debug_assert!(self.hcv.is_valid());
        let delta = self.delta_size.for_chroma();
        match self
            .hcv
            .try_incr_chroma_prop(delta, self.set_scalar_policy, self.saved_hue)
        {
            Outcome::Ok(hcv) | Outcome::Clamped(hcv) | Outcome::Accommodated(hcv) => {
                self.hcv = hcv;
                true
            }
            _ => false,
        }
    }

    pub fn decr_value(&mut self) -> bool {
        if let Some(hue) = self.hcv.hue() {
            self.saved_hue = hue
        };
        debug_assert!(self.hcv.is_valid());
        let delta = self.delta_size.for_value();
        match self.hcv.try_decr_value(delta, self.set_scalar_policy) {
            Outcome::Ok(hcv) | Outcome::Clamped(hcv) | Outcome::Accommodated(hcv) => {
                self.hcv = hcv;
                true
            }
            _ => false,
        }
    }

    pub fn incr_value(&mut self) -> bool {
        if let Some(hue) = self.hcv.hue() {
            self.saved_hue = hue
        };
        debug_assert!(self.hcv.is_valid());
        let delta = self.delta_size.for_value();
        match self.hcv.try_incr_value(delta, self.set_scalar_policy) {
            Outcome::Ok(hcv) | Outcome::Clamped(hcv) | Outcome::Accommodated(hcv) => {
                self.hcv = hcv;
                true
            }
            _ => false,
        }
    }

    pub fn rotate_clockwise(&mut self) -> bool {
        let angle = self.delta_size.for_hue();
        if let Some(hcv) = self.hcv.try_rotated(-angle, self.rotation_policy) {
            self.hcv = hcv;
            true
        } else {
            false
        }
    }

    pub fn rotate_anti_clockwise(&mut self) -> bool {
        let angle = self.delta_size.for_hue();
        if let Some(hcv) = self.hcv.try_rotated(angle, self.rotation_policy) {
            self.hcv = hcv;
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Default)]
pub struct ColourManipulatorBuilder {
    init_hcv: Option<HCV>,
    clamped: Option<bool>,
    rotation_policy: SetHuePolicy,
}

impl ColourManipulatorBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init_rgb<L: LightLevel>(&mut self, rgb: RGB<L>) -> &mut Self {
        self.init_hcv = Some(rgb.into());
        self
    }

    pub fn init_hcv(&mut self, hcv: &HCV) -> &mut Self {
        self.init_hcv = Some(*hcv);
        self
    }

    pub fn clamped(&mut self, clamped: bool) -> &mut Self {
        self.clamped = Some(clamped);
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
        let set_scalar_policy = if let Some(clamped) = self.clamped {
            if clamped {
                SetScalarPolicy::Clamp
            } else {
                SetScalarPolicy::Accommodate
            }
        } else {
            SetScalarPolicy::default()
        };
        ColourManipulator {
            hcv,
            saved_hue,
            set_scalar_policy,
            rotation_policy: self.rotation_policy,
            delta_size: DeltaSize::Normal,
        }
    }
}

// #[cfg(test)]
// mod colour_manipulator_tests;
