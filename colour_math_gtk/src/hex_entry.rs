// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

use gtk3_ext::{
    gdk,
    glib::Propagation,
    gtk::{self, prelude::*},
    wrapper::*,
};

use colour_math::{hex_entry, ColouredLabel};

use crate::coloured::GtkColouredLabel;

type ChangeCallback<U> = Box<dyn Fn(U)>;

#[derive(PWO)]
pub struct HexEntry<U: hex_entry::Hexable> {
    hbox: gtk::Box,
    entry: gtk::Entry,
    hex_entry: hex_entry::HexEntry<U>,
    callbacks: RefCell<Vec<ChangeCallback<U>>>,
}

impl<U: hex_entry::Hexable> HexEntry<U> {
    pub fn builder() -> HexEntryBuilder<U> {
        HexEntryBuilder::<U>::default()
    }

    #[allow(non_upper_case_globals)]
    pub fn new(hex_entry: hex_entry::HexEntry<U>) -> Rc<Self> {
        let hbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();
        if let Some(label) = hex_entry.label {
            let label = GtkColouredLabel::from(label);
            hbox.pack_start(label.pwo(), true, false, 1);
        }
        let entry = gtk::Entry::builder()
            .width_chars(U::bytes() as i32 * 2 + 2)
            .build();
        hbox.pack_start(&entry, false, false, 1);
        let callbacks: RefCell<Vec<ChangeCallback<U>>> = RefCell::new(Vec::new());
        let rc_hex_entry = Rc::new(HexEntry {
            hbox,
            entry,
            hex_entry,
            callbacks,
        });
        rc_hex_entry.set_entry_text();

        let hex_entry_c = Rc::clone(&rc_hex_entry);
        rc_hex_entry
            .entry
            .connect_key_press_event(move |entry, event| {
                use gdk::keys::constants::*;

                const KEY_0: gdk::keys::Key = _0;
                const KEY_1: gdk::keys::Key = _1;
                const KEY_2: gdk::keys::Key = _2;
                const KEY_3: gdk::keys::Key = _3;
                const KEY_4: gdk::keys::Key = _4;
                const KEY_5: gdk::keys::Key = _5;
                const KEY_6: gdk::keys::Key = _6;
                const KEY_7: gdk::keys::Key = _7;
                const KEY_8: gdk::keys::Key = _8;
                const KEY_9: gdk::keys::Key = _9;
                let key = event.keyval();
                match key {
                    Return | Tab | ISO_Left_Tab => {
                        let text = entry.text();
                        if text.is_empty() {
                            hex_entry_c.set_entry_text();
                        } else {
                            hex_entry_c.set_value_from_text(&text);
                        }
                        // NB: this will nobble the "activate" signal
                        // but let the Tab key move the focus
                        if key == Return {
                            Propagation::Stop
                        } else {
                            Propagation::Proceed
                        }
                    }
                    Up => {
                        hex_entry_c.incr_value();
                        Propagation::Stop
                    }
                    Down => {
                        hex_entry_c.decr_value();
                        Propagation::Stop
                    }
                    KEY_0 | KEY_1 | KEY_2 | KEY_3 | KEY_4 | KEY_5 | KEY_6 | KEY_7 | KEY_8
                    | KEY_9 | A | B | C | D | E | F | BackSpace | Delete | Copy | Paste | x | a
                    | b | c | d | e | f | Left | Right => Propagation::Proceed, //Inhibit(false),
                    _ => Propagation::Stop,
                }
            });

        let hex_entry_c = Rc::clone(&rc_hex_entry);
        rc_hex_entry
            .entry
            .connect_key_release_event(move |_, event| {
                use gdk::keys::constants::*;
                match event.keyval() {
                    Up | Down => {
                        hex_entry_c.hex_entry.reset_current_step();
                        Propagation::Stop
                    }
                    _ => Propagation::Proceed,
                }
            });

        rc_hex_entry
    }

    pub fn value(&self) -> U {
        self.hex_entry.value()
    }

    pub fn set_value(&self, value: U) {
        let _ = self.hex_entry.set_value(value);
        self.set_entry_text();
    }

    pub fn connect_value_changed<F: 'static + Fn(U)>(&self, callback: F) {
        self.callbacks.borrow_mut().push(Box::new(callback))
    }

    fn incr_value(&self) {
        if self.hex_entry.incr_value().is_some() {
            self.set_entry_text();
            self.inform_value_changed();
        }
    }

    fn decr_value(&self) {
        if self.hex_entry.decr_value().is_some() {
            self.set_entry_text();
            self.inform_value_changed();
        }
    }

    fn set_entry_text(&self) {
        self.entry.set_text(&format!(
            "{:#0width$X}",
            self.value(),
            width = U::bytes() * 2 + 2
        ));
    }

    fn set_value_from_text(&self, text: &str) {
        match self.hex_entry.set_value_from_text(text) {
            Ok(result) => {
                self.set_entry_text();
                if result.is_some() {
                    self.inform_value_changed()
                }
            }
            Err(_) => self.set_entry_text(),
        }
    }

    fn inform_value_changed(&self) {
        let value = self.value();
        for callback in self.callbacks.borrow().iter() {
            callback(value);
        }
    }
}

#[derive(Default)]
pub struct HexEntryBuilder<U: hex_entry::Hexable> {
    initial_value: Option<U>,
    max_step: Option<U>,
    label: Option<ColouredLabel>,
}

impl<U: hex_entry::Hexable> HexEntryBuilder<U> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn max_step(&mut self, max_step: U) -> &mut Self {
        self.max_step = Some(max_step);
        self
    }

    pub fn initial_value(&mut self, initial_value: U) -> &mut Self {
        self.initial_value = Some(initial_value);
        self
    }

    #[allow(non_upper_case_globals)]
    pub fn build(&self) -> Rc<HexEntry<U>> {
        let max_step = if let Some(max_step) = self.max_step {
            max_step
        } else {
            cmp::max(U::max_val() >> 5, U::one())
        };
        let mut hex_entry_builder = hex_entry::HexEntry::builder();
        let _ = hex_entry_builder.max_step(max_step);
        if let Some(initial_value) = self.initial_value {
            let _ = hex_entry_builder.initial_value(initial_value);
        };
        if let Some(label) = self.label {
            let _ = hex_entry_builder.label(label);
        };
        HexEntry::new(hex_entry_builder.build())
    }
}
