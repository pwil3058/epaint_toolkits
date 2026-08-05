// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

pub mod attributes;
pub mod cads;
pub mod colour_edit;
pub mod hue_wheel;
pub mod manipulator;
pub mod rgb_entry;

pub mod colour {
    use gtk_ext::gdk;

    use colour_math::{LightLevel, ManipulatedColour, HCV, RGB};

    pub trait GdkColour: colour_math::ColourIfce {
        fn gdk_rgba(&self) -> gdk::RGBA {
            let rgb = self.rgb::<f64>();
            gdk::RGBA::new(rgb[0], rgb[1], rgb[2], 1.0)
        }
    }

    impl<L: LightLevel> GdkColour for RGB<L> {}
    impl GdkColour for HCV {}

    pub trait ManipGdkColour: GdkColour + ManipulatedColour {}

    impl<L: LightLevel> ManipGdkColour for RGB<L> {}
    impl ManipGdkColour for HCV {}
}

pub mod coloured {
    use gtk_ext::glib;
    use gtk_ext::gtk::{self, prelude::*};

    use crate::colour::*;

    pub trait NewColourable: WidgetExt + LabelExt {
        fn set_widget_colour(&self, colour: &impl GdkColour) {
            let bg = colour.gdk_rgba();
            let fg = colour.best_foreground().gdk_rgba();
            let bg_hex = format!(
                "#{:02x}{:02x}{:02x}",
                (bg.red() * 255.0) as u8,
                (bg.green() * 255.0) as u8,
                (bg.blue() * 255.0) as u8
            );
            let fg_hex = format!(
                "#{:02x}{:02x}{:02x}",
                (fg.red() * 255.0) as u8,
                (fg.green() * 255.0) as u8,
                (fg.blue() * 255.0) as u8
            );
            let markup = format!(
                "<span background=\"{}\" foreground=\"{}\"> {} </span>",
                bg_hex,
                fg_hex,
                glib::markup_escape_text("whatever")
            );
            self.set_markup(&markup);
        }
    }
    pub trait Colourable: WidgetExt {
        fn set_widget_colour(&self, colour: &impl GdkColour);
    }

    impl Colourable for gtk::Button {
        fn set_widget_colour(&self, colour: &impl GdkColour) {
            //     let _bg_gdk_rgba = colour.gdk_rgba();
            //     let _fg_gdk_rgba = colour.best_foreground().gdk_rgba();
            // self.override_background_color(gtk::StateFlags::empty(), Some(&bg_gdk_rgba));
            // self.override_color(gtk::StateFlags::empty(), Some(&fg_gdk_rgba));
            // for child in self.get_children().iter() {
            //     child.set_widget_colour(colour);
            // }
        }
    }

    // impl Colourable for gtk::Bin {}
    // impl Colourable for gtk::Box {}
    // impl Colourable for gtk::ButtonBox {}
    // impl Colourable for gtk::CheckButton {}
    // impl Colourable for gtk::ComboBox {}
    // impl Colourable for gtk::ComboBoxText {}
    // impl Colourable for gtk::Container {}
    // impl Colourable for gtk::Entry {}
    // impl Colourable for gtk::EventBox {}
    // impl Colourable for gtk::FlowBox {}
    // impl Colourable for gtk::Frame {}
    // impl Colourable for gtk::Grid {}
    impl NewColourable for gtk::Label {}
    // impl Colourable for gtk::LinkButton {}
    // impl Colourable for gtk::MenuBar {}
    // impl Colourable for gtk::RadioButton {}
    // impl Colourable for gtk::Scrollbar {}
    // impl Colourable for gtk::SpinButton {}
    // impl Colourable for gtk::ToggleButton {}
    // impl Colourable for gtk::ToolButton {}
    // impl Colourable for gtk::Toolbar {}
    // impl Colourable for gtk::Widget {}
}
