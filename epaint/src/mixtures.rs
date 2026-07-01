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
    Angle, Chroma, ColourAttributes, ColourBasics, Greyness, HCV, Hue, LightLevel, Prop, RGB,
    Value, Warmth,
    beigui::hue_wheel::{ColouredShape, MakeColouredShape, Shape},
    mixing::SubtractiveMixer,
};

use colour_math_derive::Colour;

use crate::paint::{CompomentPaintIfce, PropertyTypes};
use crate::properties::PropertyType;
use crate::series::PaintFinder;
use crate::{LabelText, TooltipText, paint::PaintEssentialsIfce, series::SeriesId};

// TODO: make an untargeted version of TargetedMixture
#[derive(Debug, Colour)]
pub struct Mixture<P: CompomentPaintIfce> {
    colour: HCV,
    // #[cfg(feature = "targeted_mixtures")]
    targeted_colour: Option<HCV>,
    name: String,
    notes: String,
    series_id: Rc<SeriesId>,
    // properties: Vec<Property>,
    variants_64: Vec<f64>,
    components: Vec<(Paint<P>, u64)>,
}

impl<P: CompomentPaintIfce> Mixture<P> {
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

    pub fn components(&self) -> impl Iterator<Item = &(Paint<P>, u64)> {
        self.components.iter()
    }
}

impl<P: CompomentPaintIfce> PaintEssentialsIfce for Mixture<P> {
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

impl<P: CompomentPaintIfce> PropertyTypes for Mixture<P> {
    const PROPERTY_TYPES: &'static [PropertyType] = P::PROPERTY_TYPES;

    fn property_variants_f64(&self) -> Vec<f64> {
        self.variants_64.clone()
    }
}

impl<P: CompomentPaintIfce> TooltipText for Mixture<P> {
    fn tooltip_text(&self) -> String {
        let mut string = self.label_text();
        if !self.notes.is_empty() {
            string.push('\n');
            string.push_str(&self.notes);
        };

        string
    }
}

impl<P: CompomentPaintIfce> LabelText for Mixture<P> {
    fn label_text(&self) -> String {
        format!("Mix {}", self.name)
    }
}

impl<P: CompomentPaintIfce> MakeColouredShape for Mixture<P> {
    fn coloured_shape(&self) -> ColouredShape {
        let tooltip_text = self.tooltip_text();
        ColouredShape::new(&self.colour, &self.name, &tooltip_text, Shape::Diamond)
    }
}

impl<P: CompomentPaintIfce> PartialEq for Mixture<P> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.series_id == other.series_id
    }
}

impl<P: CompomentPaintIfce> Eq for Mixture<P> {}

impl<P: CompomentPaintIfce> PartialOrd for Mixture<P> {
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

impl<P: CompomentPaintIfce> Ord for Mixture<P> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug)]
pub struct MixingSession<P: CompomentPaintIfce> {
    notes: String,
    mixtures: Vec<Rc<Mixture<P>>>,
}

