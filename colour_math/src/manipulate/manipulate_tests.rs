// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::fixed_denominator_rational_numbers::{self::*, debug::ApproxEq};

use crate::{
    ColourAttributes, HueConstants, IntoProp, ManipulatedColour, RGBConstants,
    attributes::Chroma,
    hcv::*,
    hue::{Hue, angle::*},
    rgb::RGB,
};

use crate::Outcome::NoChange;
use approximately_equal::assert_approx_eq;

#[test]
fn decr_chroma() {
    use crate::Outcome::*;
    use crate::SetScalarPolicy::*;
    // clamping should make no difference to chroma decrementing
    assert_eq!(
        RGB::<Prop>::BLACK.try_decr_chroma_prop(0.1_f64.into_prop(), Clamp),
        NoChange
    );
    assert_eq!(
        RGB::<Prop>::BLACK.try_decr_chroma_prop(0.1_f64.into_prop(), Accommodate),
        NoChange
    );
    assert_eq!(
        RGB::<Prop>::WHITE.try_decr_chroma_prop(0.1_f64.into_prop(), Clamp),
        NoChange
    );
    assert_eq!(
        RGB::<Prop>::WHITE.try_decr_chroma_prop(0.1_f64.into_prop(), Accommodate),
        NoChange
    );
    // let pink: RGB<Prop> = RGB::<Prop>::RED / 2;
    // assert_approx_eq!(
    //     pink.try_decr_chroma_prop(0.1_f64.into_prop(), Clamp),
    //     Accommodated(RGB::<Prop>::RED),
    //     1000
    // );
    // manipulator.set_colour(&RGB::<u64>::YELLOW);
    // assert_eq!(manipulator.hcv.chroma(), Chroma::ONE);
    // let saved_hue = manipulator.hcv.hue();
    // let decr = Prop::prop_from(0.1);
    // let mut expected = manipulator.hcv.chroma_prop() - decr;
    // while manipulator.try_decr_chroma_prop(decr) {
    //     assert_approx_eq!(manipulator.hcv.chroma_prop(), expected);
    //     expected = if manipulator.hcv.chroma_prop() > decr {
    //         manipulator.hcv.chroma_prop() - decr
    //     } else {
    //         Prop::ZERO
    //     };
    //     assert_approx_eq!(manipulator.hcv.sum(), UFDRNum::TWO, 0xA);
    //     if manipulator.hcv.chroma() > Chroma::ZERO {
    //         assert_eq!(manipulator.hcv.hue(), saved_hue);
    //     }
    // }
    // if !clamped {
    //     assert!(manipulator.hcv.is_grey());
    //     assert_eq!(manipulator.hcv.chroma(), Chroma::ZERO);
    //     assert_approx_eq!(manipulator.hcv.sum(), UFDRNum::TWO, 0x10);
    //     assert_eq!(manipulator.hcv.hue(), None);
    //     assert_eq!(manipulator.saved_hue, saved_hue.unwrap());
    // }
}

// #[test]
// fn incr_chroma_clamped() {
//     let mut manipulator = super::ColourManipulatorBuilder::new().clamped(true).build();
//     assert_eq!(manipulator.hcv, HCV::BLACK);
//     assert!(!manipulator.incr_chroma(0.1_f64.into_prop()));
// Test where clamping makes a difference and where it doesn't
// for array in &[[0.0_f64, 0.0, 0.0], [0.55, 0.5, 0.0], [0.55, 0.5, 0.55]] {
//     let rgb = RGB::from(*array);
//     manipulator.set_colour(&rgb);
//     let start_sum = manipulator.hcv.sum;
//     let saved_hue = if let Some(hue) = manipulator.hcv.hue() {
//         hue
//     } else {
//         Hue::RED
//     };
//     let incr = Prop::from(0.1_f64);
//     let mut max_chroma = if let Some(chroma) = saved_hue.max_chroma_for_sum(manipulator.hcv.sum)
//     {
//         chroma
//     } else {
//         manipulator.hcv.chroma()
//     };
//     let mut expected: Prop = (manipulator.hcv.chroma_prop() + incr)
//         .min(max_chroma.into_prop().into())
//         .into();
//     while manipulator.incr_chroma(incr) {
//         assert!(!manipulator.hcv.is_grey());
//         assert_approx_eq!(manipulator.hcv.chroma_prop(), expected);
//         max_chroma = saved_hue.max_chroma_for_sum(manipulator.hcv.sum).unwrap();
//         expected = (manipulator.hcv.chroma_prop() + incr)
//             .min(max_chroma.into_prop().into())
//             .into();
//         assert_approx_eq!(manipulator.hcv.sum, start_sum);
//         assert_eq!(manipulator.hcv.hue(), Some(saved_hue));
//     }
//     if !rgb.is_grey() {
//         assert!(!manipulator.hcv.is_grey());
//         assert_approx_eq!(
//             manipulator.hcv.chroma(),
//             saved_hue.max_chroma_for_sum(start_sum).unwrap()
//         );
//         assert_approx_eq!(manipulator.hcv.sum, start_sum);
//         assert_eq!(manipulator.hcv.hue(), Some(saved_hue));
//     }
// }
// }

