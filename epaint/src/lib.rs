// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::error;
use std::fmt;
use std::io;

use serde_json;

pub mod mixtures;
pub mod paint;
pub mod properties;
pub mod series;
pub mod watercolour;

pub trait TooltipText {
    fn tooltip_text(&self) -> String;
}

pub trait LabelText {
    fn label_text(&self) -> String;
}

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    SerdeJsonError(serde_json::Error),
    NotFound(String),
    UnknownSeries(series::SeriesId),
    UnknownSeriesPaint(series::SeriesId, String),
    NotAValidLegacySpec,
    NotImplemented,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IOError(err) => write!(f, "IOError: {err}"),
            Error::SerdeJsonError(err) => write!(f, "Serde Json Error: {err}"),
            Error::NotFound(string) => write!(f, "{string}: Not found."),
            Error::UnknownSeries(series_id) => write!(f, "{series_id}: unknown paint series"),
            Error::UnknownSeriesPaint(series_id, id) => {
                write!(f, "{id}:({series_id}): unknown paint")
            }
            Error::NotAValidLegacySpec => write!(f, "Not a valid specification."),
            Error::NotImplemented => write!(f, "Feature not yet implemented."),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::IOError(err) => Some(err),
            Error::SerdeJsonError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IOError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerdeJsonError(err)
    }
}
