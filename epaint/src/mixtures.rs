// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::{
    cmp::Ordering,
    io::{Read, Write},
    rc::Rc,
};

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

use crate::paint::Paint;
use crate::properties::{Properties, PropertiesMixer, Property, PropertyType};
use crate::series::PaintFinder;
use crate::{AbbrevKey, GetSeriesId, LabelText, SeriesId, TooltipText};

pub trait MixtureIfce {
    fn id(&self) -> &str;

    fn name(&self) -> &str;

    fn notes(&self) -> &str;

    fn colour(&self) -> HCV;

    fn iter_property_types(&self) -> impl Iterator<Item = PropertyType>;

    fn iter_properties(&self) -> impl Iterator<Item = Property>;

    #[cfg(feature = "targeted_mixtures")]
    fn targeted_colour(&self) -> Option<HCV>;
    fn components(&self) -> impl Iterator<Item = (Rc<Paint>, u64)>;

    #[cfg(feature = "targeted_mixtures")]
    fn targeted_rgb<L: LightLevel>(&self) -> Option<RGB<L>> {
        if let Some(ref colour) = self.targeted_colour() {
            return Some(colour.rgb::<L>());
        }
        None
    }
}

#[derive(Debug, Colour, Clone)]
pub struct Mixture {
    pub id: String,
    #[colour]
    pub colour: HCV,
    #[cfg(feature = "targeted_mixtures")]
    pub targeted_colour: Option<HCV>,
    pub name: String,
    pub notes: String,
    pub properties: Properties,
    pub series_id: Rc<SeriesId>,
    pub components: Vec<(Rc<Paint>, u64)>,
}

impl AbbrevKey for Mixture {
    fn abbrev_key(&self) -> &str {
        &self.id
    }
}

impl MixtureIfce for Mixture {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn notes(&self) -> &str {
        &self.notes
    }

    fn colour(&self) -> HCV {
        self.colour.clone()
    }

    fn iter_property_types(&self) -> impl Iterator<Item = PropertyType> {
        self.properties.iter_property_types()
    }

    fn iter_properties(&self) -> impl Iterator<Item = Property> {
        self.properties.properties()
    }

    #[cfg(feature = "targeted_mixtures")]
    fn targeted_colour(&self) -> Option<HCV> {
        self.targeted_colour.into()
    }

    fn components(&self) -> impl Iterator<Item = (Rc<Paint>, u64)> {
        self.components.iter().map(|(rc, p)| (rc.clone(), *p))
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

    pub fn components(&self) -> impl Iterator<Item = &(Rc<Paint>, u64)> {
        self.components.iter()
    }
}

impl GetSeriesId for Mixture {
    fn series_id(&self) -> Rc<SeriesId> {
        self.series_id.clone()
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
        ColouredShape::new(&self.colour, &self.name, &tooltip_text, Shape::Diamond)
    }
}

impl PartialEq for Mixture {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.series_id == other.series_id
    }
}

impl Eq for Mixture {}

impl PartialOrd for Mixture {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.id.cmp(&other.id) {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Equal => match self.series_id.cmp(&other.series_id) {
                Ordering::Less => Some(Ordering::Less),
                Ordering::Greater => Some(Ordering::Greater),
                Ordering::Equal => Some(Ordering::Equal),
            },
        }
    }
}

impl Ord for Mixture {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug)]
pub struct MixingSession {
    notes: String,
    mixtures: Vec<Rc<Mixture>>,
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

    pub fn mixtures(&self) -> impl Iterator<Item = &Rc<Mixture>> {
        self.mixtures.iter()
    }

    pub fn series_paints(&self) -> Vec<Rc<Paint>> {
        let mut v: Vec<Rc<Paint>> = vec![];

        for mixture in self.mixtures.iter() {
            for (paint, _parts) in mixture.components.iter() {
                match v.binary_search_by_key(&paint.key(), |p: &Rc<Paint>| p.key()) {
                    Ok(_) => (),
                    Err(index) => v.insert(index, Rc::clone(paint)),
                }
            }
        }

        v
    }

