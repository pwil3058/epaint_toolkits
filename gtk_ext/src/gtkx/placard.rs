// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

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
    pub fn new() -> Self {
        glib::Object::builder::<Placard>().build()
    }

    pub fn with_label(label: &str) -> Placard {
        let placard = Self::new();
        placard.set_label(label);
        placard
    }
}

impl ColourableWidgetExt for Placard {}

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
    }
}
