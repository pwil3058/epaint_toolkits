// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::rc::Rc;

use colour_math::{Colour, HCV, HueConstants, IntoHCV, RGB};

use crate::{ColouredLabel, HexEntry};

#[derive(Debug)]
pub struct RGBHexEntry {
    pub entries: [Rc<HexEntry<u8>>; 3],
}

impl RGBHexEntry {
    pub fn builder() -> RGBHexEntryBuilder {
        RGBHexEntryBuilder::default()
    }

    pub fn rgb(&self) -> RGB<u8> {
        self.entries
            .iter()
            .map(|e| e.value())
            .collect::<Vec<_>>()
            .into()
    }

    pub fn hcv(&self) -> HCV {
        self.rgb().into_hcv()
    }

    pub fn set_rgb(&self, rgb: RGB<u8>) {
        for (entry, value) in self.entries.iter().zip(rgb.iter()) {
            entry.set_value(*value);
        }
    }

    pub fn set_colour(&self, colour: impl Colour) {
        self.set_rgb(colour.into_rgb::<u8>())
    }
}

#[derive(Debug, Default)]
pub struct RGBHexEntryBuilder {
    initial_rgb: Option<RGB<u8>>,
    max_step: Option<u8>,
}

impl RGBHexEntryBuilder {
    pub fn initial_rgb(&mut self, rgb: RGB<u8>) -> &mut Self {
        self.initial_rgb = Some(rgb);
        self
    }

    pub fn max_step(&mut self, max_step: u8) -> &mut Self {
        self.max_step = Some(max_step);
        self
    }

    pub fn build(&self) -> RGBHexEntry {
        let entries: [Rc<HexEntry<u8>>; 3] = [
            ("Red:", RGB::<u8>::RED / 2),
            ("Green:", RGB::<u8>::GREEN / 2),
            ("Blue:", RGB::<u8>::BLUE),
        ]
        .into_iter()
        .map(|label_data| {
            let label: ColouredLabel = label_data.into();
            if let Some(max_step) = self.max_step {
                Rc::new(HexEntry::builder().max_step(max_step).label(label).build())
            } else {
                Rc::new(HexEntry::builder().label(label).build())
            }
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
        let rgb_entry = RGBHexEntry { entries };
        if let Some(initial_rgb) = self.initial_rgb {
            rgb_entry.set_rgb(initial_rgb);
        }
        rgb_entry
    }
}
