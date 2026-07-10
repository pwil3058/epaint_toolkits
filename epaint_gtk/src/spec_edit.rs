// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::cell::RefCell;
use std::rc::Rc;

use colour_math::ScalarAttribute;
use colour_math_gtk::colour_edit::{ColourEditor, ColourEditorBuilder};
use pw_gtk_ext::sav_state::ConditionalWidgetGroupsBuilder;
use pw_gtk_ext::{
    gtk::{self, prelude::*},
    sav_state::{ConditionalWidgetGroups, MaskedCondns, WidgetStatesControlled},
    wrapper::*,
};

use epaint::PaintEssence;
use epaint::paint::SerializablePaintData;
use epaint::properties::{Properties, Property, PropertyType};

use crate::properties::PropertyEntry;
use crate::sav_state::*;

type AddCallback = Box<dyn Fn(&SerializablePaintData)>;
type AcceptCallback = Box<dyn Fn(&str, &SerializablePaintData)>;
type ChangeCallback = Box<dyn Fn(u64)>;

const CHANGED_MASK: u64 = SAV_ID_CHANGED
    + SAV_NAME_CHANGED
    + SAV_NOTES_CHANGED
    + SAV_RGB_CHANGED
    + SAV_FINISH_CHANGED
    + SAV_PERMANENCE_CHANGED
    + SAV_TRANSPARENCY_CHANGED
    + SAV_FLUORESCENCE_CHANGED
    + SAV_METALLICNESS_CHANGED;

pub fn property_sav_changed(property_type: PropertyType) -> u64 {
    match property_type {
        PropertyType::Finish => SAV_FINISH_CHANGED,
        PropertyType::Transparency => SAV_TRANSPARENCY_CHANGED,
        PropertyType::Metallicness => SAV_METALLICNESS_CHANGED,
        PropertyType::Opacity => SAV_OPACITY_CHANGED,
        PropertyType::Permanence => SAV_PERMANENCE_CHANGED,
        PropertyType::Luminescence => SAV_LIGHTFASTNESS_CHANGED,
        PropertyType::Granulation => SAV_GRANULATION_CHANGED,
        PropertyType::Staining => SAV_STAINING_CHANGED,
        PropertyType::Lightfastness => SAV_GRANULATION_CHANGED,
        PropertyType::Fluorescence => SAV_FLUORESCENCE_CHANGED,
    }
}

#[derive(PWO, Wrapper)]
pub struct BasicPaintSpecEditor {
    vbox: gtk::Box,
    name_entry: gtk::Entry,
    notes_entry: gtk::Entry,
    colour_editor: Rc<ColourEditor<u16>>,
    property_entries: Vec<Rc<PropertyEntry>>,
    buttons: ConditionalWidgetGroups<gtk::Button>,
    current_spec: RefCell<Option<SerializablePaintData>>,
    add_callbacks: RefCell<Vec<AddCallback>>,
    accept_callbacks: RefCell<Vec<AcceptCallback>>,
    change_callbacks: RefCell<Vec<ChangeCallback>>,
}

