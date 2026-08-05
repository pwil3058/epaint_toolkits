// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::cell::RefCell;

use crate::{
    gdk::RGBA,
    glib,
    gtk::prelude::*,
    gtk::subclass::prelude::*,
    gtk::{CssProvider, EventBox, Label, STYLE_PROVIDER_PRIORITY_APPLICATION},
};

pub struct ColourLabelImp {
    label: Label,
    // Storing the provider here guarantees it stays alive as long as the widget does
    provider: RefCell<Option<CssProvider>>,
}

impl Default for ColourLabelImp {
    fn default() -> Self {
        Self {
            label: Label::new(None),
            provider: RefCell::new(None),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for ColourLabelImp {
    const NAME: &'static str = "ColourLabel";
    type Type = ColourLabel;
    type ParentType = EventBox;
}

// 3. Implement required boilerplates
impl ObjectImpl for ColourLabelImp {
    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();
        obj.set_visible_window(true);
        obj.set_app_paintable(true);
        let style_context = obj.style_context();
        style_context.add_class("colour-container");
        // let container = obj.upcast_ref::<gtk::Container>();
        obj.add(&self.label);
    }
}
impl WidgetImpl for ColourLabelImp {}
impl ContainerImpl for ColourLabelImp {}
impl EventBoxImpl for ColourLabelImp {}
impl BinImpl for ColourLabelImp {}

glib::wrapper! {
    pub struct ColourLabel(ObjectSubclass<ColourLabelImp>)
        @extends gtk::Label, gtk::Widget, gtk::Container, gtk::EventBox;
}

impl ColourLabel {
    pub fn new(text: Option<&str>) -> Self {
        let obj: ColourLabel = glib::object::Object::builder().build();
        if let Some(text) = text {
            obj.imp().label.set_label(text);
        }

        obj
    }

    pub fn set_label(&self, text: &str) {
        self.imp().label.set_label(text);
    }

    pub fn set_text(&self, text: &str) {
        self.imp().label.set_text(text);
    }

    // Call this method programmatically from your state management loop
    pub fn set_colours(&self, bg: &RGBA, fg: &RGBA) {
        println!("set_colours: {:?} {:?}", bg, fg);
        let imp = self.imp();
        let style_context = self.style_context();

        // Safe cleanup: remove the previous provider to avoid stacking styling layers
        if let Some(ref old_provider) = *imp.provider.borrow() {
            style_context.remove_provider(old_provider);
        }

        // Build the explicit CSS rule targeting the local label element
        let css = format!(
            "colour-container {{ background-image: none; background-color: {0}; }} \
             colour-container * {{ color: {1}; }}",
            bg.to_string(),
            fg.to_string()
        );

        let new_provider = CssProvider::new();
        new_provider.load_from_data(css.as_bytes()).unwrap();

        // Feed the styles directly into the live layout engine
        style_context.add_provider(&new_provider, STYLE_PROVIDER_PRIORITY_APPLICATION);

        // Safely mutate our private state to retain ownership of the provider
        *imp.provider.borrow_mut() = Some(new_provider);
    }
}
