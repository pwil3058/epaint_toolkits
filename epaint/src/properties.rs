// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

//! Types to describe paint properties that cannot be derived from their colour.

use epaint_derive::Property;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::{fmt, str::FromStr};

pub trait PropertyIfce:
    FromStr<Err = String> + PartialEq + Eq + PartialOrd + Ord + fmt::Debug
{
    const NAME: &'static str;
    const PROMPT: &'static str;
    const LIST_HEADER: &'static str;
    const VARIANT_STRS: &'static [&'static str];
    const ABBREV_VARIANT_STRS: &'static [&'static str];

    fn abbrev_value(&self) -> &'static str;
    fn value(&self) -> &'static str;
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Property)]
pub enum Transparency {
    Clear,
    #[default]
    Transparent,
    SemiTransparent,
    SemiOpaque,
    Opaque,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Property)]
#[list_header = "Lf."]
pub enum Lightfastness {
    #[abbreviation = "I"]
    Excellent,
    #[default]
    #[abbreviation = "II"]
    VeryGood,
    #[abbreviation = "III"]
    Fair,
    #[abbreviation = "IV"]
    Fugitive,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Property)]
pub enum Staining {
    HighStaining,
    #[default]
    ModerateStaining,
    LowStaining,
    NonStaining,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Property)]
pub enum Finish {
    Gloss,
    SemiGloss,
    SemiFlat,
    Flat,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Property)]
pub enum Opacity {
    Opaque,
    SemiOpaque,
    SemiTransparent,
    Transparent,
    Clear,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Property)]
pub enum Permanence {
    #[abbreviation = "AA"]
    ExtremelyPermanent,
    #[default]
    #[abbreviation = "A"]
    Permanent,
    #[abbreviation = "B"]
    ModeratelyDurable,
    #[abbreviation = "C"]
    Fugitive,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Property)]
pub enum Fluorescence {
    Fluorescent,
    SemiFluorescent,
    SemiNonFluorescent,
    #[default]
    NonFluorescent,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Property)]
pub enum Metallicness {
    Metal,
    Metallic,
    SemiMetallic,
    #[default]
    NonMetallic,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Property)]
pub enum Granulation {
    Granulating,
    SomeGranulation,
    #[default]
    NonGranulating,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Property)]
