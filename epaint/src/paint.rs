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

pub trait HasProperties {
    const PROPERTY_TYPES: &'static [PropertyType];
    fn property_types() -> impl Iterator<Item = PropertyType> {
        Self::PROPERTY_TYPES.iter().copied()
    }

    fn properties_variants(&self, values: &[&str]) -> Vec<Property> {
        let mut properties = vec![];
        for (pt, value) in Self::property_types().zip(values) {
            let property = Property::from((pt, *value));
            properties.push(property);
        }
        properties
    }

    fn properties_variants_f64(&self, values: &[f64]) -> Vec<Property> {
        let mut properties = vec![];
        for (pt, value) in Self::property_types().zip(values) {
            let property = Property::from((pt, *value));
            properties.push(property);
        }
        properties
    }
}

pub trait PaintIfce:
    ColourBasics + ColourAttributes + HasProperties + From<(PaintSpec, SeriesId)>
{
    fn name(&self) -> &str;
    fn series_id(&self) -> &SeriesId;

    fn notes(&self) -> Option<&str> {
        None
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

    use crate::paint::{HasProperties, PaintSpec, SeriesId};
    use crate::properties::PropertyType;
    use colour_math::HCV;
    use colour_math::HueConstants;
    use colour_math::LightLevel;
    use colour_math_derive::Colour;

    #[derive(Debug, Serialize, Deserialize, Colour, Clone, PartialEq)]
    pub struct TestPaint {
        name: String,
        series_id: Option<SeriesId>,
        #[colour]
        colour: HCV,
        notes: String,
    }

    impl From<(PaintSpec, SeriesId)> for TestPaint {
        fn from(value: (PaintSpec, SeriesId)) -> Self {
            TestPaint {
                name: value.0.name,
                notes: value.0.notes,
                colour: value.0.colour,
                series_id: Some(value.1),
            }
        }
    }

    impl HasProperties for TestPaint {
        const PROPERTY_TYPES: &'static [PropertyType] = &[PropertyType::Transparency];
    }

    impl TestPaint {
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
        let paint: TestPaint = (paint_spec, series_id.clone()).into();
        assert_eq!(paint.colour(), HCV::RED_MAGENTA);
        assert_eq!(paint.name(), "Red");
        assert_eq!(paint.notes(), "");
        assert_eq!(paint.series_id(), Some(series_id));
    }
}
