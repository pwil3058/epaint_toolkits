// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::ops::Deref;
use std::str::FromStr;

use egui::{Id, Ui};
use serde::{Deserialize, Serialize};

use epaint::{
    Finish, Fluorescence, Granulation, Lightfastness, Luminescence, Metallicness, Opacity,
    Permanence, PropertyType, Staining, Transparency,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Property {
    Transparency(Transparency),
    Lightfastness(Lightfastness),
    Staining(Staining),
    Finish(Finish),
    Opacity(Opacity),
    Permanence(Permanence),
    Luminescence(Luminescence),
    Fluorescence(Fluorescence),
    Metallicness(Metallicness),
    Granulation(Granulation),
}

/// Returns the default Property for the PropertyType
impl From<PropertyType> for Property {
    fn from(property_type: PropertyType) -> Self {
        use PropertyType::*;
        match property_type {
            Transparency => Self::Transparency(epaint::Transparency::default()),
            Lightfastness => Self::Lightfastness(epaint::Lightfastness::default()),
            Staining => Self::Staining(epaint::Staining::default()),
            Finish => Self::Finish(epaint::Finish::default()),
            Opacity => Self::Opacity(epaint::Opacity::default()),
            Permanence => Self::Permanence(epaint::Permanence::default()),
            Granulation => Self::Granulation(epaint::Granulation::default()),
            Luminescence => Self::Luminescence(epaint::Luminescence::default()),
            Fluorescence => Self::Fluorescence(epaint::Fluorescence::default()),
            Metallicness => Self::Metallicness(epaint::Metallicness::default()),
        }
    }
}

impl FromStr for Property {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut split = string.split("::");
        let type_name = split.next().unwrap();
        let property_type = PropertyType::from_str(type_name).unwrap();
        let variant_name = split.next().unwrap();
        use PropertyType::*;
        let result = match property_type {
            Transparency => Property::Transparency(epaint::Transparency::from_str(variant_name)?),
            Lightfastness => {
                Property::Lightfastness(epaint::Lightfastness::from_str(variant_name)?)
            }
            Fluorescence => Property::Fluorescence(epaint::Fluorescence::from_str(variant_name)?),
            Finish => Property::Finish(epaint::Finish::from_str(variant_name)?),
            Staining => Property::Staining(epaint::Staining::from_str(variant_name)?),
            Opacity => Property::Opacity(epaint::Opacity::from_str(variant_name)?),
            Permanence => Property::Permanence(epaint::Permanence::from_str(variant_name)?),
            Luminescence => Property::Luminescence(epaint::Luminescence::from_str(variant_name)?),
            Granulation => Property::Granulation(epaint::Granulation::from_str(variant_name)?),
            Metallicness => Property::Metallicness(epaint::Metallicness::from_str(variant_name)?),
        };
        debug_assert_eq!(split.next(), None);
        Ok(result)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Properties(pub Vec<Property>);

impl Deref for Properties {
    type Target = [Property];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Tracks temporary UI selections across frames before committing.
#[derive(Clone, Debug)]
struct PropertyScratchState {
    /// Stores the current selected variant string for this specific property row
    pub selected_variant: String,
}

impl PropertyScratchState {
    /// Loads the working choice from egui's frame cache, falling back to macro defaults.
    fn load_or_default(ui: &Ui, id: Id, property_type: PropertyType) -> Self {
        ui.ctx()
            .data(|map| map.get_temp::<Self>(id))
            .unwrap_or_else(|| Self {
                selected_variant: property_type.default_str().to_string(),
            })
    }

    /// Flushes the local UI modification down into egui's memory loop.
    fn save(&self, ui: &Ui, id: Id) {
        ui.ctx().data_mut(|map| map.insert_temp(id, self.clone()));
    }
}

/// Renders the property workspace stack and handles delayed button execution.
/// Returns `Some(Properties)` ONLY on the exact frame the submission button is pressed.
pub fn draw_properties_editor_stack(
    ui: &mut Ui,
    unique_editor_namespace: &str, // Keeps this component instance separated from others
    active_types: &[PropertyType],
) -> Option<Properties> {
    // 1. Loop through and draw each drop-down item immutably
    for &prop_type in active_types {
        ui.horizontal(|ui| {
            ui.label(prop_type.prompt());

            // Build a stable identifier for this specific row in egui's layout cache
            let row_id = Id::new(unique_editor_namespace).with(prop_type.name());
            let mut state = PropertyScratchState::load_or_default(ui, row_id, prop_type);

            egui::ComboBox::from_id_salt(row_id)
                .selected_text(&state.selected_variant)
                .show_ui(ui, |ui| {
                    for variant_str in prop_type.variant_strings() {
                        let is_selected = variant_str == state.selected_variant;

                        if ui.selectable_label(is_selected, variant_str).clicked() {
                            state.selected_variant = variant_str.to_string();
                            state.save(ui, row_id); // Save change locally without mutating the core model
                        }
                    }
                });
        });
    }

    ui.add_space(10.0);

    // 2. The Final Commit Phase
    if ui.button("Add / Accept Paint").clicked() {
        let mut compiled_vec = Vec::with_capacity(active_types.len());

        for &prop_type in active_types {
            let row_id = Id::new(unique_editor_namespace).with(prop_type.name());
            let state = PropertyScratchState::load_or_default(ui, row_id, prop_type);

            // Turn our cached strings back into a physical, validated Property instance
            if let Ok(property) =
                Property::from_str(&format!("{}::{}", prop_type.name(), state.selected_variant))
            {
                compiled_vec.push(property);
            }
        }

        // Return the clean, complete properties collection asset
        return Some(Properties(compiled_vec));
    }

    None
}
