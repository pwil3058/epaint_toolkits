// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::fmt;
use std::io::{Read, Write};
use std::rc::Rc;

use crypto_hash::{Algorithm, Hasher};
use serde::{Deserialize, Serialize};

use crate::paint::{PaintEssentialsIfce, PaintSpec};

#[derive(Serialize, Deserialize, Debug, Default, PartialOrd, Ord, PartialEq, Eq, Clone)]
pub struct SeriesId {
    pub(crate) proprietor: String,
    pub(crate) series_name: String,
}

impl SeriesId {
    pub fn new(proprietor: String, series_name: String) -> Self {
        Self {
            proprietor,
            series_name,
        }
    }
}

impl fmt::Display for SeriesId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:({})", self.series_name, self.proprietor)
    }
}

#[derive(Debug)]
pub struct PaintSeries<P: PaintEssentialsIfce> {
    series_id: Rc<SeriesId>,
    paint_list: Vec<Rc<P>>,
}

impl<P: PaintEssentialsIfce + PartialOrd> PaintSeries<P> {
    pub fn new(series_id: &Rc<SeriesId>) -> Self {
        let series_id = series_id.clone();
        Self {
            series_id,
            paint_list: Vec::new(),
        }
    }

    pub fn series_id(&self) -> Rc<SeriesId> {
        self.series_id.clone()
    }

    pub fn paints(&self) -> impl Iterator<Item = Rc<P>> {
        self.paint_list.iter().cloned()
    }

    pub fn is_sorted_unique(&self) -> bool {
        while let Some(pair) = self.paint_list.windows(2).next() {
            if pair[0] >= pair[1] {
                return false;
            }
        }
        true
    }

    pub fn find(&self, name: &str) -> Option<Rc<P>> {
        debug_assert!(self.is_sorted_unique());
        match self.paint_list.binary_search_by_key(&name, |p| p.name()) {
            Ok(index) => self.paint_list.get(index).cloned(),
            Err(_) => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PaintSeriesSpec {
    pub(crate) series_id: SeriesId,
    pub(crate) paint_list: Vec<PaintSpec>,
}

impl PaintSeriesSpec {
    pub fn series_id(&self) -> &SeriesId {
        &self.series_id
    }

    pub fn set_proprietor(&mut self, proprietor: &str) {
        self.series_id.proprietor = proprietor.to_string()
    }

    pub fn set_series_name(&mut self, series_name: &str) {
        self.series_id.series_name = series_name.to_string()
    }

    pub fn paints(&self) -> impl Iterator<Item = &PaintSpec> {
        self.paint_list.iter()
    }

    pub fn add(&mut self, paint: &PaintSpec) -> Option<PaintSpec> {
        debug_assert!(self.is_sorted_unique());
        match self
            .paint_list
            .binary_search_by_key(&paint.name, |p| p.name.clone())
        {
            Ok(index) => {
                self.paint_list.push(paint.clone());
                let old = self.paint_list.swap_remove(index);
                debug_assert!(self.is_sorted_unique());
                Some(old)
            }
            Err(index) => {
                self.paint_list.insert(index, paint.clone());
                None
            }
        }
    }

    pub fn remove(&mut self, id: &str) -> Result<PaintSpec, crate::Error> {
        debug_assert!(self.is_sorted_unique());
        match self.paint_list.binary_search_by_key(&id, |p| &p.name) {
            Ok(index) => Ok(self.paint_list.remove(index)),
            Err(_) => Err(crate::Error::NotFound(id.to_string())),
        }
    }

    pub fn remove_all(&mut self) {
        self.paint_list.clear()
    }

    pub fn find(&self, id: &str) -> Option<&PaintSpec> {
        debug_assert!(self.is_sorted_unique());
        match self.paint_list.binary_search_by_key(&id, |p| &p.name) {
            Ok(index) => self.paint_list.get(index),
            Err(_) => None,
        }
    }

    pub fn is_sorted_unique(&self) -> bool {
        while let Some(pair) = self.paint_list.windows(2).next() {
            if pair[0] >= pair[1] {
                return false;
            };
        }
        true
    }
}

impl PaintSeriesSpec {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self, crate::Error> {
        let mut string = String::new();
        reader.read_to_string(&mut string)?;
        let series: Self = serde_json::from_str(&string)?;
        Ok(series)
    }
}

impl PaintSeriesSpec {
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

pub trait PaintFinder<P: PaintEssentialsIfce> {
    fn get_paint(
        &self,
        paint_id: &str,
        series_id: Option<&SeriesId>,
    ) -> Result<Rc<P>, crate::Error>;
}

#[cfg(test)]
mod test {
    use crate::properties::PropertyType;
    use crate::series::{PaintSeriesSpec, PaintSpec};
    use colour_math::{HCV, HueConstants, RGB};

    #[test]
    fn save_and_recover() {
        let mut series_spec = PaintSeriesSpec::default();
        series_spec.set_proprietor("owner");
        series_spec.set_series_name("series name");
        assert!(series_spec.paints().next().is_none());
        series_spec.add(&PaintSpec::new(
            &RGB::<f64>::RED,
            "red",
            "whatever",
            &Vec::<PropertyType>::new(),
        ));
        series_spec.add(&PaintSpec::new(
            &HCV::YELLOW,
            "yellow",
            "whatever",
            &Vec::<PropertyType>::new(),
        ));
        let mut buffer: Vec<u8> = vec![];
        let _digest = series_spec.write(&mut buffer);
        let read_spec = PaintSeriesSpec::read(&mut &buffer[..]).unwrap();
        assert_eq!(series_spec.series_id(), read_spec.series_id());
        assert_eq!(series_spec.paint_list.len(), read_spec.paint_list.len());
        for (pspec1, pspec2) in series_spec.paints().zip(read_spec.paints()) {
            assert_eq!(*pspec1, *pspec2);
        }
    }
}
