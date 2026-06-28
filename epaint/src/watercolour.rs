// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use serde::{Deserialize, Serialize};

use colour_math::{HCV, LightLevel};
use colour_math_derive::Colour;

use crate::paint::{HasProperties, PaintSpec, SeriesId};
use crate::properties::{Property, PropertyType};

#[derive(Debug, Serialize, Deserialize, Colour, Clone, PartialEq)]
pub struct WaterColour {
    name: String,
    series_id: Option<SeriesId>,
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
            series_id: Some(value.1),
            variants_64: value.0.property_variants.clone(),
        }
    }
}

impl HasProperties for WaterColour {
    const PROPERTY_TYPES: &'static [PropertyType] = &[PropertyType::Transparency];

    fn property_variants(&self) -> Vec<Property> {
        let mut variants = vec![];
        for (property_type, value) in Self::property_types().zip(self.variants_64.iter()) {
            let property = Property::from((property_type, *value));
            variants.push(property);
        }
        variants
    }

    fn property_variants_f64(&self) -> Vec<f64> {
        self.variants_64.clone()
    }
}

impl WaterColour {
    pub fn colour(&self) -> HCV {
        self.colour
    }

    pub fn name(&self) -> String {
        self.name.to_string()
    }

    pub fn notes(&self) -> String {
        self.notes.to_string()
    }

    pub fn series_id(&self) -> Option<SeriesId> {
        self.series_id.clone()
    }
}
