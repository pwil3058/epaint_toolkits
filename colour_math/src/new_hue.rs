// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::{cmp::Ordering, fmt::Debug};

use crate::fixed_denominator_rational_numbers::{self::*, debug::*};
use crate::{Angle, Chroma, Family, HueConstants, LightLevel, RGB, Warmth, ordered_triplet::*};

#[derive(Debug, PartialEq, Eq)]
pub enum HueResult<T: Sized + Clone + Copy + PartialEq + Eq + Debug> {
    Hued(T),
    Grey(T),
}

impl<T: Sized + Clone + Copy + PartialEq + Eq + Debug> HueResult<T> {
    fn hued(&self) -> Result<T, &'static str> {
        match self {
            Self::Hued(t) => Ok(*t),
            Self::Grey(_) => Err("attempt to get hued value from grey"),
        }
    }
}

impl<T: Sized + Clone + Copy + PartialEq + Eq + Debug + ApproxEq> ApproxEq for HueResult<T> {
    fn approx_eq(&self, other: &Self, limit: Option<u64>) -> bool {
        match self {
            Self::Hued(t) => match other {
                Self::Hued(o) => t.approx_eq(o, limit),
                _ => false,
            },
            Self::Grey(t) => match other {
                Self::Grey(o) => t.approx_eq(o, limit),
                _ => false,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// NB. These are in ascending order in agreement with Hue angle. So don't mess with them.
pub enum Sextant {
    BlueGreen,
    BlueRed,
    RedBlue,
    RedGreen,
    GreenRed,
    GreenBlue,
}

impl<L: LightLevel + Ord> TryFrom<RGB<L>> for Sextant {
    type Error = &'static str;

    fn try_from(rgb: RGB<L>) -> Result<Self, Self::Error> {
        println!("RGB: {rgb:?}");
        use Sextant::*;
        match rgb.red().cmp(&rgb.green()) {
            Ordering::Equal => match rgb.red().cmp(&rgb.blue()) {
                Ordering::Equal => Err("grey"),    // R == G == B
                Ordering::Less => Ok(BlueRed),     // B > R == G - Blue
                Ordering::Greater => Ok(RedGreen), // R == G > B - Yellow
            },
            Ordering::Less => match rgb.red().cmp(&rgb.blue()) {
                Ordering::Equal => Ok(GreenBlue), // G > R == B - Green
                Ordering::Less => {
                    // G > R && B > R
                    match rgb.green().cmp(&rgb.blue()) {
                        Ordering::Equal => Ok(GreenBlue),   // G == B > R - Cyan
                        Ordering::Less => Ok(BlueGreen),    // B > G > R - BG
                        Ordering::Greater => Ok(GreenBlue), // G > B > R - GB
                    }
                }
                Ordering::Greater => Ok(GreenRed), // G > R > B - GR
            },
            Ordering::Greater => match rgb.red().cmp(&rgb.blue()) {
                Ordering::Equal => Ok(RedBlue), // R == B > G - Magenta
                Ordering::Less => Ok(BlueRed),  // B > R > G - BR
                Ordering::Greater => {
                    // R > G && R > B
                    match rgb.green().cmp(&rgb.blue()) {
                        Ordering::Equal => Ok(RedBlue),    // R > G == B - Red
                        Ordering::Less => Ok(RedBlue),     // R > B > G - RB
                        Ordering::Greater => Ok(RedGreen), // R > G > B - RG
                    }
                }
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Hue {
    pub sextant: Sextant,
    pub param: Prop,
}

impl HueConstants for Hue {
    const RED: Self = Self {
        sextant: Sextant::RedBlue,
        param: Prop::ZERO,
    };
    const GREEN: Self = Self {
        sextant: Sextant::GreenRed,
        param: Prop::ZERO,
    };
    const BLUE: Self = Self {
        sextant: Sextant::BlueGreen,
        param: Prop::ZERO,
    };

    const CYAN: Self = Self {
        sextant: Sextant::GreenBlue,
        param: Prop::ONE,
    };
    const MAGENTA: Self = Self {
        sextant: Sextant::BlueRed,
        param: Prop::ONE,
    };
    const YELLOW: Self = Self {
        sextant: Sextant::RedGreen,
        param: Prop::ONE,
    };

    const BLUE_CYAN: Self = Self {
        sextant: Sextant::BlueGreen,
        param: Prop::HALF,
    };
    const BLUE_MAGENTA: Self = Self {
        sextant: Sextant::BlueRed,
        param: Prop::HALF,
    };
    const RED_MAGENTA: Self = Self {
        sextant: Sextant::RedBlue,
        param: Prop::HALF,
    };
    const RED_YELLOW: Self = Self {
        sextant: Sextant::RedGreen,
        param: Prop::HALF,
    };
    const GREEN_YELLOW: Self = Self {
        sextant: Sextant::GreenRed,
        param: Prop::HALF,
    };
    const GREEN_CYAN: Self = Self {
        sextant: Sextant::GreenBlue,
        param: Prop::HALF,
    };
}

impl PartialEq for Hue {
    fn eq(&self, other: &Self) -> bool {
        // We cant' gaurantee pure Red, Green, Blue, Cyan, Magenta and Yellow hues don't have different sextants
        use Sextant::*;
        match self.param {
            Prop::ZERO => {
                if other.param == Prop::ZERO {
                    match self.sextant {
                        RedGreen | RedBlue => [RedGreen, RedBlue].contains(&other.sextant),
                        GreenRed | GreenBlue => [GreenRed, GreenBlue].contains(&other.sextant),
                        BlueGreen | BlueRed => [BlueGreen, BlueRed].contains(&other.sextant),
                    }
                } else {
                    false
                }
            }
            Prop::ONE => {
                if other.param == Prop::ONE {
                    match self.sextant {
                        RedGreen | GreenRed => [RedGreen, GreenRed].contains(&other.sextant),
                        BlueGreen | GreenBlue => [BlueGreen, GreenBlue].contains(&other.sextant),
                        RedBlue | BlueRed => [RedBlue, BlueRed].contains(&other.sextant),
                    }
                } else {
                    false
                }
            }
            param => self.sextant == other.sextant && param == other.param,
        }
    }
}

impl Eq for Hue {}

impl ApproxEq for Hue {
    fn approx_eq(&self, other: &Self, limit: Option<u64>) -> bool {
        self.angle().approx_eq(&other.angle(), limit)
    }
}

impl Ord for Hue {
    fn cmp(&self, other: &Hue) -> Ordering {
        use Sextant::*;
        if self == other {
            Ordering::Equal
        } else {
            match self.sextant.cmp(&other.sextant) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => match self.sextant {
                    GreenBlue | RedGreen | BlueRed => self.param.cmp(&other.param),
                    GreenRed | RedBlue | BlueGreen => other.param.cmp(&self.param),
                },
            }
        }
    }
}

impl PartialOrd for Hue {
    fn partial_cmp(&self, other: &Hue) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<L: LightLevel + Ord> TryFrom<RGB<L>> for Hue {
    type Error = &'static str;

    /// Try to convert a `Hue` from an `RGB`. Fails if RGB is a grey.
    ///
    /// Examples:
    ///
    /// ```
    /// use colour_math::{new_hue::*, hue::angle::*, HueConstants, RGB};
    /// use colour_math::fixed_denominator_rational_numbers::*;
    ///
    /// assert_eq!(Hue::try_from(RGB::<Prop>::RED), Ok(Hue::RED));
    /// assert_eq!(Hue::try_from(RGB::<Prop>::GREEN), Ok(Hue::GREEN));
    /// assert_eq!(Hue::try_from(RGB::<Prop>::BLUE), Ok(Hue::BLUE));
    /// assert_eq!(Hue::try_from(RGB::<Prop>::CYAN), Ok(Hue::CYAN));
    /// assert_eq!(Hue::try_from(RGB::<Prop>::MAGENTA), Ok(Hue::MAGENTA));
    /// assert_eq!(Hue::try_from(RGB::<Prop>::YELLOW), Ok(Hue::YELLOW));
    /// assert_eq!(Hue::try_from(RGB::<Prop>::RED_YELLOW), Ok(Hue::RED_YELLOW));
    /// assert_eq!(Hue::try_from(RGB::<Prop>::GREEN_YELLOW), Ok(Hue::GREEN_YELLOW));
    /// assert_eq!(Hue::try_from(RGB::<Prop>::RED_MAGENTA), Ok(Hue::RED_MAGENTA));
    /// assert_eq!(Hue::try_from(RGB::<Prop>::BLUE_MAGENTA), Ok(Hue::BLUE_MAGENTA));
    /// assert_eq!(Hue::try_from(RGB::<Prop>::GREEN_CYAN), Ok(Hue::GREEN_CYAN));
    /// assert_eq!(Hue::try_from(RGB::<Prop>::BLUE_CYAN), Ok(Hue::BLUE_CYAN));
    ///```
    fn try_from(rgb: RGB<L>) -> Result<Hue, Self::Error> {
        let sextant = Sextant::try_from(rgb)?;
        let param = OrderedTriplet::from(rgb)
            .sextant_param()
            .expect("sextant exists");
        Ok(Self { sextant, param })
    }
}

impl Hue {
    /// Convert a `Hue` into an `Angle` from Red on the circular spectrum `+`ve towards Green and`-`ve towards Blue.
    ///
    /// Examples:
    ///
    /// ```
    /// use colour_math::{new_hue::*, hue::angle::*, HueConstants, debug::*};
    /// use approximately_equal::*;
    ///
    /// assert_eq!(Hue::RED.angle(), Angle::RED);
    /// assert_eq!(Hue::GREEN.angle(), Angle::GREEN);
    /// assert_eq!(Hue::BLUE.angle(), Angle::BLUE);
    /// assert_approx_eq!(Hue::CYAN.angle(), Angle::CYAN);
    /// assert_approx_eq!(Hue::MAGENTA.angle(), Angle::MAGENTA, 1000);
    /// assert_approx_eq!(Hue::YELLOW.angle(), Angle::YELLOW, 1000);
    /// ```
    pub fn angle(&self) -> Angle {
        let k: FDRNum = self.param.prop_into();
        let dividend = FDRNum::SQRT_3 * k / 2;
        let divisor = (FDRNum::ONE - k + k * k).sqrt();
        let angle = Angle::asin(dividend / divisor);
        match self.sextant {
            Sextant::BlueGreen => Angle::BLUE - angle,
            Sextant::BlueRed => Angle::BLUE + angle,
            Sextant::RedBlue => -angle,
            Sextant::RedGreen => angle,
            Sextant::GreenRed => Angle::GREEN - angle,
            Sextant::GreenBlue => Angle::GREEN + angle,
        }
    }

    pub fn warmth(&self) -> Warmth {
        use Sextant::*;
        let delta_warmth = Warmth::ONE_THIRD * self.param;
        match self.sextant {
            GreenBlue | BlueGreen => Warmth::BLUE - delta_warmth,
            GreenRed | BlueRed => Warmth::BLUE + delta_warmth,
            RedGreen | RedBlue => Warmth::RED - delta_warmth,
        }
    }

    pub fn family(&self) -> Family {
        use Sextant::*;
        match self.sextant {
            GreenBlue => {
                if self.param < Prop::HALF {
                    Family::Greens
                } else {
                    Family::Cyans
                }
            }
            BlueGreen => {
                if self.param < Prop::HALF {
                    Family::Blues
                } else {
                    Family::Cyans
                }
            }
            GreenRed => {
                if self.param < Prop::HALF {
                    Family::Greens
                } else {
                    Family::Yellows
                }
            }
            BlueRed => {
                if self.param < Prop::HALF {
                    Family::Blues
                } else {
                    Family::Magentas
                }
            }
            RedGreen => {
                if self.param < Prop::HALF {
                    Family::Reds
                } else {
                    Family::Yellows
                }
            }
            RedBlue => {
                if self.param < Prop::HALF {
                    Family::Reds
                } else {
                    Family::Magentas
                }
            }
        }
    }

    pub fn sum_for_max_chroma(&self) -> UFDRNum {
        UFDRNum::ONE + self.param
    }

    pub fn min_sum_for_chroma_prop(&self, c_prop: Prop) -> Option<UFDRNum> {
        match c_prop {
            Prop::ZERO => None,
            c_prop => Some(c_prop + self.param * c_prop),
        }
    }

    pub fn max_sum_for_chroma_prop(&self, c_prop: Prop) -> Option<UFDRNum> {
        match (UFDRNum::TWO - self.param) * c_prop {
            UFDRNum::ZERO => None,
            sum => Some(UFDRNum::THREE - sum),
        }
    }

    pub fn sum_range_for_chroma_prop(&self, c_prop: Prop) -> Option<(UFDRNum, UFDRNum)> {
        let min = self.min_sum_for_chroma_prop(c_prop)?;
        let max = self.max_sum_for_chroma_prop(c_prop)?;
        Some((min, max))
    }

    pub fn min_sum_for_chroma(&self, chroma: Chroma) -> Option<UFDRNum> {
        debug_assert!(chroma.is_valid());
        match chroma {
            Chroma::ZERO => None,
            Chroma::Neither(_) => Some(UFDRNum::ONE + self.param),
            Chroma::Shade(c_prop) => Some(c_prop + self.param * c_prop),
            Chroma::Tint(_) => Some(UFDRNum::ONE + self.param + UFDRNum(1)),
        }
    }

    pub fn max_sum_for_chroma(&self, chroma: Chroma) -> Option<UFDRNum> {
        debug_assert!(chroma.is_valid());
        match chroma {
            Chroma::ZERO => None,
            Chroma::Neither(_) => Some(UFDRNum::ONE + self.param),
            Chroma::Shade(_) => Some(UFDRNum::ONE + self.param - UFDRNum(1)),
            Chroma::Tint(c_prop) => self.max_sum_for_chroma_prop(c_prop),
        }
    }

    pub fn sum_range_for_chroma(&self, chroma: Chroma) -> Option<(UFDRNum, UFDRNum)> {
        debug_assert!(chroma.is_valid());
        let min = self.min_sum_for_chroma(chroma)?;
        let max = self.max_sum_for_chroma(chroma)?;
        Some((min, max))
    }

    pub fn max_chroma_prop_for_sum(&self, sum: UFDRNum) -> Option<Prop> {
        Some(self.max_chroma_for_sum(sum)?.into_prop())
    }

    pub fn max_chroma_for_sum(&self, sum: UFDRNum) -> Option<Chroma> {
        debug_assert!(sum.is_valid_sum(), "sum: {sum:?}");
        if sum.is_hue_valid() {
            match sum.cmp(&(UFDRNum::ONE + self.param)) {
                Ordering::Equal => Some(Chroma::ONE),
                Ordering::Less => {
                    let mut temp = sum / (UFDRNum::ONE + self.param);
                    // NB: check for and fix rounding error
                    while sum < (UFDRNum::ONE + self.param) * temp {
                        temp -= UFDRNum(1);
                    }
                    Some(Chroma::Shade(temp.into_prop()))
                }
                Ordering::Greater => {
                    // NB: make sure it doesn't round to one
                    let c_prop = ((UFDRNum::THREE - sum) / (UFDRNum::TWO - self.param))
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

impl Hue {
    /// Trim `sum` to the largest rounded value compatible with `c_prop` for this `Hue`.
    ///
    /// Examples:
    ///
    /// ```
    /// use approximately_equal::assert_approx_eq;
    /// use colour_math::new_hue::*;
    /// use colour_math::fixed_denominator_rational_numbers::{self::*, debug::*};
    /// use Sextant::*;
    ///
    /// assert_eq!(
    ///     Hue{sextant: RedGreen, param: Prop::ZERO}.adjusted_sum(UFDRNum::TWO, Prop::ONE),
    ///     HueResult::Hued(UFDRNum::ONE)
    /// ) ;
    /// assert_approx_eq!(
    ///     Hue{sextant: RedGreen, param: Prop::HALF}.adjusted_sum(UFDRNum::TWO, Prop::ONE),
    ///     HueResult::Hued(UFDRNum::from_rational_num(3, 2))
    /// ) ;
    /// ```
    pub fn adjusted_sum(self, sum: UFDRNum, c_prop: Prop) -> HueResult<UFDRNum> {
        debug_assert!(sum.is_valid_sum());
        if c_prop == Prop::ZERO {
            match sum % 3u8 {
                UFDRNum(1) => HueResult::Grey(sum - UFDRNum(1)),
                UFDRNum(2) => HueResult::Grey(sum + UFDRNum(1)),
                _ => HueResult::Grey(sum),
            }
        } else {
            let min_sum = self
                .min_sum_for_chroma_prop(c_prop)
                .expect("Chroma {c_prop:?} is not zero");
            match sum.cmp(&min_sum) {
                Ordering::Less => HueResult::Hued(min_sum),
                Ordering::Equal => HueResult::Hued(sum),
                Ordering::Greater => {
                    let max_sum = self
                        .max_sum_for_chroma_prop(c_prop)
                        .expect("Chroma {c_prop:?} is not zero");
                    if sum > max_sum {
                        HueResult::Hued(max_sum)
                    } else {
                        let third = (sum - min_sum) / 3u8;
                        let first = (third + c_prop).min(UFDRNum::ONE);
                        let second = (sum - first - third).min(UFDRNum::ONE);
                        debug_assert!(!(first == second && first == third));
                        HueResult::Hued(first + second + third)
                    }
                }
            }
        }
    }

    pub fn adjust_preferencing_chroma_prop(
        &self,
        c_prop: Prop,
        sum: UFDRNum,
    ) -> HueResult<(Prop, UFDRNum)> {
        use HueResult::*;
        debug_assert!(sum.is_valid_sum());
        match self.adjusted_sum(sum, c_prop) {
            Hued(adj_sum) => Hued((c_prop, adj_sum)),
            Grey(adj_sum) => Grey((Prop::ZERO, adj_sum)),
        }
    }

    pub fn adjust_preferencing_sum(
        &self,
        sum: UFDRNum,
        c_prop: Prop,
    ) -> HueResult<(Prop, UFDRNum)> {
        use HueResult::*;
        debug_assert!(sum.is_valid_sum());
        if let Some(max_c_prop) = self.max_chroma_prop_for_sum(sum)
            && c_prop != Prop::ZERO
        {
            let c_prop = c_prop.min(max_c_prop);
            let sum = self
                .adjusted_sum(sum, c_prop)
                .hued()
                .expect("c_prop is non zero");
            Hued((c_prop, sum))
        } else {
            Grey((c_prop, sum.as_valid_grey_sum()))
        }
    }
}
