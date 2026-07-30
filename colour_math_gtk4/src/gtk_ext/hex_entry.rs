// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.
use std::cell::{Cell, RefCell};
use std::cmp;

use gtk::Entry;
use gtk::prelude::*;

use num_traits::Num;
use num_traits_plus::NumberConstants;

use colour_math::UnsignedLightLevel;

pub trait Hexable:
    UnsignedLightLevel + NumberConstants + Num + std::ops::Shr<u8, Output = Self> + 'static
{
}

impl Hexable for u8 {}
impl Hexable for u16 {}
impl Hexable for u32 {}
impl Hexable for u64 {}

type ChangeCallback<U> = Box<dyn Fn(U)>;

pub struct HexEntry<U: Hexable> {
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
