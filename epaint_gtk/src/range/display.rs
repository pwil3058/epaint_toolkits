// Copyright 2020 Peter Williams <pwil3058@gmail.com> <pwil3058@bigpond.net.au>

use std::cell::RefCell;
use std::collections::HashMap;
use std::{collections::BTreeMap, rc::Rc};

use pw_gtk_ext::{
    gtk::{self, prelude::*},
    gtkx::dialog_user::TopGtkWindow,
    sav_state::{ChangedCondnsNotifier, ConditionalWidgetsBuilder},
    wrapper::*,
};

use colour_math::{ColourBasics, HCV, ScalarAttribute};
#[cfg(feature = "targeted_mixtures")]
use colour_math_gtk::attributes::ColourAttributeDisplayStack;
use colour_math_gtk::attributes::ColourAttributeDisplayStackBuilder;
use colour_math_gtk::colour::GdkColour;
use colour_math_gtk::coloured::Colourable;

use epaint::{paint::RangePaint, properties::PropertyTypes};

use crate::range::PaintActionCallback;

#[derive(PWO)]
pub struct PaintDisplay {
    vbox: gtk::Box,
    range_paint: RangePaint,
    #[cfg(feature = "targeted_mixtures")]
    target_label: gtk::Label,
    #[cfg(feature = "targeted_mixtures")]
    cads: Rc<ColourAttributeDisplayStack>,
}

impl PaintDisplay {
    #[cfg(feature = "targeted_mixtures")]
    pub fn set_target_colour(&self, new_target: Option<&impl GdkColour>) {
        if let Some(colour) = new_target {
            self.target_label.set_label("Current Target");
            self.target_label.set_widget_colour(colour);
            self.cads.set_target_colour(Some(colour));
        } else {
            self.target_label.set_label("");
            self.target_label.set_widget_colour(&self.range_paint.hcv());
            self.cads.set_target_colour(Option::<&HCV>::None);
        };
    }

    pub fn paint(&self) -> &RangePaint {
        &self.range_paint
    }
}

#[derive(Default)]
pub struct PaintDisplayBuilder {
    attributes: Vec<ScalarAttribute>,
    property_types: PropertyTypes,
    #[cfg(feature = "targeted_mixtures")]
    target_colour: Option<HCV>,
}

impl PaintDisplayBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn attributes(&mut self, attributes: &[ScalarAttribute]) -> &mut Self {
        self.attributes = attributes.to_vec();
        self
    }

    pub fn property_types(&mut self, property_types: &PropertyTypes) -> &mut Self {
        self.property_types = property_types.clone();
        self
    }

    #[cfg(feature = "targeted_mixtures")]
    pub fn target_colour(&mut self, target_colour: Option<&impl GdkColour>) -> &mut Self {
        self.target_colour = if let Some(target_colour) = target_colour {
            Some(target_colour.hcv())
        } else {
            None
        };
        self
    }

    pub fn build(&self, range_paint: &RangePaint) -> PaintDisplay {
        let hcv = range_paint.hcv();
        let vbox = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Vertical)
            .build();

        #[cfg(feature = "paints_have_ids")]
        {
            let label = gtk::LabelBuilder::new().label(range_paint.id()).build();
            label.set_widget_colour(&hcv);
            vbox.pack_start(&label, false, false, 0);
        }

        let label = gtk::LabelBuilder::new().label(range_paint.name()).build();
        label.set_widget_colour(&hcv);
        vbox.pack_start(&label, false, false, 0);

        let label = gtk::LabelBuilder::new().label(range_paint.notes()).build();
        label.set_widget_colour(&hcv);
        vbox.pack_start(&label, false, false, 0);

        let series_id = range_paint.series_id();
        let label = gtk::LabelBuilder::new().label(&series_id.name).build();
        label.set_widget_colour(&hcv);
        vbox.pack_start(&label, false, false, 0);

        let series_id = range_paint.series_id();
        let label = gtk::LabelBuilder::new()
            .label(&series_id.proprietor)
            .build();
        label.set_widget_colour(&hcv);
        vbox.pack_start(&label, false, false, 0);

        let cads = ColourAttributeDisplayStackBuilder::new()
            .attributes(&self.attributes)
            .build();
        cads.set_colour(Some(&hcv));

        #[cfg(feature = "targeted_mixtures")]
        let target_label = if let Some(target_colour) = self.target_colour {
            let label = gtk::LabelBuilder::new().label("Target").build();
            label.set_widget_colour(&target_colour);
            cads.set_target_colour(Some(&target_colour));
            label
        } else {
            let label = gtk::LabelBuilder::new().build();
            label.set_widget_colour(&hcv);
            label
        };
        #[cfg(feature = "targeted_mixtures")]
        vbox.pack_start(&target_label, true, true, 0);
        vbox.pack_start(cads.pwo(), true, true, 0);

        for property in range_paint.properties() {
            let value = property.value();
            let label = gtk::LabelBuilder::new().label(value).build();
            label.set_widget_colour(&hcv);
            vbox.pack_start(&label, false, false, 0);
        }
        vbox.show_all();

        PaintDisplay {
            vbox,
            range_paint: range_paint.clone(),
            #[cfg(feature = "targeted_mixtures")]
            target_label,
            #[cfg(feature = "targeted_mixtures")]
            cads,
        }
    }
}

