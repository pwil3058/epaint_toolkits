// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::{
    cmp::Ordering,
    io::{Read, Write},
    rc::Rc,
};

use crypto_hash::{Algorithm, Hasher};
use gcd::Gcd;
use serde::{Deserialize, Serialize};

use colour_math::{
    ColourBasics, HCV, LightLevel, RGB,
    beigui::hue_wheel::{ColouredShape, MakeColouredShape, Shape},
    mixing::SubtractiveMixer,
};

use colour_math_derive::Colour;

use crate::paint::PropertiedPaint;
use crate::properties::PropertyType;
use crate::series::PaintFinder;
use crate::{
    LabelText, TooltipText, impl_paint_essential_ifce, paint::PaintEssentialsIfce, series::SeriesId,
};

// TODO: make an untargeted version of TargetedMixture
#[derive(Debug, Colour)]
pub struct Mixture<P: PropertiedPaint> {
    colour: HCV,
    // #[cfg(feature = "targeted_mixtures")]
    targeted_colour: Option<HCV>,
    name: String,
    notes: String,
    series_id: Rc<SeriesId>,
    property_variants_f64: Vec<f64>,
    components: Vec<(Rc<P>, u64)>,
}

impl<P: PropertiedPaint> Mixture<P> {
    // #[cfg(feature = "targeted_mixtures")]
    pub fn targeted_rgb<L: LightLevel>(&self) -> Option<RGB<L>> {
        if let Some(ref colour) = self.targeted_colour {
            Some(colour.rgb::<L>())
        } else {
            None
        }
    }

    // #[cfg(feature = "targeted_mixtures")]
    pub fn targeted_colour(&self) -> Option<HCV> {
        if let Some(colour) = self.targeted_colour {
            Some(colour)
        } else {
            None
        }
    }

    // #[cfg(feature = "targeted_mixtures")]
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

    // #[cfg(feature = "targeted_mixtures")]
    pub fn targeted_rgb_id(&self) -> String {
        format!("TARGET({})", self.name)
    }

    pub fn components(&self) -> impl Iterator<Item = &(Rc<P>, u64)> {
        self.components.iter()
    }
}

impl_paint_essential_ifce!(Mixture, P);

impl<P: PropertiedPaint> PropertiedPaint for Mixture<P> {
    const PROPERTY_TYPES: &'static [PropertyType] = P::PROPERTY_TYPES;

    fn property_variants_f64(&self) -> Vec<f64> {
        self.property_variants_f64.clone()
    }
}

impl<P: PropertiedPaint> TooltipText for Mixture<P> {
    fn tooltip_text(&self) -> String {
        let mut string = self.label_text();
        if !self.notes.is_empty() {
            string.push('\n');
            string.push_str(&self.notes);
        };

        string
    }
}

impl<P: PropertiedPaint> LabelText for Mixture<P> {
    fn label_text(&self) -> String {
        format!("Mix {}", self.name)
    }
}

impl<P: PropertiedPaint> MakeColouredShape for Mixture<P> {
    fn coloured_shape(&self) -> ColouredShape {
        let tooltip_text = self.tooltip_text();
        ColouredShape::new(&self.colour, &self.name, &tooltip_text, Shape::Diamond)
    }
}

impl<P: PropertiedPaint> PartialEq for Mixture<P> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.series_id == other.series_id
    }
}

impl<P: PropertiedPaint> Eq for Mixture<P> {}

impl<P: PropertiedPaint> PartialOrd for Mixture<P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.name.cmp(&other.name) {
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

impl<P: PropertiedPaint> Ord for Mixture<P> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug)]
pub struct MixingSession<P: PropertiedPaint> {
    notes: String,
    mixtures: Vec<Rc<Mixture<P>>>,
}

impl<P: PropertiedPaint> MixingSession<P> {
    pub fn new(notes: &str) -> Self {
        Self {
            notes: notes.to_string(),
            mixtures: Vec::new(),
        }
    }