// #[test]
// fn incr_chroma_unclamped() {
//     let mut manipulator = super::ColourManipulatorBuilder::new()
//         .clamped(false)
//         .build();
//     assert_eq!(manipulator.hcv, HCV::BLACK);
//     assert!(manipulator.incr_chroma(0.1_f64.into_prop()));
// Test where clamping makes a difference and where it doesn't
// for array in &[[0.75_f64, 0.5, 0.0], [0.75, 0.5, 0.75]] {
//     let rgb = RGB::from(*array);
//     manipulator.set_colour(&rgb);
//     let start_sum = manipulator.hcv.sum;
//     let saved_hue = manipulator.hcv.hue().unwrap();
//     let incr: Prop = 0.1_f64.into();
//     let mut expected: Prop = (manipulator.hcv.chroma_prop() + incr)
//         .min(UFDRNumber::ONE)
//         .into();
//     while manipulator.incr_chroma(incr) {
//         assert_eq!(manipulator.hcv.chroma_prop(), expected);
//         expected = (manipulator.hcv.chroma_prop() + incr)
//             .min(UFDRNumber::ONE)
//             .into();
//         let (min_sum, max_sum) = manipulator.hcv.sum_range_for_current_chroma();
//         assert!(
//             start_sum.approx_eq(&manipulator.hcv.sum, None)
//                 || min_sum.approx_eq(&manipulator.hcv.sum, None)
//                 || max_sum.approx_eq(&manipulator.hcv.sum, None)
//         );
//         assert_eq!(manipulator.hcv.hue(), Some(saved_hue));
//     }
//     assert!(!manipulator.hcv.is_grey());
//     assert_eq!(manipulator.hcv.chroma(), Chroma::ONE);
//     let (min_sum, max_sum) = manipulator.hcv.sum_range_for_current_chroma();
//     assert!(
//         start_sum.approx_eq(&manipulator.hcv.sum, None)
//             || min_sum.approx_eq(&manipulator.hcv.sum, None)
//             || max_sum.approx_eq(&manipulator.hcv.sum, None)
//     );
//     assert_eq!(manipulator.hcv.hue(), Some(saved_hue));
// }
// }

// #[test]
// fn round_trip_chroma() {
//     let mut manipulator = super::ColourManipulatorBuilder::new().clamped(true).build();
//     manipulator.set_colour(&colour_math::rgb::RGB::<u64>::CYAN);
//     while manipulator.try_decr_chroma_prop(0.01.into_prop()) {}
//     assert!(manipulator.hcv.is_grey());
//     while manipulator.incr_chroma(0.01.into_prop()) {}
//     assert_approx_eq!(
//         manipulator.rgb::<u64>(),
//         colour_math::rgb::RGB::<u64>::CYAN,
//         0x2000
//     );
// }

// #[test]
// fn incr_decr_sum_clamped() {
//     let mut manipulator = super::ColourManipulatorBuilder::new().clamped(true).build();
//     assert!(!manipulator.decr_value(0.1.into_prop()));
//     while manipulator.incr_value(0.1.into_prop()) {}
//     assert_eq!(manipulator.hcv, colour_math::hcv::HCV::WHITE);
//     while manipulator.decr_value(0.1.into_prop()) {}
//     assert_eq!(manipulator.hcv, colour_math::hcv::HCV::BLACK);
//     manipulator.set_colour(&colour_math::rgb::RGB::<u64>::YELLOW);
//     assert!(!manipulator.decr_value(0.1.into_prop()));
//     assert!(!manipulator.incr_value(0.1.into_prop()));
//     let cur_sum = manipulator.hcv.sum();
//     manipulator.try_decr_chroma_prop(0.5.into_prop());
//     assert_approx_eq!(cur_sum, manipulator.hcv.sum(), 0x10);
//     while manipulator.decr_value(0.1.into_prop()) {}
//   //  assert_approx_eq!(manipulator.rgb(), [0.5, 0.5, 0.0].into());
// while manipulator.incr_value(0.1.into()) {}
// assert_approx_eq!(manipulator.rgb(), [1.0, 1.0, 0.5].into());
// }

