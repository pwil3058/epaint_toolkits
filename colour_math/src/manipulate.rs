// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::fixed_denominator_rational_numbers::*;
use crate::{Angle, ColourAttributes, Value};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SetScalarPolicy {
    Clamp,
    Accommodate,
    Reject,
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

    fn try_with_value(self, new_value: Value, policy: SetScalarPolicy) -> Outcome<Self>;
}
