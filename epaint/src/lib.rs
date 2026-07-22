// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::error;
use std::fmt;
use std::io;
use std::str::FromStr;

use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;

pub mod mixtures;
pub mod paint;
pub mod properties;
pub mod range;

pub trait TooltipText {
    fn tooltip_text(&self) -> String;
}

pub trait LabelText {
    fn label_text(&self) -> String;
}

#[derive(Serialize, Deserialize, Debug, Default, PartialOrd, Ord, PartialEq, Eq, Clone)]
pub struct PaintRangeId {
    pub proprietor: String,
    pub name: String,
}

impl fmt::Display for PaintRangeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:({})", self.name, self.proprietor)
    }
}

impl FromStr for PaintRangeId {
    type Err = regex::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let re = Regex::new(r"(?P<name>[^:]+):\((?P<proprietor>[^)]+)\)")?;
        let cap = re.captures(s).ok_or(regex::Error::Syntax(s.to_string()))?;
        let name = cap
            .name("name")
            .ok_or(regex::Error::Syntax(s.to_string()))?
            .as_str()
            .to_string();
        let proprietor = cap
            .name("proprietor")
            .ok_or(regex::Error::Syntax(s.to_string()))?
            .as_str()
            .to_string();
        Ok(Self { name, proprietor })
    }
}

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    SerdeJsonError(serde_json::Error),
    NotFound(String),
    UnknownSeries(PaintRangeId),
    UnknownPaint(PaintRangeId, String),
    NotAValidLegacySpec,
    NotImplemented,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IOError(err) => write!(f, "IOError: {err}"),
            Error::SerdeJsonError(err) => write!(f, "Serde Json Error: {err}"),
            Error::NotFound(string) => write!(f, "{string}: Not found."),
            Error::UnknownSeries(series_id) => write!(f, "{series_id}: unknown paint range"),
            Error::UnknownPaint(series_id, id) => {
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

pub type Result<T> = std::result::Result<T, crate::Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reg_ex() {
        let paint_range_id: PaintRangeId =
            PaintRangeId::from_str("Red Magenta:(Daniel Smith)").expect("valid series_id");
        assert_eq!(paint_range_id.proprietor, "Daniel Smith");
        assert_eq!(paint_range_id.name, "Red Magenta");
    }
}
