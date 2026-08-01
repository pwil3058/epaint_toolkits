// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::cell::{Cell, RefCell};
use std::cmp;
use std::path::{MAIN_SEPARATOR, PathBuf};
use std::rc::Rc;

use gtk::Entry;
use gtk::gdk::Key;
use gtk::glib::Propagation;
use gtk::prelude::*;

use path_utilities;
//use pw_pathux;

use crate::{gtkx::list_store::*, wrapper::*};

// Labelled Text Entry

#[derive(Debug, PWO)]
pub struct LabelledTextEntry {
    h_box: gtk::Box,
    entry: gtk::Entry,
}

impl LabelledTextEntry {
    pub fn new(label: &str) -> Rc<Self> {
        let lte = Rc::new(Self {
            h_box: gtk::Box::new(gtk::Orientation::Horizontal, 0),
            entry: gtk::Entry::new(),
        });

        let label = gtk::Label::new(Some(label));
        lte.h_box.pack_start(&label, false, false, 0);
        lte.h_box.pack_start(&lte.entry, true, true, 0);

        lte
    }

    pub fn entry(&self) -> &gtk::Entry {
        &self.entry
    }
}

// Hex Entry

type ChangeCallback<U> = Box<dyn Fn(U)>;

pub trait Hexable:
    Default
    + Ord
    + Copy
    + num_traits_plus::NumberConstants
    + num_traits::Num
    + std::fmt::UpperHex
    + std::ops::Shr<u8, Output = Self>
    + 'static
{
}

impl Hexable for u8 {}
impl Hexable for u16 {}
impl Hexable for u32 {}
impl Hexable for u64 {}

#[derive(PWO)]
pub struct HexEntry<U: Hexable>
{
    entry: Entry,
    value: Cell<U>,
    current_step: Cell<U>,
    max_step: U,
    callbacks: RefCell<Vec<ChangeCallback<U>>>,
}

impl<U: Hexable> HexEntry<U> {
    pub fn value(&self) -> U {
        self.value.get()
    }

    pub fn set_value(&self, value: U) {
        self.value.set(value);
        self.reset_entry_text();
    }

    pub fn connect_value_changed<F: 'static + Fn(U)>(&self, callback: F) {
        self.callbacks.borrow_mut().push(Box::new(callback))
    }

    fn incr_value(&self) {
        let value = self.value.get();
        let adj_incr = cmp::min(U::MAX - value, self.current_step.get());
        if adj_incr > U::zero() {
            self.set_value_and_notify(value + adj_incr);
        }
        if self.value.get() < U::MAX {
            self.bump_current_step()
        }
    }

    fn decr_value(&self) {
        let value = self.value.get();
        let adj_decr = cmp::min(value, self.current_step.get());
        if adj_decr > U::zero() {
            self.set_value_and_notify(value - adj_decr);
        }
        if self.value.get() > U::MIN {
            self.bump_current_step()
        }
    }

    fn reset_entry_text(&self) {
        self.entry.set_text(&format!(
            "{:#0width$X}",
            self.value.get(),
            width = U::BYTES * 2 + 2
        ));
    }

    fn set_value_from_text(&self, text: &str) {
        let value = if let Some(index) = text.find('x') {
            U::from_str_radix(&text[index + 1..], 16)
        } else {
            U::from_str_radix(text, 16)
        };
        if let Ok(value) = value {
            self.set_value_and_notify(value);
        } else {
            self.reset_entry_text();
        }
    }

    fn set_value_and_notify(&self, value: U) {
        self.set_value(value);
        self.inform_value_changed();
    }

    fn inform_value_changed(&self) {
        let value = self.value.get();
        for callback in self.callbacks.borrow().iter() {
            callback(value);
        }
    }

    fn bump_current_step(&self) {
        let new_step = cmp::min(self.current_step.get() + U::one(), self.max_step);
        self.current_step.set(new_step);
    }

    fn reset_current_step(&self) {
        self.current_step.set(U::one());
    }
}

#[derive(Default)]
pub struct HexEntryBuilder<U: Hexable> {
    initial_value: U,
    editable: bool,
}

