// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.
use std::{
    cmp::Ordering,
    convert::TryFrom,
    ops::{Add, Sub},
};

use serde::{Deserialize, Serialize};

use crate::fixed_denominator_rational_numbers::{
    AsProportion, IntoProp, NumberConstants, Prop, UFDRNum,
};

use crate::cmy::CMY;
use crate::{
    attributes::{Chroma, Family, Value, Warmth},
    hue::{
        angle::Angle, CMYHue, ColourModificationHelpers, Hue, HueBasics, HueIfce, OrderedTriplets,
        RGBHue, Sextant, SumChromaCompatibility,
    },
    rgb::{IntoRGB, RGB},
    ColourBasics, FromProps, HueConstants, IntoProps, LightLevel, ManipulatedColour, PropsFrom,
    RGBConstants,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct HCV {
    pub(crate) hue: Option<Hue>,
    pub(crate) c_prop: Prop,
    pub(crate) sum: UFDRNum,
}

impl HCV {
    pub(crate) fn try_new(hue_data: Option<(Hue, Prop)>, sum: UFDRNum) -> Result<Self, Self> {
        debug_assert!(sum.is_valid_sum());
        if let Some((hue, c_prop)) = hue_data {
            debug_assert!(hue.sum_in_chroma_prop_range(sum, c_prop) || c_prop == Prop::ZERO);
            match hue.try_rgb_ordered_triplet(sum, c_prop) {
                Some(result) => match result {
                    Ok(_) => Ok(Self {
                        hue: Some(hue),
                        c_prop,
                        sum,
                    }),
                    Err(triplet) => Err(HCV::from_proportions(triplet)),
                },
                None => {
                    debug_assert!(c_prop == Prop::ZERO && sum % 3u8 == UFDRNum::ZERO);
                    Ok(Self {
                        hue: None,
                        c_prop: Prop::ZERO,
                        sum,
                    })
                }
            }
        } else {
            debug_assert!(sum % 3u8 == UFDRNum::ZERO);
            Ok(Self {
                hue: None,
                c_prop: Prop::ZERO,
                sum,
            })
        }
    }

    pub(crate) fn new_grey_sum(sum: UFDRNum) -> Self {
        debug_assert!(sum % 3u8 == UFDRNum::ZERO);
        Self {
            hue: None,
            c_prop: Prop::ZERO,
            sum,
        }
    }

    pub fn new_grey(value: Value) -> Self {
        Self::new_grey_sum(value * 3)
    }

    pub fn is_grey(&self) -> bool {
        self.c_prop == Prop::ZERO
    }

    pub(crate) fn is_valid(&self) -> bool {
        if let Some(hue) = self.hue {
            hue.sum_and_chroma_prop_are_compatible(self.sum, self.c_prop)
        } else {
            self.c_prop == Prop::ZERO && self.sum <= UFDRNum::THREE && self.sum.0.is_multiple_of(3)
        }
    }

    pub fn sum_range_for_current_chroma(&self) -> (UFDRNum, UFDRNum) {
        if let Some(hue) = self.hue {
            if let Some(range) = hue.sum_range_for_chroma(self.chroma()) {
                range
            } else {
                (UFDRNum::ZERO, UFDRNum::THREE)
            }
        } else {
            (UFDRNum::ZERO, UFDRNum::THREE)
        }
    }

    pub fn sum_range_for_current_chroma_prop(&self) -> (UFDRNum, UFDRNum) {
        if let Some(hue) = self.hue {
            if let Some(range) = hue.sum_range_for_chroma_prop(self.c_prop) {
                range
            } else {
                (UFDRNum::ZERO, UFDRNum::THREE)
            }
        } else {
            (UFDRNum::ZERO, UFDRNum::THREE)
        }
    }

    pub fn max_chroma_prop_for_current_sum(&self) -> Prop {
        if let Some(hue) = self.hue {
            if let Some(max_c_prop) = hue.max_chroma_prop_for_sum(self.sum) {
                max_c_prop
            } else {
                Prop::ZERO
            }
        } else {
            Prop::ZERO
        }
    }
}

const ONE_PT_5: UFDRNum = UFDRNum(u64::MAX as u128 + u64::MAX as u128 / 2);

impl HueConstants for HCV {
    const RED: Self = Self {
        hue: Some(Hue::RED),
        c_prop: Prop::ONE,
        sum: UFDRNum::ONE,
    };

    const GREEN: Self = Self {
        hue: Some(Hue::GREEN),
        c_prop: Prop::ONE,
        sum: UFDRNum::ONE,
    };

    const BLUE: Self = Self {
        hue: Some(Hue::BLUE),
        c_prop: Prop::ONE,
        sum: UFDRNum::ONE,
    };

    const CYAN: Self = Self {
        hue: Some(Hue::CYAN),
        c_prop: Prop::ONE,
        sum: UFDRNum::TWO,
    };

    const MAGENTA: Self = Self {
        hue: Some(Hue::MAGENTA),
        c_prop: Prop::ONE,
        sum: UFDRNum::TWO,
    };

    const YELLOW: Self = Self {
        hue: Some(Hue::YELLOW),
        c_prop: Prop::ONE,
        sum: UFDRNum::TWO,
    };

    const BLUE_CYAN: Self = Self {
        hue: Some(Hue::BLUE_CYAN),
        c_prop: Prop::ONE,
        sum: ONE_PT_5,
    };

    const BLUE_MAGENTA: Self = Self {
        hue: Some(Hue::BLUE_MAGENTA),
        c_prop: Prop::ONE,
        sum: ONE_PT_5,
    };

    const RED_MAGENTA: Self = Self {
        hue: Some(Hue::RED_MAGENTA),
        c_prop: Prop::ONE,
        sum: ONE_PT_5,
    };

    const RED_YELLOW: Self = Self {
        hue: Some(Hue::RED_YELLOW),
        c_prop: Prop::ONE,
        sum: ONE_PT_5,
    };

    const GREEN_YELLOW: Self = Self {
        hue: Some(Hue::GREEN_YELLOW),
        c_prop: Prop::ONE,
        sum: ONE_PT_5,
    };

    const GREEN_CYAN: Self = Self {
        hue: Some(Hue::GREEN_CYAN),
        c_prop: Prop::ONE,
        sum: ONE_PT_5,
    };
}

impl RGBConstants for HCV {
    const WHITE: Self = Self {
        hue: None,
        c_prop: Prop::ZERO,
        sum: UFDRNum::THREE,
    };

    const LIGHT_GREY: Self = Self {
        hue: None,
        c_prop: Prop::ZERO,
        sum: UFDRNum(u64::MAX as u128 / 4 * 3),
    };

    const MEDIUM_GREY: Self = Self {
        hue: None,
        c_prop: Prop::ZERO,
        sum: UFDRNum(u64::MAX as u128 + u64::MAX as u128 / 6 * 3),
    };

    const DARK_GREY: Self = Self {
        hue: None,
        c_prop: Prop::ZERO,
        sum: UFDRNum(u64::MAX as u128 * 2 + u64::MAX as u128 / 12 * 3),
    };

    const BLACK: Self = Self {
        hue: None,
        c_prop: Prop::ZERO,
        sum: UFDRNum::ZERO,
    };
}

fn sum_and_chroma_prop((hue, [red, green, blue]): (Hue, [Prop; 3])) -> (UFDRNum, Prop) {
    let sum = red + green + blue;
    let c_prop = match hue {
        Hue::Primary(RGBHue::Red) => red - blue,
        Hue::Primary(RGBHue::Green) => green - red,
        Hue::Primary(RGBHue::Blue) => blue - green,
        Hue::Secondary(CMYHue::Cyan) => blue - red,
        Hue::Secondary(CMYHue::Magenta) => red - green,
        Hue::Secondary(CMYHue::Yellow) => green - blue,
        Hue::Sextant(sextant_hue) => match sextant_hue.sextant() {
            Sextant::RedYellow => red - blue,
            Sextant::RedMagenta => red - green,
            Sextant::GreenYellow => green - blue,
            Sextant::GreenCyan => green - red,
            Sextant::BlueCyan => blue - red,
            Sextant::BlueMagenta => blue - green,
        },
    };
    (sum, c_prop)
}

impl<L: LightLevel> From<RGB<L>> for HCV {
    fn from(rgb: RGB<L>) -> Self {
        Self::from_proportions(rgb.into_proportions())
    }
}

// impl<L: LightLevel> From<HCV> for RGB<L> {
//     fn from(hcv: HCV) -> Self {
//         RGB::<L>::from_proportions(<[Prop; 3]>::proportions_from(hcv))
//     }
// }

impl IntoRGB for HCV {
    fn into_rgb<L: LightLevel>(self) -> RGB<L> {
        self.into_proportions().into_rgb::<L>()
    }
}

impl ColourBasics for HCV {
    fn hue(&self) -> Option<Hue> {
        self.hue
    }

    fn hue_angle(&self) -> Option<Angle> {
        self.hue.map(|hue| hue.angle())
    }

    fn family(&self) -> Option<Family> {
        Some(self.hue()?.family())
    }

    fn is_grey(&self) -> bool {
        self.c_prop == Prop::ZERO
    }

    fn chroma(&self) -> Chroma {
        match self.c_prop {
            Prop::ZERO => {
                debug_assert_eq!(self.hue, None);
                Chroma::ZERO
            }
            _ => match self.hue {
                Some(hue) => match self.sum.cmp(&hue.sum_for_max_chroma()) {
                    Ordering::Less => Chroma::Shade(self.c_prop),
                    Ordering::Greater => Chroma::Tint(self.c_prop),
                    Ordering::Equal => Chroma::Neither(self.c_prop),
                },
                None => {
                    debug_assert_eq!(self.c_prop, Prop::ZERO);
                    Chroma::ZERO
                }
            },
        }
    }

    fn chroma_prop(&self) -> Prop {
        self.c_prop
    }

    fn value(&self) -> Value {
        (self.sum / 3u8).into()
    }

    fn warmth(&self) -> Warmth {
        if let Some(hue) = self.hue {
            hue.warmth()
        } else {
            Warmth::HALF
        }
    }

    fn rgb<L: LightLevel>(&self) -> RGB<L> {
        (*self).into_rgb()
    }

    fn hcv(&self) -> HCV {
        *self
    }

    fn cmy<L: LightLevel>(&self) -> CMY<L> {
        self.rgb::<L>().into()
    }
}

impl IntoProps for HCV {
    fn proportions(self) -> impl Iterator<Item = Prop> {
        debug_assert!(self.is_valid());
        if let Some(hue) = self.hue {
            hue.rgb_ordered_triplet(self.sum, self.c_prop)
                .expect("Invalid Hue")
                .into_iter()
        } else {
            let value: Prop = (self.sum / 3u8).into_prop();
            [value, value, value].into_iter()
        }
    }
}

impl FromProps for HCV {
    fn from_proportions(props: [Prop; 3]) -> Self {
        if let Ok(hue) = Hue::try_from(props) {
            let (sum, c_prop) = sum_and_chroma_prop((hue, props));
            Self {
                hue: Some(hue),
                c_prop,
                sum,
            }
        } else {
            Self {
                hue: None,
                c_prop: Prop::ZERO,
                sum: props[0] + props[1] + props[2],
            }
        }
    }
}

impl ManipulatedColour for HCV {
    fn lightened(&self, prop: Prop) -> Self {
        let compl = Prop::ONE - prop;
        let mut array = <[Prop; 3]>::proportions_from(*self);
        for item in &mut array {
            *item = (*item * compl + prop).into_prop();
        }
        HCV::from_proportions(array)
    }

    fn darkened(&self, prop: Prop) -> Self {
        let compl = Prop::ONE - prop;
        let mut array = <[Prop; 3]>::proportions_from(*self);
        for item in &mut array {
            *item = *item * compl;
        }
        HCV::from_proportions(array)
    }

    fn saturated(&self, prop: Prop) -> Self {
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
            *self
        }
    }

    fn greyed(&self, prop: Prop) -> Self {
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
            *self
        }
    }

    fn rotated(&self, angle: Angle) -> Self {
        *self + angle
    }
}

impl Add<Angle> for HCV {
    type Output = Self;

    fn add(self, angle: Angle) -> Self {
        if let Some(hue) = self.hue {
            let new_hue = hue + angle;
            if let Some((c_prop, sum)) = new_hue.adjusted_favouring_chroma(self.sum, self.c_prop) {
                // near enough is good enough
                match HCV::try_new(Some((new_hue, c_prop)), sum) {
                    Ok(hcv) => hcv,
                    Err(hcv) => hcv,
                }
            } else {
                HCV::new_grey_sum(self.sum)
            }
        } else {
            self
        }
    }
}

impl Sub<Angle> for HCV {
    type Output = Self;

    fn sub(self, angle: Angle) -> Self {
        if let Some(hue) = self.hue {
            let new_hue = hue - angle;
            if let Some((c_prop, sum)) = new_hue.adjusted_favouring_chroma(self.sum, self.c_prop) {
                // near enough is good enough
                match HCV::try_new(Some((new_hue, c_prop)), sum) {
                    Ok(hcv) => hcv,
                    Err(hcv) => hcv,
                }
            } else {
                HCV::new_grey_sum(self.sum)
            }
        } else {
            self
        }
    }
}

#[cfg(test)]
mod hcv_tests;
