// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use eframe::egui;
use egui_extras::{Column, TableBuilder};

#[derive(Debug, Default, Clone)]
pub struct ColourTableView {
    pub search_filter: String,
    pub selected_row_index: Option<usize>,
}

impl ColourTableView {
    pub fn new() -> Self {
        Self {
            search_filter: String::new(),
            selected_row_index: None,
        }
    }

    /// Renders the virtualized data grid table panel natively.
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // Replaces text buffer change listeners with a clean immediate-mode single line entry
            ui.horizontal(|ui| {
                ui.label("🔍 Filter Search: ");
                ui.text_edit_singleline(&mut self.search_filter);

                if !self.search_filter.is_empty() && ui.button("❌ Clear").clicked() {
                    self.search_filter.clear();
                }
            });

            ui.add_space(8.0);

            // Placeholder array containing some mock paint records
            let pigment_database = vec![
                ("Cobalt Blue", "PB28", "Series 4", "Excellent"),
                ("Ultramarine Blue", "PB29", "Series 2", "Excellent"),
                ("Cadmium Red", "PR108", "Series 4", "Very Good"),
                ("Burnt Sienna", "PBr7", "Series 1", "Excellent"),
                ("Raw Umber", "PBr7", "Series 1", "Excellent"),
                ("Yellow Ochre", "PY43", "Series 1", "Excellent"),
            ];

            // Filter rows instantly before feeding them into the layout virtualizer
            let filtered_paints: Vec<_> = pigment_database
                .into_iter()
                .filter(|p| {
                    self.search_filter.is_empty()
                        || p.0
                            .to_lowercase()
                            .contains(&self.search_filter.to_lowercase())
                        || p.1
                            .to_lowercase()
                            .contains(&self.search_filter.to_lowercase())
                })
                .collect();

            // Instantiate our declarative high-performance data grid table
            TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::initial(150.0).at_least(100.0)) // Paint Name
                .column(Column::initial(80.0)) // Pigment Index
                .column(Column::initial(100.0)) // Series Tier
                .column(Column::remainder()) // Notes Column
                .header(25.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("🎨 Paint Name");
                    });
                    header.col(|ui| {
                        ui.strong("🧬 Index");
                    });
                    header.col(|ui| {
                        ui.strong("📊 Series");
                    });
                    header.col(|ui| {
                        ui.strong("📝 Lightfastness");
                    });
                })
                .body(|body| {
                    let row_height = 22.0;
                    body.rows(row_height, filtered_paints.len(), |mut row| {
                        let idx = row.index();
                        let paint = &filtered_paints[idx];

                        // Row selection check replaces complex tree view iter matching paths
                        let is_selected = self.selected_row_index == Some(idx);

                        row.col(|ui| {
                            let resp = ui.selectable_label(is_selected, paint.0);
                            if resp.clicked() {
                                self.selected_item_clicked(idx);
                            }
                        });
                        row.col(|ui| {
                            ui.label(paint.1);
                        });
                        row.col(|ui| {
                            ui.strong(paint.2);
                        });
                        row.col(|ui| {
                            ui.label(paint.3);
                        });
                    });
                });
        });
    }

    fn selected_item_clicked(&mut self, index: usize) {
        if self.selected_row_index == Some(index) {
            self.selected_row_index = None; // Toggle selection off if clicked again
        } else {
            self.selected_row_index = Some(index);
        }
    }
}
