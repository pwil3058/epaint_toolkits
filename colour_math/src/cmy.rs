// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::ops::{Deref, Div, Index, IndexMut, Mul};

use serde::{Deserialize, Serialize};

use fixed_denominator_rational_numbers::{debug::*, AsProportion, Prop, UnsignedInteger};

use crate::{
    Chroma, ColourBasics, Family, FromProps, Hue, HueConstants, LightLevel, RGBConstants, Value,
    Warmth, HCV, RGB,
};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct CMY<L: LightLevel>([L; 3]);

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct ComponentMask(u8);

impl Deref for ComponentMask {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ComponentMask {
    pub fn is_valid(&self) -> bool {
        (self.0 & (u8::MAX - 7)) == 0
    }

    pub fn intersection(self, other: ComponentMask) -> Self {
        ComponentMask(self.0 & other.0)
    }

    pub fn indices(self) -> Vec<usize> {
        let mut indices = vec![];
        for i in 0..3 {
            if 1 << i & self.0 != 0 {
                indices.push(i);
            }
        }
        indices
    }
}

impl<L: LightLevel> CMY<L> {
    pub fn component_mask(self) -> ComponentMask {
        let mut mask = ComponentMask(0);
        for (i, val) in self.0.iter().enumerate() {
            if *val != L::ZERO {
                mask.0 |= 1 << i;
            }
        }
        mask
    }
}

impl<L: LightLevel> PropDiff for CMY<L> {}

impl<L: LightLevel + ApproxEq> ApproxEq for CMY<L> {
    fn approx_eq(&self, other: &Self, max_diff: Option<u64>) -> bool {
        self[0].approx_eq(&other[0], max_diff)
            && self[1].approx_eq(&other[1], max_diff)
            && self[2].approx_eq(&other[2], max_diff)
    }
}

impl<L: LightLevel> FromProps for CMY<L> {
    fn from_proportions(props: [Prop; 3]) -> Self {
        Self([
            L::from_as_prop(props[0]),
            L::from_as_prop(props[1]),
            L::from_as_prop(props[2]),
        ])
    }
}

impl<L: LightLevel> Index<usize> for CMY<L> {
    type Output = L;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<L: LightLevel> IndexMut<usize> for CMY<L> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

/// Convert RGB to CMY
///
/// Example
/// ```
/// //use colour_math::{RGB, CMY};
/// ```
impl<L: LightLevel> From<RGB<L>> for CMY<L> {
    fn from(rgb: RGB<L>) -> Self {
        Self([L::ONE - rgb[0], L::ONE - rgb[1], L::ONE - rgb[2]])
    }
}

impl<L: LightLevel> From<CMY<L>> for RGB<L> {
    fn from(cmy: CMY<L>) -> Self {
        Self([L::ONE - cmy[0], L::ONE - cmy[1], L::ONE - cmy[2]])
    }
}

impl<L: LightLevel> From<[L; 3]> for CMY<L> {
    fn from(cmy: [L; 3]) -> Self {
        println!("{:?} -> {:?}", cmy, Self([cmy[0], cmy[1], cmy[2]]));
        Self([cmy[0], cmy[1], cmy[2]])
    }
}

impl<L: LightLevel> HueConstants for CMY<L> {
    const RED: Self = Self([L::ZERO, L::ONE, L::ONE]);
    const GREEN: Self = Self([L::ONE, L::ZERO, L::ONE]);
    const BLUE: Self = Self([L::ONE, L::ONE, L::ZERO]);

    const CYAN: Self = Self([L::ONE, L::ZERO, L::ZERO]);
    const MAGENTA: Self = Self([L::ZERO, L::ONE, L::ZERO]);
    const YELLOW: Self = Self([L::ZERO, L::ZERO, L::ONE]);

