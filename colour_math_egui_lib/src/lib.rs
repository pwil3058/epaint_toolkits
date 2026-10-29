// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

pub mod colour;
pub mod components;
pub mod egui_drawer;
pub mod widgets;

use colour_math::ColourBasics;
use colour_math::fixed_denominator_rational_numbers::*;
use colour_math::rgb::RGB;
use eframe::egui;

/// Local translation trait to bridge foreign types cleanly within this crate boundary
pub trait EguiColorBridge {
    fn to_color32(&self) -> egui::Color32;
    fn from_color32(color: egui::Color32) -> Self;
}

impl EguiColorBridge for RGB<u64> {
    fn to_color32(&self) -> egui::Color32 {
        // 🌟 FIX: Pass as a standard slice block to satisfy your domain type bounds
        let prop_array: [Prop; 3] = self.into_proportions(); //<[Prop; 3]>::from(*self);

        // Convert out the raw element positions cleanly
        let r_f64 = f64::from_prop(prop_array[0]);
        let g_f64 = f64::from_prop(prop_array[1]);
        let b_f64 = f64::from_prop(prop_array[2]);

        egui::Color32::from_rgb(
            (r_f64 * 255.0).clamp(0.0, 255.0) as u8,
            (g_f64 * 255.0).clamp(0.0, 255.0) as u8,
            (b_f64 * 255.0).clamp(0.0, 255.0) as u8,
        )
    }

    fn from_color32(color: egui::Color32) -> Self {
        let r_prop = Prop::prop_from(color.r() as f64 / 255.0);
        let g_prop = Prop::prop_from(color.g() as f64 / 255.0);
        let b_prop = Prop::prop_from(color.b() as f64 / 255.0);

        let prop_array = [r_prop, g_prop, b_prop];
        RGB::<u64>::from_proportions(prop_array)
    }
}
