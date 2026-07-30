// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{
    Box, Button, DrawingArea, EventControllerKey, Orientation, Widget, gdk, gdk::gdk_pixbuf, glib,
};
use std::rc::Rc;

use colour_math::{Angle, HCV, LightLevel, Prop, RGB, Value};

use crate::cm_cairo::Point;
use crate::colour::ManipGdkColour;

#[derive(Clone, Copy, Default)]
pub enum ChromaLabel {
    #[default]
    Chroma,
    Greyness,
    Both,
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum DeltaSize {
    Small,
    #[default]
    Normal,
    Large,
}

impl DeltaSize {
    fn for_value(self) -> Prop {
        match self {
            DeltaSize::Small => 0.0025.into(),
            DeltaSize::Normal => 0.005.into(),
            DeltaSize::Large => 0.01.into(),
        }
    }

    fn for_chroma(self) -> Prop {
        match self {
            DeltaSize::Small => 0.0025.into(),
            DeltaSize::Normal => 0.005.into(),
            DeltaSize::Large => 0.01.into(),
        }
    }

    fn for_hue_anticlockwise(self) -> Angle {
        match self {
            DeltaSize::Small => 0.5.into(),
            DeltaSize::Normal => 1.0.into(),
            DeltaSize::Large => 5.0.into(),
        }
    }

    fn for_hue_clockwise(self) -> Angle {
        -self.for_hue_anticlockwise()
    }
}

pub struct Sample {
    pub pixbuf: gdk_pixbuf::Pixbuf,
    pub position: Point,
}

glib::wrapper! {
    pub struct ColourManipulator(ObjectSubclass<imp::ColourManipulator>)
    @extends Box, Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl ColourManipulator {
    pub fn new(clamped: bool, chroma_label: ChromaLabel, extra_btns: &[gtk::Button]) -> Self {
        let obj: ColourManipulator = glib::Object::builder()
            .property("orientation", Orientation::Vertical)
            .property("receives_default", true)
            .build();

        let key_controller = EventControllerKey::new();
        let delta_size = Rc::clone(&obj.imp().delta_size);
        key_controller.connect_key_pressed(move |_, key, _, _| {
            match key {
                gdk::Key::Shift_L => delta_size.set(DeltaSize::Large),
                gdk::Key::Shift_R => delta_size.set(DeltaSize::Small),
                _ => {}
            };
            glib::Propagation::Proceed
        });
        let delta_size = Rc::clone(&obj.imp().delta_size);
        key_controller.connect_key_released(move |_, key, _, _| {
            match key {
                gdk::Key::Shift_L | gdk::Key::Shift_R => delta_size.set(DeltaSize::Normal),
                _ => {}
            };
        });
        obj.add_controller(key_controller);

        obj.imp()
            .colour_manipulator
            .borrow_mut()
            .set_clamped(clamped);

        let drawing_area = DrawingArea::builder()
            .height_request(150)
            .width_request(150)
            .build();
        let gesture = gtk::GestureClick::new();
        gesture.connect_pressed(|gesture, n_press, x, y| {
            let button = gesture.current_button();
            println!("Pressed button 3 at ({x}, {y}) count {n_press}");
            if button == 3 {
                println!("This will pop up a paste/delete sample menu");
            }
        });
        drawing_area.add_controller(gesture);
        let colour_manipulator = Rc::clone(&obj.imp().colour_manipulator);
        let samples = Rc::clone(&obj.imp().samples);
        drawing_area.set_draw_func(move |_, cairo_context, _, _| {
            let rgb = colour_manipulator.borrow().rgb();
            cairo_context.set_source_rgb(rgb[0], rgb[1], rgb[2]);
            cairo_context.paint().expect("manipultor failed to paint");
            for sample in samples.borrow().iter() {
                let buffer = sample
                    .pixbuf
                    .save_to_bufferv("png", &[])
                    .expect("pixbuf to png error");
                let mut reader = std::io::Cursor::new(buffer);
                let surface = gtk::cairo::ImageSurface::create_from_png(&mut reader).unwrap();
                cairo_context
                    .set_source_surface(&surface, sample.position.x, sample.position.y)
                    .expect("mainpualor failed to construct source surface");
                cairo_context.paint().expect("manipultor failed to paint");
            }
        });

        macro_rules! connect_clicked {
            ($obj:ident, $button:ident, $for:ident, $action:ident) => {
                let obj_c = $obj.clone();
                $button.connect_clicked(move |button| {
                    let delta = obj_c.imp().delta_size.get().$for();
                    let changed = obj_c.imp().colour_manipulator.borrow_mut().$action(delta);
                    if changed {
                        let new_hcv = obj_c.imp().colour_manipulator.borrow().hcv();
                        obj_c.set_colour_and_inform(&new_hcv);
                    } else {
                        button.error_bell();
                    }
                });
            };
        }

        let incr_value_btn = gtk::Button::with_label("Value++");
        connect_clicked!(obj, incr_value_btn, for_value, incr_value);

        let decr_value_btn = gtk::Button::with_label("Value--");
        connect_clicked!(obj, decr_value_btn, for_value, decr_value);

        let hue_left_btn = gtk::Button::with_label("<");
        connect_clicked!(obj, hue_left_btn, for_hue_anticlockwise, rotate);

        let hue_right_btn = gtk::Button::with_label(">");
        connect_clicked!(obj, hue_right_btn, for_hue_clockwise, rotate);

        let incr_chroma_bth = match chroma_label {
            ChromaLabel::Chroma => Button::with_label("Chroma++"),
            ChromaLabel::Greyness => Button::with_label("Greyness--"),
            ChromaLabel::Both => Button::with_label("Chroma++/Greyness--"),
        };
        connect_clicked!(obj, incr_chroma_bth, for_chroma, incr_chroma);

        let decr_chroma_btn = match chroma_label {
            ChromaLabel::Chroma => Button::with_label("Chroma--"),
            ChromaLabel::Greyness => Button::with_label("Greyness++"),
            ChromaLabel::Both => Button::with_label("Chroma--/Greyness++"),
        };
        connect_clicked!(obj, decr_chroma_btn, for_chroma, decr_chroma);

        let auto_match_btn = gtk::Button::with_label("Auto Match");
        let obj_c = obj.clone();
        auto_match_btn.connect_clicked(move |_| obj_c.auto_match_samples());

        let auto_match_on_paste_btn = gtk::CheckButton::with_label("On Paste?");

        obj.append(&incr_value_btn);
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        hbox.append(&hue_left_btn);
        hbox.append(&drawing_area);
        hbox.append(&hue_right_btn);
        obj.append(&hbox);
        obj.append(&decr_chroma_btn);
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        for button in extra_btns {
            hbox.append(button);
        }
        hbox.append(&auto_match_btn);
        hbox.append(&auto_match_on_paste_btn);
        obj.append(&hbox);

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
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    use glib::Properties;
    use gtk::subclass::prelude::*;
    use gtk::{Box, glib};

    use colour_math::hcv::HCV;
    use colour_math::manipulator;

    use crate::cm_cairo::Point;
    use crate::manipulator::DeltaSize;

    type ChangeCallback = std::boxed::Box<dyn Fn(HCV)>;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::ColourManipulator)]
    pub struct ColourManipulator {
        pub colour_manipulator: Rc<RefCell<manipulator::ColourManipulator>>,
        pub change_callbacks: RefCell<Vec<ChangeCallback>>,
        pub samples: Rc<RefCell<Vec<super::Sample>>>,
        pub delta_size: Rc<Cell<DeltaSize>>,
        pub popup_menu_posn: Rc<Cell<Point>>,
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
