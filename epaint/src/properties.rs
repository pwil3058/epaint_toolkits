// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

//! Types to describe paint properties that cannot be derived from their colour.

use epaint_derive::Property;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

pub trait PropertyConsts:
    FromStr<Err = String> + PartialEq + PartialOrd + Default + fmt::Debug
{
    const NAME: &'static str;
    const PROMPT: &'static str;
    const LIST_HEADER: &'static str;
    const VARIANT_STRS: &'static [&'static str];
    const ABBREV_VARIANT_STRS: &'static [&'static str];
}

pub trait PropertyFns: FromStr<Err = String> + PartialEq + PartialOrd + fmt::Debug {
    fn name(&self) -> &'static str;
    fn prompt(&self) -> &'static str;
    fn list_header(&self) -> &'static str;
    fn str_values() -> Vec<&'static str>;
    fn abbrev_value(&self) -> &'static str;
    fn value(&self) -> &'static str;
}

pub trait PropertyIfce:
    PropertyConsts + PropertyFns + Clone + Copy + FromStr + From<f64> + Default
{
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
pub enum Lightfastness {
    ExcellentLightfastness,
    #[default]
    VeryGoodLightfastness,
    FairLightfastness,
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
    ExtremelyPermanent,
    #[default]
    Permanent,
    ModeratelyDurable,
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
    None,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct PropertyTypes(pub Vec<PropertyType>);

impl PropertyTypes {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = PropertyType> {
        self.0.iter().copied()
    }
}

pub fn str_values(property: &PropertyType) -> Vec<&'static str> {
    match property {
        PropertyType::Transparency => Transparency::str_values(),
        PropertyType::Lightfastness => Lightfastness::str_values(),
        PropertyType::Staining => Staining::str_values(),
        PropertyType::Finish => Finish::str_values(),
        PropertyType::Opacity => Opacity::str_values(),
        PropertyType::Permanence => Permanence::str_values(),
        PropertyType::Luminescence => Luminescence::str_values(),
        PropertyType::Fluorescence => Fluorescence::str_values(),
        PropertyType::Metallicness => Metallicness::str_values(),
        PropertyType::Granulation => Granulation::str_values(),
    }
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

    pub fn value(&self, arg: f64) -> &'static str {
        match self {
            Self::Transparency => Transparency::from(arg).value(),
            Self::Lightfastness => Lightfastness::from(arg).value(),
            Self::Staining => Staining::from(arg).value(),
            Self::Finish => Finish::from(arg).value(),
            Self::Opacity => Opacity::from(arg).value(),
            Self::Permanence => Permanence::from(arg).value(),
            Self::Luminescence => Luminescence::from(arg).value(),
            Self::Fluorescence => Fluorescence::from(arg).value(),
            Self::Metallicness => Metallicness::from(arg).value(),
            Self::Granulation => Granulation::from(arg).value(),
        }
    }

    pub fn default_f64(&self) -> f64 {
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
        Property {
            property_type: *self,
            value: self.default_u64(),
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

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Property {
    pub property_type: PropertyType,
    pub value: u64,
}

impl Property {
    pub fn name(&self) -> &'static str {
        self.property_type.name()
    }

    pub fn prompt(&self) -> &'static str {
        self.property_type.prompt()
    }

    pub fn list_header(&self) -> &'static str {
        self.property_type.list_header()
    }

    pub fn abbrev_value(&self) -> &'static str {
        match self.property_type {
            PropertyType::Transparency => Transparency::from(self.value).abbrev_value(),
            PropertyType::Lightfastness => Lightfastness::from(self.value).abbrev_value(),
            PropertyType::Staining => Staining::from(self.value).abbrev_value(),
            PropertyType::Finish => Finish::from(self.value).abbrev_value(),
            PropertyType::Opacity => Opacity::from(self.value).abbrev_value(),
            PropertyType::Permanence => Permanence::from(self.value).abbrev_value(),
            PropertyType::Luminescence => Luminescence::from(self.value).abbrev_value(),
            PropertyType::Fluorescence => Fluorescence::from(self.value).abbrev_value(),
            PropertyType::Granulation => Granulation::from(self.value).abbrev_value(),
            PropertyType::Metallicness => Metallicness::from(self.value).abbrev_value(),
        }
    }

    pub fn value(&self) -> &'static str {
        match self.property_type {
            PropertyType::Transparency => Transparency::from(self.value).value(),
            PropertyType::Lightfastness => Lightfastness::from(self.value).value(),
            PropertyType::Staining => Staining::from(self.value).value(),
            PropertyType::Finish => Finish::from(self.value).value(),
            PropertyType::Opacity => Opacity::from(self.value).value(),
            PropertyType::Permanence => Permanence::from(self.value).value(),
            PropertyType::Luminescence => Luminescence::from(self.value).value(),
            PropertyType::Fluorescence => Fluorescence::from(self.value).value(),
            PropertyType::Metallicness => Metallicness::from(self.value).value(),
            PropertyType::Granulation => Granulation::from(self.value).value(),
        }
    }

    pub fn property_type(&self) -> PropertyType {
        self.property_type
    }

    pub fn default_f64(&self) -> f64 {
        self.property_type.default_f64()
    }
}

// impl PartialEq for Property {
//     fn eq(&self, other: &Self) -> bool {
//         if self.property_type == other.property_type {
//             self.value == other.value
//         } else {
//             false
//         }
//     }
// }
//
// impl Eq for Property {}

