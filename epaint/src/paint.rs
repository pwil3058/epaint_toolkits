// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use serde::{Deserialize, Serialize};
use std::rc::Rc;

use colour_math::hue_wheel::{MakeColouredShape, ColouredShape, Shape};
use colour_math::{ColourIfce, LightLevel, HCV};
use colour_math_derive::Colour;

use crate::{GetSeriesId, LabelText, PaintEssence, SeriesId, TooltipText};
use crate::properties::{Properties, Property, PropertyType};

#[derive(Debug, Serialize, Deserialize, Colour, Clone)]
pub struct SerializablePaintData {
    pub name: String,
    #[colour]
    pub colour: HCV,
    pub notes: String,
    pub properties: Properties,
}

impl PaintEssence for SerializablePaintData {
    fn name(&self) -> &str {
        &self.name
    }

    fn colour(&self) -> HCV {
        self.colour.clone()
    }

    fn notes(&self) -> &str {
        &self.notes
    }

    fn properties(&self) -> impl Iterator<Item=Property> {
        self.properties.properties()
    }

    fn property_types(&self) -> impl Iterator<Item=PropertyType> {
        self.properties.property_types()
    }
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

#[derive(Debug, Colour, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Paint {
    pub data: SerializablePaintData,
    pub series_id: Rc<SeriesId>,
}

impl PaintEssence for Paint {
    fn name(&self) -> &str {
        &self.data.name
    }

    fn notes(&self) -> &str {
        &self.data.notes
    }

    fn colour(&self) -> HCV {
        self.data.colour.clone()
    }

    fn property_types(&self) -> impl Iterator<Item=PropertyType> {
        self.data.property_types()
    }

    fn properties(&self) -> impl Iterator<Item=Property> {
        self.data.properties.properties()
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

// impl PartialEq for Paint {
//     fn eq(&self, other: &Self) -> bool {
//         let mut result = false;
//         if self.data.name == other.data.name {
//             result = self.series_id == other.series_id;
//         }
//         result
//     }
// }
//
// impl Eq for Paint {}
//
// impl PartialOrd for Paint {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         match self.data.name.cmp(&other.data.name) {
//             std::cmp::Ordering::Equal => match self.series_id.cmp(&other.series_id) {
//                 std::cmp::Ordering::Equal => Some(std::cmp::Ordering::Equal),
//                 std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
//                 std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
//             },
//             std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
//             std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
//         }
//     }
// }
//
// impl Ord for Paint {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.partial_cmp(other).expect("paints are comparable")
//     }
// }

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
        self.data.clone()
    }
}

impl PaintIfce for Paint {}

pub trait PaintIfce:
PaintEssence
+ GetSeriesId
+ From<(SerializablePaintData, Rc<SeriesId>)>
+ Into<SerializablePaintData>
+ ColourIfce
+ TooltipText
+ LabelText
+ MakeColouredShape
{}

#[cfg(test)]
mod paint_tests {
    use std::convert::From;
    use std::rc::Rc;

    use super::*;
    use colour_math::ColourBasics;
    use colour_math::HueConstants;
    use colour_math::HCV;

    use crate::paint::{SerializablePaintData};
    use crate::properties::PropertyType;
    use crate::properties::*;
    use crate::*;

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
                properties: Properties(vec![Property::from((PropertyType::Transparency, 1.0))]),
            },
            series_id: series_id.clone(),
        };
        let paint_spec = SerializablePaintData {
            colour: HCV::RED_MAGENTA,
            name: "Red".to_string(),
            notes: String::new(),
            properties: Properties(vec![Property::from((PropertyType::Transparency, 2.0))]),
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
            properties: Properties(vec![Property::from((PropertyType::Transparency, 2.0))]),
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
        assert_eq!(paint.data.properties, Properties(vec![Property::from((PropertyType::Transparency, 2.0))]));
        for (target, actual) in paint_spec
            .properties()
            .zip(paint.properties())
        {
            assert_eq!(target, actual);
        }
        let recovered_paint_spec: SerializablePaintData = paint.into();
        assert_eq!(recovered_paint_spec, paint_spec);
    }
}
