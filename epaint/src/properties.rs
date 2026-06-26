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
    // fn abbrev_value(&self, value: f64) -> &'static str;
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
        Fugitive,
        Fair,
        #[default]
        VeryGood,
        Excellent,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, PropertyType)]
    pub enum TestPropertyType {
        Transparency,
        LightFastness,
    }

    #[test]
    fn test_property_type() {
        let p_type = TestPropertyType::from_str("Transparency").unwrap();
        assert_eq!(TestPropertyType::Transparency, p_type);
    }

    // Test objects that implement Property
    #[test]
    fn test_property_from_f64() {
        let transparency: Transparency = 1.0.into();
        assert_eq!(Transparency::Opaque, transparency);
        assert_eq!(Transparency::SemiOpaque, Into::<Transparency>::into(2.0));
        assert_eq!(
            Transparency::SemiTransparent,
            Into::<Transparency>::into(3.0)
        );
        assert_eq!(Transparency::Transparent, Into::<Transparency>::into(4.0));
        assert_eq!(Transparency::Clear, Into::<Transparency>::into(5.0));
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
        for a in ["opaque", "semi-opaque", "semi-transparent", "clear"].iter() {
            assert_eq!(Transparency::from_str(a).unwrap().value(), *a);
        }
    }

    #[test]
    fn defaults() {
        assert_eq!(Transparency::default(), Transparency::Transparent);
    }
}
