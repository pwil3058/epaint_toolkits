// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::rc::Rc;

use colour_math::{HCV, LightLevel};
use colour_math_derive::Colour;

use crate::paint::{PaintEssentialsIfce, PaintSpec, PropertiedType};
use crate::properties::PropertyType;
use crate::series::*;
use crate::{
    impl_eq_for_paint, impl_from_paint_spec, impl_ord_for_paint, impl_paint_essential_ifce,
};

#[derive(Debug, Colour, Clone)]
pub struct WaterColour {
    name: String,
    series_id: Rc<SeriesId>,
    #[colour]
    colour: HCV,
    notes: String,
    property_variants_f64: Vec<f64>,
}

impl_eq_for_paint!(WaterColour);
impl_ord_for_paint!(WaterColour);

impl PropertiedType for WaterColour {
    const PROPERTY_TYPES: &'static [PropertyType] = &[PropertyType::Transparency];
}

impl_paint_essential_ifce!(WaterColour);
impl_from_paint_spec!(WaterColour);
