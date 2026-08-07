// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::collections::HashMap;
#[cfg(feature = "targeted_mixtures")]
use std::rc::Rc;

use gtk_ext::{
    glib,
    gtk::{self, prelude::*},
    gtkx::{
        dialog_user::TopGtkWindow,
        list::{ListViewSpec, ListViewWithPopUpMenuBuilder},
        placard::*,
    },
    wrapper::*,
};

use colour_math::{ColourBasics, HCV, ScalarAttribute};
use colour_math_gtk::attributes::ColourAttributeDisplayStackBuilder;
use colour_math_gtk::coloured::Colourable;

#[cfg(feature = "targeted_mixtures")]
use colour_math_gtk::{attributes::ColourAttributeDisplayStack, colour::*};

use crate::list::PaintListRow;
use epaint::{mixtures::Mixture, properties::PropertyTypes};

#[derive(PWO)]
pub struct MixtureDisplay {
    vbox: gtk::Box,
    mixture: Mixture,
    #[cfg(feature = "targeted_mixtures")]
    target_placard: Placard,
    #[cfg(feature = "targeted_mixtures")]
    cads: Rc<ColourAttributeDisplayStack>,
}

impl MixtureDisplay {
    #[cfg(feature = "targeted_mixtures")]
    pub fn set_target(&self, new_target: Option<&impl GdkColour>) {
        if let Some(colour) = new_target {
            self.target_placard.set_label("Current Target");
            self.target_placard.set_widget_colour(colour);
            self.cads.set_target_colour(Some(colour));
        } else {
            self.target_placard.set_label("");
            self.target_placard.set_widget_colour(&self.mixture.hcv());
            self.cads.set_target_colour(Option::<&HCV>::None);
        };
    }

    pub fn mixture(&self) -> &Mixture {
        &self.mixture
    }
}

#[derive(Default)]
pub struct MixtureDisplayBuilder {
    attributes: Vec<ScalarAttribute>,
    property_types: PropertyTypes,
    #[cfg(feature = "targeted_mixtures")]
    target_colour: Option<HCV>,
    list_spec: ComponentsListViewSpec,
}

impl MixtureDisplayBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn attributes(&mut self, attributes: &[ScalarAttribute]) -> &mut Self {
        self.attributes = attributes.to_vec();
        self.list_spec = ComponentsListViewSpec::new(&self.attributes, &self.property_types);
        self
    }

    pub fn property_types(&mut self, properties: &PropertyTypes) -> &mut Self {
        self.property_types = properties.clone();
        self.list_spec = ComponentsListViewSpec::new(&self.attributes, &self.property_types);
        self
    }

    #[cfg(feature = "targeted_mixtures")]
    pub fn target_colour(&mut self, target_colour: Option<&impl ColourBasics>) -> &mut Self {
        self.target_colour = if let Some(target_colour) = target_colour {
            Some(target_colour.hcv())
        } else {
            None
        };
        self
    }

    pub fn build(&self, mixture: &Mixture) -> MixtureDisplay {
        let colour = mixture.hcv();
        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        let placard = Placard::builder().label(mixture.id()).build();
        placard.set_widget_colour(&colour);
        vbox.pack_start(&placard, false, false, 0);

        let placard = Placard::builder().label(mixture.name()).build();
        placard.set_widget_colour(&colour);
        vbox.pack_start(&placard, false, false, 0);

        let placard = Placard::builder().label(mixture.notes()).build();
        placard.set_widget_colour(&colour);
        vbox.pack_start(&placard, false, false, 0);

        let cads = ColourAttributeDisplayStackBuilder::new()
            .scalar_attributes(&self.attributes)
            .build();
        cads.set_colour(Some(&colour));

        #[cfg(feature = "targeted_mixtures")]
        let target_label = if let Some(target_colour) = self.target_colour {
            let placard = Placard::builder().label("Target").build();
            placard.set_widget_colour(&target_colour);
            cads.set_target_colour(Some(&target_colour));
            placard
        } else {
            let placard = Placard::builder().build();
            placard.set_widget_colour(&colour);
            placard
        };
        #[cfg(feature = "targeted_mixtures")]
        vbox.pack_start(&target_label, true, true, 0);

        #[cfg(feature = "targeted_mixtures")]
        if let Some(targeted_colour) = mixture.targeted_colour() {
            let label = gtk::Label::builder().label("Matched Colour").build();
            label.set_widget_colour(&targeted_colour);
            vbox.pack_start(&label, true, true, 0);
        }

        vbox.pack_start(cads.pwo(), true, true, 0);

        for property in mixture.iter_properties() {
            let value = property.value();
            let label = gtk::Label::builder().label(value).build();
            label.set_widget_colour(&colour);
            vbox.pack_start(&label, false, false, 0);
        }

        let list_view = ListViewWithPopUpMenuBuilder::new().build(&self.list_spec);
        vbox.pack_start(list_view.pwo(), false, false, 0);
        for (paint, parts) in mixture.components() {
            let mut row = paint.row(&self.attributes);
            let value: glib::Value = (*parts).to_value();
            row.insert(2, value);
            list_view.add_row(&row);
        }

        vbox.show_all();

        MixtureDisplay {
            vbox,
            mixture: mixture.clone(),
            #[cfg(feature = "targeted_mixtures")]
            target_placard: target_label,
            #[cfg(feature = "targeted_mixtures")]
            cads,
        }
    }
}