    pub fn add_mixture(&mut self, mixture: &Rc<Mixture>) -> Option<Rc<Mixture>> {
        debug_assert!(self.is_sorted_unique());
        match self
            .mixtures
            .binary_search_by_key(&mixture.id(), |m| m.id())
        {
            Ok(index) => {
                self.mixtures.push(Rc::clone(mixture));
                let old = self.mixtures.swap_remove(index);
                debug_assert!(self.is_sorted_unique());
                Some(old)
            }
            Err(index) => {
                self.mixtures.insert(index, Rc::clone(mixture));
                None
            }
        }
    }

    pub fn mixture(&self, id: &str) -> Option<&Rc<Mixture>> {
        debug_assert!(self.is_sorted_unique());
        match self.mixtures.binary_search_by_key(&id, |p| p.id()) {
            Ok(index) => self.mixtures.get(index),
            Err(_) => None,
        }
    }

    pub fn is_sorted_unique(&self) -> bool {
        while let Some(pair) = self.mixtures.windows(2).next() {
            if pair[0] >= pair[1] {
                return false;
            }
        }
        true
    }
}

impl MixingSession {
    pub fn read<R: Read>(
        reader: &mut R,
        series_paint_finder: &Rc<impl PaintFinder>,
    ) -> Result<Self, crate::Error> {
        let saved_session = SaveableMixingSession::read(reader)?;
        let mixing_session = saved_session.mixing_session(series_paint_finder)?;
        Ok(mixing_session)
    }
}

impl MixingSession {
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<Vec<u8>, crate::Error> {
        SaveableMixingSession::from(self).write(writer)
    }

    pub fn digest(&self) -> Result<Vec<u8>, crate::Error> {
        SaveableMixingSession::from(self).digest()
    }
}

#[derive(Debug)]
pub struct MixtureBuilder {
    id: String,
    name: String,
    series_id: Rc<SeriesId>,
    notes: String,
    series_components: Vec<(Rc<Paint>, u64)>,
    #[cfg(feature = "targeted_mixtures")]
    targeted_colour: Option<HCV>,
}

impl MixtureBuilder {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            name: String::new(),
            series_id: Rc::<SeriesId>::default(),
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

    pub fn paint_components(&mut self, components: Vec<(Rc<Paint>, u64)>) -> &mut Self {
        self.series_components = components;
        self
    }

    pub fn paint_component(&mut self, component: (Rc<Paint>, u64)) -> &mut Self {
        self.series_components.push(component);
        self
    }

