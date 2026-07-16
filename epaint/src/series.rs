// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::io::{Read, Write};
use std::rc::Rc;

use crypto_hash::{Algorithm, Hasher};
use serde::{Deserialize, Serialize};

use crate::paint::{Paint, SerializablePaintData};
use crate::{AbbrevKey, PaintEssence, SeriesId};

#[derive(Debug)]
pub struct PaintSeries {
    series_id: Rc<SeriesId>,
    paint_list: Vec<Rc<Paint>>,
}

impl PaintSeries {
    pub fn new(series_id: &SeriesId) -> Self {
        let series_id = Rc::new(series_id.clone());
        Self {
            series_id,
            paint_list: Vec::new(),
        }
    }

    pub fn series_id(&self) -> Rc<SeriesId> {
        self.series_id.clone()
    }

    pub fn paints(&self) -> impl Iterator<Item = Rc<Paint>> {
        self.paint_list.iter().cloned()
    }

    pub fn is_sorted_unique(&self) -> bool {
        for i in 1..self.paint_list.len() {
            if self.paint_list[i] <= self.paint_list[i - 1] {
                return false;
            }
        }
        true
    }

    pub fn find(&self, name: &str) -> Option<&Rc<Paint>> {
        debug_assert!(self.is_sorted_unique());
        match self.paint_list.binary_search_by_key(&name, |p| p.name()) {
            Ok(index) => self.paint_list.get(index),
            Err(_) => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PaintSeriesSpec {
    pub(crate) series_id: SeriesId,
    pub(crate) paint_spec_list: Vec<SerializablePaintData>,
}

impl PaintSeriesSpec {
    pub fn series_id(&self) -> Rc<SeriesId> {
        Rc::new(self.series_id.clone())
    }

    pub fn set_proprietor(&mut self, proprietor: &str) {
        self.series_id.proprietor = proprietor.to_string()
    }

    pub fn set_series_name(&mut self, series_name: &str) {
        self.series_id.series_name = series_name.to_string()
    }

    pub fn paints(&self) -> impl Iterator<Item = &SerializablePaintData> {
        self.paint_spec_list.iter()
    }

    pub fn add(&mut self, paint: &SerializablePaintData) -> Option<SerializablePaintData> {
        debug_assert!(self.is_sorted_unique());
        match self
            .paint_spec_list
            .binary_search_by_key(&paint.abbrev_key(), |p| p.abbrev_key())
        {
            Ok(index) => {
                self.paint_spec_list.push(paint.clone());
                let old = self.paint_spec_list.swap_remove(index);
                debug_assert!(self.is_sorted_unique());
                Some(old)
            }
            Err(index) => {
                self.paint_spec_list.insert(index, paint.clone());
                None
            }
        }
    }

    pub fn remove(&mut self, key: &str) -> Result<SerializablePaintData, crate::Error> {
        debug_assert!(self.is_sorted_unique());
        match self
            .paint_spec_list
            .binary_search_by_key(&key, |p| &p.abbrev_key())
        {
            Ok(index) => Ok(self.paint_spec_list.remove(index)),
            Err(_) => Err(crate::Error::NotFound(key.to_string())),
        }
    }

    pub fn remove_all(&mut self) {
        self.paint_spec_list.clear()
    }

    pub fn find(&self, key: &str) -> Option<&SerializablePaintData> {
        debug_assert!(self.is_sorted_unique());
        match self
            .paint_spec_list
            .binary_search_by_key(&key, |p| &p.abbrev_key())
        {
            Ok(index) => self.paint_spec_list.get(index),
            Err(_) => None,
        }
    }

    pub fn is_sorted_unique(&self) -> bool {
        for i in 1..self.paint_spec_list.len() {
            if self.paint_spec_list[i] <= self.paint_spec_list[i - 1] {
                return false;
            }
        }
        true
    }
}

impl From<&PaintSeriesSpec> for PaintSeries {
    fn from(data: &PaintSeriesSpec) -> PaintSeries {
        let series_id = data.series_id();
        let mut paint_list = Vec::new();
        for paint_spec in data.paint_spec_list.iter() {
            let paint: Paint = (paint_spec.clone(), Rc::clone(&series_id)).into();
            paint_list.push(Rc::new(paint));
        }
        PaintSeries {
            series_id,
            paint_list,
        }
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

pub trait PaintFinder {
    fn get_paint(
        &self,
        paint_name: &str,
        series_id: Option<&SeriesId>,
    ) -> Result<Rc<Paint>, crate::Error>;
}

#[cfg(test)]
mod test {
    use colour_math::{HCV, HueConstants};

    use crate::paint::SerializablePaintData;
    use crate::properties::{Properties, Property, PropertyType};
    use crate::series::{PaintSeries, PaintSeriesSpec};

    #[test]
    fn save_and_recover() {
        let mut series_spec = PaintSeriesSpec::default();
        series_spec.set_proprietor("owner");
        series_spec.set_series_name("series name");
        assert!(series_spec.paints().next().is_none());
        series_spec.add(&SerializablePaintData {
            colour: HCV::RED,
            name: "red".to_string(),
            notes: "whatever".to_string(),
            properties: Properties(vec![Property::from((PropertyType::Transparency, 1))]),
        });
        series_spec.add(&SerializablePaintData {
            colour: HCV::YELLOW,
            name: "yellow".to_string(),
            notes: "whatever".to_string(),
            properties: Properties(vec![Property::from((PropertyType::Transparency, 1))]),
        });
        let mut buffer: Vec<u8> = vec![];
        let _digest = series_spec.write(&mut buffer);
        let read_spec = PaintSeriesSpec::read(&mut &buffer[..]).unwrap();
        assert_eq!(series_spec.series_id(), read_spec.series_id());
        assert_eq!(
            series_spec.paint_spec_list.len(),
            read_spec.paint_spec_list.len()
        );
        for (pspec1, pspec2) in series_spec.paints().zip(read_spec.paints()) {
            assert_eq!(*pspec1, *pspec2);
        }
        let series: PaintSeries = (&series_spec).into();
        assert_eq!(series.series_id, series_spec.series_id());
        let found_red = series.find("red");
        assert_eq!(found_red.unwrap().data.colour, HCV::RED);
        for (spec_paint, paint) in series_spec.paints().zip(read_spec.paints()) {
            assert_eq!(spec_paint.colour, paint.colour);
        }
    }
}
