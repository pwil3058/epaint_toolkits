// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::{cell::Cell, rc::Rc};

use pw_gtk_ext::{
    gdk, gdk_pixbuf,
    gtk::{self, prelude::*},
    gtkx::window::RememberGeometry,
    wrapper::*,
};

#[derive(PWO)]
pub struct PersistentWindowButton {
    button: gtk::Button,
    window: gtk::Window,
    is_iconified: Cell<bool>,
}

pub struct PersistentWindowButtonBuilder {
    button: gtk::Button,
    window: gtk::Window,
    is_iconified: Cell<bool>,
}

impl Default for PersistentWindowButtonBuilder {
    fn default() -> Self {
        Self {
            button: gtk::ButtonBuilder::new().build(),
            window: gtk::WindowBuilder::new().destroy_with_parent(true).build(),
            is_iconified: Cell::new(false),
        }
    }
}

impl PersistentWindowButtonBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn icon<P: IsA<gtk::Widget>>(self, image: &P) -> Self {
        self.button.set_image(Some(image));
        self
    }

    pub fn label(self, label: &str) -> Self {
        self.button.set_label(label);
        self
    }

    pub fn tooltip_text(self, text: &str) -> Self {
        self.button.set_tooltip_text(Some(text));
        self
    }

    pub fn window_title(self, title: &str) -> Self {
        self.window.set_title(title);
        self
    }

    pub fn window_icon(self, icon: &gdk_pixbuf::Pixbuf) -> Self {
        self.window.set_icon(Some(icon));
        self
    }

    pub fn window_child<P: IsA<gtk::Widget>>(self, widget: &P) -> Self {
        self.window.add(widget);
        self
    }

    pub fn window_geometry(
        self,
        saved_geometry_key: Option<&str>,
        default_size: (i32, i32),
    ) -> Self {
        if let Some(saved_geometry_key) = saved_geometry_key {
            self.window
                .set_geometry_from_recollections(saved_geometry_key, default_size);
        } else {
            self.window
                .set_default_geometry(default_size.0, default_size.1);
        }
        self
    }

    pub fn build(self) -> Rc<PersistentWindowButton> {
        let pwb = Rc::new(PersistentWindowButton {
            button: self.button,
            window: self.window,
            is_iconified: self.is_iconified,
        });

        pwb.window.connect_delete_event(|w, _| {
            w.hide_on_delete();
            Inhibit(true)
        });

        let pwb_c = Rc::clone(&pwb);
        pwb.window.connect_window_state_event(move |_, event| {
            let state = event.get_new_window_state();
            pwb_c
                .is_iconified
                .set(state.contains(gdk::WindowState::ICONIFIED));
            Inhibit(false)
        });

        let pwb_c = Rc::clone(&pwb);
        pwb.button.connect_clicked(move |_| {
            // NB: diconify() is unreliable due to window manager interference
            if pwb_c.window.get_visible() && !pwb_c.is_iconified.get() {
                pwb_c.window.hide();
            } else {
                pwb_c.window.present();
            }
        });

        pwb
    }
}
