// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

pub use gtk;
pub use gtk::gdk;
pub use gtk::gdk_pixbuf;
pub use gtk::gio;
pub use gtk::glib;

pub use sourceview;

pub static UNEXPECTED: &str = "Unexpected error: please inform <pwil3058@bigpond.net.au>";

pub mod gtkx;

pub mod printer;
pub mod wrapper;
