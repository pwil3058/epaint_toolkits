// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use gtk::subclass::prelude::*;
use gtk::{Box, Orientation, Widget, glib};

use colour_math::{HCV, LightLevel, RGB, Value};

use crate::colour::ManipGdkColour;

glib::wrapper! {
    pub struct ColourManipulator(ObjectSubclass<imp::ColourManipulator>)
    @extends Box, Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl ColourManipulator {
    pub fn new() -> Self {
        let obj: ColourManipulator = glib::Object::builder()
            .property("orientation", Orientation::Vertical)
            .property("receives_default", true)
            .build();
        obj
    }

    pub fn set_colour(&self, colour: &impl ManipGdkColour) {
        self.imp()
            .colour_manipulator
            .borrow_mut()
            .set_colour(colour);
        // TODO: add button colour changes
    }

    pub fn set_colour_and_inform(&self, colour: &impl ManipGdkColour) {
        self.set_colour(colour);
        for callback in self.imp().change_callbacks.borrow().iter() {
            callback(colour.hcv())
        }
    }

    pub fn draw(&self, cairo_context: &cairo::Context) {
        let rgb = self.imp().colour_manipulator.borrow().rgb();
        cairo_context.set_source_rgb(rgb[0], rgb[1], rgb[2]);
        cairo_context.paint().expect("manipultor failed to paint");
        for sample in self.imp().samples.borrow().iter() {
            let buffer = sample
                .pixbuf
                .save_to_bufferv("png", &[])
                .expect("pixbuf to png error");
            let mut reader = std::io::Cursor::new(buffer);
            let surface = cairo::ImageSurface::create_from_png(&mut reader).unwrap();
            cairo_context
                .set_source_surface(&surface, sample.position.x, sample.position.y)
                .expect("mainpualor failed to construct source surface");
            cairo_context.paint().expect("manipultor failed to paint");
        }
    }

    pub fn auto_match_samples(&self) {
        let mut red: u64 = 0;
        let mut green: u64 = 0;
        let mut blue: u64 = 0;
        let mut npixels: u64 = 0;
        for sample in self.imp().samples.borrow().iter() {
            assert_eq!(sample.pixbuf.bits_per_sample(), 8);
            let nc = sample.pixbuf.n_channels() as usize;
            let rs = sample.pixbuf.rowstride() as usize;
            let width = sample.pixbuf.width() as usize;
            let n_rows = sample.pixbuf.height() as usize;
            unsafe {
                let data = sample.pixbuf.pixels();
                for row_num in 0..n_rows {
                    let row_start = row_num * rs;
                    let row_end = row_start + width * nc;
                    for chunk in (data[row_start..row_end]).chunks(nc) {
                        red += chunk[0] as u64;
                        green += chunk[1] as u64;
                        blue += chunk[2] as u64;
                    }
                }
            }
            npixels += (width * n_rows) as u64;
        }
        if npixels > 0 {
            let divisor = npixels; //(npixels * 255) as u64;
            let array: [u8; 3] = [
                (red / divisor) as u8,
                (green / divisor) as u8,
                (blue / divisor) as u8,
            ];
            let rgb: RGB<u8> = array.into();
            self.set_colour_and_inform(&rgb);
        }
    }

    pub fn reset(&self) {
        self.delete_samples();
        self.set_colour_and_inform(&(HCV::new_grey(Value::ONE / 2)));
    }

    pub fn delete_samples(&self) {
        self.imp().samples.borrow_mut().clear();
    }

    pub fn rgb<L: LightLevel>(&self) -> RGB<L> {
        self.imp().colour_manipulator.borrow().rgb::<L>()
    }

    pub fn hcv(&self) -> HCV {
        self.imp().colour_manipulator.borrow().hcv()
    }

    pub fn connect_changed<F: Fn(HCV) + 'static>(&self, callback: F) {
        self.imp()
            .change_callbacks
            .borrow_mut()
            .push(std::boxed::Box::new(callback))
    }
}

mod imp {
    use std::cell::RefCell;

    use gdk::gdk_pixbuf;
    use glib::Properties;
    use gtk::subclass::prelude::*;
    use gtk::{Box, glib};

    use colour_math::hcv::HCV;
    use colour_math::manipulator;

    use crate::cm_cairo::Point;

    type ChangeCallback = std::boxed::Box<dyn Fn(HCV)>;

    pub struct Sample {
        pub pixbuf: gdk_pixbuf::Pixbuf,
        pub position: Point,
    }

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::ColourManipulator)]
    pub struct ColourManipulator {
        pub colour_manipulator: RefCell<manipulator::ColourManipulator>,
        pub change_callbacks: RefCell<Vec<ChangeCallback>>,
        pub samples: RefCell<Vec<Sample>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ColourManipulator {
        const NAME: &'static str = "colourManipulator";
        type Type = super::ColourManipulator;
        type ParentType = Box;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ColourManipulator {}

    impl WidgetImpl for ColourManipulator {}

    impl BoxImpl for ColourManipulator {}
}