impl BasicPaintSpecEditor {
    pub fn new(attributes: &[ScalarAttribute], property_types: &[PropertyType]) -> Rc<Self> {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let grid = gtk::GridBuilder::new().hexpand(true).build();
        vbox.pack_start(&grid, false, false, 0);
        let label = gtk::LabelBuilder::new()
            .label("Name:")
            .halign(gtk::Align::End)
            .build();
        grid.attach(&label, 0, 1, 1, 1);
        let name_entry = gtk::EntryBuilder::new().hexpand(true).build();
        grid.attach(&name_entry, 1, 1, 1, 1);
        let label = gtk::LabelBuilder::new()
            .label("Notes:")
            .halign(gtk::Align::End)
            .build();
        grid.attach(&label, 0, 2, 1, 1);
        let notes_entry = gtk::EntryBuilder::new().hexpand(true).build();
        grid.attach(&notes_entry, 1, 2, 1, 1);

        let mut property_entries: Vec<Rc<PropertyEntry>> = Vec::new();
        for property_type in property_types.iter().copied() {
            property_entries.push(PropertyEntry::new(property_type));
        }

        let mut row: i32 = 3;
        for property_entry in property_entries.iter() {
            grid.attach(&property_entry.prompt(gtk::Align::End), 0, row, 1, 1);
            grid.attach(property_entry.pwo(), 1, row, 1, 1);
            row += 1;
        }

        let add_btn = gtk::ButtonBuilder::new().label("Add").build();
        let accept_btn = gtk::ButtonBuilder::new().label("Accept").build();
        let reset_btn = gtk::ButtonBuilder::new().label("Reset").build();
        let colour_editor = ColourEditorBuilder::new()
            .attributes(attributes)
            .extra_buttons(&[add_btn.clone(), accept_btn.clone(), reset_btn.clone()])
            .build();
        vbox.pack_start(colour_editor.pwo(), true, true, 0);
        let buttons = ConditionalWidgetGroupsBuilder::new()
            .widget_states_controlled(WidgetStatesControlled::Sensitivity)
            .build::<gtk::Button>();
        buttons
            .add_widget("add", &add_btn, SAV_ID_READY + SAV_NOT_EDITING)
            .expect("Duplicate key or button: add");
        buttons
            .add_widget(
                "accept",
                &accept_btn,
                SAV_ID_READY + SAV_HAS_CHANGES + SAV_EDITING,
            )
            .expect("Duplicate key or button: accept");
        buttons
            .add_widget("reset", &reset_btn, 0)
            .expect("Duplicate key or button: reset");
        let bpe = Rc::new(Self {
            vbox,
            name_entry,
            notes_entry,
            colour_editor,
            property_entries,
            buttons,
            current_spec: RefCell::new(None),
            add_callbacks: RefCell::new(Vec::new()),
            accept_callbacks: RefCell::new(Vec::new()),
            change_callbacks: RefCell::new(Vec::new()),
        });

        let bpe_c = Rc::clone(&bpe);
        add_btn.connect_clicked(move |_| bpe_c.process_add_action());

        let bpe_c = Rc::clone(&bpe);
        accept_btn.connect_clicked(move |_| bpe_c.process_accept_action());

        let bpe_c = Rc::clone(&bpe);
        reset_btn.connect_clicked(move |_| bpe_c.process_reset_action());

        let bpe_c = Rc::clone(&bpe);
        bpe.name_entry.connect_changed(move |entry| {
            let mut masked_condns = MaskedCondns {
                condns: 0,
                mask: SAV_NAME_READY + SAV_NAME_CHANGED,
            };
            if entry.get_text_length() > 0 {
                masked_condns.condns += SAV_NAME_READY;
            };
            if let Some(spec) = bpe_c.current_spec.borrow().as_ref() {
                if spec.name != entry.get_text() {
                    masked_condns.condns += SAV_NAME_CHANGED;
                }
            }
            bpe_c.buttons.update_condns(masked_condns);
            bpe_c.update_has_changes();
            bpe_c.inform_changed();
        });

        let bpe_c = Rc::clone(&bpe);
        bpe.notes_entry.connect_changed(move |entry| {
            let mut masked_condns = MaskedCondns {
                condns: 0,
                mask: SAV_NOTES_READY + SAV_NOTES_CHANGED,
            };
            if entry.get_text_length() > 0 {
                masked_condns.condns += SAV_NOTES_READY;
            };
            if let Some(spec) = bpe_c.current_spec.borrow().as_ref() {
                if spec.notes != entry.get_text() {
                    masked_condns.condns += SAV_NOTES_CHANGED;
                }
            }
            bpe_c.buttons.update_condns(masked_condns);
            bpe_c.update_has_changes();
            bpe_c.inform_changed();
        });

        let bpe_c = Rc::clone(&bpe);
        bpe.colour_editor.connect_changed(move |hcv| {
            let mut masked_condns = MaskedCondns {
                condns: 0,
                mask: SAV_RGB_CHANGED,
            };
            if let Some(spec) = bpe_c.current_spec.borrow().as_ref() {
                if &spec.colour != hcv {
                    masked_condns.condns += SAV_RGB_CHANGED;
                }
            }
            bpe_c.buttons.update_condns(masked_condns);
            bpe_c.update_has_changes();
            bpe_c.inform_changed();
        });

        // for (index, property_entry) in bpe.property_entries.iter().map(Rc::clone).enumerate() {
        //     let bpe_c = Rc::clone(&bpe);
        //     property_entry.connect_changed(move |entry| {
        //         let sav_condn = property_sav_changed(entry.property_type());
        //         let mut masked_condns = MaskedCondns {
        //             condns: 0,
        //             mask: sav_condn,
        //         };
        //         if let Some(spec) = bpe_c.current_spec.borrow().as_ref() {
        //             if spec.property_variants_f64[index] != entry.value().value {
        //                 masked_condns.condns += sav_condn;
        //             }
        //         }
        //         bpe_c.buttons.update_condns(masked_condns);
        //         bpe_c.update_has_changes();
        //         bpe_c.inform_changed();
        //     })
        // }
        if let Some(spec) = bpe.current_spec.borrow().as_ref() {
            for (property, property_entry) in spec
                .properties()
                .zip(bpe.property_entries.iter().map(Rc::clone))
            {
                let bpe_c = Rc::clone(&bpe);
                property_entry.connect_changed(move |property_entry| {
                    let sav_condn = property_sav_changed(property_entry.property_type());
                    let mut masked_condns = MaskedCondns {
                        condns: 0,
                        mask: sav_condn,
                    };
                    if property != property_entry.value() {
                        masked_condns.condns += sav_condn;
                    }
                    bpe_c.buttons.update_condns(masked_condns);
                    bpe_c.update_has_changes();
                    bpe_c.inform_changed();
                })
            }
        }

        // NB: needed to correctly set the current state
        bpe.set_current_spec(None);
        bpe.update_has_changes();

        bpe
    }

