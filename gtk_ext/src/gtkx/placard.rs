// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::gdk;
use crate::glib::{self, wrapper};
use crate::gtk::{prelude::*, subclass::prelude::*};

use crate::gtkx::coloured::ColourableWidgetExt;

#[derive(Default)]
pub struct PlacardImp;

#[glib::object_subclass]
impl ObjectSubclass for PlacardImp {
    const NAME: &str = "Placard";
    type Type = Placard;
    type ParentType = gtk::Button;
}

impl ObjectImpl for PlacardImp {
    fn constructed(&self) {
        self.parent_constructed();

        self.obj().set_relief(gtk::ReliefStyle::None);
        self.obj().set_focus_on_click(false);
        self.obj().set_can_focus(false);
    }
}
impl WidgetImpl for PlacardImp {}
impl ContainerImpl for PlacardImp {}
impl BinImpl for PlacardImp {}
impl ButtonImpl for PlacardImp {}

wrapper! {
    pub struct Placard(ObjectSubclass<PlacardImp>)
        @extends gtk::Button, gtk::Widget, gtk::Container, gtk::Bin;
}

impl Placard {
    pub fn builder() -> PlacardBuilder {
        PlacardBuilder::default()
    }

    pub fn new() -> Self {
        glib::Object::builder::<Placard>().build()
    }

    pub fn with_label(label: &str) -> Placard {
        let placard = Self::new();
        placard.set_label(label);
        placard
    }
}

impl Default for Placard {
    fn default() -> Self {
        Self::new()
    }
}

impl ColourableWidgetExt for Placard {}

#[derive(Default)]
pub struct PlacardBuilder {
    label: String,
    colours: Option<(gdk::RGBA, gdk::RGBA)>,
    // background: Option<gdk::RGBA>,
    // foreground: Option<gdk::RGBA>,
}

impl PlacardBuilder {
    pub fn label(&mut self, label: &str) -> &mut PlacardBuilder {
        self.label = label.to_owned();
        self
    }

    pub fn colours(
        &mut self,
        background: &gdk::RGBA,
        foreground: &gdk::RGBA,
    ) -> &mut PlacardBuilder {
        self.colours = Some((*background, *foreground));
        self
    }

    pub fn build(&self) -> Placard {
        let placard = Placard::with_label(&self.label);
        if let Some((background, foreground)) = self.colours {
            placard.set_widget_colours(&background, &foreground);
        }
        placard
    }
}

#[cfg(test)]
mod placard_tests {
    use super::*;

    use crate::gdk::RGBA;

    #[test]
    fn test_new() {
        let placard = Placard::new();
        placard.set_label("label");
        placard.set_widget_colours(
            &RGBA::new(1.0, 0.0, 0.0, 1.0),
            &RGBA::new(0.0, 1.0, 0.0, 1.0),
        );
        let placard2 = Placard::builder().label("label").build();
        debug_assert_ne!(placard.label(), placard2.label());
    }
}
