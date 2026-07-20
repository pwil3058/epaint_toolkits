// Copyright 2019 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

use pw_gtk_ext::{
    glib,
    gtk::{self, prelude::*},
    gtkx::list::ListViewSpec,
};

use epaint::{
    mixtures::Mixture,
    paint::{CollnPaint, Paint},
};

use colour_math::{HCV, ScalarAttribute};
use epaint::properties::PropertyTypes;

pub struct BasicPaintListViewSpec {
    attributes: Vec<ScalarAttribute>,
    property_types: PropertyTypes,
}

impl BasicPaintListViewSpec {
    pub fn new(attributes: &[ScalarAttribute], propery_types: &PropertyTypes) -> Self {
        Self {
            attributes: attributes.to_vec(),
            property_types: propery_types.clone(),
        }
    }
}

impl ListViewSpec for BasicPaintListViewSpec {
    fn column_types(&self) -> Vec<glib::Type> {
        let mut column_types = vec![
            #[cfg(feature = "paints_have_ids")]
            glib::Type::String,
            glib::Type::String,
            glib::Type::String,
            glib::Type::String,
            glib::Type::String,
            glib::Type::String,
            f64::static_type(),
        ];
        for _ in 0..self.attributes.len() * 3 + self.property_types.len() {
            column_types.push(glib::Type::String);
        }

        column_types
    }

    fn columns(&self) -> Vec<gtk::TreeViewColumn> {
        let mut cols = vec![];

        let mut next_col = 2;
        #[cfg(feature = "paints_have_ids")]
        let headers = ["Id", "Name", "Notes"];
        #[cfg(not(feature = "paints_have_ids"))]
        let headers = ["Name", "Notes"];

        for header in headers.iter() {
            let col = gtk::TreeViewColumnBuilder::new()
                .title(header)
                .resizable(true)
                .sort_column_id(next_col)
                .sort_indicator(true)
                .build();
            let cell = gtk::CellRendererTextBuilder::new().editable(false).build();
            col.pack_start(&cell, false);
            col.add_attribute(&cell, "text", next_col);
            col.add_attribute(&cell, "background", 0);
            col.add_attribute(&cell, "foreground", 1);
            cols.push(col);
            next_col += 1;
        }

        let col = gtk::TreeViewColumnBuilder::new()
            .title("Hue")
            .sort_column_id(next_col + 1)
            .sort_indicator(true)
            .build();
        let cell = gtk::CellRendererTextBuilder::new().editable(false).build();
        col.pack_start(&cell, false);
        col.add_attribute(&cell, "background", next_col);
        cols.push(col);
        next_col += 2;

        let mut index = next_col;
        for attr in self.attributes.iter() {
            let col = gtk::TreeViewColumnBuilder::new()
                .title(&attr.to_string())
                .sort_column_id(index)
                .sort_indicator(true)
                .build();
            let cell = gtk::CellRendererTextBuilder::new().editable(false).build();
            col.pack_start(&cell, false);
            col.add_attribute(&cell, "text", index);
            col.add_attribute(&cell, "background", index + 1);
            col.add_attribute(&cell, "foreground", index + 2);
            cols.push(col);
            index += 3;
        }

        for property_types in self.property_types.iter() {
            let col = gtk::TreeViewColumnBuilder::new()
                .title(property_types.list_header())
                .sort_column_id(index)
                .sort_indicator(true)
                .build();
            let cell = gtk::CellRendererTextBuilder::new().editable(false).build();
            col.pack_start(&cell, false);
            col.add_attribute(&cell, "text", index);
            col.add_attribute(&cell, "background", 0);
            col.add_attribute(&cell, "foreground", 1);
            cols.push(col);
            index += 1;
        }

        cols
    }
}

pub trait PaintListRow {
    fn row(&self, attributes: &[ScalarAttribute]) -> Vec<glib::Value>;
}

impl PaintListRow for Paint {
    fn row(&self, attributes: &[ScalarAttribute]) -> Vec<glib::Value> {
        use colour_math::{ColourAttributes, ColourBasics};
        let ha: f64 = if let Some(angle) = self.hue_angle() {
            angle.into()
        } else {
            -181.0 + f64::from(self.value())
        };
        let hcv_bg = if let Some(hcv) = self.hue_hcv() {
            hcv
        } else {
            HCV::new_grey(self.value())
        };
        let mut row: Vec<glib::Value> = vec![
            self.hcv().pango_string().to_value(),
            self.best_foreground().pango_string().to_value(),
            #[cfg(feature = "paints_have_ids")]
            self.id.to_value(),
            self.name.to_value(),
            self.notes.to_value(),
            hcv_bg.pango_string().to_value(),
            ha.to_value(),
        ];
        for attr in attributes.iter() {
            let string = format!("{:5.4}", f64::from(self.scalar_attribute(*attr)));
            let attr_rgb = self.scalar_attribute_rgb::<f64>(*attr);
            row.push(string.to_value());
            row.push(attr_rgb.pango_string().to_value());
            row.push(attr_rgb.best_foreground().pango_string().to_value());
        }
        for property in self.properties.iter() {
            let string = property.abbrev_value();
            row.push(string.to_value());
        }
        #[cfg(feature = "targeted_mixtures")]
        {
            row.push(self.hcv().pango_string().to_value());
        }
        row
    }
}

impl PaintListRow for CollnPaint {
    fn row(&self, attributes: &[ScalarAttribute]) -> Vec<glib::Value> {
        self.paint.row(attributes)
    }
}

pub struct MixtureListViewSpec {
    attributes: Vec<ScalarAttribute>,
    property_types: PropertyTypes,
}

impl MixtureListViewSpec {
    pub fn new(attributes: &[ScalarAttribute], propery_types: &PropertyTypes) -> Self {
        Self {
            attributes: attributes.to_vec(),
            property_types: propery_types.clone(),
        }
    }
}

