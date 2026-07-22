// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::io::{Read, Write};

use crypto_hash::{Algorithm, Hasher};
use gcd::Gcd;
use serde::{Deserialize, Serialize};

#[cfg(feature = "targeted_mixtures")]
use colour_math::RGB;
use colour_math::{
    ColourBasics, HCV, LightLevel,
    beigui::hue_wheel::{ColouredShape, MakeColouredShape, Shape},
    mixing::SubtractiveMixer,
};

use colour_math_derive::Colour;

use crate::paint::RangePaint;
use crate::properties::{Properties, PropertiesMixer, Property};
use crate::{LabelText, TooltipText};

#[derive(Serialize, Deserialize, Colour, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct Mixture {
    pub id: String,
    #[colour]
    pub colour: HCV,
    #[cfg(feature = "targeted_mixtures")]
    pub targeted_colour: Option<HCV>,
    pub name: String,
    pub notes: String,
    pub properties: Properties,
    pub components: Vec<(RangePaint, u64)>,
}

impl Mixture {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn notes(&self) -> &str {
        &self.notes
    }

    pub fn iter_properties(&self) -> impl Iterator<Item = Property> {
        self.properties.iter()
    }
}

impl Mixture {
    #[cfg(feature = "targeted_mixtures")]
    pub fn targeted_rgb<L: LightLevel>(&self) -> Option<RGB<L>> {
        if let Some(ref colour) = self.targeted_colour {
            Some(colour.rgb::<L>())
        } else {
            None
        }
    }

    #[cfg(feature = "targeted_mixtures")]
    pub fn targeted_colour(&self) -> Option<HCV> {
        if let Some(colour) = self.targeted_colour {
            Some(colour)
        } else {
            None
        }
    }

    #[cfg(feature = "targeted_mixtures")]
    pub fn targeted_rgb_shape(&self) -> ColouredShape {
        let tooltip_text = format!("Target for: {}", self.tooltip_text());
        let id = self.targeted_rgb_id();
        ColouredShape::new(
            &self.targeted_colour.expect("programmer error"),
            &id,
            &tooltip_text,
            Shape::Circle,
        )
    }

    #[cfg(feature = "targeted_mixtures")]
    pub fn targeted_rgb_id(&self) -> String {
        format!("TARGET({})", self.name)
    }

    pub fn components(&self) -> impl Iterator<Item = &(RangePaint, u64)> {
        self.components.iter()
    }
}

impl TooltipText for Mixture {
    fn tooltip_text(&self) -> String {
        let mut string = self.label_text();
        if !self.notes.is_empty() {
            string.push('\n');
            string.push_str(&self.notes);
        };

        string
    }
}

impl LabelText for Mixture {
    fn label_text(&self) -> String {
        format!("Mix {}: {}", self.id, self.name)
    }
}

impl MakeColouredShape for Mixture {
    fn coloured_shape(&self) -> ColouredShape {
        let tooltip_text = self.tooltip_text();
        ColouredShape::new(&self.colour, &self.id, &tooltip_text, Shape::Diamond)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MixingSession {
    notes: String,
    mixtures: Vec<Mixture>,
}

impl MixingSession {
    pub fn new() -> Self {
        Self {
            notes: "".to_string(),
            mixtures: Vec::new(),
        }
    }

    pub fn notes(&self) -> &str {
        &self.notes
    }

    pub fn set_notes(&mut self, notes: &str) {
        self.notes = notes.to_string()
    }

    pub fn mixtures(&self) -> impl Iterator<Item = &Mixture> {
        self.mixtures.iter()
    }

    pub fn series_paints(&self) -> Vec<RangePaint> {
        let mut v: Vec<RangePaint> = vec![];

        for mixture in self.mixtures.iter() {
            for (colln_paint, _parts) in mixture.components.iter() {
                match v.binary_search_by_key(&colln_paint.key(), |p| p.key()) {
                    Ok(_) => (),
                    Err(index) => v.insert(index, colln_paint.clone()),
                }
            }
        }

        v
    }

    pub fn add_mixture(&mut self, mixture: Mixture) -> Option<Mixture> {
        debug_assert!(self.is_sorted_unique());
        match self
            .mixtures
            .binary_search_by_key(&mixture.id(), |m| m.id())
        {
            Ok(index) => {
                self.mixtures.push(mixture);
                let old = self.mixtures.swap_remove(index);
                debug_assert!(self.is_sorted_unique());
                Some(old)
            }
            Err(index) => {
                self.mixtures.insert(index, mixture);
                None
            }
        }
    }

    pub fn mixture(&self, id: &str) -> Option<&Mixture> {
        debug_assert!(self.is_sorted_unique());
        let index = self.mixtures.binary_search_by_key(&id, |p| p.id()).ok()?;
        self.mixtures.get(index)
    }

    pub fn is_sorted_unique(&self) -> bool {
        self.mixtures
            .windows(2)
            .all(|pair| pair[0].id <= pair[1].id)
    }
}

impl MixingSession {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self, crate::Error> {
        let mut string = String::new();
        reader.read_to_string(&mut string)?;
        let session: Self = serde_json::from_str(&string)?;
        Ok(session)
    }
}

impl MixingSession {
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<Vec<u8>, crate::Error> {
        let mut hasher = Hasher::new(Algorithm::SHA256);
        let json_text = serde_json::to_string_pretty(self)?;
        hasher.write_all(json_text.as_bytes())?;
        let digest = hasher.finish();
        writer.write_all(json_text.as_bytes())?;
        Ok(digest)
    }

    pub fn digest(&self) -> Result<Vec<u8>, crate::Error> {
        let mut hasher = Hasher::new(Algorithm::SHA256);
        let json_text = serde_json::to_string_pretty(self)?;
        hasher.write_all(json_text.as_bytes())?;
        Ok(hasher.finish())
    }
}

#[derive(Debug)]
pub struct MixtureBuilder {
    id: String,
    name: String,
    notes: String,
    series_components: Vec<(RangePaint, u64)>,
    #[cfg(feature = "targeted_mixtures")]
    targeted_colour: Option<HCV>,
}

impl MixtureBuilder {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            name: String::new(),
            notes: String::new(),
            series_components: vec![],
            #[cfg(feature = "targeted_mixtures")]
            targeted_colour: None,
        }
    }

