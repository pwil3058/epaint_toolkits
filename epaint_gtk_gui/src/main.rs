// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::rc::Rc;

use pw_gtk_ext::{
    gtk::{self, BoxExt, ContainerExt, WidgetExt},
    recollections,
    wrapper::*,
};

use colour_math::hue_wheel::{ColouredShape, MakeColouredShape, Shape};
use colour_math::ScalarAttribute::Warmth;
use colour_math::{HueConstants, LightLevel, HCV};
use colour_math_derive::Colour;

use epaint::create_paint;
use epaint::{GetSeriesId, LabelText, PaintEssence, SeriesId, TooltipText};
use epaint::paint::{PaintIfce, SerializablePaintData};
use epaint::properties::{
    Property, PropertyType,
    PropertyType::{Granulation, LightFastness, Luminescence, Staining, Transparency},
};

use epaint_gtk::spec_edit::BasicPaintSpecEditor;

create_paint!(&[
    Transparency,
    LightFastness,
    Staining,
    Granulation,
    Luminescence
]);

fn main() {
    recollections::init("./.recollections");
    if gtk::init().is_err() {
        println!("GTK init failed");
        return;
    };
    let win = gtk::Window::new(gtk::WindowType::Toplevel);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let bpe = BasicPaintSpecEditor::new(
        &[Warmth],
        &[
            Transparency,
            LightFastness,
            Staining,
            Granulation,
            Luminescence,
        ],
    );
    vbox.pack_start(bpe.pwo(), false, false, 0);
    let mut paint_spec = SerializablePaintData {
        name: "Paint Nama".to_string(),
        colour: HCV::MAGENTA,
        notes: "Notes".to_string(),
        property_variants_f64: vec![1.0_f64, 1.0, 1.0, 1.0, 1.0],
    };
    paint_spec.name = "name".to_string();
    paint_spec.notes = "notes".to_string();
    bpe.edit(&paint_spec);
    vbox.show_all();
    win.add(&vbox);
    win.connect_destroy(|_| gtk::main_quit());
    win.show();
    gtk::main()
}