    pub fn notes(&self) -> &str {
        &self.notes
    }

    pub fn set_notes(&mut self, notes: &str) {
        self.notes = notes.to_string()
    }

    pub fn mixtures(&self) -> impl Iterator<Item = &Rc<Mixture<P>>> {
        self.mixtures.iter()
    }

    pub fn series_paints(&self) -> Vec<Rc<P>> {
        let mut v: Vec<Rc<P>> = vec![];

        for mixture in self.mixtures.iter() {
            for (paint, _parts) in mixture.components.iter() {
                match v.binary_search_by_key(&(paint.name(), paint.series_id()), |p: &Rc<P>| {
                    (p.name(), p.series_id())
                }) {
                    Ok(_) => (),
                    Err(index) => v.insert(index, Rc::clone(paint)),
                }
            }
        }

        v
    }

    pub fn add_mixture(&mut self, mixture: &Rc<Mixture<P>>) -> Option<Rc<Mixture<P>>> {
        debug_assert!(self.is_sorted_unique());
        match self
            .mixtures
            .binary_search_by_key(&mixture.name(), |p| p.name())
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

    pub fn mixture(&self, name: &str) -> Option<&Rc<Mixture<P>>> {
        debug_assert!(self.is_sorted_unique());
        match self.mixtures.binary_search_by_key(&name, |p| p.name()) {
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

#[derive(Debug)]
pub struct MixtureBuilder<P: PropertiedPaint> {
    name: String,
    series_id: Rc<SeriesId>,
    notes: String,
    series_components: Vec<(Rc<P>, u64)>,
    variants_64: Vec<f64>,
    // #[cfg(feature = "targeted_mixtures")]
    targeted_colour: Option<HCV>,
}

impl<P: PropertiedPaint> MixtureBuilder<P> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            series_id: Rc::<SeriesId>::default(),
            notes: String::new(),
            series_components: vec![],
            variants_64: vec![],
            // #[cfg(feature = "targeted_mixtures")]
            targeted_colour: None,
        }
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }

    pub fn notes(&mut self, notes: &str) -> &mut Self {
        self.notes = notes.to_string();
        self
    }

    //#[cfg(feature = "targeted_mixtures")]
    pub fn targeted_colour(&mut self, colour: &impl ColourBasics) -> &mut Self {
        self.targeted_colour = Some(colour.hcv());
        self
    }

    pub fn series_paint_components(&mut self, components: Vec<(Rc<P>, u64)>) -> &mut Self {
        self.series_components = components;
        self
    }

    pub fn series_paint_component(&mut self, component: (Rc<P>, u64)) -> &mut Self {
        self.series_components.push(component);
        self
    }

    pub fn build(&self) -> Rc<Mixture<P>> {
        debug_assert!(self.series_components.len() > 0);
        let mut gcd: u64 = 0;
        for (_, parts) in self.series_components.iter() {
            debug_assert!(*parts > 0);
            gcd = gcd.gcd(*parts);
        }
        debug_assert!(gcd > 0);
        let mut components = vec![];
        let mut colour_mix = SubtractiveMixer::new();
        for (paint, parts) in self.series_components.iter() {
            let adjusted_parts = *parts / gcd;
            colour_mix.add(&paint.hcv(), adjusted_parts);
            // TODO: handle proerties
            components.push((Rc::clone(paint), adjusted_parts));
        }
        let mp = Mixture::<P> {
            colour: colour_mix.mixed_colour().unwrap(),
            // #[cfg(feature = "targeted_mixtures")]
            targeted_colour: self.targeted_colour,
            name: self.name.clone(),
            series_id: self.series_id.clone(),
            notes: self.notes.clone(),
            property_variants_f64: self.variants_64.clone(),
            // TODO: handle properties
            // properties: vec![],
            components,
        };
        Rc::new(mp)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveablePaint(SeriesId, String);

impl<P: PropertiedPaint> From<&Rc<P>> for SaveablePaint {
    fn from(paint: &Rc<P>) -> SaveablePaint {
        SaveablePaint((*paint.series_id()).clone(), paint.name().to_string())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveableMixture {
    // #[cfg(feature = "targeted_mixtures")]
    targeted_colour: Option<HCV>,
    name: String,
    notes: String,
    components: Vec<(SaveablePaint, u64)>,
}

impl<P: PropertiedPaint> From<&Rc<Mixture<P>>> for SaveableMixture {
    fn from(rcmp: &Rc<Mixture<P>>) -> Self {
        let components = rcmp
            .components
            .iter()
            .map(|(paint, parts)| (SaveablePaint::from(paint), *parts))
            .collect();
        Self {
            // #[cfg(feature = "targeted_mixtures")]
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

impl<P: PropertiedPaint> From<&MixingSession<P>> for SaveableMixingSession {
    fn from(session: &MixingSession<P>) -> Self {
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

    pub fn mixing_session<P: PropertiedPaint>(
        &self,
        series_paint_finder: &Rc<impl PaintFinder<P>>,
    ) -> Result<MixingSession<P>, crate::Error> {
        let mut mixtures: Vec<Rc<Mixture<P>>> = vec![];
        for saved_mixture in self.mixtures.iter() {
            let mut mixture_builder = MixtureBuilder::new(&saved_mixture.name);
            mixture_builder.name(&saved_mixture.name);
            mixture_builder.notes(&saved_mixture.notes);
            // #[cfg(feature = "targeted_mixtures")]
            if let Some(targeted_colour) = saved_mixture.targeted_colour {
                mixture_builder.targeted_colour(&targeted_colour);
            }
            for saved_component in saved_mixture.components.iter() {
                let paint = series_paint_finder
                    .get_paint(&saved_component.0.1, Some(&saved_component.0.0))?;
                mixture_builder.series_paint_component((paint, saved_component.1));
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

    use crate::mixtures::{MixingSession, MixtureBuilder, SaveableMixingSession};
    use crate::paint::{PaintEssentialsIfce, PropertiedPaint, SerializablePaintData};
    use crate::properties::PropertyType;
    use crate::series::{PaintSeriesSpec, SeriesId};
    use crate::{
        TooltipText, impl_paint_essential_ifce, implement_propertied_paint,
        realize_propertied_paint,
    };
    use colour_math::hue_wheel::{ColouredShape, MakeColouredShape, Shape};
    use colour_math::{HCV, HueConstants, LightLevel};
    use colour_math_derive::Colour;

    realize_propertied_paint!(TestPaint, &[PropertyType::Transparency]);

    impl MakeColouredShape for TestPaint {
        fn coloured_shape(&self) -> ColouredShape {
            let tooltip_text = self.tooltip_text();
            ColouredShape::new(&self.colour, &self.name, &tooltip_text, Shape::Square)
        }
    }

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
            property_variants_f64: vec![2.0],
        });
        series_spec.add(&SerializablePaintData {
            colour: HCV::YELLOW,
            name: "yellow".to_string(),
            notes: "whatever".to_string(),
            property_variants_f64: vec![2.0],
        });
        let series = Rc::new(series_spec.generate_paint_series::<TestPaint>());
        let mut session: MixingSession<TestPaint> = MixingSession::new("test session");
        session.set_notes("a test mixing session");
        let yellow = series.find("yellow").unwrap();
        let red = series.find("red").unwrap();
        let mix = vec![(Rc::clone(&red), 1), (Rc::clone(&yellow), 1)];
        let mixture = MixtureBuilder::new("#001")
            .series_paint_components(mix)
            .name("orange")
            .build();
        assert_eq!(mixture.colour, HCV::RED_YELLOW);
        session.add_mixture(&mixture);
        let mixture = MixtureBuilder::new("#002")
            .series_paint_component((Rc::clone(&yellow), 1))
            .series_paint_component((Rc::clone(&red), 2))
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
