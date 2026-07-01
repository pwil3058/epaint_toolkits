// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use serde::{Deserialize, Serialize};
use std::rc::Rc;

use colour_math::hue_wheel::MakeColouredShape;
use colour_math::{ColourAttributes, ColourBasics, HCV, LightLevel};
use colour_math_derive::Colour;

use crate::properties::{Property, PropertyType};
use crate::series::*;

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

pub trait PaintEssentialsIfce: ColourBasics + ColourAttributes + ColourBasics {
    fn name(&self) -> &str;

    fn series_id(&self) -> Rc<SeriesId>;

    fn notes(&self) -> &str;
}

pub trait CompomentPaintIfce: PaintEssentialsIfce + PropertyTypes + MakeColouredShape {
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

// pub trait PaintIfce: CompomentPaintIfce + From<(PaintSpec, Rc<SeriesId>)> {}

#[macro_export]
macro_rules! impl_eq_for_paint {
    ($paint:ident) => {
        impl PartialEq for $paint {
            fn eq(&self, other: &Self) -> bool {
                let mut result = false;
                if self.name == other.name {
                    result = self.series_id == other.series_id;
                }
                result
            }
        }

        impl Eq for $paint {}
    };
}

#[macro_export]
macro_rules! impl_ord_for_paint {
    ($paint:ident) => {
        impl PartialOrd for $paint {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                match self.name.cmp(&other.name) {
                    std::cmp::Ordering::Equal => match self.series_id.cmp(&other.series_id) {
                        std::cmp::Ordering::Equal => Some(std::cmp::Ordering::Equal),
                        std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
                        std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
                    },
                    std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
                    std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
                }
            }
        }

        impl Ord for $paint {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.partial_cmp(other).expect("comparable")
            }
        }
    };
}

#[derive(Debug, Serialize, Deserialize, Colour, Clone, PartialEq)]
pub struct PaintSpec {
    pub name: String,
    #[colour]
    pub colour: HCV,
    pub notes: String,
    pub property_variants: Vec<f64>,
}

pub trait GeneratePaint<P: CompomentPaintIfce> {
    fn generate_paint(&self, series_id: &Rc<SeriesId>) -> P;
}

impl PartialOrd for PaintSpec {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Eq for PaintSpec {}

#[cfg(test)]
mod paint_tests {
    use crate::TooltipText;
    use crate::paint::{
        CompomentPaintIfce, GeneratePaint, PaintEssentialsIfce, PaintSpec, PropertyTypes,
    };
    use crate::properties::PropertyType;
    use crate::series::*;
    use colour_math::ColourBasics;
    use colour_math::HCV;
    use colour_math::HueConstants;
    use colour_math::LightLevel;
    use colour_math::hue_wheel::{ColouredShape, MakeColouredShape, Shape};
    use colour_math_derive::Colour;
    use std::convert::From;
    use std::rc::Rc;

    #[derive(Debug, Colour, Clone)]
    pub struct TestPaint {
        name: String,
        series_id: Rc<SeriesId>,
        #[colour]
        colour: HCV,
        notes: String,
        variants_64: Vec<f64>,
    }

    impl_eq_for_paint!(TestPaint);
    impl_ord_for_paint!(TestPaint);

    impl CompomentPaintIfce for TestPaint {}

    impl From<(PaintSpec, SeriesId)> for TestPaint {
        fn from(value: (PaintSpec, SeriesId)) -> Self {
            TestPaint {
                name: value.0.name,
                notes: value.0.notes,
                colour: value.0.colour,
                series_id: Rc::new(value.1),
                variants_64: value.0.property_variants.clone(),
            }
        }
    }

    impl GeneratePaint<TestPaint> for PaintSpec {
        fn generate_paint(&self, series_id: &Rc<SeriesId>) -> TestPaint {
            TestPaint {
                name: self.name.to_string(),
                notes: self.notes.to_string(),
                colour: self.colour,
                series_id: Rc::clone(series_id),
                variants_64: self.property_variants.clone(),
            }
        }
    }

    impl PropertyTypes for TestPaint {
        const PROPERTY_TYPES: &'static [PropertyType] = &[PropertyType::Transparency];

        fn property_variants_f64(&self) -> Vec<f64> {
            self.variants_64.clone()
        }
    }

    impl PaintEssentialsIfce for TestPaint {
        fn name(&self) -> &str {
            &self.name
        }

        fn notes(&self) -> &str {
            &self.notes
        }

        fn series_id(&self) -> Rc<SeriesId> {
            self.series_id.clone()
        }
    }

    impl TooltipText for TestPaint {
        fn tooltip_text(&self) -> String {
            let mut string = self.name.to_string();
            string.push('\n');
            string.push_str(&self.notes);
            string.push('\n');
            string.push_str(&self.series_id.series_name);
            string.push('\n');
            string.push_str(&self.series_id.proprietor);

            string
        }
    }

    impl MakeColouredShape for TestPaint {
        fn coloured_shape(&self) -> ColouredShape {
            let tooltip_text = self.tooltip_text();
            ColouredShape::new(&self.colour, &self.name, &tooltip_text, Shape::Square)
        }
    }

    #[test]
    fn test_paint_spec_generate_paint() {
        let series_id = Rc::new(SeriesId {
            series_name: "name".to_string(),
            proprietor: "Proprieter".to_string(),
        });
        let target_paint = TestPaint {
            colour: HCV::RED_MAGENTA,
            series_id: series_id.clone(),
            name: "Red".to_string(),
            notes: "".to_string(),
            variants_64: vec![2.0],
        };
        let paint_spec = PaintSpec {
            colour: HCV::RED_MAGENTA,
            name: "Red".to_string(),
            notes: String::new(),
            property_variants: vec![1.0],
        };
        let paint = paint_spec.generate_paint(&series_id);
        assert_eq!(paint, target_paint);
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
        assert_eq!(*paint.series_id(), series_id);
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
