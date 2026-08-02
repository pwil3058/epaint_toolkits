// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use pw_gtk_ext::{
    gtk::{self, BoxExt, RadioButtonExt, ToggleButtonExt, WidgetExt},
    wrapper::*,
};

use colour_math::{attr_display, ScalarAttribute, RGB};
use colour_math_cairo::{Drawer, Size};

use crate::colour::GdkColour;

pub type ChromaCAD = ColourAttributeDisplay<attr_display::ChromaCAD>;
pub type GreynessCAD = ColourAttributeDisplay<attr_display::GreynessCAD>;
pub type HueCAD = ColourAttributeDisplay<attr_display::HueCAD>;
pub type ValueCAD = ColourAttributeDisplay<attr_display::ValueCAD>;
pub type WarmthCAD = ColourAttributeDisplay<attr_display::WarmthCAD>;

pub trait DynColourAttributeDisplay: PackableWidgetObject<PWT = gtk::DrawingArea> {
    fn set_rgb(&self, rgb: Option<&RGB<f64>>);
    fn set_target_rgb(&self, rgb: Option<&RGB<f64>>);
}

#[derive(PWO, Wrapper)]
pub struct ColourAttributeDisplay<A: attr_display::ColourAttributeDisplayIfce> {
    drawing_area: gtk::DrawingArea,
    attribute: RefCell<A>,
}

impl<A> ColourAttributeDisplay<A>
where
    A: attr_display::ColourAttributeDisplayIfce + 'static,
{
    pub fn new() -> Rc<Self> {
        let cad = Rc::new(Self {
            drawing_area: gtk::DrawingArea::new(),
            attribute: RefCell::new(A::new()),
        });
        cad.drawing_area.set_size_request(90, 30);
        let cad_c = Rc::clone(&cad);
        cad.drawing_area.connect_draw(move |da, cairo_context| {
            let size = Size {
                width: da.get_allocated_width() as f64,
                height: da.get_allocated_height() as f64,
            };
            let drawer = Drawer::new(cairo_context, size);
            cad_c.attribute.borrow().draw_all(&drawer);
            gtk::Inhibit(false)
        });
        cad
    }
}

impl<A> DynColourAttributeDisplay for ColourAttributeDisplay<A>
where
    A: attr_display::ColourAttributeDisplayIfce + 'static,
{
    fn set_rgb(&self, rgb: Option<&RGB<f64>>) {
        self.attribute.borrow_mut().set_colour(rgb);
        self.drawing_area.queue_draw();
    }

    fn set_target_rgb(&self, rgb: Option<&RGB<f64>>) {
        self.attribute.borrow_mut().set_target_colour(rgb);
        self.drawing_area.queue_draw();
    }
}

#[derive(PWO, Wrapper)]
pub struct ColourAttributeDisplayStack {
    vbox: gtk::Box,
    cads: Vec<Rc<dyn DynColourAttributeDisplay<PWT = gtk::DrawingArea>>>,
}

impl ColourAttributeDisplayStack {
    pub fn set_colour(&self, colour: Option<&impl GdkColour>) {
        for cad in self.cads.iter() {
            if let Some(colour) = colour {
                cad.set_rgb(Some(&colour.rgb()));
            } else {
                cad.set_rgb(None);
            }
        }
    }

    pub fn set_target_colour(&self, colour: Option<&impl GdkColour>) {
        for cad in self.cads.iter() {
            if let Some(colour) = colour {
                cad.set_target_rgb(Some(&colour.rgb()));
            } else {
                cad.set_target_rgb(None);
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

    pub fn attributes(&mut self, attributes: &[ScalarAttribute]) -> &mut Self {
        self.attributes = attributes.to_vec();
        self
    }

    pub fn build(&self) -> Rc<ColourAttributeDisplayStack> {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 1);
        let mut cads = vec![];
        let hue_cad: Rc<dyn DynColourAttributeDisplay<PWT = gtk::DrawingArea>> = HueCAD::new();
        vbox.pack_start(hue_cad.pwo(), true, true, 0);
        cads.push(hue_cad);
        for scalar_attribute in self.attributes.iter() {
            let cad: Rc<dyn DynColourAttributeDisplay<PWT = gtk::DrawingArea>> =
                match scalar_attribute {
                    ScalarAttribute::Value => ValueCAD::new(),
                    ScalarAttribute::Chroma => ChromaCAD::new(),
                    ScalarAttribute::Warmth => WarmthCAD::new(),
                    ScalarAttribute::Greyness => GreynessCAD::new(),
                };
            vbox.pack_start(cad.pwo(), true, true, 0);
            cads.push(cad);
        }
        Rc::new(ColourAttributeDisplayStack { vbox, cads })
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

pub struct AttributeSelectorBuilder {
    attributes: Vec<ScalarAttribute>,
    orientation: gtk::Orientation,
}

impl Default for AttributeSelectorBuilder {
    fn default() -> Self {
        Self {
            attributes: vec![],
            orientation: gtk::Orientation::Horizontal,
        }
    }
}

impl AttributeSelectorBuilder {
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
                let its_us = button.get_active();
                if its_us {
                    asrb_c.notify_changed(attr);
                }
            });
        }

        asrb
    }
}
