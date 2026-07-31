// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use gtk::prelude::*;
use gtk::{Button, DrawingArea, EventControllerKey, gdk, gdk::gdk_pixbuf, glib};

use gtk4_ext::PackableWidgetObject;
use gtk4_ext_derive::PWO;

use colour_math::{Angle, HCV, LightLevel, Prop, RGB, Value, manipulator};

use crate::cm_cairo::Point;
use crate::{colour::ManipGdkColour, coloured::Colourable};

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

type ChangeCallback = std::boxed::Box<dyn Fn(HCV)>;

#[derive(PWO)]
pub struct ColourManipulator {
    pub vbox: gtk::Box,
    pub colour_manipulator: Rc<RefCell<manipulator::ColourManipulator>>,
    pub change_callbacks: RefCell<Vec<ChangeCallback>>,
    pub samples: Rc<RefCell<Vec<Sample>>>,
    pub delta_size: Rc<Cell<DeltaSize>>,
    pub popup_menu_posn: Rc<Cell<Point>>,
    drawing_area: gtk::DrawingArea,

    incr_value_btn: gtk::Button,
    decr_value_btn: gtk::Button,
    hue_left_btn: gtk::Button,
    hue_right_btn: gtk::Button,
    incr_chroma_btn: gtk::Button,
    decr_chroma_btn: gtk::Button,
}

impl ColourManipulator {
    pub fn new(clamped: bool, chroma_label: ChromaLabel, extra_btns: &[gtk::Button]) -> Rc<Self> {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 1);
        let key_controller = EventControllerKey::new();
        let delta_size = Rc::new(Cell::new(DeltaSize::default()));
        let delta_size_c = Rc::clone(&delta_size);
        key_controller.connect_key_pressed(move |_, key, _, _| {
            match key {
                gdk::Key::Shift_L => delta_size_c.set(DeltaSize::Large),
                gdk::Key::Shift_R => delta_size_c.set(DeltaSize::Small),
                _ => {}
            };
            glib::Propagation::Proceed
        });
        let delta_size_c = Rc::clone(&delta_size);
        key_controller.connect_key_released(move |_, key, _, _| {
            match key {
                gdk::Key::Shift_L | gdk::Key::Shift_R => delta_size_c.set(DeltaSize::Normal),
                _ => {}
            };
        });
        vbox.add_controller(key_controller);

        let colour_manipulator = Rc::new(RefCell::new(
            manipulator::ColourManipulatorBuilder::new()
                .clamped(clamped)
                .build(),
        ));

