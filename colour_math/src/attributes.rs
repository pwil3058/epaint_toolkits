// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::{
    cmp::Ordering,
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

use serde::{Deserialize, Serialize};

use fixed_denominator_rational_numbers::{
    debug::{AbsDiff, ApproxEq, PropDiff},
    impl_into_as_prop, impl_into_from_as_prop_number, AsProportion, FDRNum, FromAsProp, IntoAsProp,
    Prop, UFDRNum,
};

use crate::HueConstants;
use crate::{
    // debug::{AbsDiff, ApproxEq, PropDiff},
    // fdrn::{FDRNum, IntoProp, Prop, UFDRNum},
    hue::{Hue, HueBasics},
    IsValidSum,
};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Chroma {
    Shade(Prop),
    Tint(Prop),
    Neither(Prop),
}

impl Chroma {
    pub const ZERO: Self = Self::Neither(Prop::ZERO);
    pub const ONE: Self = Self::Neither(Prop::ONE);

    pub fn is_zero(self) -> bool {
        self.into_as_prop() == Prop::ZERO
    }

    pub fn is_shade(self) -> bool {
        matches!(self, Chroma::Shade(_))
    }

    pub fn is_tint(self) -> bool {
        matches!(self, Chroma::Tint(_))
    }

    pub fn is_neither(self) -> bool {
        matches!(self, Chroma::Neither(_))
    }

    pub fn is_valid(self) -> bool {
        match self {
            Chroma::Neither(_) => true,
            Chroma::Shade(c_prop) | Chroma::Tint(c_prop) => {
                c_prop > Prop::ZERO && c_prop < Prop::ONE
            }
        }
    }

    pub fn is_valid_re(self, hue: Hue, sum: UFDRNum) -> bool {
        debug_assert!(sum.is_valid_sum());
        match self {
            Chroma::Neither(_) => sum == hue.sum_for_max_chroma(),
            Chroma::Shade(c_prop) => {
                c_prop > Prop::ZERO && c_prop < Prop::ONE && sum < hue.sum_for_max_chroma()
            }
            Chroma::Tint(c_prop) => {
                c_prop > Prop::ZERO && c_prop < Prop::ONE && sum > hue.sum_for_max_chroma()
            }
        }
    }

    pub fn abs_diff(&self, other: &Self) -> Prop {
        self.into_as_prop().abs_diff(&other.into_as_prop())
    }
}

impl FromAsProp<Chroma> for Prop {
    fn from_as_prop(chroma: Chroma) -> Prop {
        use Chroma::*;
        match chroma {
            Shade(proportion) | Tint(proportion) | Neither(proportion) => proportion,
        }
    }
}
impl_into_as_prop!(Prop, Chroma);

impl Default for Chroma {
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<(Prop, Hue, UFDRNum)> for Chroma {
    fn from((prop, hue, sum): (Prop, Hue, UFDRNum)) -> Self {
        match prop {
            Prop::ZERO => Chroma::ZERO,
            Prop::ONE => Chroma::ONE,
            prop => match sum.cmp(&hue.sum_for_max_chroma()) {
                Ordering::Greater => Self::Tint(prop),
                Ordering::Less => Self::Shade(prop),
                Ordering::Equal => Self::Neither(prop),
            },
        }
    }
}

impl PartialOrd for Chroma {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Chroma {
    fn cmp(&self, rhs: &Self) -> Ordering {
        use Chroma::*;
        match self {
            Shade(proportion) => match rhs {
                Shade(other_proportion) => proportion.cmp(other_proportion),
                _ => Ordering::Less,
            },
            Tint(proportion) => match rhs {
                Tint(other_proportion) => proportion.cmp(other_proportion),
                Shade(_) => Ordering::Greater,
                Neither(_) => Ordering::Less,
            },
            Neither(proportion) => match rhs {
                Neither(other_proportion) => proportion.cmp(other_proportion),
                _ => Ordering::Greater,
            },
        }
    }
}

impl PropDiff for Chroma {
    fn prop_diff(&self, other: &Self) -> Option<Prop> {
        use Chroma::*;
        match self {
            Shade(my_prop) => match other {
                Shade(other_prop) => my_prop.prop_diff(other_prop),
                _ => None,
            },
            Tint(my_prop) => match other {
                Tint(other_prop) => my_prop.prop_diff(other_prop),
                _ => None,
            },
            Neither(my_prop) => match other {
                Neither(other_prop) => my_prop.prop_diff(other_prop),
                _ => None,
            },
        }
    }
}

impl ApproxEq for Chroma {}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Greyness {
    Shade(Prop),
    Tint(Prop),
    Neither(Prop),
}

impl Greyness {
    pub const ZERO: Self = Self::Neither(Prop::ZERO);
    pub const ONE: Self = Self::Neither(Prop::ONE);

