// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::{Colour, RGB};

#[derive(Debug, Clone, Copy)]
pub struct ColouredLabel {
    pub label: &'static str,
    pub colour: RGB<u8>,
}

impl ColouredLabel {
    pub fn new(label: &'static str, colour: impl Colour) -> Self {
        Self {
            label,
            colour: colour.into_rgb(),
        }
    }
}

impl From<(&'static str, RGB<u8>)> for ColouredLabel {
    fn from((label, colour): (&'static str, RGB<u8>)) -> Self {
        Self { label, colour }
    }
}

impl From<ColouredLabel> for (&'static str, RGB<u8>) {
    fn from(cl: ColouredLabel) -> (&'static str, RGB<u8>) {
        (&cl.label, cl.colour)
    }
}
