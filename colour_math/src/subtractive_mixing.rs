// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::{ColourBasics, LightLevel, Prop, HCV, RGB};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct SubtractiveMixer {
    colours: Vec<RGB<Prop>>,
}

impl SubtractiveMixer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_colour(&mut self, colour: impl ColourBasics) {
        self.colours.push(colour.rgb::<Prop>());
    }

    pub fn mixed_colour<L: LightLevel>(&mut self) -> RGB<L> {
        let mut rgb = RGB::<Prop>([Prop::ONE, Prop::ONE, Prop::ONE]);

        for colour in &self.colours {
            for i in 0..3 {
                rgb[i] = rgb[i] * colour[i];
            }
        }

        rgb.rgb::<L>()
    }
}
pub fn mix_colors(c1: Rgb, c2: Rgb, bias: Prop) -> Rgb {
    // 1. Calculate the standard linear light mix (base gradient direction)
    let lin_r = c1.r.linear_interpolation(c2.r, bias);
    let lin_g = c1.g.linear_interpolation(c2.g, bias);
    let lin_b = c1.b.linear_interpolation(c2.b, bias);

    // 2. Detect if we are mixing complementary/opposite subtractive components.
    // In a pure subtractive space, Magenta lacks Green, while Yellow lacks Blue.
    // If one color has Blue but no Green, and the other has Green but no Blue,
    // they must extinguish each other subtractively.
    let c1_has_b_no_g = c1.b.0 > 0 && c1.g.0 == 0;
    let c2_has_g_no_b = c2.g.0 > 0 && c2.b.0 == 0;

    let c1_has_g_no_b = c1.g.0 > 0 && c1.b.0 == 0;
    let c2_has_b_no_g = c2.b.0 > 0 && c2.g.0 == 0;

    let is_subtractive_clash = (c1_has_b_no_g && c2_has_g_no_b) || (c1_has_g_no_b && c2_has_b_no_g);

    // 3. Apply conditional masking based on the structural mixing context
    if is_subtractive_clash {
        // Magenta + Yellow mix: Mutually extinguish Green and Blue to get pure Red
        Rgb {
            r: lin_r,
            g: Prop::ZERO,
            b: Prop::ZERO,
        }
    } else {
        // Red + Yellow mix: Behave linearly to let Green bloom into pure Orange
        Rgb {
            r: lin_r,
            g: lin_g,
            b: lin_b,
        }
    }
}

/// Mix an arbitrary slice of colors based on their relative integer parts.
/// The proportion of each color is automatically calculated as: parts / total_parts.
pub fn mix_n_colors_parts(mixes: &[(Rgb, u32)]) -> Rgb {
    if mixes.is_empty() {
        return Rgb {
            r: Prop::ZERO,
            g: Prop::ZERO,
            b: Prop::ZERO,
        };
    }
    if mixes.len() == 1 {
        return mixes[0].0;
    }

    // 1. Calculate the sum of all parts
    let total_parts: u128 = mixes.iter().map(|&(_, parts)| parts as u128).sum();
    if total_parts == 0 {
        return Rgb {
            r: Prop::ZERO,
            g: Prop::ZERO,
            b: Prop::ZERO,
        };
    }

    let mut lin_r = 0_u128;
    let mut lin_g = 0_u128;
    let mut lin_b = 0_u128;

    // Volumetric weight tracking for structural clashing profiles
    let mut total_has_b_no_g_parts = 0_u128;
    let mut total_has_g_no_b_parts = 0_u128;

    let divisor = 0xFFFF_FFFF_FFFF_FFFF_u128;

    // 2. Accumulate linear light values and track pigment characteristics
    for &(color, parts) in mixes {
        if parts == 0 {
            continue;
        }
        let p = parts as u128;

        // Calculate the color's absolute fractional weight scaled into Prop space
        // weight = (parts * u64::MAX) / total_parts
        let w = (p * divisor) / total_parts;

        // Standard linear light accumulation
        lin_r += (color.r.0 as u128 * w) / divisor;
        lin_g += (color.g.0 as u128 * w) / divisor;
        lin_b += (color.b.0 as u128 * w) / divisor;

        // Categorize individual structural color properties by their parts volume
        if color.b.0 > 0 && color.g.0 == 0 {
            total_has_b_no_g_parts += p; // e.g., Magenta / Blue component volume
        }
        if color.g.0 > 0 && color.b.0 == 0 {
            total_has_g_no_b_parts += p; // e.g., Yellow / Green component volume
        }
    }

    // 3. Determine and apply the subtractive clash penalty
    let clash_parts = total_has_b_no_g_parts.min(total_has_g_no_b_parts);

    if clash_parts > 0 {
        // Normalize the clash volume against the total active clashing profiles
        let total_clash_zone = total_has_b_no_g_parts + total_has_g_no_b_parts;
        let clash_factor = (clash_parts * divisor) / total_clash_zone;
        let mask = divisor - clash_factor; // Remaining light allowed through

        // Apply the subtractive mask to the green and blue leakage channels
        lin_g = (lin_g * mask) / divisor;
        lin_b = (lin_b * mask) / divisor;
    }

    Rgb {
        r: Prop(lin_r.min(0xFFFF_FFFF_FFFF_FFFF) as u64),
        g: Prop(lin_g.min(0xFFFF_FFFF_FFFF_FFFF) as u64),
        b: Prop(lin_b.min(0xFFFF_FFFF_FFFF_FFFF) as u64),
    }
}

#[cfg(test)]
mod subtractive_add_tests {
    use super::*;
    use crate::HueConstants;

    // #[test]
    fn equal_parts_two_colours() {
        let mut subtractive_mixer = SubtractiveMixer::new();
        subtractive_mixer.add_colour(HCV::YELLOW);
        subtractive_mixer.add_colour(HCV::MAGENTA);
        assert_eq!(subtractive_mixer.mixed_colour::<u8>(), HCV::RED.rgb::<u8>());
        subtractive_mixer.add_colour(HCV::YELLOW);
        assert_eq!(
            subtractive_mixer.mixed_colour::<u8>(),
            HCV::RED_YELLOW.rgb::<u8>()
        );
    }
}
