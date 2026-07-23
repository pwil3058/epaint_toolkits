// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::rc::Rc;

use pw_gtk_ext::{
    gtk::{self, BoxExt, ContainerExt, WidgetExt},
    recollections,
    wrapper::*,
};

use colour_math::ScalarAttribute;
use colour_math::ScalarAttribute::*;
use colour_math::{HCV, HueConstants};

use epaint::PaintRangeId;
use epaint::paint::{Paint, RangePaint};
use epaint::properties::{
    Properties,
    PropertyType::{Granulation, Lightfastness, Luminescence, Staining, Transparency},
    PropertyTypes,
};

use epaint_gtk::factory::PaintFactoryBuilder;
use epaint_gtk::mixer::palette::PalettePaintMixerBuilder;
use epaint_gtk::paint_edit::PaintEditor;
use epaint_gtk::range::PaintRangeManagerBuilder;
use epaint_gtk::range::display::*;

fn main() {
    recollections::init("./.recollections");
    if gtk::init().is_err() {
        println!("GTK init failed");
        return;
    };
    let property_types = PropertyTypes(vec![
        Transparency,
        Lightfastness,
        Staining,
        Granulation,
        Luminescence,
    ]);
    let win = gtk::Window::new(gtk::WindowType::Toplevel);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.pack_start(
        PaintFactoryBuilder::new()
            .attributes(&[
                ScalarAttribute::Value,
                ScalarAttribute::Greyness,
                //ScalarAttribute::Chroma,
            ])
            .property_types(&property_types)
            .build()
            .pwo(),
        false,
        false,
        0,
    );
    let mixer = PalettePaintMixerBuilder::new()
        .attributes(&[
            ScalarAttribute::Value,
            ScalarAttribute::Greyness,
            ScalarAttribute::Chroma,
        ])
        .property_types(&property_types)
        .build();
    vbox.pack_start(mixer.pwo(), false, false, 0);
    let bpe = PaintEditor::new(&[Warmth], &property_types);
    vbox.pack_start(bpe.pwo(), false, false, 0);
    let mut paint = Paint {
        #[cfg(feature = "paints_have_ids")]
        id: "Identey Number".to_string(),
        name: "Paint Name".to_string(),
        colour: HCV::MAGENTA,
        notes: "Notes".to_string(),
        properties: Properties::from(&property_types),
    };
    paint.name = "name".to_string();
    paint.notes = "notes".to_string();
    bpe.edit(&paint);
    let colln_paint = RangePaint::from((
        paint,
        PaintRangeId {
            name: "Series".to_string(),
            proprietor: "Owner".to_string(),
        },
    ));
    let mut builder = PaintDisplayBuilder::new();
    builder
        .attributes(&[
            ScalarAttribute::Value,
            ScalarAttribute::Greyness,
            ScalarAttribute::Chroma,
        ])
        .property_types(&property_types);
    let display = builder.build(&colln_paint);
    vbox.pack_start(display.pwo(), true, true, 0);
    let mut paint_series_manager_builder = PaintRangeManagerBuilder::new();
    paint_series_manager_builder.property_types(&property_types);
    paint_series_manager_builder.attributes(&[
        ScalarAttribute::Value,
        ScalarAttribute::Greyness,
        ScalarAttribute::Chroma,
    ]);
    let paint_series_manager = paint_series_manager_builder.build();
    vbox.pack_start(paint_series_manager.pwo(), true, true, 0);
    vbox.show_all();
    win.add(&vbox);
    win.connect_destroy(|_| gtk::main_quit());
    win.show();
    gtk::main()
}
