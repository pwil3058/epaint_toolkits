// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, glib};

use colour_math::HueConstants;
use colour_math::beigui::attr_display::ColourAttributeType;
use colour_math::hcv::HCV;
use colour_math_gtk4::cads::GtkColourAttributeDisplay;

const APP_ID: &str = "ColourMathGTK4GUI";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let gcad = GtkColourAttributeDisplay::new(&ColourAttributeType::Warmth);
    gcad.set_colour(Some(&HCV::YELLOW));
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Colour Math GTK4 GUI")
        .child(&gcad)
        .build();

    // Present window
    window.present();
}
