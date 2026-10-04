// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use colour_math::{ColourBasics, HCV, LightLevel, RGB};
use eframe::egui::Color32;

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
