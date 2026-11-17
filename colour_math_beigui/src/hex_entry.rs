// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::cell::Cell;
use std::num::ParseIntError;
use std::{cmp, mem};

use crate::ColouredLabel;

pub trait Hexable:
    Default
    + Ord
    + Clone
    + Copy
    + std::ops::Shr<u8, Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Add<Output = Self>
    + std::fmt::UpperHex
    + std::fmt::LowerHex
    + 'static
{
    fn max_val() -> Self;
    fn zero() -> Self;
    fn one() -> Self;
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError>;

    fn bytes() -> usize {
        mem::size_of::<Self>()
    }
}

macro_rules! impl_hexable {
    ($type:ty) => {
        impl Hexable for $type {
            fn max_val() -> Self {
                Self::MAX
            }
            fn zero() -> Self {
                Self::MIN
            }
            fn one() -> Self {
                1
            }
            fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError> {
                Self::from_str_radix(s, radix)
            }
        }
    };
}

impl_hexable!(u8);
impl_hexable!(u16);
impl_hexable!(u32);
impl_hexable!(u64);

#[derive(Debug)]
pub struct HexEntry<U: Hexable> {
    value: Cell<U>,
    current_step: Cell<U>,
    max_step: U,
    pub label: Option<ColouredLabel>,
}

impl<U: Hexable> Default for HexEntry<U> {
    fn default() -> Self {
        HexEntry::<U>::builder().build()
    }
}

impl<U: Hexable> HexEntry<U> {
    pub fn builder() -> HexEntryBuilder<U> {
        HexEntryBuilder::<U>::default()
    }

    pub fn value(&self) -> U {
        self.value.get()
    }

    pub fn set_value(&self, value: U) -> Option<U> {
        let old_value = self.value.get();
        self.value.set(value);
        self.changed_value(old_value)
    }

    fn changed_value(&self, old_value: U) -> Option<U> {
        if old_value != self.value.get() {
            Some(self.value.get())
        } else {
            None
        }
    }

    pub fn incr_value(&self) -> Option<U> {
        let value = self.value.get();
        let adj_incr = cmp::min(U::max_val() - value, self.current_step.get());
        if adj_incr > U::zero() {
            self.set_value(value + adj_incr);
        }
        if self.value.get() < U::max_val() {
            self.bump_current_step()
        }
        self.changed_value(value)
    }

    pub fn decr_value(&self) -> Option<U> {
        let value = self.value.get();
        let adj_decr = cmp::min(value, self.current_step.get());
        if adj_decr > U::zero() {
            self.set_value(value - adj_decr);
        }
        if self.value.get() > U::zero() {
            self.bump_current_step()
        }
        self.changed_value(value)
    }

    pub fn set_value_from_text(&self, text: &str) -> Result<Option<U>, ParseIntError> {
        let value = if let Some(index) = text.find('x') {
            U::from_str_radix(&text[index + 1..], 16)
        } else {
            U::from_str_radix(text, 16)
        }?;
        Ok(self.set_value(value))
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
    initial_value: Option<U>,
    max_step: Option<U>,
    label: Option<ColouredLabel>,
}

impl<U: Hexable> HexEntryBuilder<U> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn initial_value(&mut self, initial_value: U) -> &mut Self {
        self.initial_value = Some(initial_value);
        self
    }

    pub fn max_step(&mut self, max_step: U) -> &mut Self {
        self.max_step = Some(max_step);
        self
    }

    pub fn label(&mut self, label: ColouredLabel) -> &mut Self {
        self.label = Some(label);
        self
    }

    #[allow(non_upper_case_globals)]
    pub fn build(&self) -> HexEntry<U> {
        let max_step = if let Some(max_step) = self.max_step {
            max_step
        } else {
            cmp::max(U::max_val() >> 5, U::one())
        };
        let value = if let Some(initial_value) = self.initial_value {
            Cell::new(initial_value)
        } else {
            Cell::new(U::zero())
        };
        let current_step = Cell::new(U::one());
        HexEntry {
            value,
            max_step,
            current_step,
            label: self.label,
        }
    }
}