pub enum Luminescence {
    Luminescent,
    SemiLuminescent,
    #[default]
    NonLuminescent,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum PropertyType {
    Transparency,
    Lightfastness,
    Staining,
    Finish,
    Opacity,
    Permanence,
    Luminescence,
    Fluorescence,
    Metallicness,
    Granulation,
}

impl PropertyType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Transparency => Transparency::NAME,
            Self::Lightfastness => Lightfastness::NAME,
            Self::Staining => Staining::NAME,
            Self::Finish => Finish::NAME,
            Self::Opacity => Opacity::NAME,
            Self::Permanence => Permanence::NAME,
            Self::Luminescence => Luminescence::NAME,
            Self::Fluorescence => Fluorescence::NAME,
            Self::Metallicness => Metallicness::NAME,
            Self::Granulation => Granulation::NAME,
        }
    }

    pub fn prompt(&self) -> &'static str {
        match self {
            Self::Transparency => Transparency::PROMPT,
            Self::Lightfastness => Lightfastness::PROMPT,
            Self::Staining => Staining::PROMPT,
            Self::Finish => Finish::PROMPT,
            Self::Opacity => Opacity::PROMPT,
            Self::Permanence => Permanence::PROMPT,
            Self::Luminescence => Luminescence::PROMPT,
            Self::Fluorescence => Fluorescence::PROMPT,
            Self::Metallicness => Metallicness::PROMPT,
            Self::Granulation => Granulation::PROMPT,
        }
    }

    pub fn list_header(&self) -> &'static str {
        match self {
            Self::Transparency => Transparency::LIST_HEADER,
            Self::Lightfastness => Lightfastness::LIST_HEADER,
            Self::Staining => Staining::LIST_HEADER,
            Self::Finish => Finish::LIST_HEADER,
            Self::Opacity => Opacity::LIST_HEADER,
            Self::Permanence => Permanence::LIST_HEADER,
            Self::Luminescence => Luminescence::LIST_HEADER,
            Self::Fluorescence => Fluorescence::LIST_HEADER,
            Self::Metallicness => Metallicness::LIST_HEADER,
            Self::Granulation => Granulation::LIST_HEADER,
        }
    }

    pub fn variant_strings(&self) -> impl Iterator<Item = &'static str> {
        match self {
            Self::Transparency => Transparency::VARIANT_STRS.iter().copied(),
            Self::Lightfastness => Lightfastness::VARIANT_STRS.iter().copied(),
            Self::Staining => Staining::VARIANT_STRS.iter().copied(),
            Self::Finish => Finish::VARIANT_STRS.iter().copied(),
            Self::Opacity => Opacity::VARIANT_STRS.iter().copied(),
            Self::Permanence => Permanence::VARIANT_STRS.iter().copied(),
            Self::Luminescence => Luminescence::VARIANT_STRS.iter().copied(),
            Self::Fluorescence => Fluorescence::VARIANT_STRS.iter().copied(),
            Self::Metallicness => Metallicness::VARIANT_STRS.iter().copied(),
            Self::Granulation => Granulation::VARIANT_STRS.iter().copied(),
        }
    }

    pub fn default_u64(&self) -> u64 {
        match self {
            Self::Transparency => Transparency::default().into(),
            Self::Lightfastness => Lightfastness::default().into(),
            Self::Staining => Staining::default().into(),
            Self::Finish => Finish::default().into(),
            Self::Opacity => Opacity::default().into(),
            Self::Permanence => Permanence::default().into(),
            Self::Luminescence => Luminescence::default().into(),
            Self::Fluorescence => Fluorescence::default().into(),
            Self::Metallicness => Metallicness::default().into(),
            Self::Granulation => Granulation::default().into(),
        }
    }

    pub fn default_str(&self) -> &'static str {
        match self {
            Self::Transparency => Transparency::default().value(),
            Self::Lightfastness => Lightfastness::default().value(),
            Self::Staining => Staining::default().value(),
            Self::Finish => Finish::default().value(),
            Self::Opacity => Opacity::default().value(),
            Self::Permanence => Permanence::default().value(),
            Self::Luminescence => Luminescence::default().value(),
            Self::Fluorescence => Fluorescence::default().value(),
            Self::Metallicness => Metallicness::default().value(),
            Self::Granulation => Granulation::default().value(),
        }
    }

    pub fn default_property(&self) -> Property {
        match self {
            PropertyType::Transparency => Property::Transparency(Transparency::default()),
            PropertyType::Lightfastness => Property::Lightfastness(Lightfastness::default()),
            PropertyType::Staining => Property::Staining(Staining::default()),
            PropertyType::Finish => Property::Finish(Finish::default()),
            PropertyType::Opacity => Property::Opacity(Opacity::default()),
            PropertyType::Permanence => Property::Permanence(Permanence::default()),
            PropertyType::Luminescence => Property::Luminescence(Luminescence::default()),
            PropertyType::Fluorescence => Property::Fluorescence(Fluorescence::default()),
            PropertyType::Metallicness => Property::Metallicness(Metallicness::default()),
            PropertyType::Granulation => Property::Granulation(Granulation::default()),
        }
    }
}

impl std::str::FromStr for PropertyType {
    type Err = String;

