// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::{cell::RefCell, rc::Rc};

use gtk3_ext::{
    gtk::{self, prelude::*},
    wrapper::*,
};

use colour_math::{HCV, RGB};

use crate::colour::GdkColour;

use crate::hex_entry::GtkHexEntry;

type BoxedChangeCallback<T> = Box<dyn Fn(T)>;

#[derive(PWO)]
pub struct GtkRGBHexEntry {
    hbox: gtk::Box,
    entries: [Rc<GtkHexEntry<u8>>; 3],
    colour_change_callbacks: RefCell<Vec<BoxedChangeCallback<HCV>>>,
}

impl GtkRGBHexEntry {
    pub fn builder() -> RGBHexEntryBuilder {
        RGBHexEntryBuilder::new()
    }

    pub fn rgb(&self) -> RGB<u8> {
        let v: Vec<u8> = self.entries.iter().map(|e| e.value()).collect();
        RGB::<u8>::from([v[0], v[1], v[2]])
    }

    pub fn hcv(&self) -> HCV {
        self.rgb().into()
    }

    pub fn set_colour(&self, colour: &impl GdkColour) {
        self.set_rgb(&colour.into_rgb::<u8>())
    }

    pub fn set_rgb(&self, rgb: &RGB<u8>) {
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
pub struct RGBHexEntryBuilder {
    initial_rgb: RGB<u8>,
}

impl RGBHexEntryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn initial_colour(&mut self, initial_colour: &impl GdkColour) -> &mut Self {
        self.initial_rgb = initial_colour.into_rgb::<u8>();
        self
    }

    pub fn build(&self) -> Rc<GtkRGBHexEntry> {
        let hbox = gtk::Box::builder().build();

        let mut v: Vec<Rc<GtkHexEntry<u8>>> = vec![];
        for (index, (label, rgb)) in [
            ("Red:", "#CF0000"),
            ("Green:", "#00CF00"),
            ("Blue:", "#0000FF"),
        ]
        .iter()
        .enumerate()
        {
            let entry = GtkHexEntry::builder()
                .initial_value(self.initial_rgb[index])
                .build();
            let markup = format!("<span foreground =\"{}\"><b>{}</b></span>", rgb, label);
            let label = gtk::Label::new(None);
            label.set_markup(&markup);
            hbox.pack_start(&label, true, true, 0);
            hbox.pack_start(entry.pwo(), false, false, 0);
            v.push(entry);
        }
        let entries = [Rc::clone(&v[0]), Rc::clone(&v[1]), Rc::clone(&v[2])];

        let rgb_hex_entry = Rc::new(GtkRGBHexEntry {
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
