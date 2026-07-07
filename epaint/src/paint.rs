// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use serde::{Deserialize, Serialize};
use std::rc::Rc;

use colour_math::hue_wheel::MakeColouredShape;
use colour_math::{ColourIfce, LightLevel, HCV};
use colour_math_derive::Colour;

use crate::{GetSeriesId, LabelText, PaintEssence, SeriesId, TooltipText};

#[derive(Debug, Serialize, Deserialize, Colour, Clone)]
pub struct SerializablePaintData {
    pub name: String,
    #[colour]
    pub colour: HCV,
    pub notes: String,
    pub property_variants_f64: Vec<f64>,
}

impl PartialEq for SerializablePaintData {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for SerializablePaintData {}

impl PartialOrd for SerializablePaintData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.cmp(&other.name).into()
    }
}

impl Ord for SerializablePaintData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .expect("serializable paints are comparable")
    }
}

pub trait PaintIfce:
    PaintEssence
    + GetSeriesId
    + From<(SerializablePaintData, Rc<SeriesId>)>
    + Into<SerializablePaintData>
    + ColourIfce
    + TooltipText
    + LabelText
    + MakeColouredShape
{
}

#[macro_export]
macro_rules! create_paint {
    ($property_types:expr) => {
        #[derive(Debug, Colour, Clone)]
        pub struct Paint {
            pub data: SerializablePaintData,
            pub series_id: Rc<SeriesId>,
        }

        impl PaintEssence for Paint {
            const PROPERTY_TYPES: &'static [PropertyType] = $property_types;

            fn name(&self) -> &str {
                &self.data.name
            }

            fn notes(&self) -> &str {
                &self.data.notes
            }

            fn colour(&self) -> HCV {
                self.data.colour.clone()
            }

            fn property_types() -> impl Iterator<Item = PropertyType> {
                Self::PROPERTY_TYPES.iter().copied()
            }

            fn properties(&self) -> impl Iterator<Item = Property> {
                Self::property_types()
                    .zip(self.property_variants_f64())
                    .map(|(p, v)| Property::from((p, v)))
            }

            fn property_variants_f64(&self) -> impl Iterator<Item = f64> {
                self.data.property_variants_f64.iter().copied()
            }
        }

        impl PaintEssence for SerializablePaintData {
            const PROPERTY_TYPES: &'static [PropertyType] = $property_types;

            fn name(&self) -> &str {
                &self.name
            }

            fn notes(&self) -> &str {
                &self.notes
            }

            fn colour(&self) -> HCV {
                self.colour.clone()
            }

            fn property_types() -> impl Iterator<Item = PropertyType> {
                Self::PROPERTY_TYPES.iter().copied()
            }

            fn properties(&self) -> impl Iterator<Item = Property> {
                Self::property_types()
                    .zip(self.property_variants_f64())
                    .map(|(p, v)| Property::from((p, v)))
            }

            fn property_variants_f64(&self) -> impl Iterator<Item = f64> {
                self.property_variants_f64.iter().copied()
            }
        }

        impl GetSeriesId for Paint {
            fn series_id(&self) -> Rc<SeriesId> {
                self.series_id.clone()
            }
        }

        impl MakeColouredShape for Paint {
            fn coloured_shape(&self) -> ColouredShape {
                let tooltip_text = self.tooltip_text();
                ColouredShape::new(
                    &self.data.colour,
                    &self.data.name,
                    &tooltip_text,
                    Shape::Square,
                )
            }
        }

        impl PartialEq for Paint {
            fn eq(&self, other: &Self) -> bool {
                let mut result = false;
                if self.data.name == other.data.name {
                    result = self.series_id == other.series_id;
                }
                result
            }
        }

        impl Eq for Paint {}

        impl PartialOrd for Paint {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                match self.data.name.cmp(&other.data.name) {
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

        impl Ord for Paint {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.partial_cmp(other).expect("paints are comparable")
            }
        }

        impl TooltipText for Paint {
            fn tooltip_text(&self) -> String {
                let mut string = self.data.name.to_string();
                string.push('\n');
                string.push_str(&self.data.notes);
                string.push('\n');
                string.push_str(&self.series_id.series_name);
                string.push('\n');
                string.push_str(&self.series_id.proprietor);

                string
            }
        }

        impl LabelText for Paint {
            fn label_text(&self) -> String {
                format!("Mix {}", self.data.name)
            }
        }

        impl From<(SerializablePaintData, Rc<SeriesId>)> for Paint {
            fn from(arg: (SerializablePaintData, Rc<SeriesId>)) -> Self {
                Self {
                    data: arg.0,
                    series_id: arg.1,
                }
            }
        }

        impl Into<SerializablePaintData> for Paint {
            fn into(self) -> SerializablePaintData {
                SerializablePaintData {
                    name: self.data.name,
                    colour: self.data.colour,
                    notes: self.data.notes,
                    property_variants_f64: self.data.property_variants_f64,
                }
            }
        }

        impl PaintIfce for Paint {}
    };
}

#[cfg(test)]
mod paint_tests {
    use std::convert::From;
    use std::rc::Rc;

    use super::*;
    use colour_math::hue_wheel::{ColouredShape, MakeColouredShape, Shape};
    use colour_math::ColourBasics;
    use colour_math::HueConstants;
    use colour_math::LightLevel;
    use colour_math::HCV;
    use colour_math_derive::Colour;

    use crate::paint::{PaintIfce, SerializablePaintData};
    use crate::properties::PropertyType;
    use crate::properties::*;
    use crate::*;
    use crate::{LabelText, TooltipText};

    create_paint!(&[PropertyType::Transparency]);

    #[test]
    fn test_paint_spec_generate_paint() {
        let series_id = Rc::new(SeriesId {
            series_name: "name".to_string(),
            proprietor: "Proprieter".to_string(),
        });
        let target_paint = Paint {
            data: SerializablePaintData {
                colour: HCV::RED_MAGENTA,
                name: "Red".to_string(),
                notes: "".to_string(),
                property_variants_f64: vec![2.0],
            },
            series_id: series_id.clone(),
        };
        let paint_spec = SerializablePaintData {
            colour: HCV::RED_MAGENTA,
            name: "Red".to_string(),
            notes: String::new(),
            property_variants_f64: vec![1.0],
        };
        let paint: Paint = (paint_spec.clone(), series_id.clone()).into();
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
        let series_id = Rc::new(SeriesId {
            series_name: "DS".to_string(),
            proprietor: "WC".to_string(),
        });
        let paint: Paint = (paint_spec.clone(), series_id.clone()).into();
        assert_eq!(paint.hcv(), HCV::RED_MAGENTA);
        assert_eq!(paint.name(), "Red");
        assert_eq!(paint.notes(), "");
        assert_eq!(paint.series_id, series_id.into());
        assert_eq!(paint.data.property_variants_f64, vec![2.0]);
        for (target, actual) in paint_spec
            .property_variants_f64
            .iter()
            .copied()
            .zip(paint.property_variants_f64())
        {
            assert_eq!(target, actual);
        }
        let recovered_paint_spec: SerializablePaintData = paint.into();
        assert_eq!(recovered_paint_spec, paint_spec);
    }
}