    pub fn build(&self) -> Rc<Mixture> {
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
        for (paint, parts) in self.series_components.iter() {
            let adjusted_parts = *parts / gcd;
            colour_mix.add(&paint.hcv(), adjusted_parts);
            properties_mixer.add(&paint.data.properties, adjusted_parts);
            components.push((Rc::clone(paint), adjusted_parts));
        }
        let mp = Mixture {
            id: self.id.to_string(),
            colour: colour_mix.mixed_colour().unwrap(),
            #[cfg(feature = "targeted_mixtures")]
            targeted_colour: self.targeted_colour,
            name: self.name.clone(),
            series_id: self.series_id.clone(),
            notes: self.notes.clone(),
            properties: properties_mixer.mixed_properties(),
            components,
        };
        Rc::new(mp)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveablePaint(SeriesId, String);

impl From<&Rc<Paint>> for SaveablePaint {
    fn from(paint: &Rc<Paint>) -> SaveablePaint {
        SaveablePaint((*paint.series_id()).clone(), paint.abbrev_key().to_string())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveableMixture {
    id: String,
    #[cfg(feature = "targeted_mixtures")]
    targeted_colour: Option<HCV>,
    name: String,
    notes: String,
    components: Vec<(SaveablePaint, u64)>,
}

impl From<&Rc<Mixture>> for SaveableMixture {
    fn from(rcmp: &Rc<Mixture>) -> Self {
        let components = rcmp
            .components
            .iter()
            .map(|(paint, parts)| (SaveablePaint::from(paint), *parts))
            .collect();
        Self {
            id: rcmp.id.to_string(),
            #[cfg(feature = "targeted_mixtures")]
            targeted_colour: rcmp.targeted_colour,
            name: rcmp.name.to_string(),
            notes: rcmp.notes.to_string(),
            components,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveableMixingSession {
    notes: String,
    mixtures: Vec<SaveableMixture>,
}

impl From<&MixingSession> for SaveableMixingSession {
    fn from(session: &MixingSession) -> Self {
        let mixtures = session.mixtures.iter().map(SaveableMixture::from).collect();
        Self {
            notes: session.notes.to_string(),
            mixtures,
        }
    }
}

impl SaveableMixingSession {
    pub fn notes(&self) -> &str {
        &self.notes
    }

    pub fn mixtures(&self) -> impl Iterator<Item = &SaveableMixture> {
        self.mixtures.iter()
    }

    // TODO: implement From for mixing session instead of this
    pub fn mixing_session(
        &self,
        series_paint_finder: &Rc<impl PaintFinder>,
    ) -> Result<MixingSession, crate::Error> {
        let mut mixtures: Vec<Rc<Mixture>> = vec![];
        for saved_mixture in self.mixtures.iter() {
            let mut mixture_builder = MixtureBuilder::new(&saved_mixture.id);
            mixture_builder.id(&saved_mixture.id);
            mixture_builder.notes(&saved_mixture.notes);
            #[cfg(feature = "targeted_mixtures")]
            if let Some(targeted_colour) = saved_mixture.targeted_colour {
                mixture_builder.targeted_colour(&targeted_colour);
            }
            for saved_component in saved_mixture.components.iter() {
                let paint = series_paint_finder
                    .get_paint(&saved_component.0.1, Some(&saved_component.0.0))?;
                mixture_builder.paint_component((paint, saved_component.1));
            }
            let mixture = mixture_builder.build();
            mixtures.push(mixture);
        }
        Ok(MixingSession {
            notes: self.notes.to_string(),
            mixtures,
        })
    }
}

impl SaveableMixingSession {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self, crate::Error> {
        let mut string = String::new();
        reader.read_to_string(&mut string)?;
        let session: Self = serde_json::from_str(&string)?;
        Ok(session)
    }
}

impl SaveableMixingSession {
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

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use colour_math::HCV;
    use colour_math::HueConstants;

    use crate::paint::SerializablePaintData;
    use crate::properties::{Properties, Property, PropertyType};

    use crate::mixtures::{MixingSession, MixtureBuilder, SaveableMixingSession};
    use crate::series::{PaintSeries, PaintSeriesSpec};

    #[test]
    fn test_read_write_spec() {
        let mut series_spec = PaintSeriesSpec::default();
        series_spec.set_proprietor("owner");
        series_spec.set_series_name("series name");
        assert!(series_spec.paints().next().is_none());
        series_spec.add(&SerializablePaintData {
            colour: HCV::RED,
            name: "red".to_string(),
            notes: "whatever".to_string(),
            properties: Properties(vec![Property::from((PropertyType::Transparency, 1.0))]),
        });
        series_spec.add(&SerializablePaintData {
            colour: HCV::YELLOW,
            name: "yellow".to_string(),
            notes: "whatever".to_string(),
            properties: Properties(vec![Property::from((PropertyType::Transparency, 2.0))]),
        });
        let series: PaintSeries = (&series_spec).into();
        let mut session: MixingSession = MixingSession::new();
        session.set_notes("a test mixing session");
        let yellow = series.find("yellow").unwrap();
        let red = series.find("red").unwrap();
        let mix = vec![(Rc::clone(&red), 1), (Rc::clone(&yellow), 1)];
        let mixture = MixtureBuilder::new("#001")
            .paint_components(mix)
            .name("orange")
            .build();
        assert_eq!(mixture.colour, HCV::RED_YELLOW);
        session.add_mixture(&mixture);
        let mixture = MixtureBuilder::new("#002")
            .paint_component((Rc::clone(&yellow), 1))
            .paint_component((Rc::clone(&red), 2))
            .name("reddish_orange")
            .build();
        session.add_mixture(&mixture);
        let saveable_session = SaveableMixingSession::from(&session);
        let mut buffer: Vec<u8> = vec![];
        let digest = saveable_session.write(&mut buffer).unwrap();
        let read_session = SaveableMixingSession::read(&mut &buffer[..]).unwrap();
        assert_eq!(digest, read_session.digest().unwrap());
        assert_eq!(session.notes(), read_session.notes());
        assert_eq!(session.mixtures.len(), read_session.mixtures.len());
        for (mix1, mix2) in saveable_session.mixtures().zip(read_session.mixtures()) {
            assert_eq!(mix1.name, mix2.name);
        }
    }
}