impl<P: CompomentPaintIfce> MixingSession<P> {
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
                if let Paint::Paint(series_paint) = paint {
                    match v.binary_search_by_key(&series_paint.name(), |p: &Rc<P>| p.name()) {
                        Ok(_) => (),
                        Err(index) => v.insert(index, Rc::clone(series_paint)),
                    }
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

impl<P: CompomentPaintIfce> MixingSession<P> {
    pub fn read<R: Read>(
        reader: &mut R,
        series_paint_finder: &Rc<impl PaintFinder<P>>,
    ) -> Result<Self, crate::Error> {
        let saved_session = SaveableMixingSession::read(reader)?;
        let mixing_session = saved_session.mixing_session(series_paint_finder)?;
        Ok(mixing_session)
    }
}

impl<P: CompomentPaintIfce> MixingSession<P> {
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<Vec<u8>, crate::Error> {
        SaveableMixingSession::from(self).write(writer)
    }

    pub fn digest(&self) -> Result<Vec<u8>, crate::Error> {
        SaveableMixingSession::from(self).digest()
    }
}

#[derive(Debug)]
pub struct MixtureBuilder<P: CompomentPaintIfce> {
    name: String,
    series_id: Rc<SeriesId>,
    notes: String,
    series_components: Vec<(Rc<P>, u64)>,
    mixture_components: Vec<(Rc<Mixture<P>>, u64)>,
    variants_64: Vec<f64>,
    // #[cfg(feature = "targeted_mixtures")]
    targeted_colour: Option<HCV>,
}

impl<P: CompomentPaintIfce> MixtureBuilder<P> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            series_id: Rc::new(SeriesId::default()),
            notes: String::new(),
            series_components: vec![],
            mixture_components: vec![],
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

    pub fn mixed_paint_components(&mut self, components: Vec<(Rc<Mixture<P>>, u64)>) -> &mut Self {
        self.mixture_components = components;
        self
    }

    pub fn mixed_paint_component(&mut self, component: (Rc<Mixture<P>>, u64)) -> &mut Self {
        self.mixture_components.push(component);
        self
    }

    pub fn build(&self) -> Rc<Mixture<P>> {
        debug_assert!((self.series_components.len() + self.mixture_components.len()) > 0);
        let mut gcd: u64 = 0;
        for (_, parts) in self.series_components.iter() {
            debug_assert!(*parts > 0);
            gcd = gcd.gcd(*parts);
        }
        for (_, parts) in self.mixture_components.iter() {
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
            components.push((Paint::Paint(Rc::clone(paint)), adjusted_parts));
        }
        for (paint, parts) in self.mixture_components.iter() {
            let adjusted_parts = *parts / gcd;
            colour_mix.add(&paint.hcv(), adjusted_parts);
            // TODO: handle proerties
            components.push((Paint::Mixed(Rc::clone(paint)), adjusted_parts));
        }
        let mp = Mixture::<P> {
            colour: colour_mix.mixed_colour().unwrap(),
            // #[cfg(feature = "targeted_mixtures")]
            targeted_colour: self.targeted_colour,
            name: self.name.clone(),
            series_id: self.series_id.clone(),
            notes: self.notes.clone(),
            variants_64: self.variants_64.clone(),
            // TODO: handle properties
            // properties: vec![],
            components,
        };
        Rc::new(mp)
    }
}

#[derive(Debug, PartialEq)]
pub enum Paint<P: CompomentPaintIfce> {
    Paint(Rc<P>),
    Mixed(Rc<Mixture<P>>),
}

impl<P: CompomentPaintIfce> PropertyTypes for Paint<P> {
    const PROPERTY_TYPES: &'static [PropertyType] = P::PROPERTY_TYPES;

    fn property_variants_f64(&self) -> Vec<f64> {
        match self {
            Paint::Paint(paint) => paint.property_variants_f64(),
            Paint::Mixed(paint) => paint.property_variants_f64(),
        }
    }
}

impl<P: CompomentPaintIfce> CompomentPaintIfce for Paint<P> {}

impl<P: CompomentPaintIfce> MakeColouredShape for Paint<P> {
    fn coloured_shape(&self) -> ColouredShape {
        match self {
            Paint::Paint(paint) => paint.coloured_shape(),
            Paint::Mixed(paint) => paint.coloured_shape(),
        }
    }
}

impl<P: CompomentPaintIfce> ColourBasics for Paint<P> {
    fn hue(&self) -> Option<Hue> {
        match self {
            Paint::Paint(paint) => paint.hue(),
            Paint::Mixed(paint) => paint.hue(),
        }
    }

    fn hue_angle(&self) -> Option<Angle> {
        match self {
            Paint::Paint(paint) => paint.hue_angle(),
            Paint::Mixed(paint) => paint.hue_angle(),
        }
    }

