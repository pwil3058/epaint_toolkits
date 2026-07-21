// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crypto_hash::{Algorithm, Hasher};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::str::FromStr;

use crate::PaintRangeId;
use crate::paint::{Paint, RangePaint};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct PaintRange {
    pub range_id: PaintRangeId,
    pub paint_list: Vec<Paint>,
}

impl PaintRange {
    pub fn new(range_id: &PaintRangeId) -> Self {
        Self {
            range_id: range_id.clone(),
            paint_list: Vec::new(),
        }
    }

    pub fn range_id(&self) -> &PaintRangeId {
        &self.range_id
    }

    pub fn set_proprietor(&mut self, proprietor: &str) {
        self.range_id.proprietor = proprietor.to_string()
    }

    pub fn set_range_name(&mut self, name: &str) {
        self.range_id.name = name.to_string()
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

    pub fn paints(&self) -> impl Iterator<Item = &Paint> {
        self.paint_list.iter()
    }

    pub fn range_paints(&self) -> impl Iterator<Item = RangePaint> {
        self.paint_list
            .iter()
            .map(|paint| RangePaint::from((paint, &self.range_id)))
    }

    pub fn is_sorted_unique(&self) -> bool {
        for i in 1..self.paint_list.len() {
            if self.paint_list[i] <= self.paint_list[i - 1] {
                return false;
            }
        }
        true
    }

    pub fn get_paint(&self, key: &str) -> Option<&Paint> {
        let index = self
            .paint_list
            .binary_search_by_key(&key, |p| p.key())
            .ok()?;
        self.paint_list.get(index)
    }

    pub fn get_range_paint(&self, key: &str) -> Option<RangePaint> {
        debug_assert!(self.is_sorted_unique());
        if let Some(paint) = self.get_paint(key) {
            Some(RangePaint::from((paint, &self.range_id)))
        } else {
            let split = key.split("::").collect::<Vec<_>>();
            debug_assert!(split.len() == 2);
            let range_id = PaintRangeId::from_str(split[1]).expect("Programmer error");
            if range_id == self.range_id {
                let paint = self.get_paint(split[0])?;
                Some(RangePaint::from((paint, &self.range_id)))
            } else {
                None
            }
        }
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
        series_id: Option<&PaintRangeId>,
    ) -> Result<RangePaint, crate::Error>;
}

#[cfg(test)]
mod test {
    use colour_math::{HCV, HueConstants};

    use crate::paint::Paint;
    use crate::properties::{Properties, Property, PropertyType};
    use crate::range::PaintRange;

    #[test]
    fn save_and_recover() {
        let mut paint_series = PaintRange::default();
        paint_series.set_proprietor("owner");
        paint_series.set_range_name("range name");
        assert!(paint_series.range_paints().next().is_none());
        paint_series.add(&Paint {
            #[cfg(feature = "paints_have_ids")]
            id: "red".to_string(),
            colour: HCV::RED,
            name: "red".to_string(),
            notes: "whatever".to_string(),
            properties: Properties(vec![Property::from((PropertyType::Transparency, 1))]),
        });
        paint_series.add(&Paint {
            #[cfg(feature = "paints_have_ids")]
            id: "yellow".to_string(),
            colour: HCV::YELLOW,
            name: "yellow".to_string(),
            notes: "whatever".to_string(),
            properties: Properties(vec![Property::from((PropertyType::Transparency, 1))]),
        });
        let mut buffer: Vec<u8> = vec![];
        let _digest = paint_series.write(&mut buffer);
        let read_series = PaintRange::read(&mut &buffer[..]).unwrap();
        assert_eq!(paint_series.range_id(), read_series.range_id());
        assert_eq!(paint_series.paint_list.len(), read_series.paint_list.len());
        for (colln_paint1, colln_paint2) in
            paint_series.range_paints().zip(read_series.range_paints())
        {
            assert_eq!(colln_paint1, colln_paint2);
        }
    }
}
