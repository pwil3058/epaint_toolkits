// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

pub mod cads;
pub mod cm_cairo;
pub mod manipulator;
pub mod rgb_entry;

pub mod colour {
    use gtk::gdk;

    use colour_math::{HCV, LightLevel, ManipulatedColour, RGB};

    pub trait GdkColour: colour_math::ColourIfce {
        fn gdk_rgba(&self) -> gdk::RGBA {
            let rgb = self.rgb::<f32>();
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
    use gtk::{self, prelude::*};

    use crate::colour::*;

    #[allow(deprecated)]
    pub trait Colourable: WidgetExt {
        fn set_widget_colour(&self, colour: &impl GdkColour) {
            let _bg_gdk_rgba = colour.gdk_rgba();
            let _fg_gdk_rgba = colour.best_foreground().gdk_rgba();
            // self.override_background_color(gtk::StateFlags::empty(), Some(&bg_gdk_rgba));
            // self.override_color(gtk::StateFlags::empty(), Some(&fg_gdk_rgba));
        }
    }

    #[allow(deprecated)]
    impl Colourable for gtk::Button {
        fn set_widget_colour(&self, colour: &impl GdkColour) {
            let _bg_gdk_rgba = colour.gdk_rgba();
            let _fg_gdk_rgba = colour.best_foreground().gdk_rgba();
            // self.override_background_color(gtk::StateFlags::empty(), Some(&bg_gdk_rgba));
            // self.override_color(gtk::StateFlags::empty(), Some(&fg_gdk_rgba));
            // for child in self.get_children().iter() {
            //     child.set_widget_colour(colour);
            // }
        }
    }

    // impl Colourable for gtk::Bin {}
    impl Colourable for gtk::Box {}
    // impl Colourable for gtk::Button {}
    impl Colourable for gtk::CheckButton {}
    // impl Colourable for gtk::ComboBox {}
    // impl Colourable for gtk::ComboBoxText {}
    // impl Colourable for gtk::Container {}
    impl Colourable for gtk::Entry {}
    // impl Colourable for gtk::EventBox {}
    impl Colourable for gtk::FlowBox {}
    impl Colourable for gtk::Frame {}
    impl Colourable for gtk::Grid {}
    impl Colourable for gtk::Label {}
    impl Colourable for gtk::LinkButton {}
    // impl Colourable for gtk::MenuBar {}
    // impl Colourable for gtk::RadioButton {}
    impl Colourable for gtk::Scrollbar {}
    impl Colourable for gtk::SpinButton {}
    impl Colourable for gtk::ToggleButton {}
    // impl Colourable for gtk::ToolButton {}
    // impl Colourable for gtk::Toolbar {}
    impl Colourable for gtk::Widget {}
}
