// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use serde::{Deserialize, Serialize};
use std::rc::Rc;

use colour_math::{HCV, LightLevel};
use colour_math_derive::Colour;

use crate::{PaintEssence, SeriesId};

#[derive(Debug, Serialize, Deserialize, Colour, Clone, PartialEq, PartialOrd)]
pub struct SerializablePaintData {
    pub name: String,
    #[colour]
    pub colour: HCV,
    pub notes: String,
    pub property_variants_f64: Vec<f64>,
}

pub trait Paint:
    PaintEssence
    + From<(SerializablePaintData, Rc<SeriesId>)>
    + Into<SerializablePaintData>
{
}

#[macro_export]
macro_rules! create_paint {
    ($name:ident, $property_types:expr) => {
        #[derive(Debug, Colour, Clone)]
        pub struct $name {
            name: String,
            series_id: Rc<SeriesId>,
            #[colour]
            colour: HCV,
            notes: String,
            property_variants_f64: Vec<f64>,
        }

        #[cfg(test)]
        impl $name {
            pub fn new(
                name: &str,
                series_id: Rc<SeriesId>,
                colour: HCV,
                notes: &str,
                variants: &[f64],
            ) -> Self {
                $name {
                    name: name.to_string(),
                    series_id,
                    colour,
                    notes: notes.to_string(),
                    property_variants_f64: variants.to_vec(),
                }
            }
        }

        impl PaintEssence for $name {
            const PROPERTY_TYPES: &'static [PropertyType] = $property_types;

            fn name(&self) -> &str {
                &self.name
            }

            fn series_id(&self) -> Rc<SeriesId> {
                self.series_id.clone()
            }

            fn notes(&self) -> &str {
                &self.notes
            }

            fn properties(&self) -> impl Iterator<Item = Property> {
                Self::property_types()
                    .zip(self.property_variants_f64.iter())
                    .map(|(p, v)| Property::from((p, *v)))
            }

            fn property_variants_f64(&self) -> impl Iterator<Item=f64> {
                self.property_variants_f64.iter().copied()
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                let mut result = false;
                if self.name == other.name {
                    result = self.series_id == other.series_id;
                }
                result
            }
        }

        impl Eq for $name {}

        impl PartialOrd for $name {
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

        impl Ord for $name {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.partial_cmp(other).expect("paints are comparable")
            }
        }

        impl TooltipText for $name {
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

        impl LabelText for $name {
            fn label_text(&self) -> String {
                format!("Mix {}", self.name)
            }
        }

        impl From<(SerializablePaintData, Rc<SeriesId>)> for $name {
            fn from(arg: (SerializablePaintData, Rc<SeriesId>)) -> Self {
                Self {
                    name: arg.0.name,
                    notes: arg.0.notes,
                    colour: arg.0.colour,
                    series_id: arg.1,
                    property_variants_f64: arg.0.property_variants_f64.clone(),
                }
            }
        }

        impl Into<SerializablePaintData> for  $name{
            fn into(self) -> SerializablePaintData {
                SerializablePaintData {
                    name: self.name,
                    colour: self.colour,
                    notes: self.notes,
                    property_variants_f64: self.property_variants_f64,
                }
            }
        }

        impl Paint for $name {}
    };
}

#[cfg(test)]
mod paint_tests {
    use super::*;
    use colour_math::HueConstants;
    use colour_math::LightLevel;
    use colour_math_derive::Colour;

    use crate::properties::*;
    use crate::*;

    create_paint!(TestPaint, &[PropertyType::Transparency]);

    #[test]
    fn test_making_an_example() {
        let paint = TestPaint::new(
            "whatever",
            Rc::new(SeriesId {
                proprietor: "joe".to_string(),
                series_name: "blah".to_string(),
            }),
            HCV::RED,
            "notes",
            &[1_f64],
        );
        assert_eq!(paint.hcv(), HCV::RED);
        assert_eq!(paint.name(), "whatever");
    }

    // old test starts here
    //     use crate::{TooltipText, LabelText};
    //     use crate::paint::{PaintEssentialsIfce, Properties, PropertiedPaintPlus, SerializablePaintData};
    //     use crate::properties::PropertyType;
    //     use crate::series::*;
    //     use colour_math::ColourBasics;
    //     use colour_math::HCV;
    //     use colour_math::HueConstants;
    //     use colour_math::LightLevel;
    //     use colour_math::hue_wheel::{ColouredShape, MakeColouredShape, Shape};
    //     use colour_math_derive::Colour;
    //     use std::convert::From;
    //     use std::rc::Rc;
    //
    //     realize_propertied_paint_plus!(SeriesTestPaint, &[PropertyType::Transparency]);
    //
    //     impl MakeColouredShape for SeriesTestPaint {
    //         fn coloured_shape(&self) -> ColouredShape {
    //             let tooltip_text = self.tooltip_text();
    //             ColouredShape::new(&self.colour, &self.name, &tooltip_text, Shape::Square)
    //         }
    //     }
    //
    //     #[test]
    //     fn test_paint_spec_generate_paint() {
    //         let series_id = Rc::new(SeriesId {
    //             series_name: "name".to_string(),
    //             proprietor: "Proprieter".to_string(),
    //         });
    //         let target_paint = SeriesTestPaint {
    //             colour: HCV::RED_MAGENTA,
    //             series_id: series_id.clone(),
    //             name: "Red".to_string(),
    //             notes: "".to_string(),
    //             property_variants_f64: vec![2.0],
    //         };
    //         let paint_spec = SerializablePaintData {
    //             colour: HCV::RED_MAGENTA,
    //             name: "Red".to_string(),
    //             notes: String::new(),
    //             property_variants_f64: vec![1.0],
    //         };
    //         let paint: SeriesTestPaint = (paint_spec.clone(), series_id.clone()).into();
    //         assert_eq!(paint, target_paint);
    //     }
    //
    //     #[test]
    //     fn test_paint_to_from_paint_spec() {
    //         let paint_spec = SerializablePaintData {
    //             colour: HCV::RED_MAGENTA,
    //             name: "Red".to_string(),
    //             notes: "".to_string(),
    //             property_variants_f64: vec![2.0],
    //         };
    //         let series_id = SeriesId::new("DS".to_string(), "WC".to_string());
    //         let paint: SeriesTestPaint = (paint_spec.clone(), Rc::new(series_id.clone())).into();
    //         assert_eq!(paint.hcv(), HCV::RED_MAGENTA);
    //         assert_eq!(paint.name(), "Red");
    //         assert_eq!(paint.notes(), "");
    //         assert_eq!(paint.series_id, series_id.into());
    //         assert_eq!(paint.property_variants_f64, vec![2.0]);
    //         for (target, actual) in paint_spec
    //             .property_variants_f64
    //             .iter()
    //             .zip(paint.property_variants_f64().iter())
    //         {
    //             assert_eq!(target, actual);
    //         }
    //         let recovered_paint_spec: SerializablePaintData = (&paint).into();
    //         assert_eq!(recovered_paint_spec, paint_spec);
    //     }
}