impl ListViewSpec for MixtureListViewSpec {
    fn column_types(&self) -> Vec<glib::Type> {
        let mut column_types = vec![
            glib::Type::String,
            glib::Type::String,
            glib::Type::String,
            glib::Type::String,
            glib::Type::String,
            glib::Type::String,
            f64::static_type(),
        ];
        for _ in 0..self.attributes.len() * 3 + self.property_types.len() {
            column_types.push(glib::Type::String);
        }
        #[cfg(feature = "targeted_mixtures")]
        column_types.push(glib::Type::String);

        column_types
    }

    fn columns(&self) -> Vec<gtk::TreeViewColumn> {
        let mut cols = vec![];
        #[cfg(feature = "targeted_mixtures")]
        let target_col = 7 + self.attributes.len() as i32 * 3 + self.property_types.len() as i32;

        let col = gtk::TreeViewColumnBuilder::new()
            .title("Id")
            .resizable(false)
            .sort_column_id(0)
            .sort_indicator(true)
            .build();
        let cell = gtk::CellRendererTextBuilder::new().editable(false).build();
        col.pack_start(&cell, false);
        col.add_attribute(&cell, "text", 0);
        col.add_attribute(&cell, "background", 1);
        col.add_attribute(&cell, "foreground", 2);
        cols.push(col);

        let col = gtk::TreeViewColumnBuilder::new()
            .title("Name")
            .resizable(true)
            .sort_column_id(3)
            .sort_indicator(true)
            .build();
        let cell = gtk::CellRendererTextBuilder::new().editable(false).build();
        col.pack_start(&cell, false);
        col.add_attribute(&cell, "text", 3);
        col.add_attribute(&cell, "background", 1);
        col.add_attribute(&cell, "foreground", 2);
        cols.push(col);

        let col = gtk::TreeViewColumnBuilder::new()
            .title("Notes")
            .resizable(true)
            .sort_column_id(4)
            .sort_indicator(true)
            .build();
        let cell = gtk::CellRendererTextBuilder::new().editable(false).build();
        col.pack_start(&cell, false);
        col.add_attribute(&cell, "text", 4);
        col.add_attribute(&cell, "background", 1);
        col.add_attribute(&cell, "foreground", 2);
        cols.push(col);

        #[cfg(feature = "targeted_mixtures")]
        {
            let col = gtk::TreeViewColumnBuilder::new()
                .title("Target")
                .sort_column_id(target_col)
                .sort_indicator(true)
                .build();
            let cell = gtk::CellRendererTextBuilder::new().editable(false).build();
            col.pack_start(&cell, false);
            col.add_attribute(&cell, "background", target_col);
            cols.push(col);
        }

        let col = gtk::TreeViewColumnBuilder::new()
            .title("Hue")
            .sort_column_id(6)
            .sort_indicator(true)
            .build();
        let cell = gtk::CellRendererTextBuilder::new().editable(false).build();
        col.pack_start(&cell, false);
        col.add_attribute(&cell, "background", 5);
        cols.push(col);

        let mut index = 7;
        for attr in self.attributes.iter() {
            let col = gtk::TreeViewColumnBuilder::new()
                .title(&attr.to_string())
                .sort_column_id(index)
                .sort_indicator(true)
                .build();
            let cell = gtk::CellRendererTextBuilder::new().editable(false).build();
            col.pack_start(&cell, false);
            col.add_attribute(&cell, "text", index);
            col.add_attribute(&cell, "background", index + 1);
            col.add_attribute(&cell, "foreground", index + 2);
            cols.push(col);
            index += 3;
        }

        for property_types in self.property_types.iter() {
            let col = gtk::TreeViewColumnBuilder::new()
                .title(property_types.list_header())
                .sort_column_id(index)
                .sort_indicator(true)
                .build();
            let cell = gtk::CellRendererTextBuilder::new().editable(false).build();
            col.pack_start(&cell, false);
            col.add_attribute(&cell, "text", index);
            col.add_attribute(&cell, "background", 1);
            col.add_attribute(&cell, "foreground", 2);
            cols.push(col);
            index += 1;
        }

        cols
    }
}

impl PaintListRow for Mixture {
    fn row(&self, attributes: &[ScalarAttribute]) -> Vec<glib::Value> {
        use colour_math::ColourAttributes;
        use colour_math::ColourBasics;
        let ha: f64 = if let Some(angle) = self.hue_angle() {
            angle.into()
        } else {
            -181.0 + f64::from(self.value())
        };
        let hcv_bg = if let Some(hcv) = self.hue_hcv() {
            hcv
        } else {
            HCV::new_grey(self.value())
        };
        let mut row: Vec<glib::Value> = vec![
            self.id.to_value(),
            self.hcv().pango_string().to_value(),
            self.best_foreground().pango_string().to_value(),
            self.name.to_value(),
            self.notes.to_value(),
            hcv_bg.pango_string().to_value(),
            ha.to_value(),
        ];
        for attr in attributes.iter() {
            let string = format!("{:5.4}", f64::from(self.scalar_attribute(*attr)));
            let attr_rgb = self.scalar_attribute_rgb::<f64>(*attr);
            row.push(string.to_value());
            row.push(attr_rgb.pango_string().to_value());
            row.push(attr_rgb.best_foreground().pango_string().to_value());
        }
        for property in self.properties.iter() {
            let string = property.abbrev_value();
            row.push(string.to_value());
        }
        #[cfg(feature = "targeted_mixtures")]
        if let Some(target_colour) = self.targeted_colour {
            row.push(target_colour.pango_string().to_value());
        } else {
            row.push(self.colour.pango_string().to_value());
        }
        row
    }
}
