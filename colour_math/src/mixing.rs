// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::fixed_denominator_rational_numbers::*;

use crate::{
    hcv::HCV,
    rgb::{IntoRGB, RGB},
    Colour, FromProps, IntoHCV, IntoProps, LightLevel,
};

#[derive(Default, Debug, Clone, Copy)]
pub struct WeightedAverageMixer {
    sums: [UFDRNum; 3],
    total_parts: u64,
}

impl WeightedAverageMixer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, colour: &impl Colour, parts: u64) {
        for (sum, prop) in self
            .sums
            .iter_mut()
            .zip(colour.into_rgb::<Prop>().proportions())
        {
            *sum += prop * parts;
        }
        self.total_parts += parts;
    }

    fn mixed_proportions(&self) -> Option<[Prop; 3]> {
        if self.total_parts > 0 {
            Some(
                self.sums
                    .into_iter()
                    .map(|p| (p / self.total_parts).into_prop())
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("div failed"),
            )
        } else {
            None
        }
    }

    pub fn mixed_colour(&self) -> Option<HCV> {
        Some(HCV::from_proportions(self.mixed_proportions()?))
    }

    pub fn mixed_rgb<L: LightLevel>(&self) -> Option<RGB<L>> {
        Some(self.mixed_colour()?.into_rgb())
    }

    pub fn reset(&mut self) {
        self.sums = [UFDRNum::ZERO, UFDRNum::ZERO, UFDRNum::ZERO];
        self.total_parts = 0;
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct AltWeightedAverageMixer {
    pub mixed_colour: RGB<Prop>,
    accumulated_parts: u64,
}

impl AltWeightedAverageMixer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, colour: &impl Colour, parts: u64) {
        if self.accumulated_parts == 0 {
            self.mixed_colour = colour.into_rgb::<Prop>();
        } else {
            let rgb = colour.into_rgb::<Prop>();
            let other_prop = Prop::from_fraction(parts, self.accumulated_parts + parts);
            // NB: ensure total isn't greater than ONE due to roundin errors
            let my_prop = Prop::ONE - other_prop;
            self.mixed_colour = RGB::<Prop>(
                self.mixed_colour
                    .into_iter()
                    .zip(rgb.into_iter())
                    .map(|(m, o)| (m * my_prop + o * other_prop).into_prop())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            );
        }
        self.accumulated_parts += parts;
    }

    pub fn mixed_colour(&self) -> Option<HCV> {
        if self.accumulated_parts > 0 {
            Some(self.mixed_colour.into_hcv())
        } else {
            None
        }
    }

    pub fn mixed_rgb<L: LightLevel>(&self) -> Option<RGB<L>> {
        if self.accumulated_parts > 0 {
            Some(self.mixed_colour.into_rgb::<L>())
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.mixed_colour = RGB::<Prop>::default();
        self.accumulated_parts = 0;
    }
}

#[cfg(test)]
mod mixing_tests {
    use super::*;
    use crate::fixed_denominator_rational_numbers::debug::*;
    use crate::{FromProps, HueConstants};
    use approximately_equal::*;

    #[test]
    fn subtractive_mixing() {
        let mut subtractve_mixer = WeightedAverageMixer::new();
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

    #[test]
    fn equivalence() {
        let mut subtractive_mixer = WeightedAverageMixer::new();
        let mut subtractive_mixer_alt = AltWeightedAverageMixer::new();
        for (colour, parts) in [
            (RGB::<u8>::RED, 5u64),
            (RGB::YELLOW, 5),
            (RGB::GREEN, 4),
            (RGB::BLUE, 3),
        ] {
            subtractive_mixer.add(&colour, parts);
            subtractive_mixer_alt.add(&colour, parts);
            assert_approx_eq!(
                subtractive_mixer.mixed_rgb::<Prop>().unwrap(),
                subtractive_mixer_alt.mixed_colour
            );
        }
    }
}
