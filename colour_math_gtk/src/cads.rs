// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::{cell::RefCell, rc::Rc};

use pw_gtk_ext::{
    gtk::{self, BoxExt, DrawingArea, WidgetExt},
    wrapper::*,
};

use colour_math::attr_display::ColourAttributeType;
use colour_math::ColourBasics;

use colour_math::beigui::attr_display;

use colour_math_cairo::{Drawer, Size};
use pw_gtk_ext::gtk::DrawingAreaBuilder;

#[derive(Debug, PWO)]
pub struct ColourAttributeDisplay {
    pub drawing_area: DrawingArea,
    pub colout_attr_display: RefCell<attr_display::ColourAttributeDisplay>,
}

impl ColourAttributeDisplay {
    pub fn new(colour_attr_type: &ColourAttributeType) -> Rc<Self> {
        let drawing_area = DrawingAreaBuilder::new()
            .hexpand(true)
            .height_request(30)
            .width_request(90)
            .build();
        let colour_attr_display =
            RefCell::new(attr_display::ColourAttributeDisplay::new(colour_attr_type));
        let cad = Rc::new(Self {
            drawing_area,
            colout_attr_display: colour_attr_display,
        });
        cad.drawing_area.set_size_request(90, 30);
        let cad_c = Rc::clone(&cad);
        cad.drawing_area
            .connect_draw(move |drawing_area, cairo_context| {
                let size = Size {
                    width: drawing_area.get_allocated_width() as f64,
                    height: drawing_area.get_allocated_height() as f64,
                };
                let drawer = Drawer::new(cairo_context, size);
                cad_c.colout_attr_display.borrow().draw_all(&drawer);
                gtk::Inhibit(false)
            });
        // cad.drawing_area
        //     .set_draw_func(move |_drawing_area, cairo_context, width, height| {
        //         let size = Size {
        //             width: width.into(),
        //             height: height.into(),
        //         };
        //         let drawer = Drawer::new(cairo_context, size);
        //         cad_c.colout_attr_display.borrow().draw_all(&drawer);
        //     });

        cad
    }

    pub fn set_colour(&self, colour: Option<&impl ColourBasics>) {
        self.colout_attr_display.borrow_mut().set_colour(colour)
    }

    pub fn set_target_colour(&self, colour: Option<&impl ColourBasics>) {
        self.colout_attr_display
            .borrow_mut()
            .set_target_colour(colour)
    }
}

#[derive(Debug, PWO)]
pub struct ColourAttributeDisplayBox {
    pub vbox: gtk::Box,
    pub cads: RefCell<Vec<Rc<ColourAttributeDisplay>>>,
}

impl ColourAttributeDisplayBox {
    pub fn new(cats: &[ColourAttributeType]) -> Rc<Self> {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 1);
        let cads = RefCell::new(Vec::with_capacity(cats.len()));
        for cat in cats {
            let cad = ColourAttributeDisplay::new(cat);
            vbox.pack_start(cad.pwo(), false, false, 0);
            cads.borrow_mut().push(cad);
        }

        Rc::new(Self { vbox, cads })
    }

    pub fn set_colour(&self, colour: Option<&impl ColourBasics>) {
        for cad in self.cads.borrow().iter() {
            cad.set_colour(colour);
        }
    }

    pub fn set_target_colour(&self, colour: Option<&impl ColourBasics>) {
        for cad in self.cads.borrow().iter() {
            cad.set_target_colour(colour);
        }
    }
}