    fn from_str(string: &str) -> Result<PropertyType, String> {
        match string {
            "Transparency" => Ok(Self::Transparency),
            "Lightfastness" => Ok(Self::Lightfastness),
            "Staining" => Ok(Self::Staining),
            "Finish" => Ok(Self::Finish),
            "Opacity" => Ok(Self::Opacity),
            "Permanence" => Ok(Self::Permanence),
            "Luminescence" => Ok(Self::Luminescence),
            "Fluorescence" => Ok(Self::Fluorescence),
            "Metallicness" => Ok(Self::Metallicness),
            "Granulation" => Ok(Self::Granulation),
            &_ => Err(format!("Unknown property type: {}", string)),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct PropertyTypes(pub Vec<PropertyType>);

impl PropertyTypes {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = PropertyType> {
        self.0.iter().copied()
    }
}

impl Deref for PropertyTypes {
    type Target = [PropertyType];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Property {
    Transparency(Transparency),
    Lightfastness(Lightfastness),
    Staining(Staining),
    Finish(Finish),
    Opacity(Opacity),
    Permanence(Permanence),
    Luminescence(Luminescence),
    Fluorescence(Fluorescence),
    Metallicness(Metallicness),
    Granulation(Granulation),
}

impl Property {
    pub fn name(&self) -> &'static str {
        self.property_type().name()
    }

    pub fn prompt(&self) -> &'static str {
        self.property_type().prompt()
    }

    pub fn list_header(&self) -> &'static str {
        self.property_type().list_header()
    }

    pub fn abbrev_value(&self) -> &'static str {
        use Property::*;
        match self {
            Transparency(transparency) => transparency.abbrev_value(),
            Lightfastness(lightfastness) => lightfastness.abbrev_value(),
            Staining(staining) => staining.abbrev_value(),
            Finish(finish) => finish.abbrev_value(),
            Opacity(opacity) => opacity.abbrev_value(),
            Permanence(permanence) => permanence.abbrev_value(),
            Luminescence(luminescence) => luminescence.abbrev_value(),
            Fluorescence(fluorescence) => fluorescence.abbrev_value(),
            Granulation(granulation) => granulation.abbrev_value(),
            Metallicness(metallic) => metallic.abbrev_value(),
        }
    }

    pub fn value(&self) -> &'static str {
        use Property::*;
        match self {
            Transparency(transparency) => transparency.value(),
            Lightfastness(lightfastness) => lightfastness.value(),
            Staining(staining) => staining.value(),
            Finish(finish) => finish.value(),
            Opacity(opacity) => opacity.value(),
            Permanence(permanence) => permanence.value(),
            Luminescence(luminescence) => luminescence.value(),
            Fluorescence(fluorescence) => fluorescence.value(),
            Metallicness(metallic) => metallic.value(),
            Granulation(granulation) => granulation.value(),
        }
    }

    pub fn u64_value(&self) -> u64 {
        use Property::*;
        match self {
            Transparency(transparency) => (*transparency).into(),
            Lightfastness(lightfastness) => (*lightfastness).into(),
            Staining(staining) => (*staining).into(),
            Finish(finish) => (*finish).into(),
            Opacity(opacity) => (*opacity).into(),
            Permanence(permanence) => (*permanence).into(),
            Luminescence(luminescence) => (*luminescence).into(),
            Fluorescence(fluorescence) => (*fluorescence).into(),
            Metallicness(metallic) => (*metallic).into(),
            Granulation(granulation) => (*granulation).into(),
        }
    }

    pub fn property_type(&self) -> PropertyType {
        use Property::*;
        match self {
            Transparency(_) => PropertyType::Transparency,
            Lightfastness(_) => PropertyType::Lightfastness,
            Staining(_) => PropertyType::Staining,
            Finish(_) => PropertyType::Finish,
            Opacity(_) => PropertyType::Opacity,
            Permanence(_) => PropertyType::Permanence,
            Luminescence(_) => PropertyType::Luminescence,
            Fluorescence(_) => PropertyType::Fluorescence,
            Metallicness(_) => PropertyType::Metallicness,
            Granulation(_) => PropertyType::Granulation,
        }
    }
}

impl PartialOrd for Property {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Property {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        debug_assert_eq!(
            self.property_type(),
            other.property_type(),
            "attempt to compare properties of different types"
        );
        self.value().cmp(&other.value())
    }
}

macro_rules! prop_from_str_action {
    ($variant: ident, $property_type: ident, $split: ident) => {{
        let value = if let Some(value) = $split.next() {
            value
        } else {
            $variant::default().value()
        };
        Ok(Self {
            $property_type,
            value: <$variant as Into<u64>>::into($variant::from_str(value)?).into(),
        })
    }};
}

