// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::{cell::RefCell, rc::Rc};

use gtk3_ext::{
    gtk::{self, prelude::*},
    wrapper::*,
};

use colour_math::{RGBHexEntry, HCV, RGB};

use crate::colour::GdkColour;

use crate::hex_entry::GtkHexEntry;

type BoxedChangeCallback<T> = Box<dyn Fn(T)>;

#[derive(PWO)]
pub struct GtkRGBHexEntry {
    hbox: gtk::Box,
    rgb_hex_entry: RGBHexEntry,
    gtk_hex_entries: [Rc<GtkHexEntry<u8>>; 3],
    colour_change_callbacks: RefCell<Vec<BoxedChangeCallback<HCV>>>,
}

impl GtkRGBHexEntry {
    pub fn builder() -> GtkRGBHexEntryBuilder {
        GtkRGBHexEntryBuilder::new()
    }

    pub fn rgb(&self) -> RGB<u8> {
        self.rgb_hex_entry.rgb()
    }

    pub fn hcv(&self) -> HCV {
        self.rgb().into()
    }

    pub fn set_colour(&self, colour: &impl GdkColour) {
        self.set_rgb(&colour.into_rgb::<u8>())
    }

    pub fn set_rgb(&self, rgb: &RGB<u8>) {
        self.rgb_hex_entry.set_rgb(*rgb)
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
pub struct GtkRGBHexEntryBuilder {
    initial_rgb: RGB<u8>,
}

impl GtkRGBHexEntryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn initial_colour(&mut self, initial_colour: &impl GdkColour) -> &mut Self {
        self.initial_rgb = initial_colour.into_rgb::<u8>();
        self
    }

    pub fn build(&self) -> Rc<GtkRGBHexEntry> {
        let hbox = gtk::Box::builder().build();

        let rgb_hex_entry = RGBHexEntry::builder().initial_rgb(self.initial_rgb).build();
        let gtk_hex_entries: [Rc<GtkHexEntry<u8>>; 3] = rgb_hex_entry
            .entries
            .iter()
            .map(|hex_entry| {
                let entry = GtkHexEntry::<u8>::new(Rc::clone(hex_entry));
                hbox.pack_start(entry.pwo(), true, false, 0);
                entry
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let rgb_hex_entry = Rc::new(GtkRGBHexEntry {
            hbox,
            rgb_hex_entry,
            gtk_hex_entries,
            colour_change_callbacks: RefCell::new(vec![]),
        });

        for entry in rgb_hex_entry.gtk_hex_entries.iter() {
            let rgb_hex_entry_c = Rc::clone(&rgb_hex_entry);
            entry.connect_value_changed(move |_| rgb_hex_entry_c.inform_colour_changed());
        }

        rgb_hex_entry
    }
}
