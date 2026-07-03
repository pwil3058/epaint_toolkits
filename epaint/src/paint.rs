// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use serde::{Deserialize, Serialize};
use std::rc::Rc;

use colour_math::{ColourAttributes, ColourBasics, HCV, LightLevel};
use colour_math_derive::Colour;

use crate::properties::{Property, PropertyType};
use crate::series::*;

pub trait PaintEssentialsIfce: ColourBasics + ColourAttributes + ColourBasics {
    fn name(&self) -> &str;
    fn series_id(&self) -> Rc<SeriesId>;
    fn notes(&self) -> &str;
}

#[macro_export]
macro_rules! impl_paint_essential_ifce {
    ($paint:ident) => {
        impl PaintEssentialsIfce for $paint {
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
    };
    ($paint:ident, $type:ident) => {
        impl<$type: PropertiedPaint> PaintEssentialsIfce for $paint<$type> {
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
    };
}

pub trait PropertiedPaint: PaintEssentialsIfce {
    const PROPERTY_TYPES: &'static [PropertyType];

    fn property_types() -> impl Iterator<Item = PropertyType> {
        Self::PROPERTY_TYPES.iter().copied()
    }

    fn properties(&self) -> Vec<Property> {
        let mut properties = vec![];
        for (property_type, variant_f64) in
            Self::property_types().zip(self.property_variants_f64().iter())
        {
            let property = Property::from((property_type, *variant_f64));
            properties.push(property);
        }
        properties
    }

    fn property_variants_f64(&self) -> Vec<f64>;
}

#[macro_export]
macro_rules! implement_propertied_paint {
    ($paint:ident, $propertypes:expr) => {
        impl PropertiedPaint for $paint {
            const PROPERTY_TYPES: &'static [PropertyType] = $propertypes;

            fn property_variants_f64(&self) -> Vec<f64> {
                self.property_variants_f64.clone()
            }
        }
    };
}

#[macro_export]
macro_rules! realize_propertied_paint {
    ($name:ident, $propertypes:expr) => {
        crate::declare_propertied_paint_struct!($name);
        impl_paint_essential_ifce!($name);
        implement_propertied_paint!($name, $propertypes);
        crate::impl_eq_for_paint!($name);
        crate::impl_ord_for_paint!($name);
        crate::impl_from_paint_spec!($name);
    };
}

#[macro_export]
macro_rules! declare_propertied_paint_struct {
    ($name:ident) => {
        #[derive(Debug, Colour, Clone)]
        pub struct $name {
            name: String,
            series_id: Rc<SeriesId>,
            #[colour]
            colour: HCV,
            notes: String,
            property_variants_f64: Vec<f64>,
        }
    };
}

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

#[derive(Debug, Serialize, Deserialize, Colour, Clone, PartialEq, PartialOrd)]
pub struct SerializablePaintData {
    pub name: String,
    #[colour]
    pub colour: HCV,
    pub notes: String,
    pub property_variants_f64: Vec<f64>,
}

impl Eq for SerializablePaintData {}

impl Ord for SerializablePaintData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("comparable")
    }
}

#[macro_export]
macro_rules! impl_from_paint_spec {
    ($paint:ident) => {
        impl From<(SerializablePaintData, Rc<SeriesId>)> for $paint {
            fn from(value: (SerializablePaintData, Rc<SeriesId>)) -> Self {
                Self {
                    name: value.0.name,
                    notes: value.0.notes,
                    colour: value.0.colour,
                    series_id: value.1,
                    property_variants_f64: value.0.property_variants_f64.clone(),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_into_paint_spec {
    ($paint:ident) => {
        impl From<&$paint> for SerializablePaintData {
            fn from(paint: &$paint) -> Self {
                Self {
                    name: paint.name.to_string(),
                    notes: paint.notes.to_string(),
                    colour: paint.colour.clone(),
                    property_variants_f64: paint.property_variants_f64.clone(),
                }
            }
        }
    };
}

#[cfg(test)]
mod paint_tests {
    use crate::TooltipText;
    use crate::paint::{PaintEssentialsIfce, PropertiedPaint, SerializablePaintData};
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

    realize_propertied_paint!(TestPaint, &[PropertyType::Transparency]);
    impl_into_paint_spec!(TestPaint);

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
            property_variants_f64: vec![2.0],
        };
        let paint_spec = SerializablePaintData {
            colour: HCV::RED_MAGENTA,
            name: "Red".to_string(),
            notes: String::new(),
            property_variants_f64: vec![1.0],
        };
        let paint: TestPaint = (paint_spec.clone(), series_id.clone()).into();
        assert_eq!(paint, target_paint);
    }

    #[test]
    fn test_paint_to_from_paint_spec() {
        let paint_spec = SerializablePaintData {
            colour: HCV::RED_MAGENTA,
            name: "Red".to_string(),
            notes: "".to_string(),
            property_variants_f64: vec![2.0],
        };
        let series_id = SeriesId::new("DS".to_string(), "WC".to_string());
        let paint: TestPaint = (paint_spec.clone(), Rc::new(series_id.clone())).into();
        assert_eq!(paint.hcv(), HCV::RED_MAGENTA);
        assert_eq!(paint.name(), "Red");
        assert_eq!(paint.notes(), "");
        assert_eq!(paint.series_id, series_id.into());
        assert_eq!(paint.property_variants_f64, vec![2.0]);
        for (target, actual) in paint_spec
            .property_variants_f64
            .iter()
            .zip(paint.property_variants_f64().iter())
        {
            assert_eq!(target, actual);
        }
        let recovered_paint_spec: SerializablePaintData = (&paint).into();
        assert_eq!(recovered_paint_spec, paint_spec);
    }
}
