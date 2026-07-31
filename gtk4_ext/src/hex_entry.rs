// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.
use std::cell::{Cell, RefCell};
use std::cmp;
use std::rc::Rc;

use gtk::Entry;
use gtk::gdk::Key;
use gtk::glib::Propagation;
use gtk::prelude::*;

use gtk4_ext_derive::PWO;

use crate::PackableWidgetObject;

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
where
    U: Default
        + Ord
        + Copy
        + num_traits_plus::NumberConstants
        + num_traits::Num
        + std::fmt::UpperHex
        + std::ops::Shr<u8, Output = U>
        + 'static,
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

    pub fn incr_value(&self) {
        let value = self.value.get();
        let adj_incr = cmp::min(U::MAX - value, self.current_step.get());
        if adj_incr > U::zero() {
            self.set_value_and_notify(value + adj_incr);
        }
        if self.value.get() < U::MAX {
            self.bump_current_step()
        }
    }

    pub fn decr_value(&self) {
        let value = self.value.get();
        let adj_decr = cmp::min(value, self.current_step.get());
        if adj_decr > U::zero() {
            self.set_value_and_notify(value - adj_decr);
        }
        if self.value.get() > U::MIN {
            self.bump_current_step()
        }
    }

    pub fn reset_entry_text(&self) {
        self.entry.set_text(&format!(
            "{:#0width$X}",
            self.value.get(),
            width = U::BYTES * 2 + 2
        ));
    }

    pub fn set_value_from_text(&self, text: &str) {
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

    pub fn set_value_and_notify(&self, value: U) {
        self.set_value(value);
        self.inform_value_changed();
    }

    pub fn inform_value_changed(&self) {
        let value = self.value.get();
        for callback in self.callbacks.borrow().iter() {
            callback(value);
        }
    }

    pub fn bump_current_step(&self) {
        let new_step = cmp::min(self.current_step.get() + U::one(), self.max_step);
        self.current_step.set(new_step);
    }

    pub fn reset_current_step(&self) {
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