    pub fn is_shade(self) -> bool {
        matches!(self, Greyness::Shade(_))
    }

    pub fn is_tint(self) -> bool {
        matches!(self, Greyness::Tint(_))
    }

    pub fn is_neither(self) -> bool {
        matches!(self, Greyness::Neither(_))
    }

    pub fn is_zero(&self) -> bool {
        self.into_as_prop() == Prop::ZERO
    }

    pub fn abs_diff(&self, other: &Self) -> Prop {
        self.into_as_prop().abs_diff(&other.into_as_prop())
    }
}

impl FromAsProp<Greyness> for Prop {
    fn from_as_prop(greyness: Greyness) -> Prop {
        use Greyness::*;
        match greyness {
            Shade(proportion) | Tint(proportion) | Neither(proportion) => proportion,
        }
    }
}
impl_into_as_prop!(Prop, Greyness);

impl Default for Greyness {
    fn default() -> Self {
        Self::ZERO
    }
}

impl PartialOrd for Greyness {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Greyness {
    fn cmp(&self, rhs: &Self) -> Ordering {
        use Greyness::*;
        match self {
            Shade(proportion) => match rhs {
                Shade(other_proportion) => proportion.cmp(other_proportion),
                _ => Ordering::Less,
            },
            Tint(proportion) => match rhs {
                Tint(other_proportion) => proportion.cmp(other_proportion),
                Shade(_) => Ordering::Greater,
                Neither(_) => Ordering::Less,
            },
            Neither(proportion) => match rhs {
                Neither(other_proportion) => proportion.cmp(other_proportion),
                _ => Ordering::Greater,
            },
        }
    }
}

impl PropDiff for Greyness {
    fn prop_diff(&self, other: &Self) -> Option<Prop> {
        use Greyness::*;
        match self {
            Shade(my_prop) => match other {
                Shade(other_prop) => my_prop.prop_diff(other_prop),
                _ => None,
            },
            Tint(my_prop) => match other {
                Tint(other_prop) => my_prop.prop_diff(other_prop),
                _ => None,
            },
            Neither(my_prop) => match other {
                Neither(other_prop) => my_prop.prop_diff(other_prop),
                _ => None,
            },
        }
    }
}

impl ApproxEq for Greyness {}

impl From<Chroma> for Greyness {
    fn from(chroma: Chroma) -> Self {
        match chroma {
            Chroma::Shade(prop) => Greyness::Shade(Prop::ONE - prop),
            Chroma::Tint(prop) => Greyness::Tint(Prop::ONE - prop),
            Chroma::Neither(prop) => Greyness::Neither(Prop::ONE - prop),
        }
    }
}

#[derive(
    Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Debug,
)]
pub struct Warmth(pub(crate) u64);

impl Warmth {
    pub const ZERO: Self = Self(0);

    pub const ONE_SIXTH: Self = Self(u64::MAX / 6);

    pub const ONE_THIRD: Self = Self(u64::MAX / 3);

    pub const HALF: Self = Self(u64::MAX / 2);

    pub const TWO_THIRDS: Self = Self(u64::MAX / 3 * 2);

    pub const FIVE_SIXTHS: Self = Self(u64::MAX / 6 * 5);

    pub const ONE: Self = Self(u64::MAX);

    pub fn calculate_monochrome(value: Value) -> Self {
        ((Prop::ONE - Prop::from_as_prop(value)) / 2_u32).into_as_prop()
    }

    pub fn abs_diff(&self, other: &Self) -> Warmth {
        match self.cmp(other) {
            Ordering::Greater => Warmth(self.0 - other.0),
            Ordering::Less => Warmth(other.0 - self.0),
            Ordering::Equal => Warmth(0),
        }
    }
}

impl HueConstants for Warmth {
    const RED: Self = Warmth::ONE;
    const GREEN: Self = Warmth::ONE_THIRD;
    const BLUE: Self = Warmth::ONE_THIRD;
    const CYAN: Self = Warmth::ZERO;
    const YELLOW: Self = Warmth::TWO_THIRDS;
    const MAGENTA: Self = Warmth::TWO_THIRDS;
    const RED_MAGENTA: Self = Warmth::FIVE_SIXTHS;
    const RED_YELLOW: Self = Warmth::FIVE_SIXTHS;
    const GREEN_YELLOW: Self = Warmth::HALF;
    const GREEN_CYAN: Self = Warmth::ONE_SIXTH;
    const BLUE_CYAN: Self = Warmth::ONE_SIXTH;
    const BLUE_MAGENTA: Self = Warmth::HALF;
}

impl Mul<Prop> for Warmth {
    type Output = Self;

