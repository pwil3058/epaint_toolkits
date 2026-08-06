// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use gtk_ext::{
    glib,
    gtk::{
        self,
        prelude::{BoxExt, RadioButtonExt, ToggleButtonExt, WidgetExt},
        DrawingArea,
    },
    wrapper::*,
};

use colour_math::{
    attr_display, attr_display::ColourAttributeType, ColourBasics, ScalarAttribute, HCV,
};
use colour_math_cairo::{Drawer, Size};

use crate::colour::GdkColour;

#[derive(PWO, Wrapper)]
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
        cad.drawing_area.set_size_request(90, 30);
        let cad_c = Rc::clone(&cad);
        cad.drawing_area
            .connect_draw(move |drawing_area, cairo_context| {
                let size = Size {
                    width: drawing_area.allocated_width() as f64,
                    height: drawing_area.allocated_height() as f64,
                };
                let drawer = Drawer::new(cairo_context, size);
                cad_c.colout_attr_display.borrow().draw_all(&drawer);
                glib::Propagation::Stop
                // gtk::Inhibit(false)
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

#[derive(PWO, Wrapper)]
pub struct ColourAttributeDisplayStack {
    vbox: gtk::Box,
    cads: RefCell<Vec<Rc<ColourAttributeDisplay>>>,
}

impl ColourAttributeDisplayStack {
    pub fn builder() -> ColourAttributeDisplayStackBuilder {
        ColourAttributeDisplayStackBuilder::new()
    }

    pub fn new(scalar_attributes: &[ScalarAttribute]) -> Rc<Self> {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 1);
        let cads = RefCell::new(Vec::with_capacity(scalar_attributes.len() + 1));
        let huecad = ColourAttributeDisplay::new(&ColourAttributeType::Hue);
        vbox.pack_start(huecad.pwo(), false, false, 0);
        cads.borrow_mut().push(huecad);
        for scalar_attribute in scalar_attributes {
            let cad = ColourAttributeDisplay::new(&scalar_attribute.into());
            vbox.pack_start(cad.pwo(), false, false, 0);
            cads.borrow_mut().push(cad);
        }

        Rc::new(Self { vbox, cads })
    }
    pub fn set_colour(&self, colour: Option<&impl GdkColour>) {
        for cad in self.cads.borrow().iter() {
            if let Some(colour) = colour {
                cad.set_colour(Some(&colour.hcv()));
            } else {
                cad.set_colour(None::<&HCV>);
            }
        }
    }

    pub fn set_target_colour(&self, colour: Option<&impl GdkColour>) {
        for cad in self.cads.borrow().iter() {
            if let Some(colour) = colour {
                cad.set_target_colour(Some(&colour.hcv()));
            } else {
                cad.set_target_colour(None::<&HCV>);
            }
        }
    }
}

#[derive(Default)]
pub struct ColourAttributeDisplayStackBuilder {
    // TODO: add orientation as an option for CAD stacks
    attributes: Vec<ScalarAttribute>,
}

impl ColourAttributeDisplayStackBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn scalar_attributes(&mut self, scalar_attributes: &[ScalarAttribute]) -> &mut Self {
        self.attributes = scalar_attributes.to_vec();
        self
    }

    pub fn build(&self) -> Rc<ColourAttributeDisplayStack> {
        ColourAttributeDisplayStack::new(&self.attributes)
    }
}

type SelectionCallback = Box<dyn Fn(ScalarAttribute)>;

#[derive(PWO)]
pub struct ScalarAttributeSelector {
    gtk_box: gtk::Box,
    attribute: Cell<ScalarAttribute>,
    callbacks: RefCell<Vec<SelectionCallback>>,
}

impl ScalarAttributeSelector {
    pub fn builder() -> ScalarAttributeSelectorBuilder {
        ScalarAttributeSelectorBuilder::new()
    }

    pub fn attribute(&self) -> ScalarAttribute {
        self.attribute.get()
    }

    pub fn connect_changed<F: Fn(ScalarAttribute) + 'static>(&self, callback: F) {
        self.callbacks.borrow_mut().push(Box::new(callback))
    }

    fn notify_changed(&self, attr: ScalarAttribute) {
        self.attribute.set(attr);
        for callback in self.callbacks.borrow().iter() {
            callback(attr);
        }
    }
}

pub struct ScalarAttributeSelectorBuilder {
    attributes: Vec<ScalarAttribute>,
    orientation: gtk::Orientation,
}

impl Default for ScalarAttributeSelectorBuilder {
    fn default() -> Self {
        Self {
            attributes: vec![],
            orientation: gtk::Orientation::Horizontal,
        }
    }
}

impl ScalarAttributeSelectorBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn attributes(&mut self, attributes: &[ScalarAttribute]) -> &mut Self {
        self.attributes = attributes.to_vec();
        self
    }

    pub fn orientation(&mut self, orientation: gtk::Orientation) -> &mut Self {
        self.orientation = orientation;
        self
    }

    pub fn build(&self) -> Rc<ScalarAttributeSelector> {
        let asrb = Rc::new(ScalarAttributeSelector {
            gtk_box: gtk::Box::new(self.orientation, 0),
            attribute: Cell::new(*self.attributes.first().expect("programmer error")),
            callbacks: RefCell::new(vec![]),
        });

        let mut first: Option<gtk::RadioButton> = None;
        for attr in self.attributes.iter() {
            let button = gtk::RadioButton::with_label(&attr.to_string());
            asrb.gtk_box.pack_start(&button, false, false, 0);
            if let Some(ref first) = first {
                button.join_group(Some(first))
            } else {
                first = Some(button.clone())
            }
            let asrb_c = Rc::clone(&asrb);
            let attr = *attr;
            button.connect_toggled(move |button| {
                let its_us = button.is_active();
                if its_us {
                    asrb_c.notify_changed(attr);
                }
            });
        }

        asrb
    }
}
