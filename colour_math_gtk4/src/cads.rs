// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::cell::RefCell;
use std::rc::Rc;

use glib::Object;

use gtk::prelude::WidgetExt;
use gtk::subclass::prelude::*;
use gtk::{DrawingArea, Widget, glib};

use colour_math::ColourBasics;
use colour_math::attr_display::{ColourAttributeDisplay, ColourAttributeType};

glib::wrapper! {
    pub struct GtkColourAttributeDisplay(ObjectSubclass<imp::GtkColourAttributeDisplay>)
        @extends DrawingArea, Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl GtkColourAttributeDisplay {
    pub fn new(colour_attr_type: &ColourAttributeType) -> Self {
        let mut obj: GtkColourAttributeDisplay = glib::Object::new();
        obj.set_size_request(90, 30);
        obj.imp().cad.borrow_mut().set_cat(colour_attr_type);
        obj
    }

    pub fn set_colour(&self, colour: Option<&impl ColourBasics>) {
        self.imp().cad.borrow_mut().set_colour(colour)
    }
}

mod imp {
    use std::cell::RefCell;
    use std::rc::Rc;

    use glib::Properties;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{DrawingArea, glib};

    use colour_math::ColourBasics;
    use colour_math::attr_display::ColourAttributeType;
    use colour_math::beigui::attr_display::ColourAttributeDisplay;

    use crate::cm_cairo::{Drawer, Size};

    #[derive(Properties, Debug)]
    #[properties(wrapper_type = super::GtkColourAttributeDisplay)]
    pub struct GtkColourAttributeDisplay {
        pub cad: Rc<RefCell<ColourAttributeDisplay>>,
    }

    impl Default for GtkColourAttributeDisplay {
        fn default() -> Self {
            Self {
                cad: Rc::new(RefCell::new(ColourAttributeDisplay::default())),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GtkColourAttributeDisplay {
        const NAME: &'static str = "colourAttributeDisplay";
        type Type = super::GtkColourAttributeDisplay;
        type ParentType = DrawingArea;
    }

    #[glib::derived_properties]
    impl ObjectImpl for GtkColourAttributeDisplay {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().set_size_request(90, 30);
            let cad_c = Rc::clone(&self.cad);
            self.obj()
                .set_draw_func(move |_drawing_area, cairo_context, width, height| {
                    let size = Size {
                        width: width.into(),
                        height: height.into(),
                    };
                    let drawer = Drawer::new(cairo_context, size);
                    cad_c.borrow().draw_all(&drawer);
                });
        }
    }

    impl WidgetImpl for GtkColourAttributeDisplay {}

    impl DrawingAreaImpl for GtkColourAttributeDisplay {
        // fn set_drawing_area(&self, area: gtk::Rectangle) {}
    }
}