// impl FromStr for Property {
//     type Err = String;
//
//     fn from_str(string: &str) -> Result<Self, Self::Err> {
//         let mut split = string.split("::");
//         let type_name = split.next().unwrap();
//         let property_type = PropertyType::from_str(type_name).unwrap();
//         let result = match property_type {
//             PropertyType::Transparency => prop_from_str_action!(Transparency, property_type, split),
//             PropertyType::Lightfastness => {
//                 prop_from_str_action!(Lightfastness, property_type, split)
//             }
//             PropertyType::Fluorescence => prop_from_str_action!(Fluorescence, property_type, split),
//             PropertyType::Finish => prop_from_str_action!(Finish, property_type, split),
//             PropertyType::Staining => prop_from_str_action!(Staining, property_type, split),
//             PropertyType::Opacity => prop_from_str_action!(Opacity, property_type, split),
//             PropertyType::Permanence => prop_from_str_action!(Permanence, property_type, split),
//             PropertyType::Luminescence => prop_from_str_action!(Luminescence, property_type, split),
//             PropertyType::Granulation => prop_from_str_action!(Granulation, property_type, split),
//             PropertyType::Metallicness => prop_from_str_action!(Metallicness, property_type, split),
//         };
//         debug_assert_eq!(split.next(), None);
//         result
//     }
// }

impl From<(PropertyType, u64)> for Property {
    fn from((property_type, value): (PropertyType, u64)) -> Self {
        match property_type {
            PropertyType::Transparency => Property::Transparency(Transparency::from(value)),
            PropertyType::Lightfastness => Property::Lightfastness(Lightfastness::from(value)),
            PropertyType::Fluorescence => Property::Fluorescence(Fluorescence::from(value)),
            PropertyType::Finish => Property::Finish(Finish::from(value)),
            PropertyType::Staining => Property::Staining(Staining::from(value)),
            PropertyType::Opacity => Property::Opacity(Opacity::from(value)),
            PropertyType::Permanence => Property::Permanence(Permanence::from(value)),
            PropertyType::Luminescence => Property::Luminescence(Luminescence::from(value)),
            PropertyType::Granulation => Property::Granulation(Granulation::from(value)),
            PropertyType::Metallicness => Property::Metallicness(Metallicness::from(value)),
        }
    }
}

