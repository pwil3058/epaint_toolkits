// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::io::{Read, Write};

use crypto_hash::{Algorithm, Hasher};
use serde::{Deserialize, Serialize};

use crate::paint::{CollnPaint, Paint};
use crate::{PaintKey, SeriesId};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct PaintSeries {
    pub series_id: SeriesId,
    pub paint_list: Vec<Paint>,
}

impl PaintSeries {
    pub fn new(series_id: &SeriesId) -> Self {
        Self {
            series_id: series_id.clone(),
            paint_list: Vec::new(),
        }
    }

    pub fn series_id(&self) -> &SeriesId {
        &self.series_id
    }

    pub fn set_proprietor(&mut self, proprietor: &str) {
        self.series_id.proprietor = proprietor.to_string()
    }

    pub fn set_series_name(&mut self, series_name: &str) {
        self.series_id.series_name = series_name.to_string()
    }

    pub fn add(&mut self, paint: &Paint) -> Option<Paint> {
        debug_assert!(self.is_sorted_unique());
        match self
            .paint_list
            .binary_search_by_key(&paint.key(), |p| p.key())
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

    pub fn remove(&mut self, key: &str) -> Result<Paint, crate::Error> {
        debug_assert!(self.is_sorted_unique());
        match self.paint_list.binary_search_by_key(&key, |p| &p.key()) {
            Ok(index) => Ok(self.paint_list.remove(index)),
            Err(_) => Err(crate::Error::NotFound(key.to_string())),
        }
    }

    pub fn remove_all(&mut self) {
        self.paint_list.clear()
    }

    pub fn colln_paints(&self) -> impl Iterator<Item = CollnPaint> {
        self.paint_list.iter().cloned().map(|paint| CollnPaint {
            paint,
            series_id: self.series_id.clone(),
        })
    }

    pub fn is_sorted_unique(&self) -> bool {
        for i in 1..self.paint_list.len() {
            if self.paint_list[i] <= self.paint_list[i - 1] {
                return false;
            }
        }
        true
    }

    pub fn find_colln_paint(&self, key: &str) -> Option<CollnPaint> {
        debug_assert!(self.is_sorted_unique());
        let index = self
            .paint_list
            .binary_search_by_key(&key, |p| p.key())
            .ok()?;
        let paint = self.paint_list[index].clone();
        Some(CollnPaint {
            paint,
            series_id: self.series_id.clone(),
        })
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<Self, crate::Error> {
        let mut string = String::new();
        reader.read_to_string(&mut string)?;
        let series: Self = serde_json::from_str(&string)?;
        Ok(series)
    }

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
    ) -> Result<CollnPaint, crate::Error>;
}

#[cfg(test)]
mod test {
    use colour_math::{HCV, HueConstants};

    use crate::paint::Paint;
    use crate::properties::{Properties, Property, PropertyType};
    use crate::series::PaintSeries;

    #[test]
    fn save_and_recover() {
        let mut paint_series = PaintSeries::default();
        paint_series.set_proprietor("owner");
        paint_series.set_series_name("series name");
        assert!(paint_series.colln_paints().next().is_none());
        paint_series.add(&Paint {
            id: "red".to_string(),
            colour: HCV::RED,
            name: "red".to_string(),
            notes: "whatever".to_string(),
            properties: Properties(vec![Property::from((PropertyType::Transparency, 1))]),
        });
        paint_series.add(&Paint {
            id: "yellow".to_string(),
            colour: HCV::YELLOW,
            name: "yellow".to_string(),
            notes: "whatever".to_string(),
            properties: Properties(vec![Property::from((PropertyType::Transparency, 1))]),
        });
        let mut buffer: Vec<u8> = vec![];
        let _digest = paint_series.write(&mut buffer);
        let read_series = PaintSeries::read(&mut &buffer[..]).unwrap();
        assert_eq!(paint_series.series_id(), read_series.series_id());
        assert_eq!(paint_series.paint_list.len(), read_series.paint_list.len());
        for (colln_paint1, colln_paint2) in
            paint_series.colln_paints().zip(read_series.colln_paints())
        {
            assert_eq!(colln_paint1, colln_paint2);
        }
    }
}
