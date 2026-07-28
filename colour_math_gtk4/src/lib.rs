// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

pub mod cads;
pub mod cm_cairo;
pub mod manipulator;

pub mod colour {
    use gdk;

    use colour_math::{HCV, LightLevel, ManipulatedColour, RGB};

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