struct PaintDisplayDialog {
    dialog: gtk::Dialog,
    #[cfg(feature = "targeted_mixtures")]
    display: PaintDisplay,
}

pub struct PaintDisplayDialogManager<W: TopGtkWindow> {
    caller: W,
    buttons: Vec<(u16, &'static str, Option<&'static str>, u64)>,
    button_callbacks: RefCell<HashMap<u16, Vec<PaintActionCallback>>>,
    paint_display_builder: RefCell<PaintDisplayBuilder>,
    conditional_widgets_builder: ConditionalWidgetsBuilder,
    dialogs: RefCell<BTreeMap<RangePaint, PaintDisplayDialog>>,
}

impl<W: TopGtkWindow> PaintDisplayDialogManager<W> {
    fn new_dialog(&self) -> gtk::Dialog {
        let dialog = gtk::DialogBuilder::new().build();
        if let Some(parent) = self.caller.get_toplevel_gtk_window() {
            dialog.set_transient_for(Some(&parent));
        }
        // TODO: think about removal from map as an optional action to hiding
        dialog.connect_delete_event(|d, _| {
            d.hide_on_delete();
            Inhibit(true)
        });
        dialog
    }

    #[cfg(feature = "targeted_mixtures")]
    pub fn set_target_colour(&self, colour: Option<&impl GdkColour>) {
        self.paint_display_builder
            .borrow_mut()
            .target_colour(colour);
        for pdd in self.dialogs.borrow().values() {
            pdd.display.set_target_colour(colour);
        }
    }

    fn inform_button_action(&self, action: u16, paint: RangePaint) {
        let button_callbacks = self.button_callbacks.borrow();
        for callback in button_callbacks
            .get(&action)
            .expect("programmer error")
            .iter()
        {
            callback(paint.clone())
        }
    }

    pub fn connect_action_button<F: Fn(RangePaint) + 'static>(&self, action: u16, callback: F) {
        self.button_callbacks
            .borrow_mut()
            .get_mut(&action)
            .expect("programmer error")
            .push(Box::new(callback));
    }
}

pub trait DisplayPaint {
    fn display_paint(&self, paint: &RangePaint);
}