// #[test]
// fn incr_decr_sum_unclamped() {
//     let mut manipulator = super::ColourManipulatorBuilder::new()
//         .clamped(false)
//         .build();
//     assert!(!manipulator.decr_value(0.1.into_prop()));
//     while manipulator.incr_value(0.1.into_prop()) {}
//     assert_eq!(manipulator.hcv, colour_math::hcv::HCV::WHITE);
//     while manipulator.decr_value(0.1.into_prop()) {}
//     assert_eq!(manipulator.hcv, colour_math::hcv::HCV::BLACK);
//     for rgb in &colour_math::rgb::RGB::<u64>::SECONDARIES {
//         manipulator.set_colour(rgb);
//         while manipulator.incr_value(0.1.into_prop()) {}
//         assert_eq!(manipulator.hcv, colour_math::hcv::HCV::WHITE);
//         while manipulator.decr_value(0.1.into_prop()) {}
//         assert_eq!(manipulator.hcv, colour_math::hcv::HCV::BLACK);
//         assert!(manipulator.incr_chroma(1.0.into_prop()));
//         assert_eq!(manipulator.hcv, HCV::from(*rgb));
//     }
// }

// #[test]
// fn rotate_rgb_favouring_chroma() {
//     let mut manipulator = ColourManipulator::builder()
//         .rotation_policy(SetHuePolicy::FavourChroma)
//         .build();
//     let deltas = [
//         -Angle::from(180),
//         -Angle::from(120),
//         -Angle::from(60),
//         -Angle::from(30),
//         -Angle::from(10),
//         -Angle::from(5),
//         Angle::from(5),
//         Angle::from(10),
//         Angle::from(30),
//         Angle::from(60),
//         Angle::from(120),
//         Angle::from(180),
//     ];
//     for delta in deltas.iter() {
//         assert!(!manipulator.rotate((*delta).into()));
//     }
// pure colours
// for rgb in colour_math::rgb::RGB::<u64>::PRIMARIES
//     .iter()
//     .chain(colour_math::rgb::RGB::SECONDARIES.iter())
// {
//     manipulator.set_colour(rgb);
//     for delta in deltas.iter() {
//         let cur_chroma = manipulator.hcv.chroma();
//         let cur_sum = manipulator.hcv.sum();
//         let cur_angle = manipulator.hcv.hue_angle().unwrap();
//         assert!(manipulator.rotate((*delta).into()));
//         assert_eq!(cur_chroma, manipulator.hcv.chroma());
//         let (min_sum, max_sum) = manipulator.hcv.sum_range_for_current_chroma();
//         assert!(
//             cur_sum.approx_eq(&manipulator.hcv.sum(), None)
//                 || min_sum.approx_eq(&manipulator.hcv.sum(), None)
//                 || max_sum.approx_eq(&manipulator.hcv.sum(), None)
//         );
//         let expected_angle = cur_angle + (*delta).into();
//         assert_approx_eq!(expected_angle, manipulator.hcv.hue_angle().unwrap(), 0x1000);
//     }
// }
// shades and tints
// let half = u64::MAX / 2;
// for array in [
//     [half, 0, 0],
//     [0, half, 0],
//     [0, 0, half],
//     [half, half, 0],
//     [half, 0, half],
//     [0, half, half],
//     [u64::MAX, half, half],
//     [half, u64::MAX, half],
//     [half, half, u64::MAX],
//     [u64::MAX, u64::MAX, half],
//     [u64::MAX, half, u64::MAX],
//     [half, u64::MAX, u64::MAX],
// ]
// .iter()
// {
//     let rgb = RGB::from(*array);
//     manipulator.set_colour(&rgb);
//     for delta in deltas.iter() {
//         let cur_c_prop = manipulator.hcv.chroma_prop();
//         let cur_sum = manipulator.hcv.sum();
//         let cur_angle = manipulator.hcv.hue_angle().unwrap();
//         assert!(manipulator.rotate((*delta).into()));
//         assert_approx_eq!(cur_c_prop, manipulator.hcv.chroma_prop());
//         let (min_sum, max_sum) = manipulator.hcv.sum_range_for_current_chroma();
//         assert!(
//             cur_sum.approx_eq(&manipulator.hcv.sum(), Some(0x10))
//                 || min_sum.approx_eq(&manipulator.hcv.sum(), None)
//                 || max_sum.approx_eq(&manipulator.hcv.sum(), None)
//         );
//         let expected_angle = cur_angle + (*delta).into();
//         assert_approx_eq!(expected_angle, manipulator.hcv.hue_angle().unwrap(), 0x1000);
//     }
// }
// }
//
// #[test]
// fn rotate_rgb_favouring_value() {
//     let mut manipulator = ColourManipulator::builder()
//         .rotation_policy(SetHuePolicy::FavourValue)
//         .build();
//     let deltas = [
//         -Angle::from(180),
//         -Angle::from(120),
//         -Angle::from(60),
//         -Angle::from(30),
//         -Angle::from(10),
//         -Angle::from(5),
//         Angle::from(5),
//         Angle::from(10),
//         Angle::from(30),
//         Angle::from(60),
//         Angle::from(120),
//         Angle::from(180),
//     ];
//     for delta in deltas.iter() {
//         assert!(!manipulator.rotate((*delta).into()));
//     }
// pure colours
// for rgb in colour_math::rgb::RGB::<u64>::PRIMARIES
//     .iter()
//     .chain(colour_math::rgb::RGB::SECONDARIES.iter())
// {
//     manipulator.set_colour(rgb);
//     for delta in deltas.iter() {
//         let cur_c_prop = manipulator.hcv.chroma_prop();
//         let cur_sum = manipulator.hcv.sum();
//         let cur_angle = manipulator.hcv.hue_angle().unwrap();
//         assert!(manipulator.rotate((*delta).into()));
//         let max_c_prop = manipulator.hcv.max_chroma_prop_for_current_sum();
//         assert!(
//             cur_c_prop.approx_eq(&manipulator.hcv.chroma_prop(), Some(0x10))
//                 || max_c_prop.approx_eq(&manipulator.hcv.chroma_prop(), Some(0x10))
//         );
//         assert_approx_eq!(cur_sum, manipulator.hcv.sum(), 0x10);
//         let expected_angle = cur_angle + (*delta).into();
//         assert_approx_eq!(expected_angle, manipulator.hcv.hue_angle().unwrap(), 0x1000);
//     }
// }
// shades and tints
// let half = u64::MAX / 2;
// for array in [
//     [half, 0, 0],
//     [0, half, 0],
//     [0, 0, half],
//     [half, half, 0],
//     [half, 0, half],
//     [0, half, half],
//     [u64::MAX, half, half],
//     [half, u64::MAX, half],
//     [half, half, u64::MAX],
//     [u64::MAX, u64::MAX, half],
//     [u64::MAX, half, u64::MAX],
//     [half, u64::MAX, u64::MAX],
// ]
// .iter()
// {
//     let rgb = RGB::from(*array);
//     manipulator.set_colour(&rgb);
//     for delta in deltas.iter() {
//         let cur_c_prop = manipulator.hcv.chroma_prop();
//         let cur_sum = manipulator.hcv.sum();
//         let cur_angle = manipulator.hcv.hue_angle().unwrap();
//         assert!(manipulator.rotate((*delta).into()));
//         let max_c_prop = manipulator.hcv.max_chroma_prop_for_current_sum();
//         assert!(
//             cur_c_prop.approx_eq(&manipulator.hcv.chroma_prop(), Some(0x1))
//                 || max_c_prop.approx_eq(&manipulator.hcv.chroma_prop(), Some(0x1))
//         );
//         assert_approx_eq!(cur_sum, manipulator.hcv.sum(), 0x4);
//         let expected_angle = cur_angle + (*delta).into();
//         assert_approx_eq!(expected_angle, manipulator.hcv.hue_angle().unwrap(), 0x1000);
//     }
// }
// }