    pub fn id(&mut self, id: &str) -> &mut Self {
        self.id = id.to_string();
        self
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }

    pub fn notes(&mut self, notes: &str) -> &mut Self {
        self.notes = notes.to_string();
        self
    }

    #[cfg(feature = "targeted_mixtures")]
    pub fn targeted_colour(&mut self, colour: &impl ColourBasics) -> &mut Self {
        self.targeted_colour = Some(colour.hcv());
        self
    }

    pub fn paint_components(&mut self, components: Vec<(RangePaint, u64)>) -> &mut Self {
        self.series_components = components;
        self
    }

    pub fn paint_component(&mut self, component: (RangePaint, u64)) -> &mut Self {
        self.series_components.push(component);
        self
    }

    pub fn build(&self) -> Mixture {
        debug_assert!(self.series_components.len() > 0);
        let mut gcd: u64 = 0;
        for (_, parts) in self.series_components.iter() {
            debug_assert!(*parts > 0);
            gcd = gcd.gcd(*parts);
        }
        debug_assert!(gcd > 0);
        let mut components = vec![];
        let mut colour_mix = SubtractiveMixer::new();
        let mut properties_mixer = PropertiesMixer::default();
        for (colln_paint, parts) in self.series_components.iter() {
            let adjusted_parts = *parts / gcd;
            colour_mix.add(&colln_paint.hcv(), adjusted_parts);
            properties_mixer.add(&colln_paint.paint.properties, adjusted_parts);
            components.push((colln_paint.clone(), adjusted_parts));
        }
        Mixture {
            id: self.id.to_string(),
            colour: colour_mix.mixed_colour().unwrap(),
            #[cfg(feature = "targeted_mixtures")]
            targeted_colour: self.targeted_colour,
            name: self.name.clone(),
            // series_id: self.series_id.clone(),
            notes: self.notes.clone(),
            properties: properties_mixer.mixed_properties(),
            components,
        }
    }
}

#[cfg(test)]
mod test {
    use colour_math::HCV;
    use colour_math::HueConstants;

    use crate::paint::Paint;
    use crate::properties::{Properties, Property, PropertyType};

    use crate::mixtures::{MixingSession, MixtureBuilder};
    use crate::range::PaintRange;

    #[test]
    fn test_read_write_paint_series() {
        let mut paint_series = PaintRange::default();
        paint_series.set_proprietor("owner");
        paint_series.set_range_name("range name");
        assert!(paint_series.range_paints().next().is_none());
        paint_series.add(Paint {
            #[cfg(feature = "paints_have_ids")]
            id: "red".to_string(),
            colour: HCV::RED,
            name: "red".to_string(),
            notes: "whatever".to_string(),
            properties: Properties(vec![Property::from((PropertyType::Transparency, 1.0))]),
        });
        paint_series.add(Paint {
            #[cfg(feature = "paints_have_ids")]
            id: "yellow".to_string(),
            colour: HCV::YELLOW,
            name: "yellow".to_string(),
            notes: "whatever".to_string(),
            properties: Properties(vec![Property::from((PropertyType::Transparency, 2.0))]),
        });
        let mut session: MixingSession = MixingSession::new();
        session.set_notes("a test mixing session");
        let yellow = paint_series.get_range_paint("yellow").unwrap();
        let red = paint_series.get_range_paint("red").unwrap();
        let mix = vec![(red.clone(), 1), (yellow.clone(), 1)];
        let mixture = MixtureBuilder::new("#001")
            .paint_components(mix)
            .name("orange")
            .build();
        assert_eq!(mixture.colour, HCV::RED_YELLOW);
        session.add_mixture(mixture);
        let mixture = MixtureBuilder::new("#002")
            .paint_component((yellow.clone(), 1))
            .paint_component((red.clone(), 2))
            .name("reddish_orange")
            .build();
        session.add_mixture(mixture);
        let mut buffer: Vec<u8> = vec![];
        let digest = session.write(&mut buffer).unwrap();
        let read_session = MixingSession::read(&mut &buffer[..]).unwrap();
        assert_eq!(digest, read_session.digest().unwrap());
        assert_eq!(session.notes(), read_session.notes());
        assert_eq!(session.mixtures.len(), read_session.mixtures.len());
        for (mix1, mix2) in session.mixtures().zip(read_session.mixtures()) {
            assert_eq!(mix1.name, mix2.name);
        }
    }
}
