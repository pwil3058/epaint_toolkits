// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::{error, fmt, io};

pub mod icons;
pub mod properties;
pub mod sav_state;
pub mod spec_edit;

pub mod colour {
    pub use colour_math_gtk::{colour::*, coloured::*};

    pub trait PartsColour: colour_math::ColourIfce + Ord + 'static {}

    use epaint::{mixtures::Mixture, paint::Paint};

    impl PartsColour for Paint {}
    impl PartsColour for Mixture {}
}

#[derive(Debug)]
pub enum Error {
    APaintError(epaint::Error),
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

impl From<epaint::Error> for Error {
    fn from(err: epaint::Error) -> Self {
        Error::APaintError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IOError(err)
    }
}