    fn mul(self, rhs: Prop) -> Self::Output {
        Self((Prop::from_as_prop(self) * rhs).into_as_prop())
        // Self(((self.0 as u128 * rhs.0 as u128) / u64::MAX as u128) as u64)
    }
}

impl Sub for Warmth {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        debug_assert!(rhs <= self);
        Self(self.0 - rhs.0)
    }
}

impl Add for Warmth {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        debug_assert!(Self::ONE.0 - self.0 >= rhs.0);
        Self(self.0 + rhs.0)
    }
}

impl PropDiff for Warmth {
    fn prop_diff(&self, other: &Self) -> Option<Prop> {
        Some(self.abs_diff(other).into_as_prop())
    }
}

impl ApproxEq for Warmth {}

impl FromAsProp<f64> for Warmth {
    fn from_as_prop(value: f64) -> Self {
        debug_assert!(value >= 0.0 && value <= 1.0);
        Self((value * u64::MAX as f64) as u64)
    }
}
impl_into_as_prop!(Warmth, f64);

impl FromAsProp<Warmth> for f64 {
    fn from_as_prop(warmth: Warmth) -> Self {
        warmth.0 as f64 / u64::MAX as f64
    }
}
impl_into_as_prop!(f64, Warmth);

impl FromAsProp<Prop> for Warmth {
    fn from_as_prop(prop: Prop) -> Self {
        Self(u64::from_as_prop(prop))
    }
}
impl_into_as_prop!(Warmth, Prop);

impl FromAsProp<Warmth> for Prop {
    fn from_as_prop(warmth: Warmth) -> Self {
        Self::from_as_prop(warmth.0)
    }
}
impl_into_as_prop!(Prop, Warmth);

impl_into_from_as_prop_number!(UFDRNum, u128, Warmth);
impl_into_from_as_prop_number!(FDRNum, i128, Warmth);

#[derive(
    Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default, Debug,
)]
pub struct Value(pub(crate) u64);

impl Value {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(u64::MAX);

    pub fn abs_diff(&self, other: &Self) -> Value {
        match self.cmp(other) {
            Ordering::Greater => Value(self.0 - other.0),
            Ordering::Less => Value(other.0 - self.0),
            Ordering::Equal => Value(0),
        }
    }
}

impl PropDiff for Value {
    fn prop_diff(&self, other: &Self) -> Option<Prop> {
        Some(self.abs_diff(other).into_prop())
    }
}

impl ApproxEq for Value {}

impl Div<i32> for Value {
    type Output = Value;

    fn div(self, rhs: i32) -> Self {
        debug_assert!(rhs >= 0);
        Self(self.0 / rhs as u64)
    }
}

impl Mul<i32> for Value {
    type Output = UFDRNum;

    fn mul(self, rhs: i32) -> UFDRNum {
        debug_assert!(rhs >= 0);
        UFDRNum(self.0 as u128 * rhs as u128)
    }
}

impl FromAsProp<f64> for Value {
    fn from_as_prop(value: f64) -> Self {
        debug_assert!(value >= 0.0 && value <= 1.0);
        Self((value * u64::MAX as f64) as u64)
    }
}
impl_into_as_prop!(Value, f64);

impl FromAsProp<Value> for f64 {
    fn from_as_prop(value: Value) -> Self {
        value.0 as f64 / u64::MAX as f64
    }
}
impl_into_as_prop!(f64, Value);

impl FromAsProp<Prop> for Value {
    fn from_as_prop(prop: Prop) -> Self {
        Self(prop.0)
    }
}
impl_into_as_prop!(Value, Prop);

impl FromAsProp<Value> for Prop {
    fn from_as_prop(value: Value) -> Self {
        Self(value.0)
    }
}
impl_into_as_prop!(Prop, Value);

impl_into_from_as_prop_number!(UFDRNum, u128, Value);
impl_into_from_as_prop_number!(FDRNum, i128, Value);

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Family {
    Reds,
    Greens,
    Blues,
    Cyans,
    Yellows,
    Magentas,
}
