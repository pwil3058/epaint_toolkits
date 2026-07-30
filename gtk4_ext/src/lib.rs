// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use gtk::{glib, Widget};

pub trait PackableWidgetObject {
    type PWT: glib::object::IsA<Widget>;

    fn pwo(&self) -> &Self::PWT;
}

pub mod hex_entry;
