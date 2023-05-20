use std::{path::Path, time::Duration};

use crate::{ModViewModel, TemplateApp};

impl TemplateApp {
    /// right panel
    pub fn right_side_view(&mut self, ui: &mut egui::Ui) {
        // Tab view
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.current_tab_view,
                crate::app::ETabView::Plugins,
                "Plugins",
            );

            ui.selectable_value(
                &mut self.current_tab_view,
                crate::app::ETabView::Downloads,
                "Downloads",
            );

            ui.selectable_value(
                &mut self.current_tab_view,
                crate::app::ETabView::Properties,
                "Properties",
            );

            ui.selectable_value(
                &mut self.current_tab_view,
                crate::app::ETabView::Settings,
                "Settings",
            );
        });

        match self.current_tab_view {
            crate::app::ETabView::Plugins => {
                self.plugins_view(ui);
            }
            crate::app::ETabView::Downloads => {
                self.downloads_view(ui);
            }
            crate::app::ETabView::Properties => {
                self.properties_view(ui);
            }
            crate::app::ETabView::Settings => {
                self.settings_view(ui);
            }
        }
    }

    /// list of plugins
    pub fn plugins_view(&mut self, ui: &mut egui::Ui) {
        ui.label("TODO");
    }

    /// list of mod packages
    pub fn downloads_view(&mut self, ui: &mut egui::Ui) {
        // library folder path
        ui.horizontal(|ui| {
            if let Some(p) = self.downloads_library.clone() {
                ui.label(p.as_str());
            } else {
                ui.label("Choose library path ...");
            }
            if ui.button("...").clicked() {
                // TODO pick folder
                self.downloads_library = Some("/Users/ghost/Documents/omwmm/downloads".into());
            }
        });

        ui.separator();

        // downloads list
        if let Some(library_path) = self.downloads_library.clone() {
            // refresh downloads list
            if self.downloads.is_empty() {
                self.refresh_downloads(library_path);
            }
            // populate list
            egui::ScrollArea::vertical().show(ui, |ui| {
                // TODO use table
                for path in self.downloads.iter() {
                    // create viewmodel
                    if let Some(filename) = path.file_name() {
                        if ui
                            .add(
                                egui::Label::new(filename.to_string_lossy())
                                    .sense(egui::Sense::click()),
                            )
                            .double_clicked()
                        {
                            // install mod
                            // extract to mod lib
                            // add to mods
                            if let Some(library) = self.mods_library.clone() {
                                let mut install_path = Path::new(library.as_str()).join(filename);

                                install_path.set_extension("");
                                let mod_info = ModViewModel {
                                    enabled: false,
                                    full_name: install_path.clone(),
                                };

                                if !self.mods.iter().any(|e| e.full_name == install_path) {
                                    // TODO install mod

                                    self.mods.push(mod_info);
                                    self.toasts
                                        .success("Mod installed")
                                        .set_duration(Some(Duration::from_secs(3)));
                                }
                            } else {
                                // TODO log
                            }
                        }
                    }
                }
            });
        }

        // plugin view

        // TODO plugin view
        // plugins are assembled from the enabled mods
        // and can still individually be enabled
        // the enabled state is synced to the omw.cfg
        // TODO caching to avoid IO reads per frame?
    }

    /// mod property view
    pub fn properties_view(&mut self, ui: &mut egui::Ui) {
        ui.label("TODO");
    }

    /// app settings view
    pub fn settings_view(&mut self, ui: &mut egui::Ui) {
        ui.label("TODO");
    }
}
