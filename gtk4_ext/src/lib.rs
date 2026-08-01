// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use gtk::{Widget, glib};

pub trait PackableWidgetObject {
    type PWT: glib::object::IsA<Widget>;

    fn pwo(&self) -> &Self::PWT;
}

// pub mod gdx;
pub mod gtkx;
pub mod hex_entry;
pub mod wrapper;