impl<U: Hexable> HexEntryBuilder<U> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn editable(&mut self, editable: bool) -> &mut Self {
        self.editable = editable;
        self
    }

    pub fn initial_value(&mut self, initial_value: U) -> &mut Self {
        self.initial_value = initial_value;
        self
    }

    #[allow(non_upper_case_globals)]
    pub fn build(&self) -> Rc<HexEntry<U>> {
        let entry = Entry::builder()
            .width_chars(U::BYTES as i32 * 2 + 2)
            .editable(self.editable)
            .build();
        let value = Cell::new(self.initial_value);
        let max_step = cmp::max(U::MAX >> 5, U::ONE);
        let current_step = Cell::new(U::ONE);
        let callbacks: RefCell<Vec<ChangeCallback<U>>> = RefCell::new(Vec::new());
        let hex_entry = Rc::new(HexEntry {
            entry,
            value,
            max_step,
            current_step,
            callbacks,
        });
        hex_entry.reset_entry_text();

        let hex_entry_c = Rc::clone(&hex_entry);
        let key_controller = gtk::EventControllerKey::new();
        // key_controller.set_propagation_phase(gtk::PropagationPhase::Capture);
        key_controller.connect_key_pressed(move |_, key, _, _| match key {
            Key::Return | Key::Tab | Key::ISO_Left_Tab => {
                let text = hex_entry_c.entry.text();
                if text.is_empty() {
                    hex_entry_c.reset_entry_text();
                } else {
                    hex_entry_c.set_value_from_text(&text);
                }
                // NB: this will nobble the "activate" signal
                // but let the Tab key move the focus
                if key == Key::Return {
                    Propagation::Stop
                } else {
                    Propagation::Proceed
                }
            }
            Key::Up => {
                hex_entry_c.incr_value();
                Propagation::Stop
            }
            Key::Down => {
                hex_entry_c.decr_value();
                Propagation::Stop
            }
            Key::_0
            | Key::_1
            | Key::_2
            | Key::_3
            | Key::_4
            | Key::_5
            | Key::_6
            | Key::_7
            | Key::_8
            | Key::_9
            | Key::A
            | Key::B
            | Key::C
            | Key::D
            | Key::E
            | Key::F
            | Key::BackSpace
            | Key::Delete
            | Key::Copy
            | Key::Paste
            | Key::x
            | Key::a
            | Key::b
            | Key::c
            | Key::d
            | Key::e
            | Key::f
            | Key::Left
            | Key::Right => Propagation::Proceed,
            _ => Propagation::Stop,
        });
        let hex_entry_c = Rc::clone(&hex_entry);
        key_controller.connect_key_released(move |_, key, _, _| match key {
            Key::Up | Key::Down => {
                hex_entry_c.reset_entry_text();
                // Propagation::Stop
            }
            _ => (), //Propagation::Proceed,
        });
        hex_entry.entry.add_controller(key_controller);

        hex_entry
    }
}

// FILEPATH COMPLETION

pub trait PathCompletion: EntryExt + EditableSignals {
    fn _enable_path_completion(&self, dirs_only: bool) {
        let entry_completion = gtk::EntryCompletion::new();
        entry_completion.pack_start(&gtk::CellRendererText::new(), true);
        entry_completion.set_text_column(0);
        entry_completion.set_inline_completion(true);
        entry_completion.set_inline_selection(true);
        entry_completion.set_minimum_key_length(0);
        let list_store = gtk::ListStore::new(&[glib::Type::String]);
        entry_completion.set_model(Some(&list_store));

        self.set_completion(Some(&entry_completion));
        self.connect_changed(move |editable| {
            list_store.clear();
            let dir_pathbuf = match PathBuf::from(editable.get_text().as_str()).parent() {
                Some(path) => path.to_path_buf(),
                None => PathBuf::new(),
            };
            let dir_path = match path_utilities::absolute_pathbuf(&dir_pathbuf) {
                Some(abs_pathbuf) => abs_pathbuf,
                None => dir_pathbuf.clone(),
            };
            if let Ok(entries) = path_utilities::usable_dir_entries(&dir_path) {
                if dirs_only {
                    for entry in entries {
                        if !entry.is_dir() {
                            continue;
                        };
                        let mut path = dir_pathbuf.clone();
                        path.push(&entry.file_name());
                        if let Some(string) = path.to_str() {
                            list_store.append_row(&[string.to_value()]);
                        }
                    }
                } else {
                    let msep = format!("{}", MAIN_SEPARATOR);
                    for entry in entries {
                        let mut path = dir_pathbuf.clone();
                        path.push(&entry.file_name());
                        if entry.is_dir() {
                            path.push(&msep);
                        };
                        if let Some(string) = path.to_str() {
                            list_store.append_row(&[string.to_value()]);
                        }
                    }
                }
            };
        });
    }

    fn enable_dir_path_completion(&self) {
        self._enable_path_completion(true)
    }

    fn enable_file_path_completion(&self) {
        self._enable_path_completion(false)
    }
}

impl PathCompletion for gtk::Entry {}