    fn hue_rgb<L: LightLevel>(&self) -> Option<RGB<L>> {
        match self {
            Paint::Paint(paint) => paint.hue_rgb::<L>(),
            Paint::Mixed(paint) => paint.hue_rgb::<L>(),
        }
    }

    fn hue_hcv(&self) -> Option<HCV> {
        match self {
            Paint::Paint(paint) => paint.hue_hcv(),
            Paint::Mixed(paint) => paint.hue_hcv(),
        }
    }

    fn is_grey(&self) -> bool {
        match self {
            Paint::Paint(paint) => paint.is_grey(),
            Paint::Mixed(paint) => paint.is_grey(),
        }
    }

    fn chroma(&self) -> Chroma {
        match self {
            Paint::Paint(paint) => paint.chroma(),
            Paint::Mixed(paint) => paint.chroma(),
        }
    }

    fn chroma_prop(&self) -> Prop {
        match self {
            Paint::Paint(paint) => paint.chroma_prop(),
            Paint::Mixed(paint) => paint.chroma_prop(),
        }
    }

    fn value(&self) -> Value {
        match self {
            Paint::Paint(paint) => paint.value(),
            Paint::Mixed(paint) => paint.value(),
        }
    }

    fn greyness(&self) -> Greyness {
        match self {
            Paint::Paint(paint) => paint.greyness(),
            Paint::Mixed(paint) => paint.greyness(),
        }
    }

    fn warmth(&self) -> Warmth {
        match self {
            Paint::Paint(paint) => paint.warmth(),
            Paint::Mixed(paint) => paint.warmth(),
        }
    }

    fn hcv(&self) -> HCV {
        match self {
            Paint::Paint(paint) => paint.hcv(),
            Paint::Mixed(paint) => paint.hcv(),
        }
    }

    fn rgb<L: LightLevel>(&self) -> RGB<L> {
        match self {
            Paint::Paint(paint) => paint.rgb(),
            Paint::Mixed(paint) => paint.rgb(),
        }
    }

    fn monochrome_hcv(&self) -> HCV {
        match self {
            Paint::Paint(paint) => paint.monochrome_hcv(),
            Paint::Mixed(paint) => paint.monochrome_hcv(),
        }
    }

    fn monochrome_rgb<L: LightLevel>(&self) -> RGB<L> {
        match self {
            Paint::Paint(paint) => paint.monochrome_rgb::<L>(),
            Paint::Mixed(paint) => paint.monochrome_rgb::<L>(),
        }
    }

    fn best_foreground(&self) -> HCV {
        match self {
            Paint::Paint(paint) => paint.best_foreground(),
            Paint::Mixed(paint) => paint.best_foreground(),
        }
    }
}

impl<P: CompomentPaintIfce> ColourAttributes for Paint<P> {}

impl<P: CompomentPaintIfce> PaintEssentialsIfce for Paint<P> {
    fn name(&self) -> &str {
        match self {
            Paint::Paint(paint) => paint.name(),
            Paint::Mixed(paint) => paint.name(),
        }
    }
    fn series_id(&self) -> Rc<SeriesId> {
        match self {
            Paint::Paint(paint) => paint.series_id(),
            Paint::Mixed(paint) => paint.series_id(),
        }
    }

