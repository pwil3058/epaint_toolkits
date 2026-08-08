// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use gtk_ext::{
    glib,
    gtk::{self, prelude::*},
    gtkx::list::ListViewSpec,
};

use epaint::{
    mixtures::Mixture,
    paint::{Paint, RangePaint},
};

use colour_math::{HCV, ScalarAttribute};
use epaint::properties::PropertyTypes;

pub struct PaintListViewSpec {
    attributes: Vec<ScalarAttribute>,
    property_types: PropertyTypes,
}

impl PaintListViewSpec {
    pub fn new(attributes: &[ScalarAttribute], propery_types: &PropertyTypes) -> Self {
        Self {
            attributes: attributes.to_vec(),
            property_types: propery_types.clone(),
        }
    }
}

impl ListViewSpec for PaintListViewSpec {
    fn column_types(&self) -> Vec<glib::Type> {
        let mut column_types = vec![
            #[cfg(feature = "paints_have_ids")]
            glib::Type::STRING,
            glib::Type::STRING,
            glib::Type::STRING,
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

        let mut next_col = 2;
        #[cfg(feature = "paints_have_ids")]
        let headers = ["Id", "Name", "Notes"];
        #[cfg(not(feature = "paints_have_ids"))]
        let headers = ["Name", "Notes"];

        for header in headers {
            let col = gtk::TreeViewColumn::builder()
                .title(header)
                .resizable(true)
                .sort_column_id(next_col)
                .sort_indicator(true)
                .build();
            let cell = gtk::CellRendererText::builder().editable(false).build();
            TreeViewColumnExt::pack_start(&col, &cell, false);
            // col.pack_start(&cell, false);
            TreeViewColumnExt::add_attribute(&col, &cell, "text", next_col);
            // col.add_attribute(&cell, "text", next_col);
            TreeViewColumnExt::add_attribute(&col, &cell, "background", 0);
            // col.add_attribute(&cell, "background", 0);
            TreeViewColumnExt::add_attribute(&col, &cell, "foreground", 1);
            // col.add_attribute(&cell, "foreground", 1);
            cols.push(col);
            next_col += 1;
        }

        let col = gtk::TreeViewColumn::builder()
            .title("Hue")
            .sort_column_id(next_col + 1)
            .sort_indicator(true)
            .build();
        let cell = gtk::CellRendererText::builder().editable(false).build();
        TreeViewColumnExt::pack_start(&col, &cell, false);
        // col.pack_start(&cell, false);
        TreeViewColumnExt::add_attribute(&col, &cell, "background", next_col);
        // col.add_attribute(&cell, "background", next_col);
        cols.push(col);
        next_col += 2;

        let mut index = next_col;
        for attr in self.attributes.iter() {
            let col = gtk::TreeViewColumn::builder()
                .title(attr.to_string())
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

        for property_types in self.property_types.iter() {
            let col = gtk::TreeViewColumn::builder()
                .title(property_types.list_header())
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

        row
    }
}

impl PaintListRow for RangePaint {
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
            glib::Type::STRING,
            glib::Type::STRING,
            glib::Type::STRING,
            glib::Type::STRING,
            glib::Type::STRING,
            glib::Type::STRING,
            f64::static_type(),
        ];
        for _ in 0..self.attributes.len() * 3 + self.property_types.len() {
            column_types.push(glib::Type::STRING);
        }
        #[cfg(feature = "targeted_mixtures")]
        column_types.push(glib::Type::STRING);

        column_types
    }

    fn columns(&self) -> Vec<gtk::TreeViewColumn> {
        let mut cols = vec![];

        let col = gtk::TreeViewColumn::builder()
            .title("Id")
            .resizable(false)
            .sort_column_id(0)
            .sort_indicator(true)
            .build();
        let cell = gtk::CellRendererText::builder().editable(false).build();
        TreeViewColumnExt::pack_start(&col, &cell, false);
        // col.pack_start(&cell, false);
        TreeViewColumnExt::add_attribute(&col, &cell, "text", 0);
        // col.add_attribute(&cell, "text", 0);
        TreeViewColumnExt::add_attribute(&col, &cell, "background", 1);
        // col.add_attribute(&cell, "background", 1);
        TreeViewColumnExt::add_attribute(&col, &cell, "foreground", 2);
        // col.add_attribute(&cell, "foreground", 2);
        cols.push(col);

        let mut next_col = 3;
        for header in ["Name", "Notes"] {
            let col = gtk::TreeViewColumn::builder()
                .title(header)
                .resizable(true)
                .sort_column_id(next_col)
                .sort_indicator(true)
                .build();
            let cell = gtk::CellRendererText::builder().editable(false).build();
            TreeViewColumnExt::pack_start(&col, &cell, false);
            // col.pack_start(&cell, false);
            TreeViewColumnExt::add_attribute(&col, &cell, "text", next_col);
            // col.add_attribute(&cell, "text", next_col);
            TreeViewColumnExt::add_attribute(&col, &cell, "background", 1);
            // col.add_attribute(&cell, "background", 1);
            TreeViewColumnExt::add_attribute(&col, &cell, "foreground", 2);
            // col.add_attribute(&cell, "foreground", 2);
            cols.push(col);
            next_col += 1;
        }

        #[cfg(feature = "targeted_mixtures")]
        {
            let col = gtk::TreeViewColumn::builder()
                .title("Target")
                .sort_column_id(next_col)
                .sort_indicator(true)
                .build();
            let cell = gtk::CellRendererText::builder().editable(false).build();
            TreeViewColumnExt::pack_start(&col, &cell, false);
            // col.pack_start(&cell, false);
            TreeViewColumnExt::add_attribute(&col, &cell, "background", next_col);
            // col.add_attribute(&cell, "background", next_col);
            cols.push(col);
            next_col += 1;
        }

        let col = gtk::TreeViewColumn::builder()
            .title("Hue")
            .sort_column_id(next_col + 1)
            .sort_indicator(true)
            .build();
        let cell = gtk::CellRendererText::builder().editable(false).build();
        TreeViewColumnExt::pack_start(&col, &cell, false);
        // col.pack_start(&cell, false);
        TreeViewColumnExt::add_attribute(&col, &cell, "background", next_col);
        // col.add_attribute(&cell, "background", next_col);
        cols.push(col);
        next_col += 2;

        for attr in self.attributes.iter() {
            let col = gtk::TreeViewColumn::builder()
                .title(attr.to_string())
                .sort_column_id(next_col)
                .sort_indicator(true)
                .build();
            let cell = gtk::CellRendererText::builder().editable(false).build();
            TreeViewColumnExt::pack_start(&col, &cell, false);
            // col.pack_start(&cell, false);
            TreeViewColumnExt::add_attribute(&col, &cell, "text", next_col);
            // col.add_attribute(&cell, "text", next_col);
            TreeViewColumnExt::add_attribute(&col, &cell, "background", next_col + 1);
            // col.add_attribute(&cell, "background", next_col + 1);
            TreeViewColumnExt::add_attribute(&col, &cell, "foreground", next_col + 2);
            // col.add_attribute(&cell, "foreground", next_col + 2);
            cols.push(col);
            next_col += 3;
        }

        for property_types in self.property_types.iter() {
            let col = gtk::TreeViewColumn::builder()
                .title(property_types.list_header())
                .sort_column_id(next_col)
                .sort_indicator(true)
                .build();
            let cell = gtk::CellRendererText::builder().editable(false).build();
            TreeViewColumnExt::pack_start(&col, &cell, false);
            // col.pack_start(&cell, false);
            TreeViewColumnExt::add_attribute(&col, &cell, "text", next_col);
            // col.add_attribute(&cell, "text", next_col);
            TreeViewColumnExt::add_attribute(&col, &cell, "background", 1);
            // col.add_attribute(&cell, "background", 1);
            TreeViewColumnExt::add_attribute(&col, &cell, "foreground", 2);
            // col.add_attribute(&cell, "foreground", 2);
            cols.push(col);
            next_col += 1;
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
        #[cfg(feature = "targeted_mixtures")]
        let targeted_colour = if let Some(targeted_colour) = self.targeted_colour() {
            targeted_colour.pango_string().to_value()
        } else {
            self.colour.pango_string().to_value()
        };
        let mut row: Vec<glib::Value> = vec![
            self.id.to_value(),
            self.hcv().pango_string().to_value(),
            self.best_foreground().pango_string().to_value(),
            self.name.to_value(),
            self.notes.to_value(),
            #[cfg(feature = "targeted_mixtures")]
            targeted_colour,
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
        row
    }
}
