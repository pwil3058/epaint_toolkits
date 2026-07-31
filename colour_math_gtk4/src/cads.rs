// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::cell::RefCell;
use std::rc::Rc;

use gtk::DrawingArea;
use gtk::prelude::*;

use gtk4_ext::PackableWidgetObject;
use gtk4_ext_derive::PWO;

use colour_math::ColourBasics;
use colour_math::attr_display::ColourAttributeType;

use colour_math::beigui::attr_display;

use crate::cm_cairo::{Drawer, Size};

#[derive(Debug, PWO)]
pub struct ColourAttributeDisplay {
    pub drawing_area: DrawingArea,
    pub colout_attr_display: RefCell<attr_display::ColourAttributeDisplay>,
}

impl ColourAttributeDisplay {
    pub fn new(colour_attr_type: &ColourAttributeType) -> Rc<Self> {
        let drawing_area = DrawingArea::builder()
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
        let cad_c = Rc::clone(&cad);
        cad.drawing_area
            .set_draw_func(move |_drawing_area, cairo_context, width, height| {
                let size = Size {
                    width: width.into(),
                    height: height.into(),
                };
                let drawer = Drawer::new(cairo_context, size);
                cad_c.colout_attr_display.borrow().draw_all(&drawer);
            });

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
            vbox.append(cad.pwo());
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