struct MixtureDisplayDialog {
    pub dialog: gtk::Dialog,
    #[cfg(feature = "targeted_mixtures")]
    pub display: MixtureDisplay,
}

pub struct MixtureDisplayDialogManager<W: TopGtkWindow> {
    caller: W,
    buttons: Vec<(&'static str, Option<&'static str>, u16)>,
    mixture_display_builder: MixtureDisplayBuilder,
    dialogs: HashMap<String, MixtureDisplayDialog>,
}

impl<W: TopGtkWindow> MixtureDisplayDialogManager<W> {
    fn new_dialog(&self) -> gtk::Dialog {
        let dialog = gtk::Dialog::builder().build();
        if let Some(parent) = self.caller.get_toplevel_gtk_window() {
            dialog.set_transient_for(Some(&parent));
        }
        for (label, tooltip_text, response) in self.buttons.iter() {
            dialog
                .add_button(label, gtk::ResponseType::Other(*response))
                .set_tooltip_text(*tooltip_text);
        }
        // TODO: think about removal from map as an optional action to hiding
        dialog.connect_delete_event(|d, _| {
            d.hide_on_delete();
            glib::Propagation::Stop
            // Inhibit(true)
        });
        dialog
    }

    pub fn display_mixture(&mut self, mixture: &Mixture) {
        if !self.dialogs.contains_key(&mixture.id) {
            let dialog = self.new_dialog();
            let display = self.mixture_display_builder.build(mixture);
            dialog
                .content_area()
                .pack_start(display.pwo(), true, true, 0);
            let pdd = MixtureDisplayDialog {
                dialog,
                #[cfg(feature = "targeted_mixtures")]
                display,
            };
            self.dialogs.insert(mixture.id.to_string(), pdd);
        };
        let pdd = self.dialogs.get(&mixture.id).expect("we just put it there");
        pdd.dialog.present();
    }

    #[cfg(feature = "targeted_mixtures")]
    pub fn set_target_colour(&mut self, rgb: Option<&impl GdkColour>) {
        self.mixture_display_builder.target_colour(rgb);
        for pdd in self.dialogs.values() {
            pdd.display.set_target(rgb);
        }
    }
}

pub struct MixtureDisplayDialogManagerBuilder<W: TopGtkWindow> {
    caller: W,
    buttons: Vec<(&'static str, Option<&'static str>, u16)>,
    attributes: Vec<ScalarAttribute>,
    property_types: PropertyTypes,
    target_colour: Option<HCV>,
}

impl<W: TopGtkWindow + Clone> MixtureDisplayDialogManagerBuilder<W> {
    pub fn new(caller: &W) -> Self {
        Self {
            caller: caller.clone(),
            buttons: vec![],
            attributes: vec![],
            property_types: PropertyTypes(vec![]),
            target_colour: None,
        }
    }

    pub fn attributes(&mut self, attributes: &[ScalarAttribute]) -> &mut Self {
        self.attributes = attributes.to_vec();
        self
    }

    pub fn properties(&mut self, property_types: &PropertyTypes) -> &mut Self {
        self.property_types = property_types.clone();
        self
    }

    pub fn buttons(&mut self, buttons: &[(&'static str, Option<&'static str>, u16)]) -> &mut Self {
        self.buttons = buttons.to_vec();
        self
    }

    pub fn target_colour(&mut self, target_colour: &impl ColourBasics) {
        self.target_colour = Some(target_colour.hcv());
    }

    pub fn build(&self) -> MixtureDisplayDialogManager<W> {
        let mut mixture_display_builder = MixtureDisplayBuilder::new();
        mixture_display_builder
            .attributes(&self.attributes)
            .property_types(&self.property_types);
        #[cfg(feature = "targeted_mixtures")]
        if let Some(target_colour) = self.target_colour {
            mixture_display_builder.target_colour(Some(&target_colour));
        }
        MixtureDisplayDialogManager {
            caller: self.caller.clone(),
            buttons: self.buttons.clone(),
            mixture_display_builder,
            dialogs: HashMap::new(),
        }
    }
}

#[derive(Default)]
pub struct ComponentsListViewSpec {
    attributes: Vec<ScalarAttribute>,
    property_types: PropertyTypes,
}

impl ComponentsListViewSpec {
    pub fn new(attributes: &[ScalarAttribute], property_types: &PropertyTypes) -> Self {
        Self {
            attributes: attributes.to_vec(),
            property_types: property_types.clone(),
        }
    }
}