    const BLUE_CYAN: Self = Self([L::ONE, L::HALF, L::ZERO]);
    const BLUE_MAGENTA: Self = Self([L::HALF, L::ONE, L::ZERO]);
    const RED_MAGENTA: Self = Self([L::ZERO, L::ONE, L::HALF]);
    const RED_YELLOW: Self = Self([L::ZERO, L::HALF, L::ONE]);
    const GREEN_YELLOW: Self = Self([L::HALF, L::ZERO, L::ONE]);
    const GREEN_CYAN: Self = Self([L::ONE, L::ZERO, L::HALF]);
}

impl<L: LightLevel> RGBConstants for CMY<L> {
    const WHITE: Self = Self([L::ZERO, L::ZERO, L::ZERO]);
    const LIGHT_GREY: Self = Self([L::ONE_QUARTER, L::ONE_QUARTER, L::ONE_QUARTER]);
    const MEDIUM_GREY: Self = Self([L::HALF, L::HALF, L::HALF]);
    const DARK_GREY: Self = Self([L::THREE_QUARTERS, L::THREE_QUARTERS, L::THREE_QUARTERS]);
    const BLACK: Self = Self([L::ONE, L::ONE, L::ONE]);
}

impl<L: LightLevel> ColourBasics for CMY<L> {
    fn hue(&self) -> Option<Hue> {
        RGB::<L>::from(*self).hue()
    }

    fn chroma(&self) -> Chroma {
        RGB::<L>::from(*self).chroma()
    }

    fn chroma_prop(&self) -> Prop {
        RGB::<L>::from(*self).chroma_prop()
    }

    fn value(&self) -> Value {
        RGB::<L>::from(*self).value()
    }

    fn warmth(&self) -> Warmth {
        RGB::<L>::from(*self).warmth()
    }

    fn family(&self) -> Option<Family> {
        RGB::<L>::from(*self).family()
    }

    fn hcv(&self) -> HCV {
        RGB::<L>::from(*self).hcv()
    }

    fn rgb<LO: LightLevel>(&self) -> RGB<LO> {
        let rgb = RGB::<L>::from(*self);
        RGB::<LO>::from_proportions(rgb.into_proportions())
    }

    // fn cmy<LO: LightLevel>(&self) -> CMY<LO> {
    //     CMY::<LO>::from_proportions(self.into_proportions())
    // }

    fn into_proportions(self) -> [Prop; 3] {
        self.0
            .iter()
            .map(|c| c.into_prop())
            .collect::<Vec<_>>()
            .try_into()
            .expect("Array conversion failed")
        // [
        //     self[0].into_prop(),
        //     self[1].into_prop(),
        //     self[2].into_prop(),
        // ]
    }
}

impl Mul<Prop> for CMY<Prop> {
    type Output = Self;

    fn mul(self, rhs: Prop) -> Self {
        Self([self[0] * rhs, self[1] * rhs, self[2] * rhs])
    }
}

impl<L: LightLevel + Div<U, Output = L>, U: AsProportion + UnsignedInteger> Div<U> for CMY<L> {
    type Output = Self;

