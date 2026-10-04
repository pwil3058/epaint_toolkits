// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use colour_math::{ColourBasics, HCV, HueConstants, LightLevel, RGB};
use eframe::egui::{Color32, ColorImage};
use std::ops::{Deref, DerefMut};

pub trait Depuis<T: Copy>: Sized {
    fn depuis(arg: T) -> Self;
}

pub trait Dedans<T: Copy>: Sized {
    fn dedans(self) -> T;
}

impl<L: LightLevel> Depuis<Color32> for RGB<L> {
    fn depuis(color: Color32) -> Self {
        let (red, green, blue, _) = color.to_tuple();
        let colour: RGB<u8> = [red, green, blue].into();
        colour.rgb::<L>()
    }
}

impl<L: LightLevel> Dedans<RGB<L>> for Color32 {
    fn dedans(self) -> RGB<L> {
        RGB::<L>::depuis(self)
    }
}

impl<L: LightLevel> Depuis<RGB<L>> for Color32 {
    fn depuis(arg: RGB<L>) -> Self {
        let (red, green, blue) = arg.rgb::<u8>().into();
        Self::from_rgb(red, green, blue)
    }
}

impl<L: LightLevel> Dedans<Color32> for RGB<L> {
    fn dedans(self) -> Color32 {
        Color32::depuis(self)
    }
}

impl Depuis<Color32> for HCV {
    fn depuis(arg: Color32) -> Self {
        RGB::<u64>::depuis(arg).hcv()
    }
}

impl Dedans<HCV> for Color32 {
    fn dedans(self) -> HCV {
        HCV::depuis(self)
    }
}

impl Depuis<HCV> for Color32 {
    fn depuis(arg: HCV) -> Self {
        arg.rgb::<u8>().dedans()
    }
}

impl Dedans<Color32> for HCV {
    fn dedans(self) -> Color32 {
        Color32::depuis(self)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AverageColour {
    sums: [u128; 3],
    count: u128,
}

impl AverageColour {
    pub fn add_rgb(&mut self, rgb: RGB<u8>) {
        self.sums[0] += rgb[0] as u128;
        self.sums[1] += rgb[1] as u128;
        self.sums[2] += rgb[2] as u128;
        self.count += 1;
    }

    pub fn add_image(&mut self, image: &ColorImage) {
        for colour in &image.pixels {
            self.add_rgb(colour.dedans())
        }
    }

    pub fn average(&self) -> RGB<u8> {
        let avg: [u8; 3] = [
            (self.sums[0] / self.count).try_into().unwrap(),
            (self.sums[1] / self.count).try_into().unwrap(),
            (self.sums[2] / self.count).try_into().unwrap(),
        ];
        RGB::<u8>::from(avg)
    }
}

pub struct Images(pub Vec<ColorImage>);

impl Deref for Images {
    type Target = [ColorImage];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Images {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Images {
    pub fn average_colour(&self) -> RGB<u8> {
        let mut average = AverageColour::default();

        for image in &self.0 {
            average.add_image(image);
        }
        average.average()
    }
}

#[cfg(test)]
mod colour_conversioon_tests {
    use super::*;
    use colour_math::{ColourBasics, HCV, HueConstants, LightLevel, RGB};
    use eframe::egui::Color32;

    #[test]
    fn from_colour32_to_rgb() {
        assert_eq!(RGB::<u16>::depuis(Color32::MAGENTA), RGB::<u16>::MAGENTA);
        assert_eq!(RGB::<u32>::CYAN, Color32::CYAN.dedans());
    }

    #[test]
    fn from_rgb_to_Color32() {
        assert_eq!(Color32::depuis(RGB::<u16>::MAGENTA), Color32::MAGENTA);
        assert_eq!(Color32::YELLOW, RGB::<u32>::YELLOW.dedans());
    }

    #[test]
    fn from_colour32_to_hcv() {
        assert_eq!(HCV::depuis(Color32::MAGENTA), HCV::MAGENTA);
        assert_eq!(HCV::CYAN, Color32::CYAN.dedans());
    }

    #[test]
    fn from_hcv_to_Color32() {
        assert_eq!(Color32::depuis(HCV::MAGENTA), Color32::MAGENTA);
        assert_eq!(Color32::YELLOW, HCV::YELLOW.dedans());
    }
}
