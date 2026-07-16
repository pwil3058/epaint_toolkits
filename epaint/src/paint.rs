// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use serde::{Deserialize, Serialize};
use std::rc::Rc;

use colour_math::hue_wheel::{ColouredShape, MakeColouredShape, Shape};
use colour_math::{HCV, LightLevel};
use colour_math_derive::Colour;

use crate::properties::{Properties, Property, PropertyType};
use crate::{AbbrevKey, GetSeriesId, LabelText, PaintEssence, SeriesId, TooltipText};

#[derive(Debug, Serialize, Deserialize, Colour, Clone)]
pub struct SerializablePaintData {
    #[cfg(feature = "paints_have_ids")]
    pub id: String,
    pub name: String,
    #[colour]
    pub colour: HCV,
    pub notes: String,
    pub properties: Properties,
}

impl AbbrevKey for SerializablePaintData {
    #[cfg(feature = "paints_have_ids")]
    fn abbrev_key(&self) -> &str {
        &self.id
    }

    #[cfg(not(feature = "paints_have_ids"))]
    fn abbrev_key(&self) -> &str {
        &self.name
    }
}

impl PaintEssence for SerializablePaintData {
    #[cfg(feature = "paints_have_ids")]
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn colour(&self) -> HCV {
        self.colour.clone()
    }

    fn notes(&self) -> &str {
        &self.notes
    }

    fn iter_properties(&self) -> impl Iterator<Item = Property> {
        self.properties.properties()
    }

    fn iter_property_types(&self) -> impl Iterator<Item = PropertyType> {
        self.properties.iter_property_types()
    }
}

impl MakeColouredShape for SerializablePaintData {
    fn coloured_shape(&self) -> ColouredShape {
        let tooltip_text = self.tooltip_text();
        ColouredShape::new(&self.colour, &self.name, &tooltip_text, Shape::Square)
    }
}

impl TooltipText for SerializablePaintData {
    fn tooltip_text(&self) -> String {
        let mut string = self.name.to_string();
        string.push('\n');
        string.push_str(&self.notes);
        string.push('\n');

        string
    }
}

impl PartialEq for SerializablePaintData {
    fn eq(&self, other: &Self) -> bool {
        self.abbrev_key() == other.abbrev_key()
    }
}

impl Eq for SerializablePaintData {}

impl PartialOrd for SerializablePaintData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.abbrev_key().cmp(&other.abbrev_key()).into()
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
    #[cfg(feature = "paints_have_ids")]
    fn id(&self) -> &str {
        &self.data.id
    }

    fn name(&self) -> &str {
        &self.data.name
    }

    fn notes(&self) -> &str {
        &self.data.notes
    }

    fn colour(&self) -> HCV {
        self.data.colour.clone()
    }

    fn iter_property_types(&self) -> impl Iterator<Item = PropertyType> {
        self.data.iter_property_types()
    }

    fn iter_properties(&self) -> impl Iterator<Item = Property> {
        self.data.properties.properties()
    }
}

impl AbbrevKey for Paint {
    fn abbrev_key(&self) -> &str {
        self.data.abbrev_key()
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

#[cfg(test)]
mod paint_tests {
    use std::convert::From;
    use std::rc::Rc;

    use super::*;
    use colour_math::ColourBasics;
    use colour_math::HCV;
    use colour_math::HueConstants;

    use crate::paint::SerializablePaintData;
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
        assert_eq!(
            paint.data.properties,
            Properties(vec![Property::from((PropertyType::Transparency, 2.0))])
        );
        for (target, actual) in paint_spec.iter_properties().zip(paint.iter_properties()) {
            assert_eq!(target, actual);
        }
        let recovered_paint_spec: SerializablePaintData = paint.into();
        assert_eq!(recovered_paint_spec, paint_spec);
    }
}
