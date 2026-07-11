// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::{cell::RefCell, rc::Rc};

use pw_gtk_ext::{
    gtk,
    gtk::{BoxExt, ComboBoxExt, ComboBoxTextExt, WidgetExt},
    wrapper::*,
};

use colour_math::HCV;
use colour_math_gtk::coloured::Colourable;

use epaint::mixtures::Mixture;
use epaint::paint::Paint;
pub use epaint::properties::{
    Finish, Fluorescence, Granulation, Lightfastness, Metallicness, Opacity, Permanence,
    Properties, Property, PropertyIfce, PropertyType, Staining, Transparency, str_values,
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
        for str_value in str_values(&property_type).iter() {
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

    pub fn property_type(&self) -> PropertyType {
        self.property_type
    }
}

pub trait PropertyEntries {
    fn property_entries(&self) -> impl Iterator<Item = Rc<PropertyEntry>>;
}

impl PropertyEntries for Properties {
    fn property_entries(&self) -> impl Iterator<Item = Rc<PropertyEntry>> {
        self.property_types().map(|p| PropertyEntry::new(p))
    }
}

impl PropertyEntries for Paint {
    fn property_entries(&self) -> impl Iterator<Item = Rc<PropertyEntry>> {
        self.data.properties.property_entries()
    }
}

impl PropertyEntries for Mixture {
    fn property_entries(&self) -> impl Iterator<Item = Rc<PropertyEntry>> {
        self.properties.property_entries()
    }
}

#[derive(PWO)]
pub struct PropertiesDisplay {
    vbox: gtk::Box,
}

impl PropertiesDisplay {
    pub fn create(properties: &Properties, colour: Option<&HCV>) -> Rc<Self> {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        for property in properties.properties() {
            let value = property.value();
            let label = gtk::LabelBuilder::new().label(value).build();
            if let Some(colour) = colour {
                label.set_widget_colour(colour)
            };
            vbox.pack_start(&label, false, false, 0);
        }
        vbox.show_all();
        Rc::new(PropertiesDisplay { vbox: vbox })
    }
}

pub trait PropertiesDisplayIfce {
    fn properties_display(&self, colour: Option<&HCV>) -> Rc<PropertiesDisplay>;
}

impl PropertiesDisplayIfce for Properties {
    fn properties_display(&self, colour: Option<&HCV>) -> Rc<PropertiesDisplay> {
        PropertiesDisplay::create(self, colour)
    }
}

impl PropertiesDisplayIfce for Paint {
    fn properties_display(&self, colour: Option<&HCV>) -> Rc<PropertiesDisplay> {
        self.data.properties.properties_display(colour)
    }
}

impl PropertiesDisplayIfce for Mixture {
    fn properties_display(&self, colour: Option<&HCV>) -> Rc<PropertiesDisplay> {
        self.properties.properties_display(colour)
    }
}
