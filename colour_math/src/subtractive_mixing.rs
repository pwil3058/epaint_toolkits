// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::{ColourBasics, FromProps, LightLevel, Prop, RGBConstants, UFDRNumber, CMY, HCV, RGB};
use approximately_equal::*;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul};

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
/// using a generalized array-based Kubelka-Munk absorption and scattering model.
pub fn mix_n_colors_parts(mixes: &[(RGB<Prop>, u64)]) -> Option<RGB<Prop>> {
    if mixes.is_empty() {
        return None;
    }

    // 1. Calculate the total volumetric parts across the entire mixing vessel
    let total_parts: u64 = mixes.iter().map(|&(_, parts)| parts as u64).sum();
    if total_parts == 0 {
        return None;
    }

    // Linear accumulators for the physical pigment properties across [0..3]
    let mut mixed_k = [UFDRNumber::ZERO; 3];
    let mut mixed_s = [UFDRNumber::ZERO; 3];

    // 2. Synthesize and accumulate K and S arrays for all n colors based on their parts volume
    for &(colour, parts) in mixes {
        if parts == 0 {
            continue;
        }

        // Represent parts and total volume as a native fractional Prop or weight factor
        let weight = Prop::from([parts, total_parts]);
        let props = colour.into_proportions(); // Yields [Prop; 3]

        for i in 0..3 {
            let r = props[i];

            // Safely clamp the boundaries to avoid division-by-zero on absolute pure states
            let r_clamped = if r == Prop::ZERO {
                Prop(1)
            } else if r == Prop::ONE {
                Prop(u64::MAX - 1)
            } else {
                r
            };

            // Calculate 1.0 - R using your native complement mapping
            let one_minus_r = r_clamped.comp();

            // Compute the Kubelka-Munk K/S ratio using UFDRNumber: (1 - R)^2 / (2 * R)
            // This can easily yield a ratio > 1.0, which UFDRNumber handles natively.
            let numerator = UFDRNumber::from(one_minus_r).mul(UFDRNumber::from(one_minus_r));
            let denominator = UFDRNumber::from(r_clamped).mul(UFDRNumber::from(Prop(2)));
            eprintln!("numerator: {:?}, denominator: {:?}", numerator, denominator);
            let ks_ratio = numerator / denominator;

            // Define the primary scattering coefficient profile (S)
            // Highly reflective channels scatter light; dark channels absorb it.
            let s_base = UFDRNumber::from(r_clamped);

            // Derive the pure absorption constant (K) via K = (K/S) * S
            let k_base = ks_ratio.mul(s_base);

            // Accumulate physical properties linearly weighted by their parts volume factor
            mixed_k[i] = mixed_k[i] + k_base.mul(UFDRNumber::from(weight));
            mixed_s[i] = mixed_s[i] + s_base.mul(UFDRNumber::from(weight));
        }
    }

    // 3. Reconstruct final composite reflectance channels from aggregated properties
    let mut final_cmy_props = [Prop::ZERO; 3];

    for i in 0..3 {
        let total_k = mixed_k[i];
        let total_s = mixed_s[i];

        if total_s == UFDRNumber::ZERO {
            final_cmy_props[i] = Prop::ONE; // Total absorption if no scattering occurs
            continue;
        }

        // Compute final K/S ratio for this channel = total_k / total_s
        let final_ks = total_k / total_s;

        // Invert the KM formula back to visible light: R = 1 + K/S - sqrt((K/S)^2 + 2*(K/S))
        let term_sq = final_ks.mul(final_ks);
        let term_2ks = final_ks.mul(UFDRNumber::from(Prop(2)));
        let radical_input = term_sq + term_2ks;

        let val_1 = UFDRNumber::from(Prop::ONE) + final_ks;
        let val_2 = radical_input.sqrt(); // Uses your new native integer sqrt operation

        // Compute remaining light reflection
        let out_light_ufdr = val_1 - val_2;

        // Convert the UFDRNumber back to a strict Prop.
        // If the math breaks out of bounds, this will trigger your built-in panic.
        let out_light = Prop::from(out_light_ufdr);

        // Convert light reflection levels directly into subtractive CMY ink masses
        // Ink_Mass = 1.0 - Light
        final_cmy_props[i] = out_light.comp();
    }

    // 4. Construct the final CMY struct and convert it natively back into RGB light space
    let final_cmy = CMY::from_proportions(final_cmy_props);

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
