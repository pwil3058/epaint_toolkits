// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::{
    fdrn::{Prop, UFDRNum},
    hcv::HCV,
    rgb::RGB,
    ColourBasics, LightLevel,
};

#[derive(Default, Debug)]
pub struct SubtractiveMixer {
    red: UFDRNum,
    green: UFDRNum,
    blue: UFDRNum,
    total_parts: u64,
}

impl SubtractiveMixer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, colour: &impl ColourBasics, parts: u64) {
        let [red, green, blue] = <[Prop; 3]>::from(colour.hcv());
        self.red += UFDRNum(red.0 as u128 * parts as u128);
        self.green += UFDRNum(green.0 as u128 * parts as u128);
        self.blue += UFDRNum(blue.0 as u128 * parts as u128);
        self.total_parts += parts;
    }

    pub fn mixed_colour(&self) -> Option<HCV> {
        if self.total_parts > 0 {
            let red = Prop((self.red.0 / self.total_parts as u128) as u64);
            let green = Prop((self.green.0 / self.total_parts as u128) as u64);
            let blue = Prop((self.blue.0 / self.total_parts as u128) as u64);
            Some(HCV::from(&[red, green, blue]))
        } else {
            None
        }
    }

    pub fn mixed_rgb<L: LightLevel>(&self) -> Option<RGB<L>> {
        Some(self.mixed_colour()?.into())
    }

    pub fn reset(&mut self) {
        self.red = UFDRNum::ZERO;
        self.green = UFDRNum::ZERO;
        self.blue = UFDRNum::ZERO;
        self.total_parts = 0;
    }
}

#[cfg(test)]
mod mixing_tests {
    use super::*;
    use crate::HueConstants;

    #[test]
    fn subtractive_mixing() {
        let mut subtractve_mixer = SubtractiveMixer::new();
        assert_eq!(subtractve_mixer.mixed_colour(), None);
        assert_eq!(subtractve_mixer.mixed_rgb::<u16>(), None);
        subtractve_mixer.add(&RGB::<u8>::RED, 5);
        assert_eq!(subtractve_mixer.mixed_colour(), Some(HCV::RED));
        assert_eq!(subtractve_mixer.mixed_rgb::<u16>(), Some(RGB::<u16>::RED));
        subtractve_mixer.add(&RGB::<u8>::GREEN, 10);
        let expected =
            RGB::<u16>::from_proportions([Prop::from([1, 3]), Prop::from([2, 3]), Prop::ZERO]);
        assert_eq!(subtractve_mixer.mixed_colour(), Some(expected.into()));
        assert_eq!(subtractve_mixer.mixed_rgb::<u16>(), Some(expected));
    }
}
