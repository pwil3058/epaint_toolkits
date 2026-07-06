// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::{cell::RefCell, rc::Rc};

use pw_gtk_ext::{
    gtk,
    gtk::{ComboBoxExt, ComboBoxTextExt},
    wrapper::*,
};

pub use apaint::properties::{
    Finish, Fluorescence, Metallicness, Permanence, PropertyIfce, PropertyType, Transparency,
};
use apaint::properties::{Granulation, LightFastness, Opacity, Staining};

type ChangeCallback<T> = Box<dyn Fn(&T)>;

#[derive(PWO)]
pub struct PropertyEntry<C: 'static + PropertyIfce> {
    combo_box_text: gtk::ComboBoxText,
    callbacks: RefCell<Vec<ChangeCallback<Self>>>,
    marker: std::marker::PhantomData<C>,
}

impl<C: PropertyIfce> PropertyEntry<C> {
    pub fn new() -> Rc<Self> {
        let combo_box_text = gtk::ComboBoxText::new();
        for str_value in C::str_values().iter() {
            combo_box_text.append_text(str_value);
        }
        combo_box_text.set_id_column(0);
        let ce = Rc::new(Self {
            combo_box_text,
            callbacks: RefCell::new(vec![]),
            marker: std::marker::PhantomData,
        });
        ce.set_value(None);
        let ce_clone = Rc::clone(&ce);
        ce.combo_box_text.connect_changed(move |_| {
            for callback in ce_clone.callbacks.borrow().iter() {
                callback(&ce_clone);
            }
        });
        ce
    }

    pub fn label(&self) -> gtk::Label {
        gtk::Label::new(Some(C::NAME))
    }

    pub fn prompt(&self, align: gtk::Align) -> gtk::Label {
        gtk::LabelBuilder::new()
            .label(C::PROMPT)
            .halign(align)
            .build()
    }

    pub fn value(&self) -> C {
        if let Some(text) = self.combo_box_text.get_active_text() {
            C::from_str(&text).expect("all strings should be valid")
        } else {
            C::default()
        }
    }

    pub fn set_value(&self, new_value: Option<C>) {
        let id = if let Some(new_value) = new_value {
            new_value.full()
        } else {
            C::default().full()
        };
        self.combo_box_text.set_active_id(Some(id));
    }

    pub fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) {
        self.callbacks.borrow_mut().push(Box::new(f))
    }
}

pub type FinishEntry = PropertyEntry<Finish>;
pub type TransparencyEntry = PropertyEntry<Transparency>;
pub type PermanenceEntry = PropertyEntry<Permanence>;
pub type FluorescenceEntry = PropertyEntry<Fluorescence>;
pub type MetallicnessEntry = PropertyEntry<Metallicness>;
pub type GranulationEntry = PropertyEntry<Granulation>;
pub type LightFastnessEntry = PropertyEntry<LightFastness>;
pub type StainingEntry = PropertyEntry<Staining>;
pub type OpacityEntry = PropertyEntry<Opacity>;
