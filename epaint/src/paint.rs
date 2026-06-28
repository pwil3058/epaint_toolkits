// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use serde::{Deserialize, Serialize};

use colour_math::{ColourAttributes, ColourBasics, HCV, LightLevel};
use colour_math_derive::Colour;

use crate::properties::{Property, PropertyType};

#[derive(Serialize, Deserialize, Debug, Default, PartialOrd, Ord, PartialEq, Eq, Clone)]
pub struct SeriesId {
    pub(crate) proprietor: String,
    pub(crate) series_name: String,
}

impl SeriesId {
    pub fn new(proprietor: String, series_name: String) -> Self {
        Self {
            proprietor,
            series_name,
        }
    }
}

pub trait PropertyTypes {
    const PROPERTY_TYPES: &'static [PropertyType];

    fn property_types() -> impl Iterator<Item = PropertyType> {
        Self::PROPERTY_TYPES.iter().copied()
    }

    fn properties_variants_for(&self, values: &[&str]) -> Vec<Property> {
        let mut properties = vec![];
        for (pt, value) in Self::property_types().zip(values) {
            let property = Property::from((pt, *value));
            properties.push(property);
        }
        properties
    }

    fn properties_variants_f64_for(&self, values: &[f64]) -> Vec<Property> {
        let mut properties = vec![];
        for (pt, value) in Self::property_types().zip(values) {
            let property = Property::from((pt, *value));
            properties.push(property);
        }
        properties
    }

    // fn property_variants(&self) -> Vec<Property>;
    fn property_variants_f64(&self) -> Vec<f64>;
}

pub trait PaintIfce:
    ColourBasics + ColourAttributes + PropertyTypes + From<(PaintSpec, SeriesId)> + ColourBasics
{
    fn name(&self) -> &str;

    fn series_id(&self) -> &SeriesId;

    fn notes(&self) -> &str;

    fn property_variants(&self) -> Vec<Property> {
        let mut variants = vec![];
        for (property_type, value) in
            Self::property_types().zip(self.property_variants_f64().iter())
        {
            let property = Property::from((property_type, *value));
            variants.push(property);
        }
        variants
    }
}

#[derive(Debug, Serialize, Deserialize, Colour, Clone, PartialEq)]
pub struct PaintSpec {
    pub name: String,
    #[colour]
    pub colour: HCV,
    pub notes: String,
    pub property_variants: Vec<f64>,
}

impl PaintSpec {
    pub fn new(
        colour: &impl ColourBasics,
        name: &str,
        notes: &str,
        properties: &[PropertyType],
    ) -> Self {
        let mut property_variants = vec![];
        for property in properties {
            let variant: f64 = property.default_f64();
            property_variants.push(variant);
        }
        Self {
            colour: colour.hcv(),
            name: name.to_string(),
            notes: notes.to_string(),
            property_variants,
        }
    }
}

impl Eq for PaintSpec {}

#[cfg(test)]
mod paint_tests {
    use serde::{Deserialize, Serialize};
    use std::convert::From;

    use crate::paint::{PaintIfce, PaintSpec, PropertyTypes, SeriesId};
    use crate::properties::PropertyType;
    use colour_math::ColourBasics;
    use colour_math::HCV;
    use colour_math::HueConstants;
    use colour_math::LightLevel;
    use colour_math_derive::Colour;

    #[derive(Debug, Serialize, Deserialize, Colour, Clone, PartialEq)]
    pub struct TestPaint {
        name: String,
        series_id: SeriesId,
        #[colour]
        colour: HCV,
        notes: String,
        variants_64: Vec<f64>,
    }

    impl From<(PaintSpec, SeriesId)> for TestPaint {
        fn from(value: (PaintSpec, SeriesId)) -> Self {
            TestPaint {
                name: value.0.name,
                notes: value.0.notes,
                colour: value.0.colour,
                series_id: value.1,
                variants_64: value.0.property_variants.clone(),
            }
        }
    }

    impl PropertyTypes for TestPaint {
        const PROPERTY_TYPES: &'static [PropertyType] = &[PropertyType::Transparency];

        fn property_variants_f64(&self) -> Vec<f64> {
            self.variants_64.clone()
        }
    }

    impl PaintIfce for TestPaint {
        fn name(&self) -> &str {
            &self.name
        }

        fn notes(&self) -> &str {
            &self.notes
        }

        fn series_id(&self) -> &SeriesId {
            &self.series_id
        }
    }

    #[test]
    fn test_paint_spec_default_f64() {
        let target = PaintSpec {
            colour: HCV::RED_MAGENTA,
            name: "Red".to_string(),
            notes: "".to_string(),
            property_variants: vec![2.0],
        };
        let paint = PaintSpec::new(&HCV::RED_MAGENTA, "Red", "", &[PropertyType::Transparency]);
        assert_eq!(paint, target);
    }

    #[test]
    fn test_paint_from_paint_spec() {
        let paint_spec = PaintSpec {
            colour: HCV::RED_MAGENTA,
            name: "Red".to_string(),
            notes: "".to_string(),
            property_variants: vec![2.0],
        };
        let series_id = SeriesId::new("DS".to_string(), "WC".to_string());
        let paint: TestPaint = (paint_spec.clone(), series_id.clone()).into();
        assert_eq!(paint.hcv(), HCV::RED_MAGENTA);
        assert_eq!(paint.name(), "Red");
        assert_eq!(paint.notes(), "");
        assert_eq!(paint.series_id(), &series_id);
        assert_eq!(paint.variants_64, vec![2.0]);
        for (target, actual) in paint_spec
            .property_variants
            .iter()
            .zip(paint.property_variants_f64().iter())
        {
            assert_eq!(target, actual);
        }
    }
}
