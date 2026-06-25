// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

//! Types to describe paint properties that cannot be derived from their colour.

// use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::{fmt, str::FromStr};

use epaint_derive::Property;
use std::marker::PhantomData;

pub trait PropertyConsts:
    FromStr<Err = String> + PartialEq + PartialOrd + Default + fmt::Debug
{
    const NAME: &'static str;
    const PROMPT: &'static str;
    const LIST_HEADER: &'static str;
}

pub trait PropertyFns:
    FromStr<Err = String> + PartialEq + PartialOrd + Default + fmt::Debug
{
    fn name(&self) -> &'static str;
    fn prompt(&self) -> &'static str;
    fn list_header(&self) -> &'static str;
    /// Possible property values as strings
    fn str_values() -> Vec<&'static str>;
    fn abbrev_value(&self) -> &'static str;
    fn value(&self) -> &'static str;
}

pub trait PropertyIfce: PropertyConsts + PropertyFns {}

pub trait PropertyTypeIfce {
    fn name(&self) -> &'static str;
    fn prompt(&self) -> &'static str;
    fn list_header(&self) -> &'static str;
    fn str_values(&self) -> Vec<&'static str>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use epaint_derive::{Property, PropertyType};
    use serde::{Deserialize, Serialize};

    #[derive(
        Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Property,
    )]
    pub enum Transparency {
        Opaque,
        SemiOpaque,
        SemiTransparent,
        #[default]
        Transparent,
        Clear,
    }

    #[derive(
        Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Property,
    )]
    pub enum LightFastness {
        Excellent,
        #[default]
        VeryGood,
        Fair,
        Fugitive,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, PropertyType)]
    pub enum TestPropertyType {
        Transparency,
        LightFastness,
    }

    #[test]
    fn test_property_type() {
        let p_type = TestPropertyType::from_str("Transparency");
        assert_eq!(Ok(TestPropertyType::Transparency), p_type);
    }

    #[test]
    fn paint_transparency_property() {
        assert_eq!(Transparency::NAME, "Transparency");
        assert_eq!(Transparency::PROMPT, "Transparency:");
        assert_eq!(Transparency::Transparent.abbrev_value(), "T");
        for a in ["O", "SO", "ST", "C"].iter() {
            assert_eq!(Transparency::from_str(a).unwrap().abbrev_value(), *a);
        }
        for a in ["opaque", "semi-opaque", "semi-transparent", "clear"].iter() {
            assert_eq!(Transparency::from_str(a).unwrap().value(), *a);
        }
    }

    #[test]
    fn defaults() {
        assert_eq!(Transparency::default(), Transparency::Transparent);
    }

    // #[test]
    // fn mixture() {
    //     let mut mixer = PropertyMixer::<Finish>::new();
    //     assert_eq!(mixer.property(), None);
    //     mixer.add(Finish::Gloss, 1);
    //     mixer.add(Finish::Flat, 10);
    //     assert_eq!(mixer.property(), Some(Finish::Flat));
    //     mixer.add(Finish::Gloss, 6);
    //     assert_eq!(mixer.property(), Some(Finish::SemiFlat));
    // }
}
