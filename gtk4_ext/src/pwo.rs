// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use gtk;
use gtk::glib;

pub trait PackableWidgetObject {
    type PWT: glib::object::IsA<gtk::Widget>;

    fn pwo(&self) -> &Self::PWT;
}
