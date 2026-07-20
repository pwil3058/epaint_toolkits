// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use serde::{Deserialize, Serialize};

use colour_math::hue_wheel::{ColouredShape, MakeColouredShape, Shape};
use colour_math::{HCV, LightLevel};
use colour_math_derive::Colour;

use crate::properties::{Properties, Property};
use crate::{LabelText, SeriesId, TooltipText};

#[derive(Debug, Serialize, Deserialize, Colour, Clone)]
pub struct Paint {
    #[cfg(feature = "paints_have_ids")]
    pub id: String,
    pub name: String,
    #[colour]
    pub colour: HCV,
    pub notes: String,
    pub properties: Properties,
}

impl Paint {
    #[cfg(feature = "paints_have_ids")]
    pub fn key(&self) -> &str {
        &self.id
    }

    #[cfg(not(feature = "paints_have_ids"))]
    pub fn key(&self) -> &str {
        &self.name
    }
}

impl MakeColouredShape for Paint {
    fn coloured_shape(&self) -> ColouredShape {
        let tooltip_text = self.tooltip_text();
        ColouredShape::new(&self.colour, &self.name, &tooltip_text, Shape::Square)
    }
}

impl TooltipText for Paint {
    fn tooltip_text(&self) -> String {
        let mut string = self.name.to_string();
        string.push('\n');
        string.push_str(&self.notes);
        string.push('\n');

        string
    }
}

impl PartialEq for Paint {
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}

impl Eq for Paint {}

impl PartialOrd for Paint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.key().cmp(&other.key()).into()
    }
}

impl Ord for Paint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .expect("serializable paints are comparable")
    }
}

#[derive(Serialize, Deserialize, Debug, Colour, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CollnPaint {
    pub key: String,
    #[colour]
    pub paint: Paint,
    pub series_id: SeriesId,
}

impl From<(&Paint, &SeriesId)> for CollnPaint {
    fn from((paint, series_id): (&Paint, &SeriesId)) -> Self {
        let key = format!("{}::{}", paint.key(), series_id);
        Self {
            key,
            paint: paint.clone(),
            series_id: series_id.clone(),
        }
    }
}

impl CollnPaint {
    pub fn key(&self) -> &str {
        &self.key
    }

    #[cfg(feature = "paints_have_ids")]
    pub fn id(&self) -> &str {
        &self.paint.id
    }

    pub fn name(&self) -> &str {
        &self.paint.name
    }

    pub fn notes(&self) -> &str {
        &self.paint.notes
    }

    pub fn colour(&self) -> HCV {
        self.paint.colour.clone()
    }

    pub fn properties(&self) -> impl Iterator<Item = Property> {
        self.paint.properties.iter()
    }

    pub fn series_id(&self) -> &SeriesId {
        &self.series_id
    }
}

impl MakeColouredShape for CollnPaint {
    fn coloured_shape(&self) -> ColouredShape {
        let tooltip_text = self.tooltip_text();
        ColouredShape::new(&self.paint.colour, &self.key, &tooltip_text, Shape::Square)
    }
}

impl TooltipText for CollnPaint {
    fn tooltip_text(&self) -> String {
        let mut string = String::new();
        #[cfg(feature = "paints_have_ids")]
        string.push_str(&self.paint.id);
        string.push('\n');
        string.push_str(&self.paint.name);
        string.push('\n');
        string.push_str(&self.paint.notes);
        string.push('\n');
        string.push_str(&self.series_id.series_name);
        string.push('\n');
        string.push_str(&self.series_id.proprietor);

        string
    }
}

impl LabelText for CollnPaint {
    fn label_text(&self) -> String {
        format!("{}:{}", self.paint.name, self.series_id)
    }
}

impl Into<Paint> for CollnPaint {
    fn into(self) -> Paint {
        self.paint.clone()
    }
}

#[cfg(test)]
mod paint_tests {
    use std::convert::From;

    use super::*;
    use colour_math::ColourBasics;
    use colour_math::HCV;
    use colour_math::HueConstants;

    use crate::paint::Paint;
    use crate::properties::PropertyType;
    use crate::properties::*;
    use crate::*;

    #[test]
    fn test_paint_from() {
        let series_id = SeriesId {
            series_name: "name".to_string(),
            proprietor: "Proprieter".to_string(),
        };
        let target_paint = CollnPaint {
            key: "Magenta::name:(Proprieter)".to_string(),
            paint: Paint {
                #[cfg(feature = "paints_have_ids")]
                id: "Magenta".to_string(),
                colour: HCV::MAGENTA,
                name: "Magenta".to_string(),
                notes: "".to_string(),
                properties: Properties(vec![Property::from((PropertyType::Transparency, 1.0))]),
            },
            series_id: series_id.clone(),
        };
        let paint = Paint {
            #[cfg(feature = "paints_have_ids")]
            id: "magenta".to_string(),
            colour: HCV::MAGENTA,
            name: "Magenta".to_string(),
            notes: String::new(),
            properties: Properties(vec![Property::from((PropertyType::Transparency, 1.0))]),
        };
        let colln_paint: CollnPaint = (&paint, &series_id).into();
        assert_eq!(colln_paint, target_paint);
    }

    #[test]
    fn test_paint_to_from() {
        let paint = Paint {
            #[cfg(feature = "paints_have_ids")]
            id: "red magenta".to_string(),
            colour: HCV::RED_MAGENTA,
            name: "Red Magenta".to_string(),
            notes: "".to_string(),
            properties: Properties(vec![Property::from((PropertyType::Transparency, 2.0))]),
        };
        let series_id = SeriesId {
            series_name: "DS".to_string(),
            proprietor: "WC".to_string(),
        };
        let colln_paint: CollnPaint = (&paint, &series_id).into();
        assert_eq!(colln_paint.hcv(), HCV::RED_MAGENTA);
        assert_eq!(colln_paint.name(), "Red Magenta");
        assert_eq!(colln_paint.notes(), "");
        assert_eq!(colln_paint.series_id, series_id.into());
        assert_eq!(
            colln_paint.paint.properties,
            Properties(vec![Property::from((PropertyType::Transparency, 2.0))])
        );
        for (target, actual) in paint.properties.iter().zip(colln_paint.properties()) {
            assert_eq!(target, actual);
        }
        let recovered_paint_spec: Paint = colln_paint.into();
        assert_eq!(recovered_paint_spec, paint);
    }
}