        let drawing_area = DrawingArea::builder()
            .height_request(150)
            .width_request(150)
            .vexpand(true)
            .hexpand(true)
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
        let colour_manipulator_c = Rc::clone(&colour_manipulator);
        let samples = Rc::new(RefCell::new(Vec::<Sample>::new()));
        let samples_c = Rc::clone(&samples);
        drawing_area.set_draw_func(move |_, cairo_context, _, _| {
            let rgb = colour_manipulator_c.borrow().rgb();
            cairo_context.set_source_rgb(rgb[0], rgb[1], rgb[2]);
            cairo_context.paint().expect("manipultor failed to paint");
            for sample in samples_c.borrow().iter() {
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

        let incr_value_btn = gtk::Button::with_label("Value++");
        let decr_value_btn = gtk::Button::with_label("Value--");
        let hue_left_btn = gtk::Button::with_label("<");
        let hue_right_btn = gtk::Button::with_label(">");
        let incr_chroma_btn = match chroma_label {
            ChromaLabel::Chroma => Button::with_label("Chroma++"),
            ChromaLabel::Greyness => Button::with_label("Greyness--"),
            ChromaLabel::Both => Button::with_label("Chroma++/Greyness--"),
        };
        let decr_chroma_btn = match chroma_label {
            ChromaLabel::Chroma => Button::with_label("Chroma--"),
            ChromaLabel::Greyness => Button::with_label("Greyness++"),
            ChromaLabel::Both => Button::with_label("Chroma--/Greyness++"),
        };

        let cm = Rc::new(Self {
            vbox,
            colour_manipulator: Rc::clone(&colour_manipulator),
            change_callbacks: RefCell::new(Vec::new()),
            samples: Rc::clone(&samples),
            delta_size: Rc::clone(&delta_size),
            popup_menu_posn: Rc::new(Cell::new(crate::cm_cairo::Point::default())),
            drawing_area,
            incr_value_btn,
            decr_value_btn,
            incr_chroma_btn,
            decr_chroma_btn,
            hue_left_btn,
            hue_right_btn,
        });

        macro_rules! connect_clicked {
            ($cm:ident, $button:ident, $for:ident, $action:ident) => {
                let cm_c = Rc::clone(&$cm);
                $cm.$button.connect_clicked(move |button| {
                    let delta = cm_c.delta_size.get().$for();
                    let changed = cm_c.colour_manipulator.borrow_mut().$action(delta);
                    if changed {
                        let new_hcv = cm_c.colour_manipulator.borrow().hcv();
                        cm_c.set_colour_and_inform(&new_hcv);
                    } else {
                        button.error_bell();
                    }
                });
            };
        }

        connect_clicked!(cm, incr_value_btn, for_value, incr_value);
        connect_clicked!(cm, decr_value_btn, for_value, decr_value);
        connect_clicked!(cm, hue_left_btn, for_hue_anticlockwise, rotate);
        connect_clicked!(cm, hue_right_btn, for_hue_clockwise, rotate);
        connect_clicked!(cm, incr_chroma_btn, for_chroma, incr_chroma);
        connect_clicked!(cm, decr_chroma_btn, for_chroma, decr_chroma);

        let auto_match_btn = gtk::Button::with_label("Auto Match");
        let cm_c = cm.clone();
        auto_match_btn.connect_clicked(move |_| cm_c.auto_match_samples());

        let auto_match_on_paste_btn = gtk::CheckButton::with_label("On Paste?");

        cm.vbox.append(&cm.incr_value_btn);
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 1);
        hbox.append(&cm.hue_left_btn);
        hbox.append(&cm.drawing_area);
        hbox.append(&cm.hue_right_btn);
        cm.vbox.append(&hbox);
        cm.vbox.append(&cm.decr_value_btn);
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 1);
        hbox.append(&cm.decr_chroma_btn);
        hbox.append(&cm.incr_chroma_btn);
        cm.vbox.append(&hbox);
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 1);
        for button in extra_btns {
            hbox.append(button);
        }
        hbox.append(&auto_match_btn);
        hbox.append(&auto_match_on_paste_btn);
        cm.vbox.append(&hbox);

        cm
    }

    pub fn set_colour(&self, colour: &impl ManipGdkColour) {
        self.colour_manipulator.borrow_mut().set_colour(colour);
        let offset: Prop = (Prop::ONE / 10 * 2).into();
        self.incr_value_btn
            .set_widget_colour(&colour.lightened(offset));
        self.decr_value_btn
            .set_widget_colour(&colour.darkened(offset));
        self.decr_chroma_btn
            .set_widget_colour(&colour.greyed(offset));
        self.incr_chroma_btn
            .set_widget_colour(&colour.saturated(offset));
        let angle_offset = Angle::from(45);
        self.hue_left_btn
            .set_widget_colour(&colour.rotated(angle_offset));
        self.hue_right_btn
            .set_widget_colour(&colour.rotated(-angle_offset));
        self.drawing_area.queue_draw();
    }

    pub fn set_colour_and_inform(&self, colour: &impl ManipGdkColour) {
        self.set_colour(colour);
        for callback in self.change_callbacks.borrow().iter() {
            callback(colour.hcv())
        }
    }

    pub fn auto_match_samples(&self) {
        let mut red: u64 = 0;
        let mut green: u64 = 0;
        let mut blue: u64 = 0;
        let mut npixels: u64 = 0;
        for sample in self.samples.borrow().iter() {
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
        self.samples.borrow_mut().clear();
    }

    pub fn rgb<L: LightLevel>(&self) -> RGB<L> {
        self.colour_manipulator.borrow().rgb::<L>()
    }

    pub fn hcv(&self) -> HCV {
        self.colour_manipulator.borrow().hcv()
    }

    pub fn connect_changed<F: Fn(HCV) + 'static>(&self, callback: F) {
        self.change_callbacks
            .borrow_mut()
            .push(std::boxed::Box::new(callback))
    }
}
