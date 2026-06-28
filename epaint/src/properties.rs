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
pub enum LightFastness {
    Excellent,
    #[default]
    VeryGood,
    Fair,
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum PropertyType {
    Transparency,
    LightFastness,
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
            Self::LightFastness => LightFastness::NAME,
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
            Self::LightFastness => LightFastness::PROMPT,
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
            Self::LightFastness => LightFastness::PROMPT,
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
            Self::LightFastness => LightFastness::from(arg).value(),
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
            Self::LightFastness => LightFastness::default().into(),
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
}

impl std::str::FromStr for PropertyType {
    type Err = String;

    fn from_str(string: &str) -> Result<PropertyType, String> {
        match string {
            "Transparency" => Ok(Self::Transparency),
            "LightFastness" => Ok(Self::LightFastness),
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

#[derive(Clone, Copy, Debug)]
pub struct Property {
    pub property_type: PropertyType,
    pub value: f64,
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
            PropertyType::LightFastness => LightFastness::from(self.value).abbrev_value(),
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
            PropertyType::LightFastness => LightFastness::from(self.value).value(),
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

impl PartialEq for Property {
    fn eq(&self, other: &Self) -> bool {
        if self.property_type == other.property_type {
            self.value == other.value
        } else {
            false
        }
    }
}

impl Eq for Property {}

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

impl FromStr for Property {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut split = string.split("::");
        let type_name = split.next().unwrap();
        let property_type = PropertyType::from_str(type_name).unwrap();
        // TODO: write a declarative macro for this
        let result = match property_type {
            PropertyType::Transparency => {
                let value = if let Some(value) = split.next() {
                    value
                } else {
                    Transparency::default().value()
                };
                Ok(Self {
                    property_type,
                    value: <Transparency as Into<f64>>::into(Transparency::from_str(value)?).into(),
                })
            }
            PropertyType::LightFastness => {
                let value = if let Some(value) = split.next() {
                    value
                } else {
                    LightFastness::default().value()
                };
                Ok(Self {
                    property_type,
                    value: <LightFastness as Into<f64>>::into(LightFastness::from_str(value)?)
                        .into(),
                })
            }
            PropertyType::Fluorescence => {
                let value = if let Some(value) = split.next() {
                    value
                } else {
                    Fluorescence::default().value()
                };
                Ok(Self {
                    property_type,
                    value: <Fluorescence as Into<f64>>::into(Fluorescence::from_str(value)?).into(),
                })
            }
            PropertyType::Finish => {
                let value = if let Some(value) = split.next() {
                    value
                } else {
                    Finish::default().value()
                };
                Ok(Self {
                    property_type,
                    value: <Finish as Into<f64>>::into(Finish::from_str(value)?).into(),
                })
            }
            PropertyType::Staining => {
                let value = if let Some(value) = split.next() {
                    value
                } else {
                    Staining::default().value()
                };
                Ok(Self {
                    property_type,
                    value: <Staining as Into<f64>>::into(Staining::from_str(value)?),
                })
            }
            PropertyType::Granulation => {
                let value = if let Some(value) = split.next() {
                    value
                } else {
                    Granulation::default().value()
                };
                Ok(Self {
                    property_type,
                    value: <Granulation as Into<f64>>::into(Granulation::from_str(value)?),
                })
            }
            PropertyType::Opacity => {
                let value = if let Some(value) = split.next() {
                    value
                } else {
                    Opacity::default().value()
                };
                Ok(Self {
                    property_type,
                    value: <Opacity as Into<f64>>::into(Opacity::from_str(value)?),
                })
            }
            PropertyType::Permanence => {
                let value = if let Some(value) = split.next() {
                    value
                } else {
                    Permanence::default().value()
                };
                Ok(Self {
                    property_type,
                    value: <Permanence as Into<f64>>::into(Permanence::from_str(value)?),
                })
            }
            PropertyType::Luminescence => {
                let value = if let Some(value) = split.next() {
                    value
                } else {
                    Luminescence::default().value()
                };
                Ok(Self {
                    property_type,
                    value: <Luminescence as Into<f64>>::into(Luminescence::from_str(value)?),
                })
            }
            PropertyType::Metallicness => {
                let value = if let Some(value) = split.next() {
                    value
                } else {
                    Metallicness::default().value()
                };
                Ok(Self {
                    property_type,
                    value: <Metallicness as Into<f64>>::into(Metallicness::from_str(value)?),
                })
            }
        };
        debug_assert_eq!(split.next(), None);
        result
    }
}

impl From<(PropertyType, f64)> for Property {
    fn from((property_type, value): (PropertyType, f64)) -> Self {
        Self {
            property_type,
            value,
        }
    }
}

impl From<(PropertyType, &str)> for Property {
    fn from((property_type, value): (PropertyType, &str)) -> Self {
        let variant = match property_type {
            PropertyType::Transparency => Transparency::from_str(value).unwrap().into(),
            PropertyType::LightFastness => LightFastness::from_str(value).unwrap().into(),
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
            PropertyType::LightFastness,
            PropertyType::from_str("LightFastness").unwrap()
        )
    }

    #[test]
    fn test_property_type_real() {
        assert_eq!(
            PropertyType::Transparency.value(1.0),
            Transparency::Clear.value()
        );
        assert_eq!(
            PropertyType::LightFastness.value(1.0),
            LightFastness::Excellent.value()
        );
    }

    #[test]
    fn test_split() {
        assert_eq!(
            "Transparency::Transparent".split("::").next().unwrap(),
            "Transparency"
        );
    }

    #[test]
    fn test_property_from_string() {
        assert_eq!(
            Property::from_str("LightFastness::Excellent"),
            Ok(Property {
                property_type: PropertyType::LightFastness,
                value: 1.0
            })
        );
        assert_eq!(
            Property::from_str("LightFastness"),
            Ok(Property {
                property_type: PropertyType::LightFastness,
                value: 2.0
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
        assert_eq!(LightFastness::default(), LightFastness::VeryGood);
    }

    #[test]
    fn paint_transparency_property() {
        assert_eq!(Transparency::NAME, "Transparency");
        assert_eq!(Transparency::PROMPT, "Transparency:");
        assert_eq!(Transparency::Transparent.abbrev_value(), "T");
        for a in ["O", "SO", "ST", "C"].iter() {
            assert_eq!(Transparency::from_str(a).unwrap().abbrev_value(), *a);
        }
        for a in ["Opaque", "SemiOpaque", "SemiTransparent", "Clear"].iter() {
            assert_eq!(Transparency::from_str(a).unwrap().value(), *a);
        }
    }

    #[test]
    fn defaults() {
        assert_eq!(Transparency::default(), Transparency::Transparent);
    }
}
