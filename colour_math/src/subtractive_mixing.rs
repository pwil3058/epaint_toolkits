// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::{ColourBasics, FromProps, LightLevel, Prop, RGBConstants, UFDRNumber, CMY, HCV, RGB};
use approximately_equal::*;
use std::ops::{Add, AddAssign, Index, IndexMut};

#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct UnsignedTriple([UFDRNumber; 3]);

impl Index<usize> for UnsignedTriple {
    type Output = UFDRNumber;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl IndexMut<usize> for UnsignedTriple {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}

impl<C: ColourBasics> AddAssign<C> for UnsignedTriple {
    fn add_assign(&mut self, rhs: C) {
        for (i, prop) in rhs.into_proportions().iter().enumerate() {
            self[i] = self[i] + *prop;
        }
    }
}

impl Add for UnsignedTriple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self([self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]])
    }
}

impl UnsignedTriple {
    pub fn cross_mask(self) -> Self {
        Self([
            self[1].min(self[2]),
            self[0].min(self[2]),
            self[0].min(self[1]),
        ])
    }

    pub fn into_proportions(self) -> [Prop; 3] {
        debug_assert!(
            self.0.iter().all(|v| v.is_proportion()),
            "{:?}:{:?}:{:?}",
            self[0],
            self[1],
            self[2]
        );
        [self[0].into(), self[1].into(), self[2].into()]
    }
}

/// Universally mixes an arbitrary slice of colors and their relative parts
/// using a fully generalized CMY pigment-mass and chroma-preservation model.
pub fn mix_n_colors_parts(mixes: &[(RGB<Prop>, u64)]) -> Option<RGB<Prop>> {
    if mixes.is_empty() {
        return None;
    }

    // 1. Calculate the total volumetric parts across the entire mixing vessel
    let total_parts: u64 = mixes.iter().map(|&(_, parts)| parts as u64).sum();
    if total_parts == 0 {
        return None;
    }

    let divisor = u64::MAX as u128;

    // Linear pigment ink mass accumulators
    let mut total_cmy = UnsignedTriple::default();

    // 2. Accumulate the pure subtractive ink masses for all n colors weighted by their volume
    for &(colour, parts) in mixes {
        if parts == 0 {
            continue;
        }
        let w = Prop::from([parts, total_parts]);

        // Convert incoming additive RGB light into subtractive CMY ink masses
        let cmy_ink: CMY<Prop> = colour.into();
        total_cmy += (cmy_ink * w);
    }
    // Replace Step 3 and 4 with this universally complete intersection math:
    let cross_mask = total_cmy.cross_mask();

    // Apply the mutual physical masking constraints
    let final_cmy = CMY::from_proportions((total_cmy + cross_mask).into_proportions());

    Some(final_cmy.into())
}

fn add_two_colours(rgb1: RGB<Prop>, rgb2: RGB<Prop>) -> RGB<Prop> {
    RGB::WHITE
}

#[cfg(test)]
mod subtractive_add_tests {
    use super::*;
    use crate::HueConstants;

    #[test]
    fn equal_parts_two_colours() {
        assert_approx_eq!(
            mix_n_colors_parts(&[(RGB::<Prop>::YELLOW, 1), (RGB::<Prop>::RED, 1)]).unwrap(),
            RGB::<Prop>::RED_YELLOW
        );
        assert_approx_eq!(
            mix_n_colors_parts(&[(RGB::<Prop>::YELLOW, 1), (RGB::<Prop>::MAGENTA, 1)]).unwrap(),
            RGB::<Prop>::RED
        );
    }
}
