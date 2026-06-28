// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use serde::{Deserialize, Serialize};

use colour_math::{HCV, LightLevel};
use colour_math_derive::Colour;

use crate::paint::{PaintIfce, PaintSpec, PropertyTypes, SeriesId};
use crate::properties::PropertyType;

#[derive(Debug, Serialize, Deserialize, Colour, Clone, PartialEq)]
pub struct WaterColour {
    name: String,
    series_id: SeriesId,
    #[colour]
    colour: HCV,
    notes: String,
    variants_64: Vec<f64>,
}

impl From<(PaintSpec, SeriesId)> for WaterColour {
    fn from(value: (PaintSpec, SeriesId)) -> Self {
        Self {
            name: value.0.name,
            notes: value.0.notes,
            colour: value.0.colour,
            series_id: value.1,
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

impl PaintIfce for WaterColour {
    fn name(&self) -> &str {
        &self.name
    }

    fn series_id(&self) -> &SeriesId {
        &self.series_id
    }

    fn notes(&self) -> &str {
        &self.notes
    }
}
