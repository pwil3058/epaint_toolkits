// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use pw_gtk_ext::{
    gtk::{self, BoxExt, ContainerExt, WidgetExt},
    recollections,
    wrapper::*,
};

use colour_math::ScalarAttribute::Warmth;
use colour_math::{HCV, HueConstants};

use epaint::paint::SerializablePaintData;
use epaint::properties::{
    Properties,
    PropertyType::{Granulation, Lightfastness, Luminescence, Staining, Transparency},
};

use epaint_gtk::spec_edit::BasicPaintSpecEditor;

fn main() {
    recollections::init("./.recollections");
    if gtk::init().is_err() {
        println!("GTK init failed");
        return;
    };
    let win = gtk::Window::new(gtk::WindowType::Toplevel);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let property_types = vec![
        Transparency,
        Lightfastness,
        Staining,
        Granulation,
        Luminescence,
    ];
    let bpe = BasicPaintSpecEditor::new(&[Warmth], &property_types);
    vbox.pack_start(bpe.pwo(), false, false, 0);
    let mut paint_spec = SerializablePaintData {
        name: "Paint Nama".to_string(),
        colour: HCV::MAGENTA,
        notes: "Notes".to_string(),
        properties: Properties::new_fm_types(&property_types),
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