    fn div(self, rhs: U) -> Self {
        Self(
            self.0
                .iter()
                .map(|c| *c / rhs)
                .collect::<Vec<_>>()
                .try_into()
                .expect("failed array conversion"),
        )
    }
}

// #[derive(Copy, Clone, Debug, Default)]
// pub struct CMYSubtractiveMix {
//     colour: CMY<Prop>,
//     accumulated_parts: u32,
//     component_mask: ComponentMask,
// }
//
// impl CMYSubtractiveMix {
//     pub fn reset(&mut self) {
//         *self = Self::default();
//     }
//
//     fn use_weighted_average(&mut self, colour: CMY<Prop>, parts: u32) {
//         let triplet: [Prop; 3] = (0..3)
//             .map(|i| {
//                 Prop::weighted_average(&[
//                     (self.colour[i], self.accumulated_parts as u32),
//                     (colour[i], parts),
//                 ])
//             })
//             .collect::<Vec<_>>()
//             .try_into()
//             .expect("Should never fail");
//         self.colour = CMY::<Prop>::from_proportions(triplet);
//         self.component_mask = self.colour.component_mask();
//     }
//
//     pub fn add<C: ColourBasics>(&mut self, colour: C, parts: u32) {
//         if self.accumulated_parts == 0 {
//             self.colour = colour.cmy::<Prop>();
//             self.accumulated_parts = parts;
//             self.component_mask = self.colour.component_mask();
//             println!("add colour: {:?} : {}", self.colour, parts);
//         } else {
//             let colour = colour.cmy::<Prop>();
//             println!("add colour: {:?} : {}", colour, parts);
//             match self.component_mask.count_ones() {
//                 1 => {
//                     let component_mask = colour.component_mask();
//                     match component_mask.count_ones() {
//                         1 => {
//                             let overlap = self.component_mask.intersection(component_mask);
//                             if overlap.count_ones() == 1 {
//                                 self.use_weighted_average(colour, parts);
//                             } else {
//                                 use std::cmp::Ordering::*;
//                                 let mut new_colour = CMY::<Prop>::default();
//                                 match self.accumulated_parts.cmp(&parts) {
//                                     Less => {
//                                         let ratio = Prop::from([
//                                             self.accumulated_parts as u64,
//                                             parts as u64,
//                                         ]);
//                                         let i = self.component_mask.indices()[0];
//                                         new_colour[i] = self.colour[i] * ratio;
//                                         let i = component_mask.indices()[0];
//                                         new_colour[i] = colour[i];
//                                     }
//                                     Equal => {
//                                         let i = self.component_mask.indices()[0];
//                                         new_colour[i] = self.colour[i];
//                                         let i = component_mask.indices()[0];
//                                         new_colour[i] = colour[i];
//                                     }
//                                     Greater => {
//                                         let i = self.component_mask.indices()[0];
//                                         new_colour[i] = self.colour[i];
//                                         let ratio = Prop::from([
//                                             parts as u64,
//                                             self.accumulated_parts as u64,
//                                         ]);
//                                         let i = component_mask.indices()[0];
//                                         new_colour[i] = colour[i] * ratio;
//                                     }
//                                 }
//                                 self.colour = new_colour;
//                                 self.component_mask = colour.component_mask();
//                             }
//                         }
//                         _ => self.use_weighted_average(colour, parts),
//                     }
//                 }
//                 2 => {
//                     let _component_mask = colour.component_mask();
//                 }
//                 _ => self.use_weighted_average(colour, parts),
//             }
//             self.accumulated_parts += parts;
//         }
//     }
//
//     pub fn rgb<L: LightLevel>(&self) -> RGB<L> {
//         self.colour.rgb::<L>()
//     }
// }

#[cfg(test)]
mod cmy_tests {
    use super::*;
    use approximately_equal::assert_approx_eq;

    #[test]
    fn rgb_to_cmy() {
        assert_eq!(RGB::<Prop>::RED, CMY::<Prop>::RED.into());
        assert_eq!(RGB::<Prop>::YELLOW, CMY::<Prop>::YELLOW.into());
        assert_approx_eq!(RGB::<Prop>::LIGHT_GREY, CMY::<Prop>::LIGHT_GREY.into());
        assert_eq!(RGB::<Prop>::BLACK, CMY::<Prop>::BLACK.into());
    }

    #[test]
    fn divides() {
        assert_eq!(CMY::<u8>::RED / 2, CMY::<u8>::from([0, 127, 127]));
    }

    // #[test]
    // fn rgb_to_cmy_halves() {
    //     println!(
    //         "{:?} == {:?} == {:?}",
    //         CMY::<u8>::RED / 2,
    //         RGB::<u8>::RED / 2,
    //         std::convert::Into::<RGB<u8>>::into((CMY::<u8>::RED / 2))
    //     );
    //     assert_eq!(RGB::<u8>::RED / 2, (CMY::<u8>::RED / 2).into());
    //     assert_eq!(RGB::<Prop>::YELLOW, CMY::<Prop>::YELLOW.into());
    //     assert_approx_eq!(RGB::<Prop>::LIGHT_GREY, CMY::<Prop>::LIGHT_GREY.into());
    //     assert_eq!(RGB::<Prop>::BLACK, CMY::<Prop>::BLACK.into());
    // }
    //
    // #[test]
    // fn yellow_magenta_to_red() {
    //     let mut mixer = CMYSubtractiveMix::default();
    //     mixer.add(RGB::<u8>::YELLOW, 3);
    //     mixer.add(RGB::<u8>::MAGENTA, 3);
    //     assert_eq!(mixer.rgb::<u8>(), RGB::<u8>::RED);
    //     mixer.reset();
    //     assert_eq!(mixer.rgb::<u8>(), RGB::<u8>::WHITE);
    //     mixer.add((RGB::<u8>::YELLOW / 2), 3);
    //     mixer.add((RGB::<u8>::MAGENTA / 2), 3);
    //     assert_eq!(mixer.rgb::<u8>(), RGB::<u8>::RED / 2);
    // }
}
