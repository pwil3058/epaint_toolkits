// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::rc::Rc;

use colour_math::{HCV, LightLevel};
use colour_math_derive::Colour;

use crate::paint::{PaintEssentialsIfce, PaintSpec, PropertyTypes};
use crate::properties::PropertyType;
use crate::series::*;
use crate::{impl_eq_for_paint, impl_ord_for_paint};

#[derive(Debug, Colour, Clone)]
pub struct WaterColour {
    name: String,
    series_id: Rc<SeriesId>,
    #[colour]
    colour: HCV,
    notes: String,
    variants_64: Vec<f64>,
}

impl_eq_for_paint!(WaterColour);
impl_ord_for_paint!(WaterColour);

impl From<(PaintSpec, SeriesId)> for WaterColour {
    fn from(value: (PaintSpec, SeriesId)) -> Self {
        Self {
            name: value.0.name,
            notes: value.0.notes,
            colour: value.0.colour,
            series_id: Rc::new(value.1),
            variants_64: value.0.property_variants.clone(),
        }
    }
}

impl PropertyTypes for WaterColour {
    const PROPERTY_TYPES: &'static [PropertyType] = &[PropertyType::Transparency];

    fn property_variants_f64(&self) -> Vec<f64> {
        self.variants_64.clone()
    }
}

impl PaintEssentialsIfce for WaterColour {
    fn name(&self) -> &str {
        &self.name
    }

    fn series_id(&self) -> Rc<SeriesId> {
        self.series_id.clone()
    }

    fn notes(&self) -> &str {
        &self.notes
    }
}
