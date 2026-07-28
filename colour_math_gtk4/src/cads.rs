// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{Box, DrawingArea, Orientation, Widget, glib};

use colour_math::ColourBasics;
use colour_math::attr_display::ColourAttributeType;

glib::wrapper! {
    pub struct ColourAttributeDisplay(ObjectSubclass<imp::ColourAttributeDisplay>)
        @extends DrawingArea, Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::CellRenderer;
}

impl ColourAttributeDisplay {
    pub fn new(colour_attr_type: &ColourAttributeType) -> Self {
        let obj: ColourAttributeDisplay = glib::Object::new();
        obj.set_size_request(90, 30);
        obj.imp().cad.borrow_mut().set_cat(colour_attr_type);
        obj
    }

    pub fn set_colour(&self, colour: Option<&impl ColourBasics>) {
        self.imp().cad.borrow_mut().set_colour(colour)
    }

    pub fn set_target_colour(&self, colour: Option<&impl ColourBasics>) {
        self.imp().cad.borrow_mut().set_target_colour(colour)
    }
}

mod imp {
    use std::cell::RefCell;
    use std::rc::Rc;

    use glib::Properties;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{DrawingArea, glib};

    use colour_math::beigui::attr_display;

    use crate::cm_cairo::{Drawer, Size};

    #[derive(Properties, Debug)]
    #[properties(wrapper_type = super::ColourAttributeDisplay)]
    pub struct ColourAttributeDisplay {
        pub cad: Rc<RefCell<attr_display::ColourAttributeDisplay>>,
    }

    impl Default for ColourAttributeDisplay {
        fn default() -> Self {
            Self {
                cad: Rc::new(RefCell::new(attr_display::ColourAttributeDisplay::default())),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ColourAttributeDisplay {
        const NAME: &'static str = "colourAttributeDisplay";
        type Type = super::ColourAttributeDisplay;
        type ParentType = DrawingArea;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ColourAttributeDisplay {
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

    impl WidgetImpl for ColourAttributeDisplay {}

    impl DrawingAreaImpl for ColourAttributeDisplay {}
}

use gtk::prelude::BoxExt;

glib::wrapper! {
    pub struct ColourAttributeDisplayBox(ObjectSubclass<imp_box::ColourAttributeDisplayBox>)
    @extends Box, Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl ColourAttributeDisplayBox {
    pub fn new(cats: &[ColourAttributeType]) -> Self {
        let obj: ColourAttributeDisplayBox = glib::Object::builder()
            .property("orientation", Orientation::Vertical)
            .build();
        for cat in cats {
            let cad = ColourAttributeDisplay::new(cat);
            obj.append(&cad);
            (obj.imp().cads).borrow_mut().push(cad);
        }

        obj
    }

    pub fn set_colour(&self, colour: Option<&impl ColourBasics>) {
        for cad in self.imp().cads.borrow().iter() {
            cad.set_colour(colour);
        }
    }

    pub fn set_target_colour(&self, colour: Option<&impl ColourBasics>) {
        for cad in self.imp().cads.borrow().iter() {
            cad.set_target_colour(colour);
        }
    }
}

mod imp_box {
    use std::cell::RefCell;

    use glib::Properties;
    use gtk::subclass::prelude::*;
    use gtk::{Box, glib};

    #[derive(Properties, Debug, Default)]
    #[properties(wrapper_type = super::ColourAttributeDisplayBox)]
    pub struct ColourAttributeDisplayBox {
        pub cads: RefCell<Vec<super::ColourAttributeDisplay>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ColourAttributeDisplayBox {
        const NAME: &'static str = "colourAttributeDisplayBox";
        type Type = super::ColourAttributeDisplayBox;
        type ParentType = Box;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ColourAttributeDisplayBox {}

    impl WidgetImpl for ColourAttributeDisplayBox {}

    impl BoxImpl for ColourAttributeDisplayBox {}
}