impl ListViewSpec for ComponentsListViewSpec {
    fn column_types(&self) -> Vec<glib::Type> {
        #[cfg(feature = "paints_have_ids")]
        let mut column_types = vec![
            glib::Type::STRING,
            glib::Type::STRING,
            u64::static_type(),
            glib::Type::STRING,
            glib::Type::STRING,
            glib::Type::STRING,
            glib::Type::STRING,
            f64::static_type(),
        ];
        #[cfg(not(feature = "paints_have_ids"))]
        let mut column_types = vec![
            glib::Type::STRING,
            glib::Type::STRING,
            u64::static_type(),
            glib::Type::STRING,
            glib::Type::STRING,
            glib::Type::STRING,
            f64::static_type(),
        ];

        for _ in 0..self.attributes.len() * 3 + self.property_types.len() {
            column_types.push(glib::Type::STRING);
        }

        column_types
    }

    fn columns(&self) -> Vec<gtk::TreeViewColumn> {
        let mut cols = vec![];

        let mut index = 2;
        let col = gtk::TreeViewColumn::builder()
            .title("Parts")
            .resizable(false)
            .sort_column_id(index)
            .sort_indicator(true)
            .build();
        let cell = gtk::CellRendererText::builder().editable(false).build();
        TreeViewColumnExt::pack_start(&col, &cell, false);
        // col.pack_start(&cell, false);
        TreeViewColumnExt::add_attribute(&col, &cell, "text", index);
        // col.add_attribute(&cell, "text", index);
        cols.push(col);
        index += 1;

        #[cfg(feature = "paints_have_ids")]
        let headers = ["Id", "Name", "Notes"];
        #[cfg(not(feature = "paints_have_ids"))]
        let headers = ["Name", "Notes"];

        for header in headers {
            let col = gtk::TreeViewColumn::builder()
                .title(header)
                .resizable(true)
                .sort_column_id(index)
                .sort_indicator(true)
                .build();
            let cell = gtk::CellRendererText::builder().editable(false).build();
            TreeViewColumnExt::pack_start(&col, &cell, false);
            // col.pack_start(&cell, false);
            TreeViewColumnExt::add_attribute(&col, &cell, "text", index);
            // col.add_attribute(&cell, "text", index);
            TreeViewColumnExt::add_attribute(&col, &cell, "background", 0);
            // col.add_attribute(&cell, "background", 0);
            TreeViewColumnExt::add_attribute(&col, &cell, "foregound", 1);
            // col.add_attribute(&cell, "foreground", 1);
            cols.push(col);
            index += 1;
        }

        let col = gtk::TreeViewColumn::builder()
            .title("Hue")
            .sort_column_id(index + 1)
            .sort_indicator(true)
            .build();
        let cell = gtk::CellRendererText::builder().editable(false).build();
        TreeViewColumnExt::pack_start(&col, &cell, false);
        // col.pack_start(&cell, false);
        TreeViewColumnExt::add_attribute(&col, &cell, "background", index);
        // col.add_attribute(&cell, "background", index);
        cols.push(col);
        index += 2;

        for attr in self.attributes.iter() {
            let col = gtk::TreeViewColumn::builder()
                .title(&attr.to_string())
                .sort_column_id(index)
                .sort_indicator(true)
                .build();
            let cell = gtk::CellRendererText::builder().editable(false).build();
            TreeViewColumnExt::pack_start(&col, &cell, false);
            // col.pack_start(&cell, false);
            TreeViewColumnExt::add_attribute(&col, &cell, "text", index);
            // col.add_attribute(&cell, "text", index);
            TreeViewColumnExt::add_attribute(&col, &cell, "background", index + 1);
            // col.add_attribute(&cell, "background", index + 1);
            TreeViewColumnExt::add_attribute(&col, &cell, "foreground", index + 2);
            // col.add_attribute(&cell, "foreground", index + 2);
            cols.push(col);
            index += 3;
        }

        for property_type in self.property_types.iter() {
            let col = gtk::TreeViewColumn::builder()
                .title(property_type.list_header())
                .sort_column_id(index)
                .sort_indicator(true)
                .build();
            let cell = gtk::CellRendererText::builder().editable(false).build();
            TreeViewColumnExt::pack_start(&col, &cell, false);
            // col.pack_start(&cell, false);
            TreeViewColumnExt::add_attribute(&col, &cell, "text", index);
            // col.add_attribute(&cell, "text", index);
            TreeViewColumnExt::add_attribute(&col, &cell, "background", 0);
            // col.add_attribute(&cell, "background", 0);
            TreeViewColumnExt::add_attribute(&col, &cell, "foreground", 1);
            // col.add_attribute(&cell, "foreground", 1);
            cols.push(col);
            index += 1;
        }

        cols
    }
}