impl From<(PropertyType, &str)> for Property {
    fn from((property_type, value): (PropertyType, &str)) -> Self {
        // let variant =
        match property_type {
            PropertyType::Transparency => {
                Property::Transparency(Transparency::from_str(value).unwrap().into())
            }
            PropertyType::Lightfastness => {
                Property::Lightfastness(Lightfastness::from_str(value).unwrap())
            }
            PropertyType::Fluorescence => {
                Property::Fluorescence(Fluorescence::from_str(value).unwrap())
            }
            PropertyType::Metallicness => {
                Property::Metallicness(Metallicness::from_str(value).unwrap().into())
            }
            PropertyType::Granulation => {
                Property::Granulation(Granulation::from_str(value).unwrap().into())
            }
            PropertyType::Luminescence => {
                Property::Luminescence(Luminescence::from_str(value).unwrap().into())
            }
            PropertyType::Granulation => {
                Property::Granulation(Granulation::from_str(value).unwrap().into())
            }
            PropertyType::Staining => Property::Staining(Staining::from_str(value).unwrap().into()),
            PropertyType::Opacity => Property::Opacity(Opacity::from_str(value).unwrap().into()),
            PropertyType::Permanence => {
                Property::Permanence(Permanence::from_str(value).unwrap().into())
            }
            PropertyType::Finish => Property::Finish(Finish::from_str(value).unwrap().into()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Properties(pub Vec<Property>);

impl Deref for Properties {
    type Target = [Property];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Properties {
    pub fn iter(&self) -> impl Iterator<Item = Property> {
        self.0.iter().copied()
    }

    pub fn new(vec: &[Property]) -> Self {
        Self(vec.to_vec())
    }

    pub fn is_compatible(&self, properties: &[Property]) -> bool {
        self.0.len() == properties.len()
            && self
                .0
                .iter()
                .zip(properties)
                .all(|(left, right)| left.property_type() == right.property_type())
    }

    pub fn update(&mut self, properties: &[Property]) {
        debug_assert!(self.is_compatible(properties));
        Self(properties.to_vec());
    }

    pub fn property_types(&self) -> PropertyTypes {
        PropertyTypes(self.0.iter().map(|p| p.property_type()).collect())
    }

    pub fn get_property(&self, property_type: PropertyType) -> Option<Property> {
        self.0
            .iter()
            .copied()
            .find(|&property| property.property_type() == property_type)
    }

    pub fn iter_property_types(&self) -> impl Iterator<Item = PropertyType> {
        self.0.iter().map(|p| p.property_type())
    }

    pub fn property_variants_u64(&self) -> Vec<u64> {
        self.0.iter().map(|p| p.u64_value()).collect()
    }
}

impl From<Properties> for PropertyTypes {
    fn from(properties: Properties) -> Self {
        properties.property_types()
    }
}

impl From<&PropertyTypes> for Properties {
    fn from(property_types: &PropertyTypes) -> Self {
        Self(
            property_types
                .0
                .iter()
                .map(|t| t.default_property())
                .collect(),
        )
    }
}

#[derive(Debug, Default)]
pub struct PropertiesMixer {
    pub property_types: PropertyTypes,
    pub sums: Vec<u64>,
    pub total_parts: u64,
}

impl PropertiesMixer {
    pub fn new(property_types: &PropertyTypes) -> Self {
        Self {
            property_types: property_types.clone(),
            sums: Vec::with_capacity(property_types.len()),
            total_parts: 0,
        }
    }

    pub fn add(&mut self, properties: &Properties, parts: u64) {
        if self.property_types.0.is_empty() {
            self.property_types = properties.property_types();
            self.sums = properties
                .property_variants_u64()
                .iter()
                .copied()
                .map(|u| u * parts)
                .collect();
            self.total_parts = parts;
        } else {
            let variant_64s = properties.property_variants_u64();
            debug_assert_eq!(variant_64s.len(), self.property_types.len());
            for (sum, value) in self.sums.iter_mut().zip(variant_64s.iter()) {
                *sum += value;
            }
            self.total_parts += parts;
        }
    }

    pub fn mixed_properties(&self) -> Properties {
        let mut properties = Vec::new();
        for property in self
            .property_types
            .0
            .iter()
            .zip(self.sums.iter())
            .map(|(t, v)| Property::from((*t, v / self.total_parts)))
        {
            properties.push(property)
        }
        Properties(properties)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::properties::{Lightfastness, Property, PropertyType, Transparency};

    #[test]
    fn test_property_type() {
        assert_eq!(
            PropertyType::Transparency,
            PropertyType::from_str("Transparency").unwrap()
        );
        assert_eq!(
            PropertyType::Lightfastness,
            PropertyType::from_str("Lightfastness").unwrap()
        )
    }

    #[test]
    fn test_split() {
        let mut split = "Transparency::Transparent".split("::");
        assert_eq!(split.next().unwrap(), "Transparency");
        assert_eq!(split.next().unwrap(), "Transparent");
    }

    #[test]
    fn test_property_from_string() {
        assert_eq!(
            Property::from_str("Lightfastness::Excellent"),
            Ok(Property {
                property_type: PropertyType::Lightfastness,
                value: 1
            })
        );
        assert_eq!(
            Property::from_str("Lightfastness::VeryGood"),
            Ok(Property {
                property_type: PropertyType::Lightfastness,
                value: 2
            })
        )
    }

    #[test]
    fn test_property_default() {
        assert_eq!(Transparency::default(), Transparency::Transparent);
        assert_eq!(Lightfastness::default(), Lightfastness::VeryGood);
    }

    #[test]
    fn paint_transparency_property() {
        assert_eq!(Transparency::NAME, "Transparency");
        assert_eq!(Transparency::PROMPT, "Transparency:");
        assert_eq!(Transparency::Transparent.abbrev_value(), "T");
        for a in ["O", "SO", "ST", "C"].iter() {
            assert_eq!(Transparency::from_str(a).unwrap().abbrev_value(), *a);
        }
        for a in ["opaque", "semi-opaque", "semi-transparent", "clear"]
            .iter()
            .cloned()
        {
            assert_eq!(Transparency::from_str(a).unwrap().value(), a);
        }
    }

    #[test]
    fn defaults() {
        assert_eq!(Transparency::default(), Transparency::Transparent);
    }

    #[test]
    fn test_properties_iter() {
        let property_types = PropertyTypes(vec![
            PropertyType::Transparency,
            PropertyType::Lightfastness,
            PropertyType::Staining,
            PropertyType::Granulation,
        ]);
        let properties: Properties = (&property_types).into();
        for (property, property_type) in properties.iter().zip(property_types.iter()) {
            assert_eq!(property.property_type, property_type);
        }
    }
}
