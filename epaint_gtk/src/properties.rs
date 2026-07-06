// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::{cell::RefCell, rc::Rc};

use epaint::paint::SerializablePaintData;
use epaint::properties::Property;
pub use epaint::properties::{
    str_values, Finish, Fluorescence, Granulation, LightFastness, Metallicness, Opacity,
    Permanence, PropertyIfce, PropertyType, Staining, Transparency,
};

use epaint::SeriesId;
use pw_gtk_ext::{
    gtk,
    gtk::{ComboBoxExt, ComboBoxTextExt},
    wrapper::*,
};

type ChangeCallback<T> = Box<dyn Fn(&T)>;

#[derive(PWO)]
pub struct PropertyEntry {
    combo_box_text: gtk::ComboBoxText,
    callbacks: RefCell<Vec<ChangeCallback<Self>>>,
    property_type: PropertyType,
}

impl PropertyEntry {
    pub fn new(property_type: PropertyType) -> Rc<Self> {
        let combo_box_text = gtk::ComboBoxText::new();
        for str_value in str_values(property_type).iter() {
            combo_box_text.append_text(str_value);
        }
        combo_box_text.set_id_column(0);
        let ce = Rc::new(Self {
            combo_box_text,
            callbacks: RefCell::new(vec![]),
            property_type,
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
        gtk::Label::new(Some(self.property_type.name()))
    }

    pub fn prompt(&self, align: gtk::Align) -> gtk::Label {
        gtk::LabelBuilder::new()
            .label(self.property_type.prompt())
            .halign(align)
            .build()
    }

    pub fn value(&self) -> Property {
        if let Some(text) = self.combo_box_text.get_active_text() {
            Property::from((self.property_type, text.as_ref()))
        } else {
            self.property_type.default_property()
        }
    }

    pub fn set_value(&self, new_value: Option<Property>) {
        let id = if let Some(new_value) = new_value {
            new_value.value()
        } else {
            self.property_type.default_str()
        };
        self.combo_box_text.set_active_id(Some(id));
    }

    pub fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) {
        self.callbacks.borrow_mut().push(Box::new(f))
    }
}
