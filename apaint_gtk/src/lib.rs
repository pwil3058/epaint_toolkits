// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::{error, fmt, io};

pub mod factory;
pub mod icons;
pub mod list;
pub mod mixer;
pub mod properties;
pub mod series;
pub mod spec_edit;
pub mod storage;
pub mod window;

pub mod colour {
    pub use colour_math_gtk::{colour::*, coloured::*};

    pub trait PartsColour:
        colour_math::ColourIfce + apaint::TooltipText + apaint::LabelText + Ord + 'static
    {
    }

    use apaint::mixtures::Mixture;
    use apaint::series::*;

    impl PartsColour for SeriesPaint {}
    impl PartsColour for Mixture {}
}

#[derive(Debug)]
pub enum Error {
    APaintError(apaint::Error),
    IOError(io::Error),
    DuplicateFile(String),
    GeneralError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::APaintError(err) => write!(f, "Error: {err}."),
            Error::IOError(err) => write!(f, "Error: {err}."),
            Error::DuplicateFile(string) => write!(f, "Error: {string}."),
            Error::GeneralError(string) => write!(f, "Error: {string}."),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::APaintError(err) => Some(err),
            Error::IOError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<apaint::Error> for Error {
    fn from(err: apaint::Error) -> Self {
        Error::APaintError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IOError(err)
    }
}