    fn update_has_changes(&self) {
        let mut masked_condns = MaskedCondns {
            condns: 0,
            mask: SAV_HAS_CHANGES,
        };
        if self.current_spec.borrow().is_some() {
            if self.buttons.current_condns() & CHANGED_MASK != 0 {
                masked_condns.condns = SAV_HAS_CHANGES;
            }
        } else if self.buttons.current_condns() & SAV_ID_READY != 0 {
            masked_condns.condns = SAV_HAS_CHANGES;
        }
        self.buttons.update_condns(masked_condns);
    }

    fn spec_from_entries(&self) -> SerializablePaintData {
        let properties: Properties = Properties(
            self.property_entries
                .iter()
                .map(|e| e.value())
                // .map(|entry| entry.value().value)
                .collect(),
        );
        SerializablePaintData {
            colour: self.colour_editor.hcv(),
            name: self.name_entry.get_text().to_string(),
            notes: self.notes_entry.get_text().to_string(),
            properties,
        }
    }

    fn process_add_action(&self) {
        let paint_spec = self.spec_from_entries();
        self.set_current_spec(Some(&paint_spec));
        self.update_has_changes();
        for callback in self.add_callbacks.borrow().iter() {
            callback(&paint_spec);
        }
    }

    fn process_accept_action(&self) {
        let edited_spec = self
            .current_spec
            .borrow()
            .clone()
            .expect("programming error");
        let paint_spec = self.spec_from_entries();
        self.set_current_spec(Some(&paint_spec));
        self.update_has_changes();
        for callback in self.accept_callbacks.borrow().iter() {
            callback(&edited_spec.name, &paint_spec);
        }
    }