impl<W: TopGtkWindow + 'static> DisplayPaint for Rc<PaintDisplayDialogManager<W>> {
    fn display_paint(&self, colln_paint: &RangePaint) {
        if !self.dialogs.borrow().contains_key(colln_paint) {
            let dialog = self.new_dialog();
            let display = self.paint_display_builder.borrow().build(colln_paint);
            let managed_buttons = self.conditional_widgets_builder.build::<u16, gtk::Widget>();
            for (response, label, tooltip_text, condns) in self.buttons.iter() {
                let button = dialog.add_button(label, gtk::ResponseType::Other(*response));
                button.set_tooltip_text(*tooltip_text);
                managed_buttons
                    .add_widget(*response, &button, *condns)
                    .expect(&std::format!("Duplicate key or button: {label:?}"));
            }
            dialog
                .get_content_area()
                .pack_start(display.pwo(), true, true, 0);
            let self_c = Rc::clone(self);
            let colln_paint_clone = colln_paint.clone();
            dialog.connect_response(move |_, response| {
                if let gtk::ResponseType::Other(code) = response {
                    self_c.inform_button_action(code, colln_paint_clone.clone());
                }
            });
            #[cfg(feature = "targeted_mixtures")]
            let pdd = PaintDisplayDialog { dialog, display };
            #[cfg(not(feature = "targeted_mixtures"))]
            let pdd = PaintDisplayDialog { dialog };
            self.dialogs.borrow_mut().insert(colln_paint.clone(), pdd);
        };
        let dialogs = self.dialogs.borrow();
        let pdd = dialogs.get(colln_paint).expect("we just put it there");
        pdd.dialog.present();
    }
}

pub struct PaintDisplayDialogManagerBuilder<W: TopGtkWindow> {
    caller: W,
    buttons: Vec<(u16, &'static str, Option<&'static str>, u64)>,
    attributes: Vec<ScalarAttribute>,
    property_types: PropertyTypes,
    target_colour: Option<HCV>,
    change_notifier: ChangedCondnsNotifier,
}

impl<W: TopGtkWindow + Clone> PaintDisplayDialogManagerBuilder<W> {
    pub fn new(caller: &W) -> Self {
        let change_notifier = ChangedCondnsNotifier::default();
        Self {
            caller: caller.clone(),
            buttons: vec![],
            attributes: vec![],
            property_types: PropertyTypes::default(),
            target_colour: None,
            change_notifier,
        }
    }

    pub fn attributes(&mut self, attributes: &[ScalarAttribute]) -> &mut Self {
        self.attributes = attributes.to_vec();
        self
    }

    pub fn property_types(&mut self, property_types: &PropertyTypes) -> &mut Self {
        self.property_types = property_types.clone();
        self
    }

    pub fn buttons(
        &mut self,
        buttons: &[(u16, &'static str, Option<&'static str>, u64)],
    ) -> &mut Self {
        self.buttons = buttons.to_vec();
        self
    }

    pub fn change_notifier(&mut self, change_notifier: &ChangedCondnsNotifier) -> &mut Self {
        self.change_notifier = change_notifier.clone();
        self
    }

    pub fn target_colour(&mut self, target_colour: &impl GdkColour) -> &mut Self {
        self.target_colour = Some(target_colour.hcv());
        self
    }

    pub fn build(&self) -> Rc<PaintDisplayDialogManager<W>> {
        let mut paint_display_builder = PaintDisplayBuilder::new();
        paint_display_builder
            .attributes(&self.attributes)
            .property_types(&self.property_types);
        #[cfg(feature = "targeted_mixtures")]
        if let Some(target_colour) = self.target_colour {
            paint_display_builder.target_colour(Some(&target_colour));
        }
        let mut hash_map: HashMap<u16, Vec<PaintActionCallback>> = HashMap::new();
        for (id, _, _, _) in self.buttons.iter() {
            hash_map.insert(*id, vec![]);
        }
        let mut conditional_widgets_builder = ConditionalWidgetsBuilder::new();
        conditional_widgets_builder.change_notifier(&self.change_notifier);
        Rc::new(PaintDisplayDialogManager {
            caller: self.caller.clone(),
            buttons: self.buttons.clone(),
            button_callbacks: RefCell::new(hash_map),
            paint_display_builder: RefCell::new(paint_display_builder),
            conditional_widgets_builder,
            dialogs: RefCell::new(BTreeMap::new()),
        })
    }
}