    fn notes(&self) -> &str {
        match self {
            Paint::Paint(paint) => paint.notes(),
            Paint::Mixed(paint) => paint.notes(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SaveablePaint {
    Series(SeriesId, String),
    Mixed(String),
}

impl<P: CompomentPaintIfce> From<&Rc<P>> for SaveablePaint {
    fn from(paint: &Rc<P>) -> Self {
        SaveablePaint::Series(
            Rc::<SeriesId>::into_inner(paint.series_id()).unwrap(),
            paint.name().to_string(),
        )
    }
}

impl<P: CompomentPaintIfce> From<&Rc<Mixture<P>>> for SaveablePaint {
    fn from(paint: &Rc<Mixture<P>>) -> Self {
        SaveablePaint::Mixed(paint.name().to_string())
    }
}

impl<P: CompomentPaintIfce> From<&Paint<P>> for SaveablePaint {
    fn from(paint: &Paint<P>) -> Self {
        match paint {
            Paint::Paint(paint) => paint.into(),
            Paint::Mixed(paint) => paint.into(),
        }
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

impl<P: CompomentPaintIfce> From<&Rc<Mixture<P>>> for SaveableMixture {
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

impl<P: CompomentPaintIfce> From<&MixingSession<P>> for SaveableMixingSession {
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

    pub fn mixing_session<P: CompomentPaintIfce>(
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
                match &saved_component.0 {
                    SaveablePaint::Series(series_id, id) => {
                        let paint = series_paint_finder.get_paint(id, Some(series_id))?;
                        mixture_builder.series_paint_component((paint, saved_component.1));
                    }
                    SaveablePaint::Mixed(name) => {
                        match mixtures.binary_search_by_key(&name.as_str(), |p| p.name()) {
                            Ok(index) => {
                                let paint = mixtures.get(index).expect("binary searched index");
                                mixture_builder
                                    .mixed_paint_component((Rc::clone(paint), saved_component.1));
                            }
                            Err(_) => return Err(crate::Error::NotFound(name.to_string())),
                        }
                    }
                }
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
    use crate::paint::{CompomentPaintIfce, PaintEssentialsIfce, PaintSpec, PropertyTypes};
    use crate::properties::PropertyType;
    use crate::series::{PaintSeriesSpec, SeriesId};
    use crate::{TooltipText, impl_eq_for_paint, impl_ord_for_paint};
    use colour_math::hue_wheel::{ColouredShape, MakeColouredShape, Shape};
    use colour_math::{HCV, HueConstants, LightLevel};
    use colour_math_derive::Colour;

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

    impl From<(PaintSpec, Rc<SeriesId>)> for TestPaint {
        fn from(value: (PaintSpec, Rc<SeriesId>)) -> Self {
            TestPaint {
                name: value.0.name,
                notes: value.0.notes,
                colour: value.0.colour,
                series_id: value.1,
                variants_64: value.0.property_variants.clone(),
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

    impl CompomentPaintIfce for TestPaint {}

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
    fn save_and_recover() {
        let mut series_spec = PaintSeriesSpec::default();
        series_spec.set_proprietor("owner");
        series_spec.set_series_name("series name");
        assert!(series_spec.paints().next().is_none());
        series_spec.add(&PaintSpec {
            colour: HCV::RED,
            name: "red".to_string(),
            notes: "whatever".to_string(),
            property_variants: vec![2.0],
        });
        series_spec.add(&PaintSpec {
            colour: HCV::YELLOW,
            name: "yellow".to_string(),
            notes: "whatever".to_string(),
            property_variants: vec![2.0],
        });
        let series = Rc::new(series_spec.generate_paint_series::<TestPaint>());
        let mut session = MixingSession::new("test session");
        session.set_notes("a test mixing session");
        let red = series.find("red").unwrap();
        let yellow = series.find("red").unwrap();
        let mix = vec![(Rc::clone(&red), 1), (Rc::clone(&yellow), 1)];
        let mixture = MixtureBuilder::new("#001")
            .series_paint_components(mix)
            .name("orange")
            .build();
        session.add_mixture(&mixture);
        let saveable_session = SaveableMixingSession::from(&session);
        let mut buffer: Vec<u8> = vec![];
        let digest = session.write(&mut buffer).unwrap();
        let read_session = SaveableMixingSession::read(&mut &buffer[..]).unwrap();
        assert_eq!(digest, read_session.digest().unwrap());
        assert_eq!(session.notes(), read_session.notes());
        assert_eq!(session.mixtures.len(), read_session.mixtures.len());
        for (mix1, mix2) in saveable_session.mixtures().zip(read_session.mixtures()) {
            assert_eq!(mix1.name, mix2.name);
        }
    }
}
