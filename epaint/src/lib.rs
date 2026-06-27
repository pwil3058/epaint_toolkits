// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use colour_math::{ColourAttributes, ColourBasics};

pub mod properties;

use properties::*;

pub trait PaintIfce: ColourBasics + ColourAttributes {
    fn name(&self) -> &str;

    fn notes(&self) -> Option<&str> {
        None
    }

    fn properties(&self) -> &[Property];
}