impl PartialOrd for Property {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        debug_assert_eq!(
            self.property_type, other.property_type,
            "attempt to compare properties of different types"
        );
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for Property {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
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

impl FromStr for Property {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut split = string.split("::");
        let type_name = split.next().unwrap();
        let property_type = PropertyType::from_str(type_name).unwrap();
        let result = match property_type {
            PropertyType::Transparency => prop_from_str_action!(Transparency, property_type, split),
            PropertyType::Lightfastness => {
                prop_from_str_action!(Lightfastness, property_type, split)
            }
            PropertyType::Fluorescence => prop_from_str_action!(Fluorescence, property_type, split),
            PropertyType::Finish => prop_from_str_action!(Finish, property_type, split),
            PropertyType::Staining => prop_from_str_action!(Staining, property_type, split),
            PropertyType::Opacity => prop_from_str_action!(Opacity, property_type, split),
            PropertyType::Permanence => prop_from_str_action!(Permanence, property_type, split),
            PropertyType::Luminescence => prop_from_str_action!(Luminescence, property_type, split),
            PropertyType::Granulation => prop_from_str_action!(Granulation, property_type, split),
            PropertyType::Metallicness => prop_from_str_action!(Metallicness, property_type, split),
        };
        debug_assert_eq!(split.next(), None);
        result
    }
}

impl From<(PropertyType, f64)> for Property {
    fn from((property_type, value): (PropertyType, f64)) -> Self {
        Self {
            property_type,
            value: value as u64,
        }
    }
}

impl From<(PropertyType, u64)> for Property {
    fn from((property_type, value): (PropertyType, u64)) -> Self {
        Self {
            property_type,
            value: value,
        }
    }
}

impl From<(PropertyType, &str)> for Property {
    fn from((property_type, value): (PropertyType, &str)) -> Self {
        let variant = match property_type {
            PropertyType::Transparency => Transparency::from_str(value).unwrap().into(),
            PropertyType::Lightfastness => Lightfastness::from_str(value).unwrap().into(),
            PropertyType::Staining => Staining::from_str(value).unwrap().into(),
            PropertyType::Finish => Finish::from_str(value).unwrap().into(),
            PropertyType::Opacity => Opacity::from_str(value).unwrap().into(),
            PropertyType::Permanence => Permanence::from_str(value).unwrap().into(),
            PropertyType::Luminescence => Luminescence::from_str(value).unwrap().into(),
            PropertyType::Metallicness => Metallicness::from_str(value).unwrap().into(),
            PropertyType::Granulation => Granulation::from_str(value).unwrap().into(),
            PropertyType::Fluorescence => Fluorescence::from_str(value).unwrap().into(),
        };
        Self {
            property_type,
            value: variant,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FuzzyProperty(f64, PropertyType);

#[derive(Debug)]
pub struct PropertyMixer {
    pub property_type: PropertyType,
    pub sum: u128,
    pub total_parts: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Properties(pub Vec<Property>);

impl Properties {
    pub fn iter(&self) -> impl Iterator<Item = Property> {
        self.0.iter().copied()
    }
}

impl Properties {
    pub fn new(vec: &[Property]) -> Self {
        Self(vec.to_vec())
    }

    pub fn is_compatible(&self, properties: &[Property]) -> bool {
        self.0.len() == properties.len()
            && self
                .0
                .iter()
                .zip(properties)
                .all(|(left, right)| left.property_type == right.property_type)
    }

    pub fn update(&mut self, properties: &[Property]) {
        debug_assert!(self.is_compatible(properties));
        Self(properties.to_vec());
    }

    pub fn property_types(&self) -> impl Iterator<Item = PropertyType> {
        self.0.iter().map(|p| p.property_type())
    }

    pub fn properties(&self) -> impl Iterator<Item = Property> {
        self.0.iter().copied()
    }
}

impl From<Properties> for PropertyTypes {
    fn from(properties: Properties) -> Self {
        PropertyTypes(properties.property_types().collect())
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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_property_type_real() {
        assert_eq!(
            PropertyType::Transparency.value(1.0),
            Transparency::Clear.value()
        );
        assert_eq!(
            PropertyType::Lightfastness.value(1.0),
            Lightfastness::ExcellentLightfastness.value()
        );
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
            Property::from_str("Lightfastness::ExcellentLightfastness"),
            Ok(Property {
                property_type: PropertyType::Lightfastness,
                value: 1
            })
        );
        assert_eq!(
            Property::from_str("Lightfastness::VeryGoodLightfastness"),
            Ok(Property {
                property_type: PropertyType::Lightfastness,
                value: 2
            })
        )
    }

    // Test objects that implement Property
    #[test]
    fn test_property_from_f64() {
        let transparency: Transparency = 1.0.into();
        assert_eq!(Transparency::Clear, transparency);
        assert_eq!(
            Transparency::SemiTransparent,
            Into::<Transparency>::into(3.0)
        );
        assert_eq!(
            Transparency::SemiTransparent,
            Into::<Transparency>::into(3.0)
        );
        assert_eq!(Transparency::SemiOpaque, Into::<Transparency>::into(4.0));
        assert_eq!(Transparency::Opaque, Into::<Transparency>::into(5.0));
    }

    #[test]
    fn test_property_default() {
        assert_eq!(Transparency::default(), Transparency::Transparent);
        assert_eq!(
            Lightfastness::default(),
            Lightfastness::VeryGoodLightfastness
        );
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
}