    fn process_reset_action(&self) {
        if self.buttons.current_condns() & SAV_HAS_CHANGES != 0 {
            if self.buttons.current_condns() & SAV_ID_READY != 0 {
                let buttons = &[
                    ("Cancel", gtk::ResponseType::Other(0)),
                    ("Save and Continue", gtk::ResponseType::Other(1)),
                    ("Continue Discarding Changes", gtk::ResponseType::Other(2)),
                ];
                match self.ask_question("There are unsaved changes!", None, buttons) {
                    gtk::ResponseType::Other(0) => return,
                    gtk::ResponseType::Other(1) => {
                        if self.buttons.current_condns() & SAV_EDITING != 0 {
                            self.process_accept_action()
                        } else {
                            self.process_add_action()
                        }
                    }
                    _ => (),
                }
            } else {
                let buttons = &[
                    ("Cancel", gtk::ResponseType::Cancel),
                    ("Continue Discarding Changes", gtk::ResponseType::Accept),
                ];
                if self.ask_question("There are unsaved changes!", None, buttons)
                    == gtk::ResponseType::Cancel
                {
                    return;
                }
            }
        }
        self.set_current_spec(None);
        self.name_entry.set_text("");
        self.notes_entry.set_text("");
        // NB: do not reset properties
        self.colour_editor.reset();
        self.update_has_changes();
    }

    fn set_current_spec(&self, spec: Option<&SerializablePaintData>) {
        let mut masked_condns = MaskedCondns {
            condns: 0,
            mask: SAV_EDITING + SAV_NOT_EDITING + CHANGED_MASK,
        };
        if let Some(spec) = spec {
            *self.current_spec.borrow_mut() = Some(spec.clone());
            masked_condns.condns = SAV_EDITING;
        } else {
            *self.current_spec.borrow_mut() = None;
            masked_condns.condns = SAV_NOT_EDITING;
        };
        self.buttons.update_condns(masked_condns);
    }

    pub fn edit(&self, spec: &SerializablePaintData) {
        self.set_current_spec(Some(spec));
        self.name_entry.set_text(&spec.name);
        self.notes_entry.set_text(&spec.notes);
        self.colour_editor.set_colour(&spec.colour);
        for (property_entry, spec_value) in self
            .property_entries
            .iter()
            .zip(spec.properties().map(|p| p.value))
        {
            let property = Property {
                property_type: property_entry.property_type(),
                value: spec_value,
            };
            property_entry.set_value(Some(property))
        }
        self.update_has_changes();
    }

    pub fn un_edit(&self, name: &str) {
        let is_being_edited = if let Some(spec) = self.current_spec.borrow().as_ref() {
            name == spec.name
        } else {
            false
        };
        if is_being_edited {
            self.set_current_spec(None);
            self.update_has_changes();
        }
    }

    pub fn connect_add_action<F: Fn(&SerializablePaintData) + 'static>(&self, callback: F) {
        self.add_callbacks.borrow_mut().push(Box::new(callback))
    }

    pub fn connect_accept_action<F: Fn(&str, &SerializablePaintData) + 'static>(
        &self,
        callback: F,
    ) {
        self.accept_callbacks.borrow_mut().push(Box::new(callback))
    }

    pub fn inform_changed(&self) {
        let status = self.buttons.current_condns();
        for callback in self.change_callbacks.borrow().iter() {
            callback(status)
        }
    }

    pub fn connect_changed<F: Fn(u64) + 'static>(&self, callback: F) {
        self.change_callbacks.borrow_mut().push(Box::new(callback))
    }

    pub fn hard_reset(&self) {
        self.set_current_spec(None);
        self.name_entry.set_text("");
        self.notes_entry.set_text("");
        for property_entry in self.property_entries.iter() {
            property_entry.set_value(None)
        }
        self.colour_editor.reset();
        self.update_has_changes();
    }

    pub fn has_unsaved_changes(&self) -> bool {
        self.buttons.current_condns() & SAV_HAS_CHANGES != 0
    }
}
