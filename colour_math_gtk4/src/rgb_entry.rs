// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use gtk::Orientation;
use std::{cell::RefCell, rc::Rc};

use colour_math::{HCV, HueConstants, RGB, UnsignedLightLevel};

use gtk::prelude::*;

use gtk4_ext::{
    PackableWidgetObject,
    hex_entry::{self, HexEntry, HexEntryBuilder},
};
use gtk4_ext_derive::PWO;

use crate::colour::GdkColour;
use crate::coloured::Colourable;

pub trait Hexable: UnsignedLightLevel + hex_entry::Hexable {}

impl Hexable for u8 {}
impl Hexable for u16 {}
impl Hexable for u32 {}
impl Hexable for u64 {}

type BoxedChangeCallback<U> = Box<dyn Fn(U)>;

#[derive(PWO)]
pub struct RGBHexEntry<U: Hexable> {
    hbox: gtk::Box,
    entries: [Rc<HexEntry<U>>; 3],
    colour_change_callbacks: RefCell<Vec<BoxedChangeCallback<HCV>>>,
}

impl<U: Hexable> RGBHexEntry<U> {
    pub fn rgb(&self) -> RGB<U> {
        let v: Vec<U> = self.entries.iter().map(|e| e.value()).collect();
        RGB::<U>::from([v[0], v[1], v[2]])
    }

    pub fn hcv(&self) -> HCV {
        self.rgb().into()
    }

    pub fn set_colour(&self, colour: &impl GdkColour) {
        self.set_rgb(&colour.rgb::<U>())
    }

    pub fn set_rgb(&self, rgb: &RGB<U>) {
        for (entry, value) in self.entries.iter().zip(rgb.iter()) {
            entry.set_value(*value);
        }
    }

    pub fn connect_colour_changed<F: 'static + Fn(HCV)>(&self, callback: F) {
        self.colour_change_callbacks
            .borrow_mut()
            .push(Box::new(callback))
    }

    fn inform_colour_changed(&self) {
        let hcv: HCV = self.rgb().into();
        for callback in self.colour_change_callbacks.borrow().iter() {
            callback(hcv)
        }
    }
}

#[derive(Default)]
pub struct RGBHexEntryBuilder<U: Hexable> {
    initial_rgb: RGB<U>,
    editable: bool,
}

impl<U: Hexable> RGBHexEntryBuilder<U> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn editable(&mut self, editable: bool) -> &mut Self {
        self.editable = editable;
        self
    }

    pub fn initial_colour(&mut self, initial_colour: &impl GdkColour) -> &mut Self {
        self.initial_rgb = initial_colour.rgb::<U>();
        self
    }

    pub fn build(&self) -> Rc<RGBHexEntry<U>> {
        let hbox = gtk::Box::new(Orientation::Horizontal, 1);

        let mut v: Vec<Rc<HexEntry<U>>> = vec![];
        for (index, (label, rgb)) in [
            ("Red:", RGB::<U>::RED),
            ("Green:", RGB::<U>::GREEN),
            ("Blue:", RGB::<U>::BLUE),
        ]
        .iter()
        .enumerate()
        {
            let entry = HexEntryBuilder::new()
                .editable(self.editable)
                .initial_value(self.initial_rgb[index])
                .build();
            let label = gtk::Label::new(Some(label));
            label.set_widget_colour(rgb);
            hbox.append(&label);
            hbox.append(entry.pwo());
            v.push(entry);
        }
        let entries = [Rc::clone(&v[0]), Rc::clone(&v[1]), Rc::clone(&v[2])];

        let rgb_hex_entry = Rc::new(RGBHexEntry {
            hbox,
            entries,
            colour_change_callbacks: RefCell::new(vec![]),
        });

        for entry in rgb_hex_entry.entries.iter() {
            let rgb_hex_entry_c = Rc::clone(&rgb_hex_entry);
            entry.connect_value_changed(move |_| rgb_hex_entry_c.inform_colour_changed());
        }

        rgb_hex_entry
    }
}
